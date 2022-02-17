use crate::common::{
    create_asc_record, get_badge_use_request_indv, get_custom_badge_as_list, get_individual_from_predicate, get_int_from_value, get_str_from_value, pw_photo_to_veda,
    set_badge_to_indv, str_value2indv, Context, CARD_NUMBER_FIELD_NAME,
};
use prowatch_client::apis::Error;
use serde_json::Value;
use v_common::az_impl::common::f_authorize;
use v_common::module::veda_backend::Backend;
use v_common::onto::datatype::Lang;
use v_common::onto::individual::Individual;
use v_common::v_api::api_client::IndvOp;
use v_common::v_api::obj::ResultCode;
use v_common::v_authorization::common::Trace;

pub fn sync_data_from_prowatch(backend: &mut Backend, ctx: &mut Context, src_indv: &mut Individual) -> Result<(), (ResultCode, String)> {
    src_indv.parse_all();
    let mut asc_indvs = vec![];

    if src_indv.get_first_literal("mnd-s:hasPassKind").is_some() {
        let res_badge = get_badge_use_request_indv(backend, ctx, None, src_indv);
        if let Err(e) = res_badge.1 {
            return match e {
                Error::Io(_) => Err((ResultCode::ConnectError, "Карта не найдена".to_owned())),
                _ => Err((ResultCode::UnprocessableEntity, "Карта не найдена".to_owned())),
            };
        }

        src_indv.clear("mnd-s:hasACSRecord");
        for el in res_badge.1.unwrap_or_default() {
            let mut acs_record = create_asc_record(&el, src_indv.get_id(), vec![]);

            if let Some(badge_id) = acs_record.get_first_literal("mnd-s:winpakCardRecordId") {
                acs_record.clear("mnd-s:cardNumber");
                for el1 in ctx.pw_api_client.badging_api().badges_badge_id_cards(&badge_id).unwrap_or_default() {
                    str_value2indv(&el1, "CardNumber", &mut acs_record, "mnd-s:cardNumber");
                }

                pw_photo_to_veda(backend, ctx, &badge_id, &mut acs_record);
            }

            asc_indvs.push(acs_record);
        }
    } else {
        let card_number = src_indv.get_first_literal(CARD_NUMBER_FIELD_NAME).unwrap_or(String::default());
        if card_number.is_empty() {
            error!("fail read {}.{}", CARD_NUMBER_FIELD_NAME, src_indv.get_id());
            return Err((ResultCode::UnprocessableEntity, "".to_owned()));
        }

        let res_card = ctx.pw_api_client.badging_api().badges_cards_card(&card_number);
        if let Err(e) = res_card {
            error!("badges_cards_card: err={:?}", e);
            return match e {
                Error::Reqwest(_) => Err((ResultCode::UnprocessableEntity, "Карта не найдена".to_owned())),
                Error::Serde(_) => Err((ResultCode::UnprocessableEntity, "Карта не найдена".to_owned())),
                Error::Io(_) => Err((ResultCode::ConnectError, "Карта не найдена".to_owned())),
            };
        }

        let res_badge = ctx.pw_api_client.badging_api().badges_cards(&card_number);
        if let Err(e) = res_badge {
            error!("badges_cards: err={:?}", e);
            return match e {
                Error::Io(_) => Err((ResultCode::ConnectError, "Карта не найдена".to_owned())),
                _ => Err((ResultCode::UnprocessableEntity, "Карта не найдена".to_owned())),
            };
        }
        let res_badge = res_badge.unwrap_or_default();

        let card = res_card.unwrap();
        if !card.is_object() {
            return Ok(());
        }
        if let Some(s) = get_str_from_value(&card, "CardNumber") {
            if s != card_number {
                error!("fail read {}.{}, request card number not equal response", CARD_NUMBER_FIELD_NAME, src_indv.get_id());
                return Err((ResultCode::UnprocessableEntity, "".to_owned()));
            }
        }

        /*
        Перед дальнейшими действиями требуется проверить соответствие организации сотрудника указанной в PW и пользователя создающего запрос:
        [инн_организации_в_PW] = CustomBadgeFields.BADGE_COMPANY_ID (параметр из CustomBadgeFields запроса держателя)
        [инн_пользователя] = [S].["v-s:creator"]["v-s:parentOrganization"]["v-s:taxId"] + [S].["v-s:creator"]["v-s:parentOrganization"]["v-s:hasContractorProfileSafety"]["mnd-s:subContractor"]["v-s:taxId"] ("mnd-s:subContractor" множественное)
        Если в перечне значений [инн_пользователя] нет [инн_организации_в_PW], то запрос отклоняется с v-s:errorMessage = "Информация о пропуске недоступна"
        */

        if let Some(badge) = res_badge.get(0) {
            if !check_company(&badge, backend, src_indv) {
                return Err((ResultCode::UnprocessableEntity, "Информация о пропуске недоступна".to_owned()));
            }
        }
        set_card_to_indv(card, src_indv, ctx);
        if let Some(badge) = res_badge.get(0) {
            set_badge_to_indv(badge, src_indv);
        }
    }

    src_indv.set_uri("v-s:lastEditor", "cfg:VedaSystemAppointment");

    match backend.mstorage_api.update_use_param(&ctx.sys_ticket, "prowatch", "", 0, IndvOp::Put, src_indv) {
        Ok(_) => {
            info!("success update 0, uri={}", src_indv.get_id());
        },
        Err(e) => {
            error!("fail update, uri={}, result_code={:?}", src_indv.get_id(), e.result);
            return Err((ResultCode::DatabaseModifiedError, "".to_owned()));
        },
    }

    for el in asc_indvs.iter_mut() {
        match backend.mstorage_api.update_use_param(&ctx.sys_ticket, "prowatch", "", 0, IndvOp::Put, el) {
            Ok(_) => {
                info!("success update 1, uri={}", src_indv.get_id());
            },
            Err(e) => {
                error!("fail update, uri={}, result_code={:?}", src_indv.get_id(), e.result);
                return Err((ResultCode::DatabaseModifiedError, "".to_owned()));
            },
        }
    }

    return Ok(());
}

