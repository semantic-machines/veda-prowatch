use array_tool::vec::Union;
use base64::decode;
use base64::encode;
use chrono::offset::LocalResult::Single;
use chrono::{DateTime, Offset};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use prowatch_client::apis::client::PWAPIClient;
use prowatch_client::apis::Error;
use serde_json::json;
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use std::{fs, io};
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::ops::{Add, Sub};
use uuid::Uuid;
use v_common::module::veda_backend::Backend;
use v_common::onto::datatype::Lang;
use v_common::onto::individual::Individual;
use v_common::onto::onto_impl::Onto;
use v_common::search::common::FTQuery;
use v_common::v_api::api_client::IndvOp;
use v_common::v_api::obj::ResultCode;
use voca_rs::chop;

#[derive(Debug, PartialEq, Clone)]
pub enum PassType {
    Vehicle,
    Human,
    Unknown,
}

pub const S_MAX_TIME: &str = "2100-01-01T00:00:00";

pub struct Context {
    pub sys_ticket: String,
    pub onto: Onto,
    pub pw_api_client: PWAPIClient,
    pub access_level_dict: HashMap<String, String>,
}

pub fn map_card_status(n: i64) -> String {
    return match n {
        0 => "Активна".to_string(),
        1 => "Отключена".to_string(),
        2 => "Утеряна".to_string(),
        3 => "Украдена".to_string(),
        4 => "Сдана".to_string(),
        5 => "Неучтенная".to_string(),
        6 => "Аннулированная".to_string(),
        7 => "Истек срок действия".to_string(),
        8 => "Авто откл.".to_string(),
        _ => "?".to_string(),
    };
}

pub fn load_access_level_dict(module: &mut Backend, sys_ticket: &str) -> io::Result<HashMap<String, String>> {
    info!("load access level dict");
    let mut dir = HashMap::new();
    let res = module.fts.query(FTQuery::new_with_ticket(&sys_ticket, "'rdf:type'=='mnd-s:AccessLevel'"));
    if res.result_code == ResultCode::Ok && res.count > 0 {
        for id in &res.result {
            if let Some(indv) = module.get_individual(id, &mut Individual::default()) {
                if let Some(s) = indv.get_first_literal("v-s:registrationNumberAdd") {
                    info!("{} - {}", s, indv.get_id());
                    dir.insert(s, indv.get_id().to_owned());
                }
            }
        }
    } else {
        return Err(io::Error::new (ErrorKind::Other, "fail load access level dict"));
    }
    Ok(dir)
}

pub fn get_equipment_list(indv: &mut Individual, list: &mut Vec<String>) {
    if let Some(pass_equipment) = indv.get_first_literal("mnd-s:passEquipment") {
        split_str_for_winpak_list(&pass_equipment, 64, list);
    }
}

pub fn split_str_for_winpak_list(src: &str, len: usize, res: &mut Vec<String>) {
    for el in src.split('\n') {
        let mut start = 0;
        let mut end = len;
        loop {
            if end >= el.len() {
                end = el.len();
            }

            let ss = chop::substring(el, start, end);
            if !ss.is_empty() {
                res.push(chop::substring(el, start, end));
            } else {
                break;
            }

            if end >= el.len() {
                break;
            }
            start = end;
            end += len;
        }
    }
}

pub fn get_individual_from_predicate(module: &mut Backend, src: &mut Individual, predicate: &str) -> Result<Individual, (ResultCode, String)> {
    let indv_id = src.get_first_literal(predicate);
    if indv_id.is_none() {
        error!("not found [{}] in {}", predicate, src.get_id());
        return Err((ResultCode::NotFound, "исходные данные некорректны".to_owned()));
    }
    let indv_id = indv_id.unwrap();
    let indv_c = module.get_individual_h(&indv_id);
    if indv_c.is_none() {
        error!("not found {}, id from {}.{}", &indv_id, src.get_id(), predicate);
        return Err((ResultCode::NotFound, "исходные данные некорректны".to_owned()));
    }
    Ok(*indv_c.unwrap())
}

pub fn clear_card_and_set_err(module: &mut Backend, sys_ticket: &str, indv: &mut Individual, err_text: &str) {
    indv.parse_all();
    indv.clear("mnd-s:briefingDate");
    indv.clear("mnd-s:hasAccessLevel");
    indv.clear("mnd-s:passEquipment");
    indv.clear("rdfs:comment");
    indv.clear("v-s:birthday");
    indv.clear("v-s:dateFrom");
    indv.clear("v-s:dateTo");
    indv.clear("v-s:description");
    indv.clear("mnd-s:passEquipment");
    indv.clear("v-s:tabNumber");
    indv.set_string("v-s:errorMessage", err_text, Lang::new_from_str("RU"));
    indv.set_uri("v-s:lastEditor", "cfg:VedaSystemAppointment");

    match module.mstorage_api.update_use_param(sys_ticket, "prowatch", "", 0, IndvOp::Put, indv) {
        Ok(_) => {
            info!("success update, uri={}", indv.get_id());
        },
        Err(e) => {
            error!("fail update, uri={}, result_code={:?}", indv.get_id(), e.result);
        },
    }
}

