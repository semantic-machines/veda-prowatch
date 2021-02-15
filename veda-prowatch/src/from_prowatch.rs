use prowatch_client::apis::Error;
use serde_json::{Map, Value};
use uuid::Uuid;
use v_module::v_api::app::ResultCode;
use v_module::v_api::*;
use v_module::module::*;
use v_module::v_onto::datatype::Lang;
use v_module::v_onto::individual::*;

use crate::common::{
    clear_card_and_set_err, concat_fields, get_badge_use_request_indv, get_int_from_value, get_str_from_value, str_value2indv, Context, CARD_NUMBER_FIELD_NAME,
};

pub fn sync_data_from_prowatch(module: &mut Module, ctx: &mut Context, src_indv: &mut Individual) -> ResultCode {
    let mut asc_indvs = vec![];

    if src_indv.get_first_literal("mnd-s:hasPassKind").is_some() {
        let res_badge = get_badge_use_request_indv(module, ctx, src_indv);
        if let Err(e) = res_badge.1 {
            error!("badges: err={:?}", e);
            clear_card_and_set_err(module, &ctx.sys_ticket, src_indv, "Карта не найдена");
            return match e {
                Error::Io(_) => ResultCode::ConnectError,
                _ => ResultCode::UnprocessableEntity,
            };
        }

        src_indv.clear("mnd-s:hasACSRecord");
        for el in res_badge.1.unwrap_or_default() {
            let mut acs_record = Individual::default();
            acs_record.set_id(&("d:asc_".to_owned() + &Uuid::new_v4().to_string()));
            acs_record.set_uri("rdf:type", "mnd-s:ACSRecord");
            acs_record.set_uri("v-s:backwardProperty", "mnd-s:hasACSRecord");
            acs_record.set_uri("v-s:backwardTarget", src_indv.get_id());
            acs_record.set_bool("v-s:canRead", true);

            set_badge_to_indv(&el, &mut acs_record);

            if let Some(badge_id) = acs_record.get_first_literal("mnd-s:winpakCardRecordId") {
                acs_record.clear("mnd-s:cardNumber");
                for el1 in ctx.pw_api_client.badging_api().badges_badge_id_cards(&badge_id).unwrap_or_default() {
                    if let Some(s) = get_int_from_value(&el1, "CardStatus") {
                        if s == 0 {
                            str_value2indv(&el1, "CardNumber", &mut acs_record, "mnd-s:cardNumber");
                        }
                    }
                }

                asc_indvs.push(acs_record);
            }
        }
    } else {
        let card_number = src_indv.get_first_literal(CARD_NUMBER_FIELD_NAME).unwrap_or(String::default());
        if card_number.is_empty() {
            error!("fail read {}.{}", CARD_NUMBER_FIELD_NAME, src_indv.get_id());
            return ResultCode::UnprocessableEntity;
        }

        let res_card = ctx.pw_api_client.badging_api().badges_cards_card(&card_number);
        if let Err(e) = res_card {
            error!("badges_cards_card: err={:?}", e);
            clear_card_and_set_err(module, &ctx.sys_ticket, src_indv, "Карта не найдена");
            return match e {
                Error::Reqwest(_) => ResultCode::UnprocessableEntity,
                Error::Serde(_) => ResultCode::UnprocessableEntity,
                Error::Io(_) => ResultCode::ConnectError,
            };
        }

        let res_badge = ctx.pw_api_client.badging_api().badges_cards(&card_number);
        if let Err(e) = res_badge {
            error!("badges_cards: err={:?}", e);
            clear_card_and_set_err(module, &ctx.sys_ticket, src_indv, "Карта не найдена");
            return match e {
                Error::Io(_) => ResultCode::ConnectError,
                _ => ResultCode::UnprocessableEntity,
            };
        }

        let card = res_card.unwrap();
        if !card.is_object() {
            return ResultCode::Ok;
        }
        if let Some(s) = get_str_from_value(&card, "CardNumber") {
            if s != card_number {
                error!("fail read {}.{}, request card number not equal response", CARD_NUMBER_FIELD_NAME, src_indv.get_id());
                return ResultCode::UnprocessableEntity;
            }
        }

        set_card_to_indv(card, src_indv, ctx);
        if let Some(badge) = res_badge.unwrap_or_default().get(0) {
            set_badge_to_indv(badge, src_indv);
        }
    }

    src_indv.set_uri("v-s:lastEditor", "cfg:VedaSystemAppointment");

    let res = module.api.update(&ctx.sys_ticket, IndvOp::Put, src_indv);
    if res.result != ResultCode::Ok {
        error!("fail update, uri={}, result_code={:?}", src_indv.get_id(), res.result);
        return ResultCode::DatabaseModifiedError;
    } else {
        info!("success update, uri={}", src_indv.get_id());
    }

    for el in asc_indvs.iter_mut() {
        let res = module.api.update(&ctx.sys_ticket, IndvOp::Put, el);
        if res.result == ResultCode::Ok {
            info!("success update, uri={}", el.get_id());
        } else {
            error!("fail update, uri={}, result_code={:?}", el.get_id(), res.result);
            return ResultCode::DatabaseModifiedError;
        }
    }

    return ResultCode::Ok;
}