fn set_card_to_indv(card: Value, indv: &mut Individual, ctx: &Context) {
    if let Some(d) = card.get("IssueDate") {
        indv.clear("v-s:dateFrom");
        let sd = d.as_str().unwrap_or_default();
        if sd.len() > 20 {
            indv.add_datetime_from_str("v-s:dateFrom", sd.split("T").next().unwrap_or_default());
        } else {
            indv.add_datetime_from_str("v-s:dateFrom", sd);
        }
    }

    if let Some(n) = get_int_from_value(&card, "CardStatus") {
        /*
        Вписать в [S]:
        mnd-s:cardStatus (текст) = на основе поля CardStatus заполнить статус:
            0 - "Активна"
            1 - "Отключена"
            2 - "Утеряна"
            3 - "Украдена"
            4 - "Сдана"
            5 - "Неучтенная"
            6 - "Аннулированная"
            7 - "Истек срок действия"
            8 - "Авто откл."
         */
        let s = match n {
            0 => "Активна",
            1 => "Отключена",
            2 => "Утеряна",
            3 => "Украдена",
            4 => "Сдана",
            5 => "Неучтенная",
            6 => "Аннулированная",
            7 => "Истек срок действия",
            8 => "Авто откл.",
            _ => "?",
        };
        indv.set_string("mnd-s:cardStatus", s, Lang::RU);
    }

    if let Some(d) = card.get("ExpireDate") {
        indv.clear("v-s:dateTo");
        indv.add_datetime_from_str("v-s:dateTo", d.as_str().unwrap_or_default());
    }

    if let Some(v) = card.get("ClearanceCodes") {
        if v.is_array() {
            for c_el in v.as_array().unwrap_or(&vec![]) {
                if let Some(v) = c_el.get("ClearCode") {
                    if let Some(v) = v.get("ClearCodeID") {
                        if let Some(v) = v.as_str() {
                            if let Some(access_level_id) = ctx.access_level_dict.get(v) {
                                indv.add_uri("mnd-s:hasAccessLevel", access_level_id);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_user_in_group(user_id: &str, group: &str) -> bool {
    let mut trace = Trace {
        acl: &mut String::new(),
        is_acl: false,
        group: &mut String::new(),
        is_group: true,
        info: &mut String::new(),
        is_info: false,
        str_num: 0,
    };

    match f_authorize(user_id, user_id, 15, true, Some(&mut trace)) {
        Ok(_res) => {
            for gr in trace.group.split('\n') {
                if gr == group {
                    return true;
                }
            }
        },
        Err(e) => error!("failed check user in group [{}], user = {}, err = {}", group, &user_id, e),
    }
    return false;
}

fn check_company(card: &Value, backend: &mut Backend, src_indv: &mut Individual) -> bool {
    if let Some(v) = get_custom_badge_as_list(&card).get("BADGE_COMPANY_ID") {
        if let Some(company_id) = v.as_str() {
            if company_id == "111111111111" || company_id == "000000000000" {
                return true;
            }
            if let Ok(mut indv1) = get_individual_from_predicate(backend, src_indv, "v-s:creator") {
                if is_user_in_group(indv1.get_id(), "mnd-s:AllAccessPW_Group") {
                    return true;
                }
                if let Ok(mut indv2) = get_individual_from_predicate(backend, &mut indv1, "v-s:parentOrganization") {
                    if indv2.get_first_literal("v-s:taxId").unwrap_or_default() == company_id {
                        return true;
                    }

                    if let Ok(mut indv3) = get_individual_from_predicate(backend, &mut indv2, "v-s:hasContractorProfileSafety") {
                        if let Some(indv4uris) = indv3.get_literals("mnd-s:subContractor") {
                            for id in indv4uris {
                                if let Some(mut indv5) = backend.get_individual_s(&id) {
                                    if indv5.get_first_literal("v-s:taxId").unwrap_or_default() == company_id {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return false;
}