pub fn set_err(module: &mut Backend, sys_ticket: &str, indv: &mut Individual, err_text: &str) {
    indv.parse_all();
    indv.set_string("v-s:errorMessage", err_text, Lang::new_from_str("RU"));
    indv.set_uri("v-s:lastEditor", "cfg:VedaSystemAppointment");

    match module.mstorage_api.update_use_param(sys_ticket, "prowatch", "", 0, IndvOp::Put, indv) {
        Ok(_) => {
            info!("success update, uri={}", indv.get_id());
        },
        Err(e) => {
            error!("fail update, uri={}, result_code={:?}", indv.get_id(), e.result);
        },
    }
}

pub fn set_hashset_from_field_field(
    module: &mut Backend,
    indv: &mut Individual,
    predicate: &str,
    inner_predicate: &str,
    out_data: &mut HashSet<String>,
) -> Vec<Box<Individual>> {
    let mut indvs: Vec<Box<Individual>> = vec![];
    if let Some(uris) = indv.get_literals(predicate) {
        for l in uris {
            if let Some(mut indv_c) = module.get_individual_h(&l) {
                if let Some(al) = indv_c.get_first_literal(inner_predicate) {
                    out_data.insert(al);
                    indvs.push(indv_c);
                }
            } else {
                error!("not found {}", l);
            }
        }
    }
    indvs
}

pub fn set_str_from_field_field(module: &mut Backend, indv: &mut Individual, predicate: &str, innner_predicate: &str) -> String {
    let mut out_data = String::new();
    if let Some(uris) = indv.get_literals(predicate) {
        for l in uris {
            if let Some(mut indv_c) = module.get_individual_h(&l) {
                if let Some(al) = indv_c.get_first_literal(innner_predicate) {
                    if !out_data.is_empty() {
                        out_data.push(' ');
                    }
                    out_data.push_str(&al);
                }
            } else {
                error!("not found {}", l);
            }
        }
    }
    out_data
}

pub fn concat_fields(fields: &[&str], els: Option<&Map<String, Value>>, delim: &str) -> Option<String> {
    if let Some(el) = els {
        let mut descr = String::new();

        for f in fields {
            if let Some(d) = el.get(f.to_owned()) {
                if let Some(s) = d.as_str() {
                    if !s.is_empty() {
                        if !descr.is_empty() {
                            descr.push_str(delim);
                        }
                        descr.push_str(s);
                    }
                }
            }
        }

        if descr.is_empty() {
            return None;
        }
        return Some(descr);
    }
    None
}

pub fn get_int_from_value(src_value: &Value, src_field: &str) -> Option<i64> {
    if let Some(v) = src_value.get(src_field) {
        return v.as_i64();
    }
    None
}

pub fn get_str_from_value<'a>(src_value: &'a Value, src_field: &str) -> Option<&'a str> {
    if let Some(v) = src_value.get(src_field) {
        return v.as_str();
    }
    None
}

pub fn str_value2indv_card_number(src_val: &Value, dest_indv: &mut Individual) {
    if let Some(v1) = src_val.get("CardNumber") {
        if let Some(card_number_str) = v1.as_str() {
            let mut s2 = card_number_str.to_string();

            if let Some(v1) = src_val.get("CardStatus") {
                if let Some(s1) = v1.as_i64() {
                    s2 = format!("{} - {}", card_number_str, map_card_status(s1));
                }
            }

            dest_indv.add_string("mnd-s:cardNumber", &s2, Lang::none());
        }
    }
}

pub fn get_pass_type(indv_p: &mut Individual) -> PassType {
    if let Some(tag) = indv_p.get_first_literal("v-s:tag") {
        if tag == "Auto" {
            return PassType::Vehicle;
        }
        if tag == "Human" {
            return PassType::Human;
        }
    } else {
        if let Some(has_kind_for_pass) = indv_p.get_first_literal("mnd-s:hasPassKind") {
            if has_kind_for_pass == "d:c94b6f98986d493cae4a3a37249101dc"
                || has_kind_for_pass == "d:5f5be080f1004af69742bc574c030609"
                || has_kind_for_pass == "d:1799f1e110054b5a9ef819754b0932ce"
            {
                return PassType::Vehicle;
            }
            if has_kind_for_pass == "d:ece7e741557e406bb996809163810c6e"
                || has_kind_for_pass == "d:a149d268628b46ae8d40c6ea0ac7f3dd"
                || has_kind_for_pass == "d:228e15d5afe544c099c337ceafa47ea6"
                || has_kind_for_pass == "d:ih7mpbsuu6xxmy7ouqlyhfqyua"
            {
                return PassType::Human;
            }
        }
    }
    PassType::Unknown
}

