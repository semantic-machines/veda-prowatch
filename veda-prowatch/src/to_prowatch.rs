use crate::common::*;
use base64::encode;
use serde_json::json;
use std::fs;
use v_module::module::Module;
use v_module::v_api::app::ResultCode;
use v_module::v_api::IndvOp;
use v_module::v_onto::datatype::Lang;
use v_module::v_onto::individual::Individual;

pub fn insert_to_prowatch(module: &mut Module, ctx: &mut Context, indv: &mut Individual) -> Result<(), (ResultCode, String)> {
    let mut new_badge_id = None;
    let backward_target = indv.get_first_literal("v-s:backwardTarget");
    if backward_target.is_none() {
        error!("not found [v-s:backwardTarget] in {}", indv.get_id());
        return Err((ResultCode::FailStore, format!("not found [v-s:backwardTarget] in {}", indv.get_id())));
    }
    let backward_target = backward_target.unwrap();
    let indv_b = module.get_individual_h(&backward_target);
    if indv_b.is_none() {
        error!("not found {}", &backward_target);
        return Err((ResultCode::FailStore, format!("not found {}", &backward_target)));
    }
    let indv_p = &mut indv_b.unwrap();
    let pass_type = get_pass_type(indv_p);

    let mut first_name = String::new();
    let mut last_name = String::new();
    let mut middle_name = String::new();
    let date_to = indv_p.get_first_datetime("v-s:dateToFact");
    let mut custom_fields = vec![];

    if pass_type == PassType::Human {
        if let Some(cp_id) = indv_p.get_first_literal("v-s:correspondentPerson") {
            let mut icp = Individual::default();
            if module.get_individual(&cp_id, &mut icp).is_some() {
                if let Some(employee) = module.get_individual(&mut icp.get_first_literal("v-s:employee").unwrap_or_default(), &mut Individual::default()) {
                    first_name = employee.get_first_literal_with_lang("v-s:firstName", &[Lang::RU, Lang::NONE]).unwrap_or_default();
                    last_name = employee.get_first_literal_with_lang("v-s:lastName", &[Lang::RU, Lang::NONE]).unwrap_or_default();
                    middle_name = employee.get_first_literal_with_lang("v-s:middleName", &[Lang::RU, Lang::NONE]).unwrap_or_default();
                    add_txt_to_fields(&mut custom_fields, "BADGE_TITLE", get_literal_of_link(module, &mut icp, "v-s:occupation", "rdfs:label"));
                    add_txt_to_fields(&mut custom_fields, "BADGE_DEPARTMENT", get_literal_of_link(module, &mut icp, "v-s:parentUnit", "rdfs:label"));
                    add_date_to_fields(&mut custom_fields, "BADGE_BIRTHDATE", employee.get_first_datetime("v-s:birthday"));
                }
            } else {
                error!("add human, invalid link v-s:correspondentPerson = {}", cp_id);
                return Err((ResultCode::BadRequest, format!("add human, invalid link v-s:correspondentPerson = {}", cp_id)));
            }
        } else {
            first_name = indv_p.get_first_literal("mnd-s:passFirstName").unwrap_or_default();
            last_name = indv_p.get_first_literal("mnd-s:passLastName").unwrap_or_default();
            middle_name = indv_p.get_first_literal("mnd-s:passMiddleName").unwrap_or_default();
            add_txt_to_fields(&mut custom_fields, "BADGE_TITLE", indv_p.get_first_literal("mnd-s:passPosition"));
            add_date_to_fields(&mut custom_fields, "BADGE_BIRTHDATE", indv_p.get_first_datetime("v-s:birthday"));
        }
    } else if pass_type == PassType::Vehicle {
        first_name = indv_p.get_first_literal("mnd-s:passVehicleRegistrationNumber").unwrap_or_default()
            + " "
            + &get_literal_of_link(module, indv_p, "v-s:hasVehicleModel", "rdfs:label").unwrap_or_default();
        last_name = ".".to_owned();
        middle_name = ".".to_owned();
        add_txt_to_fields(&mut custom_fields, "BADGE_CAR_PLATE", Some(first_name.to_owned()));
    } else if pass_type == PassType::Unknown {
        error!("not found {}", &backward_target);
        return Err((ResultCode::BadRequest, "unknown pass type".to_owned()));
    }

    equipment_to_field_list(&mut custom_fields, indv_p);
    add_txt_to_fields(&mut custom_fields, "BADGE_STATE_NAME", get_literal_of_link(module, indv_p, "mnd-s:hasPassKind", "rdfs:label"));
    add_txt_to_fields(&mut custom_fields, "BADGE_CARD", indv_p.get_first_literal("mnd-s:cardNumber"));
    add_txt_to_fields(&mut custom_fields, "BADGE_COMPANY_ID", get_literal_of_link(module, indv_p, "v-s:correspondentOrganization", "v-s:taxId"));
    add_txt_to_fields(&mut custom_fields, "BADGE_SUBDIVISION_ID", get_literal_of_link(module, indv_p, "v-s:supplier", "v-s:taxId"));
    add_txt_to_fields(&mut custom_fields, "BADGE_SUBDIVISION_NAME", get_literal_of_link(module, indv_p, "v-s:supplier", "rdfs:label"));
    add_txt_to_fields(&mut custom_fields, "BADGE_COMPANY_NAME", get_literal_of_link(module, indv_p, "v-s:correspondentOrganization", "rdfs:label"));
    add_txt_to_fields(&mut custom_fields, "BADGE_CLEARANCE_ORDER_DATE", Some(i64_to_str_date_mdy(Some(get_now_00_00_00().timestamp()))));
    add_txt_to_fields(&mut custom_fields, "BADGE_FNAME", Some(first_name.to_owned()));
    add_txt_to_fields(&mut custom_fields, "BADGE_LNAME", Some(last_name.to_owned()));

    let reqj = json!({
        "LastName": last_name,
        "FirstName": first_name,
        "MiddleName": middle_name,
        "ExpireDate": i64_to_str_date_ymdthms(date_to),
        "CustomBadgeFields": custom_fields
    });

    match ctx.pw_api_client.badging_api().badges_post(reqj) {
        Err(e) => {
            error!("not found [v-s:backwardTarget] in {}", indv.get_id());
            return Err((ResultCode::FailStore, format!("add {:?} data: badges_put: err={:?}", pass_type, e)));
        }
        Ok(r) => {
            if let Some(o) = r.as_object() {
                if let Some(id) = o.get("BadgeID") {
                    if let Some(s) = id.as_str() {
                        info!("success add {:?} data, id={}, new badge id = {}", pass_type, indv_p.get_id(), s);
                        new_badge_id = Some(s.to_owned());
                    }
                }
            }
        }
    }

    if new_badge_id.is_none() {
        error!("fail store badge");
        return Err((ResultCode::FailStore, "fail store badge".to_owned()));
    }
    let badge_id = new_badge_id.unwrap();

    add_photo_to_pw(module, ctx, &badge_id, indv_p);

    add_card_with_access_to_pw(module, ctx, &badge_id, indv_p)
}

