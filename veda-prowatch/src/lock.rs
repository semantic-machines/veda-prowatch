use crate::common::{
    create_asc_record, get_badge_use_request_indv, get_custom_badge_as_list, get_individual_from_predicate, get_now_00_00_00, get_str_from_value,
    i64_to_str_date_ymdthms, send_message_of_status_lock_unlock, set_card_status, str_date_to_i64, Context, PassType,
};
use prowatch_client::apis::Error;
use v_common::module::veda_backend::Backend;
use v_common::onto::individual::Individual;
use v_common::v_api::api_client::IndvOp;
use v_common::v_api::obj::ResultCode;

pub fn lock_holder(module: &mut Backend, ctx: &mut Context, pass_type: PassType, indv_s: &mut Individual) -> Result<(), (ResultCode, String)> {
    info!("@ indv={:?}", indv_s.to_string());
    let (_, wbadges) = get_badge_use_request_indv(module, ctx, Some(pass_type.clone()), indv_s);
    if let Err(e) = wbadges {
        error!("badges: err={:?}", e);
        return match e {
            Error::Io(_) => return Err((ResultCode::ConnectError, format!("not found, err={:?}", e))),
            _ => Err((ResultCode::UnprocessableEntity, format!("not found, err={:?}", e))),
        };
    }

    let tax_id = indv_s.get_first_literal("v-s:taxId").unwrap_or_default();
    let birthday = i64_to_str_date_ymdthms(indv_s.get_first_datetime("v-s:birthday"));

    let mut asc_indvs = vec![];
    info!("@ badges={:?}", wbadges);
    for badge in wbadges.unwrap() {
        let fields = get_custom_badge_as_list(&badge);

        let mut is_next = false;

        if pass_type == PassType::Human {
            if let Some(jv) = fields.get("BADGE_BIRTHDATE") {
                if let Some(v) = jv.as_str() {
                    if v == birthday {
                        //warn!("fields={:?}", fields);
                        is_next = true;
                    }
                }
            }
        } else {
            is_next = true;
        }

        let mut valid_cards = vec![];
        info!("@ badge={:?}", badge);
        if let Some(v) = badge.get("BadgeID") {
            info!("@ [BadgeID]={:?}", v);
            if let Some(badge_id) = v.as_str() {
                info!("@ badge_id={:?}", badge_id);
                if let Ok(list) = ctx.pw_api_client.badging_api().badges_badge_id_cards(badge_id) {
                    info!("@ list={:?}", list);
                    for card in list {
                        info!("@ card={:?}", card);

                        if let Some(s) = card.get("CardStatus") {
                            info!("@ [CardStatus]={:?}", s);
                            if let Some(status) = s.as_i64() {
                                info!("@ status={:?}", status);
                                if !(status == 2 || status == 3 || status == 5 || status == 6) {
                                    if let Some(s) = card.get("CardNumber") {
                                        info!("@ [CardNumber]={:?}",s);
                                        valid_cards.push(s.as_str().unwrap_or_default().to_owned());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if !valid_cards.is_empty() {
            if let Some(btaxid) = fields.get("BADGE_COMPANY_ID") {
                if let Some(t) = btaxid.as_str() {
                    if t == tax_id && is_next {
                        let acs_record = create_asc_record(&badge, indv_s.get_id(), valid_cards);
                        asc_indvs.push(acs_record);
                    }
                }
            }
        }
    }

        if asc_indvs.is_empty() {
            return Err((ResultCode::Ok, "Держатель не найден".to_owned()));
        }

        for el in asc_indvs.iter_mut() {
            if module.mstorage_api.update_use_param(&ctx.sys_ticket, "prowatch", "", 0, IndvOp::Put, el).is_ok() {
                info!("success update, uri={}", el.get_id());
            } else {
                return Err((ResultCode::DatabaseModifiedError, format!("fail update, uri={}", el.get_id())));
            }
        }


    Ok(())
}

pub fn lock_unlock_card(backend: &mut Backend, ctx: &mut Context, indv_e: &mut Individual, need_lock: bool) -> Result<(), (ResultCode, String)> {
    let mut indv_r = get_individual_from_predicate(backend, indv_e, "v-s:backwardTarget")?;
    let r_type = indv_r.get_first_literal("rdf:type").unwrap_or_default();
    let mut count_prepared_card = 0;
    let mut indv_p = Individual::default();

    if r_type == "mnd-s:ACSRecord" {
        if let Ok(mut indv_x) = get_individual_from_predicate(backend, &mut indv_r, "v-s:backwardTarget") {
            if let Some(indv_p_id) = indv_x.get_first_literal("v-s:backwardTarget") {
                if let Some(badge_id) = indv_r.get_first_literal("mnd-s:winpakCardRecordId") {
                    for el in ctx.pw_api_client.badging_api().badges_badge_id_cards(&badge_id).unwrap_or_default() {
                        if let Some(card_number) = get_str_from_value(&el, "CardNumber") {
                            if need_lock {
                                set_card_status(ctx, card_number, 1)?;
                                count_prepared_card += 1;
                            } else {
                                let s_expire_date = get_str_from_value(&el, "ExpireDate").unwrap_or_default();
                                if let Some(expire_date) = str_date_to_i64(s_expire_date, None) {
                                    if expire_date > get_now_00_00_00().timestamp() {
                                        set_card_status(ctx, card_number, 0)?;
                                        count_prepared_card += 1;
                                    }
                                } else {
                                    return Err((ResultCode::Ok, format!("lock_card: fail parse expire_date={}, card_number={} ", s_expire_date, card_number)));
                                }
                            }
                        }
                    }

                    if let Some(mut upd_indv) = backend.get_individual(&indv_p_id, &mut indv_p) {
                        upd_indv.parse_all();
                        upd_indv.remove("v-s:hasStatus");
                        if count_prepared_card > 0 {
                            if need_lock {
                                upd_indv.set_uri("v-s:hasStatus", "v-s:StatusLocked");
                            } else {
                                upd_indv.set_uri("v-s:hasStatus", "v-s:StatusUnlocked");
                            }
                        } else {
                            if !need_lock {
                                upd_indv.set_uri("v-s:hasStatus", "mnd-s:StatusProcessedWithoutCard");
                            } else {
                                upd_indv.set_uri("v-s:hasStatus", "v-s:StatusLocked");
                            }
                        }

                        if !upd_indv.is_empty() {
                            if backend.mstorage_api.update_use_param(&ctx.sys_ticket, "prowatch", "", 0, IndvOp::Put, &mut upd_indv).is_ok() {
                                info!("success update, uri={}", upd_indv.get_id());
                            } else {
                                return Err((ResultCode::DatabaseModifiedError, format!("fail update, uri={}", upd_indv.get_id())));
                            }
                        }
                    }

                    indv_e.add_uri("v-s:backwardTarget", &indv_p_id);
                } else {
                    return Err((ResultCode::Ok, format!("lock_card: not found mnd-s:winpakCardRecordId in {}", indv_r.get_id())));
                }
            } else {
                return Err((ResultCode::Ok, format!("lock_card: fail to read indv v-s:backwardTarget in {}", indv_x.get_id())));
            }
        } else {
            return Err((ResultCode::Ok, format!("lock_card: fail to read indv v-s:backwardTarget in {}", indv_r.get_id())));
        }
    } else {
        return Err((ResultCode::Ok, format!("rdf:type is not mnd-s:ACSRecord, {}", indv_e.get_id())));
    }

    send_message_of_status_lock_unlock(ctx, backend, &mut indv_p, need_lock)?;

    Ok(())
}
