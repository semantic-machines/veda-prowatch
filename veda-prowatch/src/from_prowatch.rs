use prowatch_client::apis::Error;
use serde_json::Value;

use crate::common::{
    clear_card_and_set_err, create_asc_record, get_badge_use_request_indv, get_str_from_value, pw_photo_to_veda, set_badge_to_indv, str_value2indv, Context,
    CARD_NUMBER_FIELD_NAME,
};
use v_common::module::veda_backend::Backend;
use v_common::onto::individual::Individual;
use v_common::v_api::api_client::IndvOp;
use v_common::v_api::obj::ResultCode;

pub fn sync_data_from_prowatch(module: &mut Backend, ctx: &mut Context, src_indv: &mut Individual) -> ResultCode {
    src_indv.parse_all();
    let mut asc_indvs = vec![];

    if src_indv.get_first_literal("mnd-s:hasPassKind").is_some() {
        let res_badge = get_badge_use_request_indv(module, ctx, None, src_indv);
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
            let mut acs_record = create_asc_record(&el, src_indv.get_id(), vec![]);

            if let Some(badge_id) = acs_record.get_first_literal("mnd-s:winpakCardRecordId") {
                acs_record.clear("mnd-s:cardNumber");
                for el1 in ctx.pw_api_client.badging_api().badges_badge_id_cards(&badge_id).unwrap_or_default() {
                    str_value2indv(&el1, "CardNumber", &mut acs_record, "mnd-s:cardNumber");
                }

                pw_photo_to_veda(module, ctx, &badge_id, &mut acs_record);
            }

            asc_indvs.push(acs_record);
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

    match module.mstorage_api.update_use_param(&ctx.sys_ticket, "prowatch", "", 0, IndvOp::Put, src_indv) {
        Ok(_) => {
            info!("success update, uri={}", src_indv.get_id());
        }
        Err(e) => {
            error!("fail update, uri={}, result_code={:?}", src_indv.get_id(), e.result);
            return ResultCode::DatabaseModifiedError;
        }
    }

    for el in asc_indvs.iter_mut() {
        match module.mstorage_api.update_use_param(&ctx.sys_ticket, "prowatch", "", 0, IndvOp::Put, el) {
            Ok(_) => {
                info!("success update, uri={}", src_indv.get_id());
            }
            Err(e) => {
                error!("fail update, uri={}, result_code={:?}", src_indv.get_id(), e.result);
                return ResultCode::DatabaseModifiedError;
            }
        }
    }

    return ResultCode::Ok;
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