pub fn set_update_status(module: &mut Module, ctx: &mut Context, indv: &mut Individual, res: Result<(), (ResultCode, String)>) -> ResultCode {
    if let Err((sync_res, info)) = res {
        if sync_res == ResultCode::ConnectError {
            return sync_res;
        }
        indv.set_uri("v-s:hasStatus", "v-s:StatusRejected");
        set_err(module, &ctx.sys_ticket, indv, &info);
        return sync_res;
    }

    indv.set_uri("v-s:lastEditor", "cfg:VedaSystemAppointment");
    indv.set_uri("v-s:hasStatus", "v-s:StatusAccepted");
    indv.clear("v-s:errorMessage");

    let res = module.api.update(&ctx.sys_ticket, IndvOp::Put, indv);
    if res.result != ResultCode::Ok {
        error!("fail update, uri={}, result_code={:?}", indv.get_id(), res.result);
    } else {
        info!("success update, uri={}", indv.get_id());
    }
    ResultCode::Ok
}

pub fn update_prowatch_data(module: &mut Module, ctx: &mut Context, indv: &mut Individual) -> Result<(), (ResultCode, String)> {
    let mut indv_p = get_individual_from_predicate(module, indv, "v-s:backwardTarget")?;
    let p_type = indv_p.get_first_literal("rdf:type").unwrap_or_default();

    if p_type == "mnd-s:ACSRecord" {
        let mut indv_r = indv_p;

        if let Some(badge_id) = indv_r.get_first_literal("mnd-s:winpakCardRecordId") {
            let mut indv_s = get_individual_from_predicate(module, &mut indv_r, "v-s:backwardTarget")?;
            let mut indv_p = get_individual_from_predicate(module, &mut indv_s, "v-s:backwardTarget")?;
            if let Err(e) = add_card_with_access_to_pw(module, ctx, &badge_id, &mut indv_p) {
                error!("update_prowatch_data::add_card_with_access_to_pw, error={:?}", e);
            }

            let mut sj = vec![];
            equipment_to_field_list(&mut sj, &mut indv_p);

            let js = if !indv_r.get_first_literal("mnd-s:cardNumber").unwrap_or_default().is_empty() {
                json!({ "BadgeID": badge_id, "CustomBadgeFields": sj })
            } else {
                let date_to = indv_p.get_first_datetime("v-s:dateToFact");
                json!({ "BadgeID": badge_id, "ExpireDate": i64_to_str_date_ymdthms (date_to), "CustomBadgeFields": sj })
            };

            if let Err(e) = ctx.pw_api_client.badging_api().badges_put(js) {
                error!("badges_put: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            }

            //  отключаем все действующие карты держателя, если пропуск временный
            if let Some(has_kind_for_pass) = indv_p.get_first_literal("mnd-s:hasPassKind") {
                if has_kind_for_pass == "d:a149d268628b46ae8d40c6ea0ac7f3dd" || has_kind_for_pass == "d:228e15d5afe544c099c337ceafa47ea6" {
                    if let Some(v) = indv_r.get_literals("mnd-s:cardNumber") {
                        for card_number in v {
                            let cnj = json!({
                                "CardNumber": card_number,
                                "CardStatus": 1,
                            });
                            if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_put(cnj) {
                                error!("block cards if exist temp card: badges_cards_card: err={:?}", e);
                                return Err((ResultCode::FailStore, format!("{:?}", e)));
                            }
                        }
                    }
                }
            }
            indv.add_uri("v-s:backwardTarget", indv_p.get_id());
        }
    } else {
        let has_change_kind_for_pass = indv_p.get_literals("mnd-s:hasChangeKindForPass");
        if p_type != "mnd-s:Pass" && p_type != "mnd-s:PassChangeRequest" && has_change_kind_for_pass.is_none() {
            error!("not found [mnd-s:hasChangeKindForPass] in {}, type={}", indv_p.get_id(), p_type);
            return Err((ResultCode::NotFound, "исходные данные некорректны".to_owned()));
        }
        let has_change_kind_for_passes = has_change_kind_for_pass.unwrap_or_default();

        let mut data_request_pass = get_individual_from_predicate(module, &mut indv_p, "mnd-s:hasSourceDataRequestForPass")?;
        let wcard_number = data_request_pass.get_first_literal("mnd-s:cardNumber");
        if wcard_number.is_none() {
            error!("not found [mnd-s:cardNumber] in {}", data_request_pass.get_id());
            return Err((ResultCode::NotFound, "исходные данные некорректны".to_owned()));
        }
        let card_number = wcard_number.unwrap();

        let wbadge_id = data_request_pass.get_first_literal("mnd-s:winpakCardRecordId");
        if wbadge_id.is_none() {
            error!("not found [mnd-s:winpakCardRecordId] in {}", data_request_pass.get_id());
            return Err((ResultCode::NotFound, "исходные данные некорректны".to_owned()));
        }
        let badge_id = wbadge_id.unwrap();

        let mut access_levels: Vec<String> = Vec::new();

        let mut is_update_access_levels = false;
        let mut is_tmp_update_access_levels = false;
        let mut is_update_access_levels_without_clean = false;
        let mut is_update_equipment = false;
        let mut is_update_family = false;
        let mut is_update_ts_number = false;
        let mut is_need_block_card = false;
        let mut is_update = false;
        let mut cardholder_family: Option<String> = None;
        let mut ts_number: Option<String> = None;

        if p_type == "mnd-s:Pass" {
            is_update_equipment = true;
            is_update_access_levels = true;
        } else {
            if has_change_kind_for_passes.is_empty() {
                is_update_access_levels = true;
            }

            for has_change_kind_for_pass in has_change_kind_for_passes {
                if has_change_kind_for_pass == "d:lt6pdbhy2qvwquzgnp22jj2r2w" {
                    is_update_equipment = true;
                } else if has_change_kind_for_pass == "d:j2dohw8s79d29mxqwoeut39q92" {
                    is_update = true;
                } else if has_change_kind_for_pass == "d:a5w44zg3l6lwdje9kw09je0wzki" {
                    is_update_access_levels = true;
                } else if has_change_kind_for_pass == "d:e8j2tpz9r613hxq4g4rbbxtfqe" {
                    is_need_block_card = true;
                } else if has_change_kind_for_pass == "d:a8kf3r1ryfotqg695yckpm2isih" {
                    cardholder_family = indv_p.get_first_literal_with_lang("mnd-s:passLastName", &[Lang::RU, Lang::NONE]);
                    is_update_family = true;
                } else if has_change_kind_for_pass == "d:a5y91zferr8t41abib4ecdlggn0" {
                    ts_number = indv_p.get_first_literal("mnd-s:passVehicleRegistrationNumber");
                    is_update_ts_number = true;
                } else if has_change_kind_for_pass == "d:efbibmgvxpr46t1qksmtkkautw" {
                    is_update_access_levels = true;
                    is_update_access_levels_without_clean = true;
                    is_tmp_update_access_levels = true;
                }
            }
        }
        if is_update_ts_number {
            let cnj = json!( {
                                "BadgeID": badge_id,
                                "FirstName": ts_number,
                                "CustomBadgeFields": [
                                    {
                                    "ColumnName": "BADGE_CAR_PLATE",
                                    "TextValue": ts_number
                                    },
                                    {
                                    "ColumnName": "BADGE_FNAME",
                                    "TextValue": ts_number
                                    }
                                ] } );
            if let Err(e) = ctx.pw_api_client.badging_api().badges_put(cnj) {
                error!("update_ts_number: badges_put: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            }
        }

        if is_update_family {
            let cnj = json!({
            "BadgeID": badge_id,
            "LastName": cardholder_family,
            "CustomBadgeFields": [
            {
                "ColumnName": "BADGE_LNAME",
                "TextValue": cardholder_family
            } ]
            });
            if let Err(e) = ctx.pw_api_client.badging_api().badges_put(cnj) {
                error!("update_family: badges_put: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            }
        }

        if is_update_equipment {
            let mut sj = vec![];
            equipment_to_field_list(&mut sj, &mut indv_p);
            if let Err(e) = ctx.pw_api_client.badging_api().badges_put(json!({ "BadgeID": badge_id, "CustomBadgeFields": sj })) {
                error!("badges_put: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            }
        }

        if is_update_access_levels {
            if !is_update_access_levels_without_clean {
                if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_card_delete_access_levels(&card_number) {
                    error!("badges_cards_card_delete_access_levels: err={:?}", e);
                    return Err((ResultCode::FailStore, format!("{:?}", e)));
                }
            }

            if is_tmp_update_access_levels {
                get_access_levels(module, &mut indv_p, "mnd-s:hasTemporaryAccessLevel", &mut access_levels);
            } else {
                get_access_levels(module, &mut indv_p, "mnd-s:hasAccessLevel", &mut access_levels);
            }

            let sj = access_levels_to_json_for_add(access_levels, is_tmp_update_access_levels, indv_p.get_first_datetime("v-s:dateToPlan"));
            if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_card_update_access_levels(&card_number, json!(sj)) {
                error!("badges_cards_card_update_access_levels: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            }
        }

        if is_update || is_need_block_card {
            let date_to = if is_need_block_card {
                Some(get_now_00_00_00().timestamp())
            } else {
                indv_p.get_first_datetime("v-s:dateToFact")
            };

            let status = if is_need_block_card {
                1
            } else {
                0
            };

            let cnj = json!({
                "CardNumber": card_number,
                "CardStatus": status,
                "ExpireDate": i64_to_str_date_ymdthms (date_to)
            });
            if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_put(cnj) {
                error!("badges_cards_card: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            }

            let sj = json!({
                "BadgeID": badge_id,
                "ExpireDate": i64_to_str_date_ymdthms (date_to)
            });
            if let Err(e) = ctx.pw_api_client.badging_api().badges_put(sj) {
                error!("badges_cards: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            }
        }
    }
    return Ok(());
}

pub fn delete_from_prowatch(_module: &mut Module, _ctx: &mut Context, _indv: &mut Individual) -> ResultCode {
    ResultCode::Ok
}

fn add_card_with_access_to_pw(module: &mut Module, ctx: &mut Context, badge_id: &str, src: &mut Individual) -> Result<(), (ResultCode, String)> {
    let mut access_levels = vec![];
    get_access_levels(module, src, "mnd-s:hasAccessLevel", &mut access_levels);
    let sj = access_levels_to_json_for_new(access_levels);

    let wcard_number = src.get_first_literal("mnd-s:cardNumber");
    if wcard_number.is_none() {
        error!("not found [mnd-s:cardNumber] in {}", src.get_id());
        return Ok(());
    }
    let card_number = wcard_number.unwrap();

    if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_card_delete(&card_number) {
        warn!("fail delete card {}, badges_cards_card, err={:?}", card_number, e);
    }

    let cnj = json!({
    "BadgeID": badge_id,
    "CardNumber": card_number,
    "CardStatus": 0,
    "ExpireDate": i64_to_str_date_ymdthms (src.get_first_datetime("v-s:dateToFact")),
    "Company": {
        "CompanyID": "0x004842343236434238382D443536302D3433"
        },
    "ClearanceCodes": sj
     });

    if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_post(cnj) {
        error!("badges_cards_card: err={:?}", e);
        return Err((ResultCode::FailStore, format!("{:?}", e)));
    }

    Ok(())
}

fn add_photo_to_pw(module: &mut Module, ctx: &mut Context, badge_id: &str, src: &mut Individual) {
    if let Ok(mut file) = get_individual_from_predicate(module, src, "v-s:hasImage") {
        info!("extract photo {} from {}", file.get_id(), src.get_id());

        let src_full_path =
            "data/files".to_owned() + &file.get_first_literal("v-s:filePath").unwrap_or_default() + "/" + &file.get_first_literal("v-s:fileUri").unwrap_or_default();

        if let Ok(f) = fs::read(src_full_path) {
            let msg_base64 = encode(f);

            if let Err(e) = ctx.pw_api_client.badging_api().badges_badge_id_photo_post(&badge_id, msg_base64) {
                error!("update_photo: badges_put: err={:?}", e);
            } else {
                info!("success update photo, {}", src.get_id())
            }
        }
    }
}