pub fn get_badge_use_request_indv(
    module: &mut Backend,
    ctx: &mut Context,
    pass_type: Option<PassType>,
    indv_p: &mut Individual,
) -> (PassType, Result<Vec<Value>, Error>) {
    let tpass: PassType = if let Some(pt) = pass_type {
        pt
    } else {
        get_pass_type(indv_p)
    };

    if tpass != PassType::Unknown {
        let res_badge = if tpass == PassType::Vehicle {
            let car_number = indv_p.get_first_literal("mnd-s:passVehicleRegistrationNumber").unwrap_or_default();

            let l1 = ctx.pw_api_client.badging_api().badges_key_key_value("BADGE_FNAME", &format!("%25{}%25", &car_number)).unwrap_or_default();
            let l2 = ctx.pw_api_client.badging_api().badges_key_key_value("BADGE_LNAME", &format!("%25{}%25", &car_number)).unwrap_or_default();

            Ok(l1.union(l2))
        } else if tpass == PassType::Human {
            let mut first_name = String::new();
            let mut last_name = String::new();
            let mut middle_name = String::new();

            let _type = indv_p.get_first_literal("rdf:type").unwrap_or_default();
            if _type == "mnd-s:SourceDataRequestForPassByNames" {
                first_name = indv_p.get_first_literal("v-s:firstName").unwrap_or_default();
                last_name = indv_p.get_first_literal("v-s:lastName").unwrap_or_default();
                middle_name = indv_p.get_first_literal("v-s:middleName").unwrap_or_default();
            } else {
                if let Some(cp_id) = indv_p.get_first_literal("v-s:correspondentPerson") {
                    let mut icp = Individual::default();
                    if module.get_individual(&cp_id, &mut icp).is_some() {
                        if let Some(employee) = module.get_individual(&mut icp.get_first_literal("v-s:employee").unwrap_or_default(), &mut Individual::default()) {
                            first_name = employee.get_first_literal_with_lang("v-s:firstName", &[Lang::new_from_str("RU"), Lang::none()]).unwrap_or_default();
                            last_name = employee.get_first_literal_with_lang("v-s:lastName", &[Lang::new_from_str("RU"), Lang::none()]).unwrap_or_default();
                            middle_name = employee.get_first_literal_with_lang("v-s:middleName", &[Lang::new_from_str("RU"), Lang::none()]).unwrap_or_default();
                        }
                    }
                } else {
                    first_name = indv_p.get_first_literal("mnd-s:passFirstName").unwrap_or_default();
                    last_name = indv_p.get_first_literal("mnd-s:passLastName").unwrap_or_default();
                    middle_name = indv_p.get_first_literal("mnd-s:passMiddleName").unwrap_or_default();
                }
            }

            ctx.pw_api_client.badging_api().badges_key_key_value_with_filter(
                "BADGE_LNAME",
                &last_name,
                &format!("FirstName eq '{}' and MiddleName eq '{}'", &first_name, &middle_name),
            )
        } else {
            Ok(vec![])
        };
        return (tpass, res_badge);
    }

    (PassType::Unknown, Ok(vec![]))
}

pub fn equipment_to_field_list(sj: &mut Vec<Value>, src_indv: &mut Individual) {
    let mut equipment_list: Vec<String> = Vec::new();
    get_equipment_list(src_indv, &mut equipment_list);

    for idx in 0..equipment_list.len() {
        let val = if idx == 10 {
            let mut r = String::default();
            for idx2 in idx..equipment_list.len() {
                if let Some(v) = equipment_list.get(idx2) {
                    r.push_str(v);
                    r.push(' ');
                }
            }
            r
        } else {
            if let Some(v) = equipment_list.get(idx) {
                v.to_owned()
            } else {
                "".to_owned()
            }
        };

        let sji = json!({
        "ColumnName": format!("BADGE_TOOL{:02}", idx + 1),
        "TextValue": val.trim()
        });
        sj.push(sji);

        if idx >= 10 {
            break;
        }
    }

    if sj.len() < 11 {
        for idx in sj.len()..11 {
            let sji = json!({
            "ColumnName": format!("BADGE_TOOL{:02}", idx + 1),
            "TextValue": ""
            });
            sj.push(sji);
        }
    }
}

pub fn add_txt_to_fields(list: &mut Vec<Value>, fname: &str, src: Option<String>) {
    if let Some(val) = src {
        let sji = json!({
        "ColumnName": fname,
        "FieldType": 2,
        "TextValue": val.trim()
        });

        list.push(sji);
    }
}