fn set_card_to_indv(card: Value, indv: &mut Individual, ctx: &Context) {
    if let Some(d) = card.get("IssueDate") {
        indv.clear("v-s:dateFrom");
        indv.add_datetime_from_str("v-s:dateFrom", d.as_str().unwrap_or_default());
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

fn set_badge_to_indv(el: &Value, dest: &mut Individual) {
    if !el.is_object() {
        return;
    }

    if let Some(v) = el.get("BadgeID") {
        if let Some(s) = v.as_str() {
            dest.set_string("mnd-s:winpakCardRecordId", &s, Lang::NONE);
        }
    }

    if let Some(s) = concat_fields(&["LastName", "FirstName", "MiddleName"], el.as_object(), " ") {
        dest.set_string("v-s:description", &s, Lang::RU);
    }

    let mut fields: Map<String, Value> = Map::default();
    if let Some(v) = el.get("CustomBadgeFields") {
        if let Some(ar) = v.as_array() {
            for c_el in ar {
                let mut field_type = 0;
                if let Some(t) = c_el.get("FieldType") {
                    if let Some(d) = t.as_i64() {
                        field_type = d;
                    }
                }

                if let Some(cn) = c_el.get("ColumnName") {
                    let f_name = cn.as_str().unwrap_or_default();

                    if field_type == 2 {
                        if let Some(v) = c_el.get("TextValue") {
                            fields.insert(f_name.to_owned(), v.to_owned());
                        }
                    } else if field_type == 0 {
                        if let Some(v) = c_el.get("DateValue") {
                            fields.insert(f_name.to_owned(), v.to_owned());
                        }
                    }
                }
            }
        }
    }

    if let Some(s) = concat_fields(&["BADGE_COMPANY_NAME", "BADGE_DEPARTMENT", "BADGE_TITLE"], Some(&fields), " ") {
        dest.set_string("rdfs:comment", &s, Lang::RU);
    }

    if let Some(d) = fields.get("BADGE_BIRTHDATE") {
        dest.clear("v-s:birthday");
        if let Some(d) = d.as_str() {
            dest.add_datetime_from_str("v-s:birthday", d);
        }
    }

    if let Some(v) = fields.get("BADGE_ID") {
        if let Some(s) = v.as_str() {
            dest.set_string("v-s:tabNumber", &s, Lang::NONE);
        }
    }

    if let Some(d) = fields.get("BADGE_SAFETY_INST_DATE") {
        dest.clear("mnd-s:briefingDate");
        if let Some(d) = d.as_str() {
            dest.add_datetime_from_str("mnd-s:briefingDate", d);
        }
    }

    if let Some(s) = concat_fields(
        &[
            "BADGE_TOOL01",
            "BADGE_TOOL02",
            "BADGE_TOOL03",
            "BADGE_TOOL04",
            "BADGE_TOOL05",
            "BADGE_TOOL06",
            "BADGE_TOOL07",
            "BADGE_TOOL08",
            "BADGE_TOOL09",
            "BADGE_TOOL10",
            "BADGE_TOOL11",
        ],
        Some(&fields),
        "\n",
    ) {
        dest.set_string("mnd-s:passEquipment", &s, Lang::RU);
    }
}
