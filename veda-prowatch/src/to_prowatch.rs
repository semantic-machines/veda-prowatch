use crate::common::*;
use serde_json::json;
use std::collections::HashSet;
use v_common::module::veda_backend::Backend;
use v_common::onto::datatype::Lang;
use v_common::onto::individual::Individual;
use v_common::v_api::obj::ResultCode;

pub fn insert_to_prowatch(module: &mut Backend, ctx: &mut Context, indv: &mut Individual) -> Result<(), (ResultCode, String)> {
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
    let date_to = set_23_59_59(indv_p.get_first_datetime("v-s:dateToFact"));

    let mut custom_fields = vec![];

    if pass_type == PassType::Human {
        if let Some(cp_id) = indv_p.get_first_literal("v-s:correspondentPerson") {
            let mut icp = Individual::default();
            if module.get_individual(&cp_id, &mut icp).is_some() {
                if let Some(employee) = module.get_individual(&mut icp.get_first_literal("v-s:employee").unwrap_or_default(), &mut Individual::default()) {
                    first_name = employee.get_first_literal_with_lang("v-s:firstName", &[Lang::new_from_str("RU"), Lang::none()]).unwrap_or_default();
                    last_name = employee.get_first_literal_with_lang("v-s:lastName", &[Lang::new_from_str("RU"), Lang::none()]).unwrap_or_default();
                    middle_name = employee.get_first_literal_with_lang("v-s:middleName", &[Lang::new_from_str("RU"), Lang::none()]).unwrap_or_default();
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

    first_name = first_name.trim().to_owned();
    last_name = last_name.trim().to_owned();
    middle_name = middle_name.trim().to_owned();

    equipment_to_field_list(&mut custom_fields, indv_p);
    add_txt_to_fields(&mut custom_fields, "BADGE_STATE_NAME", get_literal_of_link(module, indv_p, "mnd-s:hasPassKind", "rdfs:label"));
    //    add_txt_to_fields(&mut custom_fields, "BADGE_CARD", indv_p.get_first_literal("mnd-s:cardNumber"));
    add_txt_to_fields(&mut custom_fields, "BADGE_COMPANY_ID", get_literal_of_link(module, indv_p, "v-s:correspondentOrganization", "v-s:taxId"));
    add_txt_to_fields(&mut custom_fields, "BADGE_SUBDIVISION_ID", get_literal_of_link(module, indv_p, "v-s:supplier", "v-s:taxId"));
    add_txt_to_fields(&mut custom_fields, "BADGE_SUBDIVISION_NAME", get_literal_of_link(module, indv_p, "v-s:supplier", "rdfs:label"));
    add_txt_to_fields(&mut custom_fields, "BADGE_COMPANY_NAME", get_literal_of_link(module, indv_p, "v-s:correspondentOrganization", "rdfs:label"));
    add_txt_to_fields(&mut custom_fields, "BADGE_CLEARANCE_ORDER_DATE", Some(i64_to_str_date(Some(get_now_00_00_00().timestamp()), "%d.%m.%Y")));
    add_txt_to_fields(&mut custom_fields, "BADGE_FNAME", Some(first_name.to_owned()));
    add_txt_to_fields(&mut custom_fields, "BADGE_LNAME", Some(last_name.to_owned()));
    let kpp_numbers = set_str_from_field_field(module, indv_p, "mnd-s:hasAccessLevel", "mnd-s:accessLevelCheckpoints");
    if !kpp_numbers.is_empty() {
        add_txt_to_fields(&mut custom_fields, "BADGE_CAR_ENTRY_POINT", Some(kpp_numbers));
    }

    add_txt_to_fields(&mut custom_fields, "BADGE_NOTE", indv_p.get_first_literal("rdfs:comment"));

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
        },
        Ok(r) => {
            if let Some(o) = r.as_object() {
                if let Some(id) = o.get("BadgeID") {
                    if let Some(s) = id.as_str() {
                        info!("to PW: add {:?} data, id={}, new badge id = {}", pass_type, indv_p.get_id(), s);
                        new_badge_id = Some(s.to_owned());
                    }
                }
            }
        },
    }

    if new_badge_id.is_none() {
        error!("fail store badge");
        return Err((ResultCode::FailStore, "fail store badge".to_owned()));
    }
    let badge_id = new_badge_id.unwrap();

    veda_photo_to_pw(module, ctx, &badge_id, indv_p);

    add_card_with_access_to_pw(module, ctx, &badge_id, indv_p)
}

pub fn update_prowatch_data(module: &mut Backend, ctx: &mut Context, indv_e: &mut Individual) -> Result<(), (ResultCode, String)> {
    let mut indv_c = get_individual_from_predicate(module, indv_e, "v-s:backwardTarget")?;
    let r_type = indv_c.get_first_literal("rdf:type").unwrap_or_default();

    if r_type == "mnd-s:ACSRecord" {
        update_asc_record(module, ctx, &mut indv_c, indv_e)?;
    } else {
        let has_change_kind_for_pass = indv_c.get_literals("mnd-s:hasChangeKindForPass");
        if r_type != "mnd-s:Pass" && r_type != "mnd-s:PassChangeRequest" && has_change_kind_for_pass.is_none() {
            error!("not found [mnd-s:hasChangeKindForPass] in {}, type={}", indv_c.get_id(), r_type);
            return Err((ResultCode::NotFound, "исходные данные некорректны".to_owned()));
        }
        let has_change_kind_for_passes = has_change_kind_for_pass.unwrap_or_default();

        let mut data_request_pass = get_individual_from_predicate(module, &mut indv_c, "mnd-s:hasSourceDataRequestForPass")?;
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

        if let Ok(pw_badge_id) = get_badge_id_of_card_number(ctx, &card_number) {
            if badge_id != pw_badge_id {
                return Err((ResultCode::UnprocessableEntity, "Держатель карты не соответствует данным в PW".to_owned()));
            }
        } else {
            return Err((ResultCode::UnprocessableEntity, "Не удалось сравнить держателя карты с данными из PW".to_owned()));
        }

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

        if r_type == "mnd-s:Pass" {
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
                    cardholder_family = indv_c.get_first_literal_with_lang("mnd-s:passLastName", &[Lang::new_from_str("RU"), Lang::none()]);
                    is_update_family = true;
                } else if has_change_kind_for_pass == "d:a5y91zferr8t41abib4ecdlggn0" {
                    ts_number = indv_c.get_first_literal("mnd-s:passVehicleRegistrationNumber");
                    is_update_ts_number = true;
                } else if has_change_kind_for_pass == "d:efbibmgvxpr46t1qksmtkkautw" {
                    is_update_access_levels = true;
                    is_update_access_levels_without_clean = true;
                    is_tmp_update_access_levels = true;
                }
            }
        }
        if is_update_ts_number {
            if let Some(rn) = ts_number {
                let rn = rn.trim();
                let cnj = json!({
                    "BadgeID": badge_id,
                    "FirstName": rn,
                    "CustomBadgeFields": [
                        {
                            "ColumnName": "BADGE_CAR_PLATE",
                            "TextValue": rn
                        },
                        {
                            "ColumnName": "BADGE_FNAME",
                            "TextValue": rn
                        }
                    ]
                });
                if let Err(e) = ctx.pw_api_client.badging_api().badges_put(cnj) {
                    error!("to PW: update_ts_number: badges_put: err={:?}", e);
                    return Err((ResultCode::FailStore, format!("{:?}", e)));
                } else {
                    info!("to PW: badges_put(ts_number), badge_id={}", badge_id);
                }
            } else {
                error!("not found ts_number in {}", data_request_pass.get_id());
            }
        }

        if is_update_family {
            if let Some(cf) = cardholder_family {
                let cf = cf.trim();
                let cnj = json!({
                    "BadgeID": badge_id,
                    "LastName": cf,
                    "CustomBadgeFields": [
                        {
                            "ColumnName": "BADGE_LNAME",
                            "TextValue": cf
                        }
                    ]
                });
                if let Err(e) = ctx.pw_api_client.badging_api().badges_put(cnj) {
                    error!("to PW: update_family: badges_put: err={:?}", e);
                    return Err((ResultCode::FailStore, format!("{:?}", e)));
                } else {
                    info!("to PW: badges_put(family), badge_id={}", badge_id);
                }
            } else {
                error!("not found cardholder_family in {}", data_request_pass.get_id());
            }
        }

        if is_update_equipment {
            let mut sj = vec![];
            equipment_to_field_list(&mut sj, &mut indv_c);
            if let Err(e) = ctx.pw_api_client.badging_api().badges_put(json!({ "BadgeID": badge_id, "CustomBadgeFields": sj })) {
                error!("to PW: badges_put: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            } else {
                info!("to PW: badges_put(equipment), badge_id={}", badge_id);
            }
        }

        if is_update_access_levels {
            if !is_update_access_levels_without_clean {
                if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_card_delete_access_levels(&card_number) {
                    error!("to PW: badges_cards_card_delete_access_levels: err={:?}", e);
                    return Err((ResultCode::FailStore, format!("{:?}", e)));
                } else {
                    info!("to PW: badges_cards_card_delete_access_levels, card_number={}", card_number);
                }
            }

            // Collect permanent access levels
            let mut access_levels = HashSet::new();

            if is_tmp_update_access_levels {
                temp_add_level_access(module, ctx, &mut indv_c, &mut access_levels, &card_number)?;
            } else {
                set_hashset_from_field_field(module, &mut indv_c, "mnd-s:hasAccessLevel", "v-s:registrationNumberAdd", &mut access_levels);
            }

            // Collect temporary access levels
            let mut temp_clearance_codes = Vec::new();
            if let Some(temp_access_level_uris) = indv_c.get_literals("mnd-s:hasTempAccessLevel") {
                for temp_access_level_uri in temp_access_level_uris {
                    let mut temp_access_level_indv = Individual::default();
                    if module.get_individual(&temp_access_level_uri, &mut temp_access_level_indv).is_some() {
                        // Get associated access level
                        if let Some(access_level_uri) = temp_access_level_indv.get_first_literal("mnd-s:hasAccessLevel") {
                            let mut access_level_indv = Individual::default();
                            if module.get_individual(&access_level_uri, &mut access_level_indv).is_some() {
                                if let Some(clear_code_id) = access_level_indv.get_first_literal("v-s:registrationNumberAdd") {
                                    // Get dates
                                    let valid_from =
                                        temp_access_level_indv.get_first_datetime("v-s:dateFrom").map(|dt| i64_to_str_date_ymdthms(Some(dt))).unwrap_or_default();
                                    let valid_to =
                                        temp_access_level_indv.get_first_datetime("v-s:dateTo").map(|dt| i64_to_str_date_ymdthms(Some(dt))).unwrap_or_default();

                                    temp_clearance_codes.push(json!({
                                        "ClearCodeID": clear_code_id,
                                        "ClearCodeType": 3,
                                        "ValidFrom": valid_from,
                                        "ValidTo": valid_to
                                    }));
                                } else {
                                    error!("Access level {} does not have v-s:registrationNumberAdd", access_level_uri);
                                }
                            } else {
                                error!("Access level {} not found", access_level_uri);
                            }
                        } else {
                            error!("Temporary access level {} does not have mnd-s:hasAccessLevel", temp_access_level_uri);
                        }
                    } else {
                        error!("Temporary access level {} not found", temp_access_level_uri);
                    }
                }
            }

            // Prepare the JSON array to send to ProWatch
            let mut sj =
                access_levels_to_json_for_add(access_levels, is_tmp_update_access_levels, None, set_next_day_and_00_00_00(indv_c.get_first_datetime("v-s:dateToPlan")));

            // Add temporary access levels to sj
            sj.extend(temp_clearance_codes);

            // Send request to ProWatch
            if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_card_update_access_levels(&card_number, json!(sj)) {
                error!("to PW: badges_cards_card_update_access_levels: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            } else {
                info!("to PW: badges_cards_card_update_access_levels, card_number={}", card_number);
            }

            // Continue with the rest of the code (updating custom fields)
            let mut custom_fields = vec![];
            let kpp_numbers = set_str_from_field_field(module, &mut indv_c, "mnd-s:hasAccessLevel", "mnd-s:accessLevelCheckpoints");
            if !kpp_numbers.is_empty() {
                add_txt_to_fields(&mut custom_fields, "BADGE_CAR_ENTRY_POINT", Some(kpp_numbers));
            }

            if let Some(cp_id) = indv_c.get_first_literal("v-s:correspondentPerson") {
                let mut icp = Individual::default();
                if module.get_individual(&cp_id, &mut icp).is_some() {
                    add_txt_to_fields(&mut custom_fields, "BADGE_TITLE", get_literal_of_link(module, &mut icp, "v-s:occupation", "rdfs:label"));
                } else {
                    error!("add human, invalid link v-s:correspondentPerson = {}", cp_id);
                    return Err((ResultCode::BadRequest, format!("add human, invalid link v-s:correspondentPerson = {}", cp_id)));
                }
            } else {
                add_txt_to_fields(&mut custom_fields, "BADGE_TITLE", indv_c.get_first_literal("mnd-s:passPosition"));
            }

            let reqj = json!({
                "BadgeID": badge_id,
                "CustomBadgeFields": custom_fields
            });

            if let Err(e) = ctx.pw_api_client.badging_api().badges_put(reqj) {
                error!("to PW: add_card_with_access_to_pw, put CustomBadgeFields: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            } else {
                info!("to PW: add_card_with_access_to_pw, put CustomBadgeFields, badge_id={}", badge_id);
            }
        }

        if is_update || is_need_block_card {
            let date_to = if is_need_block_card {
                Some(get_now_23_59_59().timestamp())
            } else {
                set_23_59_59(indv_c.get_first_datetime("v-s:dateToFact"))
            };

            let status = if is_need_block_card {
                1
            } else {
                0
            };

            let cnj = json!({
                "CardNumber": card_number,
                "CardStatus": status,
                "ExpireDate": i64_to_str_date_ymdthms(date_to)
            });
            if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_put(cnj) {
                error!("to PW: badges_cards_card: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            } else {
                info!("to PW: badges_cards_put(equipment), card_number={}", card_number);
            }

            let sj = json!({
                "BadgeID": badge_id,
                "ExpireDate": i64_to_str_date_ymdthms(date_to)
            });
            if let Err(e) = ctx.pw_api_client.badging_api().badges_put(sj) {
                error!("to PW: badges_cards: err={:?}", e);
                return Err((ResultCode::FailStore, format!("{:?}", e)));
            } else {
                info!("to PW: badges_put(ExpireDate), badge_id={}", badge_id);
            }
        }
    }
    Ok(())
}

pub fn delete_from_prowatch(_module: &mut Backend, _ctx: &mut Context, _indv: &mut Individual) -> ResultCode {
    ResultCode::Ok
}

fn add_card_with_access_to_pw(module: &mut Backend, ctx: &mut Context, badge_id: &str, src: &mut Individual) -> Result<(), (ResultCode, String)> {
    // Собираем постоянные уровни доступа
    let mut access_levels = HashSet::new();
    set_hashset_from_field_field(module, src, "mnd-s:hasAccessLevel", "v-s:registrationNumberAdd", &mut access_levels);

    // Формируем JSON для постоянных уровней доступа с помощью функции access_levels_to_json_for_new
    let mut clearance_codes = access_levels_to_json_for_new(access_levels);

    // Собираем временные уровни доступа
    if let Some(temp_access_level_uris) = src.get_literals("mnd-s:hasTempAccessLevel") {
        for temp_access_level_uri in temp_access_level_uris {
            let mut temp_access_level_indv = Individual::default();
            if module.get_individual(&temp_access_level_uri, &mut temp_access_level_indv).is_some() {
                // Получаем связанный уровень доступа
                if let Some(access_level_uri) = temp_access_level_indv.get_first_literal("mnd-s:hasAccessLevel") {
                    let mut access_level_indv = Individual::default();
                    if module.get_individual(&access_level_uri, &mut access_level_indv).is_some() {
                        if let Some(clear_code_id) = access_level_indv.get_first_literal("v-s:registrationNumberAdd") {
                            // Получаем даты действия
                            let valid_from = temp_access_level_indv.get_first_datetime("v-s:dateFrom").map(|dt| i64_to_str_date_ymdthms(Some(dt))).unwrap_or_default();
                            let valid_to = temp_access_level_indv.get_first_datetime("v-s:dateTo").map(|dt| i64_to_str_date_ymdthms(Some(dt))).unwrap_or_default();

                            clearance_codes.push(json!({
                                "ClearCode": {
                                    "ClearCodeID": clear_code_id,
                                    "ClearCodeType": 3,
                                    "ValidFrom": valid_from,
                                    "ValidTo": valid_to
                                }
                            }));
                        } else {
                            error!("Access level {} does not have v-s:registrationNumberAdd", access_level_uri);
                        }
                    } else {
                        error!("Access level {} not found", access_level_uri);
                    }
                } else {
                    error!("Temporary access level {} does not have mnd-s:hasAccessLevel", temp_access_level_uri);
                }
            } else {
                error!("Temporary access level {} not found", temp_access_level_uri);
            }
        }
    }

    // Получаем номер карты
    let wcard_number = src.get_first_literal("mnd-s:cardNumber");
    if wcard_number.is_none() {
        error!("not found [mnd-s:cardNumber] in {}", src.get_id());
        return Ok(());
    }
    let card_number = wcard_number.unwrap();

    // Удаляем карту, если она уже существует
    if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_card_delete(&card_number) {
        warn!("fail delete card {}, badges_cards_card_delete, err={:?}", card_number, e);
    }

    // Формируем основной запрос для добавления карты
    let cnj = json!({
        "BadgeID": badge_id,
        "CardNumber": card_number,
        "CardStatus": 0,
        "ExpireDate": i64_to_str_date_ymdthms(set_23_59_59(src.get_first_datetime("v-s:dateToFact"))),
        "Company": {
            "CompanyID": "0x004842343236434238382D443536302D3433"
        },
        "ClearanceCodes": clearance_codes
    });

    // Отправляем запрос на добавление карты с уровнями доступа
    if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_post(cnj) {
        error!("to PW: add_card_with_access_to_pw: err={:?}", e);
        return Err((ResultCode::FailStore, format!("{:?}", e)));
    } else {
        info!("to PW: add_card_with_access_to_pw, badge_id={}, card_number={}", badge_id, card_number);
    }

    Ok(())
}

fn update_asc_record(backend: &mut Backend, ctx: &mut Context, indv_r: &mut Individual, indv_e: &mut Individual) -> Result<(), (ResultCode, String)> {
    if let Some(badge_id) = indv_r.get_first_literal("mnd-s:winpakCardRecordId") {
        let mut indv_s = get_individual_from_predicate(backend, indv_r, "v-s:backwardTarget")?;
        let mut indv_p = get_individual_from_predicate(backend, &mut indv_s, "v-s:backwardTarget")?;
        if let Err(e) = add_card_with_access_to_pw(backend, ctx, &badge_id, &mut indv_p) {
            error!("update_prowatch_data::add_card_with_access_to_pw, error={:?}", e);
        }

        let mut custom_fields = vec![];
        equipment_to_field_list(&mut custom_fields, &mut indv_p);

        let kpp_numbers = set_str_from_field_field(backend, &mut indv_p, "mnd-s:hasAccessLevel", "mnd-s:accessLevelCheckpoints");
        if !kpp_numbers.is_empty() {
            add_txt_to_fields(&mut custom_fields, "BADGE_CAR_ENTRY_POINT", Some(kpp_numbers));
        }

        if let Some(cp_id) = indv_p.get_first_literal("v-s:correspondentPerson") {
            let mut icp = Individual::default();
            if backend.get_individual(&cp_id, &mut icp).is_some() {
                add_txt_to_fields(&mut custom_fields, "BADGE_TITLE", get_literal_of_link(backend, &mut icp, "v-s:occupation", "rdfs:label"));
            } else {
                error!("add human, invalid link v-s:correspondentPerson = {}", cp_id);
                return Err((ResultCode::BadRequest, format!("add human, invalid link v-s:correspondentPerson = {}", cp_id)));
            }
        } else {
            add_txt_to_fields(&mut custom_fields, "BADGE_TITLE", indv_p.get_first_literal("mnd-s:passPosition"));
        }

        let js = if !indv_r.get_first_literal("mnd-s:cardNumber").unwrap_or_default().is_empty() {
            json!({ "BadgeID": badge_id, "CustomBadgeFields": custom_fields })
        } else {
            let date_to = indv_p.get_first_datetime("v-s:dateToFact");
            json!({ "BadgeID": badge_id, "ExpireDate": i64_to_str_date_ymdthms (date_to), "CustomBadgeFields": custom_fields })
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
                        set_card_status(ctx, &card_number, 1)?;
                    }
                }
            }
        }
        indv_e.add_uri("v-s:backwardTarget", indv_p.get_id());
    }

    Ok(())
}