pub fn add_date_to_fields(list: &mut Vec<Value>, fname: &str, src: Option<i64>) {
    if let Some(val) = src {
        let sji = json!({
        "ColumnName": fname,
        "DateValue": i64_to_str_date_ymdthms (Some(val))
        });

        list.push(sji);
    }
}

pub fn get_literal_of_link(module: &mut Backend, indv: &mut Individual, link: &str, field: &str) -> Option<String> {
    module.get_literal_of_link(indv, link, field, &mut Individual::default())
}

pub fn access_levels_to_json_for_add(access_levels: HashSet<String>, is_tmp_update_access_levels: bool, date_from: Option<i64>, date_to: Option<i64>) -> Vec<Value> {
    let mut sj: Vec<Value> = Vec::new();
    let df = if let Some(d) = date_from {
        Some(d)
    } else {
        Some(get_now_00_00_00().timestamp())
    };

    for lvl in access_levels {
        let sji = if is_tmp_update_access_levels {
            json!({
            "ClearCodeID": lvl,
            "ClearCodeType": 3,
            "ValidFrom": i64_to_str_date_ymdthms (df),
            "ValidTo": i64_to_str_date_ymdthms (date_to)
            })
        } else {
            json!({ "ClearCodeID": lvl })
        };

        sj.push(sji);
    }
    sj
}

pub fn access_level_to_json_for_add(access_level: &str, is_tmp_update_access_levels: bool, date_from: Option<i64>, date_to: Option<i64>) -> Vec<Value> {
    let mut sj: Vec<Value> = Vec::new();
    let df = if let Some(d) = date_from {
        Some(d)
    } else {
        Some(get_now_00_00_00().timestamp())
    };

    let sji = if is_tmp_update_access_levels {
        json!({
        "ClearCodeID": access_level,
        "ClearCodeType": 3,
        "ValidFrom": i64_to_str_date_ymdthms (df),
        "ValidTo": i64_to_str_date_ymdthms (date_to)
        })
    } else {
        json!({ "ClearCodeID": access_level })
    };

    sj.push(sji);
    sj
}

pub fn access_levels_to_json_for_new(access_levels: HashSet<String>) -> Vec<Value> {
    let mut sj: Vec<Value> = Vec::new();
    for lvl in access_levels {
        let sji = json!({
        "ClearCode": {
            "ClearCodeID": lvl
        }});

        sj.push(sji);
    }
    sj
}

pub fn get_custom_badge_as_list(el: &Value) -> Map<String, Value> {
    let mut fields: Map<String, Value> = Map::default();

    let nn = if let Some(v) = el.as_array() {
        v.get(0).unwrap_or(el)
    } else {
        el
    };

    if let Some(v) = nn.get("CustomBadgeFields") {
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
    fields
}

pub fn create_asc_record(el: &Value, backward_id: &str, cards: Vec<String>, src: &str) -> Individual {
    warn!("create_asc_record,  {}", src);
    let mut acs_record = Individual::default();
    acs_record.set_id(&("d:asc_".to_owned() + &Uuid::new_v4().to_string()));
    acs_record.set_uri("rdf:type", "mnd-s:ACSRecord");
    acs_record.set_uri("v-s:backwardProperty", "mnd-s:hasACSRecord");
    acs_record.set_uri("v-s:backwardTarget", backward_id);
    acs_record.set_bool("v-s:canRead", true);
    acs_record.set_string("mnd-s:cardNumber", &format!("{:?}", cards), Lang::none());

    set_badge_to_indv(&el, &mut acs_record);

    acs_record
}

pub fn set_badge_to_indv(el: &Value, dest: &mut Individual) {
    if !el.is_object() {
        return;
    }

    if let Some(v) = el.get("BadgeID") {
        if let Some(s) = v.as_str() {
            dest.set_string("mnd-s:winpakCardRecordId", &s, Lang::none());
        }
    }

    if let Some(v) = el.get("LastName") {
        if let Some(s) = v.as_str() {
            dest.set_string("v-s:lastName", &s, Lang::new_from_str("RU"));
        }
    }

    if let Some(v) = el.get("FirstName") {
        if let Some(s) = v.as_str() {
            dest.set_string("v-s:firstName", &s, Lang::new_from_str("RU"));
        }
    }

    if let Some(v) = el.get("MiddleName") {
        if let Some(s) = v.as_str() {
            dest.set_string("v-s:middleName", &s, Lang::new_from_str("RU"));
        }
    }

    if let Some(s) = concat_fields(&["LastName", "FirstName", "MiddleName"], el.as_object(), " ") {
        dest.set_string("v-s:description", &s, Lang::new_from_str("RU"));
    }

    let fields = get_custom_badge_as_list(el);
    if let Some(s) = concat_fields(&["BADGE_COMPANY_NAME", "BADGE_DEPARTMENT", "BADGE_TITLE"], Some(&fields), " ") {
        dest.set_string("rdfs:comment", &s, Lang::new_from_str("RU"));
    }

    if let Some(d) = fields.get("BADGE_BIRTHDATE") {
        dest.clear("v-s:birthday");
        if let Some(d) = d.as_str() {
            dest.add_datetime_from_str("v-s:birthday", d);
        }
    }

    if let Some(v) = fields.get("BADGE_NOTE_UPB") {
        if let Some(s) = v.as_str() {
            dest.set_string("mnd-s:commentUPB", &s, Lang::none());
        }
    }

    if let Some(v) = fields.get("BADGE_NOTE") {
        if let Some(s) = v.as_str() {
            dest.set_string("mnd-s:commentES", &s, Lang::none());
        }
    }

    if let Some(v) = fields.get("BADGE_ID") {
        if let Some(s) = v.as_str() {
            dest.set_string("v-s:tabNumber", &s, Lang::none());
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
        dest.set_string("mnd-s:passEquipment", &s, Lang::new_from_str("RU"));
    }
}

pub fn extract_card_number(in1: &str) -> &str {
    let out1 = if in1.find(" - ").is_some() {
        if let Some((a, _b)) = in1.split_once('-') {
            a.trim()
        } else {
            in1
        }
    } else {
        in1
    };
    out1
}
pub fn set_card_status(ctx: &mut Context, card_number0: &str, card_status: i32) -> Result<(), (ResultCode, String)> {
    let card_number = extract_card_number(card_number0);

    let cnj = json!({
        "CardNumber": card_number,
        "CardStatus": card_status,
    });
    if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_put(cnj.clone()) {
        error!("block cards if exist temp card: badges_cards_card: err={:?}, src={:?}", e, cnj);
        return Err((ResultCode::FailStore, format!("{:?}", e)));
    }
    Ok(())
}

pub fn set_update_status(
    module: &mut Backend,
    ctx: &mut Context,
    indv: &mut Individual,
    res: &Result<(), (ResultCode, String)>,
    status_if_err: &str,
    status_if_ok: &str,
) -> ResultCode {
    indv.parse_all();
    if let Err((sync_res, info)) = res {
        if *sync_res == ResultCode::ConnectError {
            return sync_res.clone();
        }
        indv.set_uri("v-s:hasStatus", status_if_err);
        set_err(module, &ctx.sys_ticket, indv, &info);
        return sync_res.clone();
    }

    indv.set_uri("v-s:lastEditor", "cfg:VedaSystemAppointment");
    indv.set_uri("v-s:hasStatus", status_if_ok);
    indv.clear("v-s:errorMessage");

    match module.mstorage_api.update_use_param(&ctx.sys_ticket, "prowatch", "", 0, IndvOp::Put, indv) {
        Ok(_) => {
            info!("success update, uri={}", indv.get_id());
        },
        Err(e) => {
            error!("fail update, uri={}, result_code={:?}", indv.get_id(), e.result);
        },
    }
    ResultCode::Ok
}

pub fn str_date_to_i64(value: &str, offset: Option<Duration>) -> Option<i64> {
    if value.contains('Z') {
        if let Ok(v) = DateTime::parse_from_rfc3339(&value) {
            return Some(v.timestamp());
        } else {
            error!("fail parse [{}] to datetime", value);
        }
    } else {
        let ndt;
        if value.len() == 10 {
            if value.contains('.') {
                ndt = NaiveDateTime::parse_from_str(&(value.to_owned() + "T00:00:00"), "%d.%m.%YT%H:%M:%S");
            } else {
                ndt = NaiveDateTime::parse_from_str(&(value.to_owned() + "T00:00:00"), "%Y-%m-%dT%H:%M:%S");
            }
        } else {
            ndt = NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S")
        }

        if let Ok(v) = ndt {
            let vo = if let Single(offset) = Local.offset_from_local_datetime(&v) {
                v.sub(offset)
            } else {
                v
            };

            return if let Some(o) = offset {
                Some(vo.add(o).timestamp())
            } else {
                Some(vo.timestamp())
            };
        } else {
            error!("fail parse [{}] to datetime", value);
        }
    }
    return None;
}

pub(crate) fn veda_photo_to_pw(module: &mut Backend, ctx: &mut Context, badge_id: &str, src: &mut Individual) {
    if let Ok(mut file) = get_individual_from_predicate(module, src, "v-s:hasImage") {
        info!("extract photo {} from {}", file.get_id(), src.get_id());

        let src_full_path =
            "data/files".to_owned() + &file.get_first_literal("v-s:filePath").unwrap_or_default() + "/" + &file.get_first_literal("v-s:fileUri").unwrap_or_default();

        match fs::read(src_full_path) {
            Ok(f) => {
                let msg_base64 = encode(f);

                if let Err(e) = ctx.pw_api_client.badging_api().badges_badge_id_photo_post(&badge_id, msg_base64) {
                    error!("to PW: update_photo: badges_put: err={:?}", e);
                } else {
                    info!("to PW: update photo, {}", src.get_id())
                }
            },
            Err(e) => {
                error!("fail send photo to prowatch, err={:?}", e);
            },
        }
    }
}

pub(crate) fn pw_photo_to_veda(module: &mut Backend, ctx: &mut Context, badge_id: &str, dest: &mut Individual) {
    if let Ok(msg_base64) = ctx.pw_api_client.badging_api().badges_badge_id_photo(&badge_id) {
        if let Ok(photo_data) = decode(&msg_base64.as_bytes()[1..msg_base64.len() - 1]) {
            let files_dir_path = "data/files";
            let file_path = format!("/{}", i64_to_str_date(Some(get_now_00_00_00().timestamp()), "%Y/%m/%d"));

            fs::create_dir_all(format!("{}/{}", files_dir_path, file_path)).unwrap_or_default();

            let file_uri = format!("badge_photo_{}", badge_id);
            let full_file_path = format!("{}/{}/{}", files_dir_path, file_path, file_uri);

            let file_name = format!("фото_{}.jpg", badge_id);

            let mut indv_file = Individual::default();
            indv_file.set_id(&(dest.get_id().to_owned() + "_photo"));

            match File::create(&full_file_path) {
                Ok(mut ofile) => {
                    if let Err(e) = ofile.write_all(&photo_data) {
                        error!("fail write file {}, {:?}", e, full_file_path);
                    } else {
                        info!("success create file {}", full_file_path);
                    }
                },
                Err(e) => {
                    error!("fail create file {}, {:?}", full_file_path, e);
                },
            }

            indv_file.set_uri("rdf:type", "v-s:File");
            indv_file.set_uri("v-s:fileUri", &file_uri);
            indv_file.set_uri("v-s:filePath", &file_path);
            indv_file.set_uri("v-s:fileName", &file_name);
            indv_file.set_integer("v-s:fileSize", photo_data.len() as i64);
            indv_file.set_uri("v-s:parent", dest.get_id());

            dest.set_uri("v-s:attachment", indv_file.get_id());

            let res = module.mstorage_api.update(&ctx.sys_ticket, IndvOp::Put, &mut indv_file);
            if res.result != ResultCode::Ok {
                error!("fail update, uri={}, result_code={:?}", indv_file.get_id(), res.result);
            } else {
                info!("success update, uri={}", indv_file.get_id());
            }
        } else {
            error!("failed to decode base64, badge_id={}", badge_id)
        }
    }
}

pub fn get_levels(card: Value) -> Vec<(String, String)> {
    let mut f_levels = vec![];
    if let Some(v) = card.get("ClearanceCodes") {
        if v.is_array() {
            for c_el in v.as_array().unwrap_or(&vec![]) {
                let mut clear_code_id = "".to_string();
                let mut valid_to = "".to_string();
                if let Some(v) = c_el.get("ClearCode") {
                    if let Some(v) = v.get("ClearCodeID") {
                        if let Some(v) = v.as_str() {
                            clear_code_id = v.to_owned();
                        }
                    }
                    if let Some(v) = v.get("ValidTo") {
                        if let Some(v) = v.as_str() {
                            valid_to = v.to_owned();
                        }
                    }

                    if !clear_code_id.is_empty() {
                        f_levels.push((clear_code_id, valid_to));
                    }
                }
            }
        }
    }
    f_levels
}

// 5. ВРЕМЕННОЕ ДОБАВЛЕНИЕ УРОВНЕЙ ДОСТУПА
pub fn temp_add_level_access(
    backend: &mut Backend,
    ctx: &mut Context,
    indv_c: &mut Individual,
    access_levels: &mut HashSet<String>,
    card_number: &str,
) -> Result<(), (ResultCode, String)> {
    let offset = Local.timestamp(0, 0).offset().fix().local_minus_utc() as i64;

    let mut mutually_exclusive_levels = HashSet::default();

    // 1) получаем список id взаимоисключающих уровней доступа, запоминаем [список исключений]
    // [список исключающих уровней] = [C]["mnd-s:hasTemporaryAccessLevel"]["mnd-s:hasMutuallyExclusiveAccessLevel"]["v-s:registrationNumberAdd"]
    // (mnd-s:hasTemporaryAccessLevel и mnd-s:hasMutuallyExclusiveAccessLevel могуть быть множественными)

    let mut alindvs = set_hashset_from_field_field(backend, indv_c, "mnd-s:hasTemporaryAccessLevel", "v-s:registrationNumberAdd", access_levels);
    for aclv in alindvs.iter_mut() {
        set_hashset_from_field_field(backend, aclv, "mnd-s:hasMutuallyExclusiveAccessLevel", "v-s:registrationNumberAdd", &mut mutually_exclusive_levels);
    }

    // 2) запрашиваем данные карты, чтобы получить уровни доступа, которые указаны сейчас в  PW, запоминаем [уровни доступа]
    // cardNumber = [C]["mnd-s:hasSourceDataRequestForPass"]["mnd-s:cardNumber"]

    let res_card = ctx.pw_api_client.badging_api().badges_cards_card(card_number);
    let card = res_card.unwrap();
    if card.is_object() {
        let permanent_levels = get_levels(card);

        for (clear_code_id, valid_to) in permanent_levels {
            if mutually_exclusive_levels.contains(&clear_code_id) {
                delete_level(ctx, card_number, &clear_code_id)?;

                let is_permanent_level = valid_to.is_empty() || valid_to == S_MAX_TIME;

                let d_valid_to = if valid_to.is_empty() {
                    str_date_to_i64(S_MAX_TIME, Some(Duration::seconds(offset))).unwrap_or_default()
                } else {
                    str_date_to_i64(&valid_to, Some(Duration::seconds(offset))).unwrap_or_default()
                };

                let date_to_plan = indv_c.get_first_datetime("v-s:dateToPlan").unwrap_or_default();

                if is_permanent_level || (!is_permanent_level && d_valid_to > date_to_plan) {
                    // - добавление в виде временных
                    add_level(ctx, card_number, &clear_code_id, set_next_day_and_00_00_00(indv_c.get_first_datetime("v-s:dateToPlan")), Some(d_valid_to))?;
                }
            }
        }
    }

    //4) предварительно удаляем и после добавляем временные уровни доступа из заявки
    for clear_code_id in access_levels.iter() {
        delete_level(ctx, card_number, clear_code_id)?;
        add_level(ctx, card_number, &clear_code_id, Some(get_now_00_00_00().timestamp()), set_next_day_and_00_00_00(indv_c.get_first_datetime("v-s:dateToPlan")))?;
    }

    Ok(())
}

fn delete_level(ctx: &mut Context, card_number: &str, clear_code_id: &str) -> Result<(), (ResultCode, String)> {
    if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_card_clearcodes_clearcode(card_number, clear_code_id) {
        error!("to PW: delete_level : err={:?}", e);
        return Err((ResultCode::FailStore, format!("{:?}", e)));
    }

    Ok(())
}

fn add_level(ctx: &mut Context, card_number: &str, clear_code_id: &str, date_from: Option<i64>, date_to: Option<i64>) -> Result<(), (ResultCode, String)> {
    let sj1 = access_level_to_json_for_add(clear_code_id, true, date_from, date_to);
    if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_card_update_access_levels(card_number, json!(sj1)) {
        error!("to PW: badges_cards_card_update_access_levels: err={:?}", e);
        return Err((ResultCode::FailStore, format!("{:?}", e)));
    } else {
        info!("to PW: badges_cards_card_update_access_levels, card_number={}", card_number);
    }

    Ok(())
}

/*
pub fn set_time(date: Option<i64>, hour: u32, min: u32, sec: u32) -> Option<i64> {
    if let Some(dd) = date {
        let d = NaiveDateTime::from_timestamp(dd, 0);
        let d_0 = NaiveDate::from_ymd(d.year(), d.month(), d.day()).and_hms(hour, min, sec);
        Some(d_0.timestamp())
    } else {
        None
    }
}
*/

pub fn set_next_day_and_00_00_00(date: Option<i64>) -> Option<i64> {
    if let Some(dd) = date {
        let d = NaiveDateTime::from_timestamp(dd, 0);
        let d_0 = NaiveDate::from_ymd(d.year(), d.month(), d.day()).and_hms(00, 00, 00);
        Some(d_0.add(Duration::days(1)).timestamp())
    } else {
        None
    }
}

pub fn get_now_00_00_00() -> NaiveDateTime {
    let d = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
    let d_0 = NaiveDate::from_ymd(d.year(), d.month(), d.day()).and_hms(0, 0, 0);
    d_0
}

pub fn get_now_23_59_59() -> NaiveDateTime {
    let d = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
    let d_0 = NaiveDate::from_ymd(d.year(), d.month(), d.day()).and_hms(23, 59, 59);
    d_0
}

pub fn set_23_59_59(d: Option<i64>) -> Option<i64> {
    if let Some(d0) = d {
        let d = NaiveDateTime::from_timestamp(d0, 0);
        let d_0 = NaiveDate::from_ymd(d.year(), d.month(), d.day()).and_hms(23, 59, 59);
        return Some(d_0.timestamp());
    }
    None
}

pub fn i64_to_str_date_ymdthms(date: Option<i64>) -> String {
    if let Some(date_to) = date {
        NaiveDateTime::from_timestamp(date_to, 0).format("%Y-%m-%dT%H:%M:%S").to_string()
    } else {
        String::new()
    }
}

pub fn i64_to_str_date(date: Option<i64>, format: &str) -> String {
    if let Some(date_to) = date {
        NaiveDateTime::from_timestamp(date_to, 0).format(format).to_string()
    } else {
        String::new()
    }
}

pub fn send_message_of_status_lock_unlock(ctx: &mut Context, backend: &mut Backend, indv_p: &mut Individual, need_lock: bool) -> Result<(), (ResultCode, String)> {
    let owner = if let Some(owner) = indv_p.get_first_literal("mnd-s:passVehicleRegistrationNumber") {
        owner
    } else {
        let mut indv = get_individual_from_predicate(backend, indv_p, "mnd-s:lockedPerson")?;
        indv.get_first_literal("rdfs:label").unwrap_or_default()
    };

    let (reason_uri, reason_txt) = if let Ok(mut indv) = get_individual_from_predicate(backend, indv_p, "v-s:hasLockedReason") {
        (indv.get_id().to_owned(), indv.get_first_literal("rdfs:label").unwrap_or_default())
    } else {
        ("".to_owned(), "".to_owned())
    };

    let mut i = get_individual_from_predicate(backend, indv_p, "v-s:responsibleOrganization")?;
    let to = get_individual_from_predicate(backend, &mut i, "v-s:hasContractorProfileSafety")?.get_first_literal("mnd-s:responsiblePersons");

    let mut message = Individual::default();
    message.set_id(&("d:msg_".to_owned() + &Uuid::new_v4().to_string()));
    message.set_uri("rdf:type", "v-s:Email");

    if let Some(v) = to {
        message.set_uri("v-wf:to", &v);
    }

    if need_lock {
        if reason_uri == "d:c820270f5f424107a5c54bfeeebfa095" {
            if let Some(v) = indv_p.get_first_literal("v-s:creator") {
                message.set_uri("v-wf:from", &v);
            }
        }

        if reason_uri == "d:a0aoowjbm91ef2lw57c8lo29772" {
            message.set_string("v-s:senderMailbox", "DocFlow.Syktyvkar@mondigroup.com", Lang::none());
        }

        message.set_string("v-s:subject", "Optiflow. Уведомление: Заблокирован пропуск", Lang::new_from_str("RU"));
        message.set_string(
            "v-s:messageBody",
            &format!(
                " \
        Выполнена блокировка карт: {} \n \
        Причина блокировки: {} \n \
        \n
        Это сообщение сформировано автоматически. Отвечать на него не нужно. \n \
        Система Optiflow ",
                owner, reason_txt
            ),
            Lang::new_from_str("RU"),
        );
    } else {
        message.set_string("v-s:senderMailbox", "DocFlow.Syktyvkar@mondigroup.com", Lang::none());

        message.set_string("v-s:subject", "Optiflow. Уведомление: Разблокирован пропуск", Lang::new_from_str("RU"));
        message.set_string(
            "v-s:messageBody",
            &format!(
                " \
        Выполнена разблокировка карт: {} \n \
        Причина блокировки: {} \n \
        \n
        Это сообщение сформировано автоматически. Отвечать на него не нужно. \n \
        Система Optiflow ",
                owner, reason_txt
            ),
            Lang::new_from_str("RU"),
        );
    }

    match backend.mstorage_api.update_use_param(&ctx.sys_ticket, "prowatch", "", 0, IndvOp::Put, &message) {
        Ok(_) => {
            info!("success update, uri={}", message.get_id());
        },
        Err(e) => {
            error!("fail update, uri={}, result_code={:?}", message.get_id(), e.result);
        },
    }

    Ok(())
}

pub fn get_badge_id_of_card_number(ctx: &mut Context, card_number: &str) -> Result<String, ResultCode> {
    let res_card = ctx.pw_api_client.badging_api().badges_cards_card(card_number);
    if let Err(e) = res_card {
        error!("badges_cards_card: err={:?}", e);
        return match e {
            Error::Reqwest(_) => Err(ResultCode::UnprocessableEntity),
            Error::Serde(_) => Err(ResultCode::UnprocessableEntity),
            Error::Io(_) => Err(ResultCode::ConnectError),
        };
    }

    let card = res_card.unwrap();
    if !card.is_object() {
        return Err(ResultCode::UnprocessableEntity);
    }
    if let Some(s) = get_str_from_value(&card, "BadgeID") {
        return Ok(s.to_owned());
    }
    Err(ResultCode::UnprocessableEntity)
}
