use chrono::offset::LocalResult::Single;
use chrono::DateTime;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use prowatch_client::apis::client::PWAPIClient;
use prowatch_client::apis::Error;
use serde_json::json;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::ops::{Add, Sub};
use uuid::Uuid;
use v_module::module::Module;
use v_module::v_api::app::ResultCode;
use v_module::v_api::IndvOp;
use v_module::v_onto::datatype::Lang;
use v_module::v_onto::individual::Individual;
use v_module::v_onto::onto::Onto;
use v_module::v_search::common::FTQuery;
use voca_rs::chop;

#[derive(Debug, PartialEq, Clone)]
pub enum PassType {
    Vehicle,
    Human,
    Unknown,
}

pub const WINPAK_TIMEZONE: i64 = 0;

pub const CARD_NUMBER_FIELD_NAME: &str = "mnd-s:cardNumber";

pub struct Context {
    pub sys_ticket: String,
    pub onto: Onto,
    pub pw_api_client: PWAPIClient,
    pub access_level_dict: HashMap<String, String>,
}

pub fn load_access_level_dir(module: &mut Module, sys_ticket: &str) -> HashMap<String, String> {
    let mut dir = HashMap::new();
    let res = module.fts.query(FTQuery::new_with_ticket(&sys_ticket, "'rdf:type'=='mnd-s:AccessLevel'"));
    if res.result_code == ResultCode::Ok && res.count > 0 {
        for id in &res.result {
            if let Some(indv) = module.get_individual(id, &mut Individual::default()) {
                if let Some(s) = indv.get_first_literal("v-s:registrationNumberAdd") {
                    dir.insert(s, indv.get_id().to_owned());
                }
            }
        }
    }
    dir
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

pub fn get_individual_from_predicate(module: &mut Module, src: &mut Individual, predicate: &str) -> Result<Individual, (ResultCode, String)> {
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

pub fn clear_card_and_set_err(module: &mut Module, sys_ticket: &str, indv: &mut Individual, err_text: &str) {
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
    indv.set_string("v-s:errorMessage", err_text, Lang::RU);
    indv.set_uri("v-s:lastEditor", "cfg:VedaSystemAppointment");

    let res = module.api.update(sys_ticket, IndvOp::Put, indv);
    if res.result != ResultCode::Ok {
        error!("fail update, uri={}, result_code={:?}", indv.get_id(), res.result);
    } else {
        info!("success update, uri={}", indv.get_id());
    }
}

pub fn set_err(module: &mut Module, sys_ticket: &str, indv: &mut Individual, err_text: &str) {
    indv.parse_all();
    indv.set_string("v-s:errorMessage", err_text, Lang::RU);
    indv.set_uri("v-s:lastEditor", "cfg:VedaSystemAppointment");

    let res = module.api.update(sys_ticket, IndvOp::Put, indv);
    if res.result != ResultCode::Ok {
        error!("fail update, uri={}, result_code={:?}", indv.get_id(), res.result);
    } else {
        info!("success update, uri={}", indv.get_id());
    }
}

pub fn set_vec_from_field_field(module: &mut Module, indv: &mut Individual, predicate: &str, innner_predicate: &str, out_data: &mut Vec<String>) {
    if let Some(uris) = indv.get_literals(predicate) {
        for l in uris {
            if let Some(mut indv_c) = module.get_individual_h(&l) {
                if let Some(al) = indv_c.get_first_literal(innner_predicate) {
                    out_data.push(al);
                }
            } else {
                error!("not found {}", l);
            }
        }
    }
}

pub fn set_str_from_field_field(module: &mut Module, indv: &mut Individual, predicate: &str, innner_predicate: &str) -> String {
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

pub fn get_now_00_00_00() -> NaiveDateTime {
    let d = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
    let d_0 = NaiveDate::from_ymd(d.year(), d.month(), d.day()).and_hms(0, 0, 0);
    d_0
}

pub fn i64_to_str_date_ymdthms(date: Option<i64>) -> String {
    if let Some(date_to) = date {
        NaiveDateTime::from_timestamp(date_to, 0).add(Duration::hours(WINPAK_TIMEZONE)).format("%Y-%m-%dT%H:%M:%S").to_string()
    } else {
        String::new()
    }
}

pub fn i64_to_str_date_mdy(date: Option<i64>) -> String {
    if let Some(date_to) = date {
        NaiveDateTime::from_timestamp(date_to, 0).add(Duration::hours(WINPAK_TIMEZONE)).format("%m.%d.%Y").to_string()
    } else {
        String::new()
    }
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

pub fn str_value2indv(src_val: &Value, src_field: &str, dest_indv: &mut Individual, dest_field: &str) {
    if let Some(v1) = src_val.get(src_field) {
        if let Some(s1) = v1.as_str() {
            dest_indv.add_string(dest_field, s1, Lang::NONE);
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

pub fn get_badge_use_request_indv(module: &mut Module, ctx: &mut Context, pass_type: Option<PassType>, indv_p: &mut Individual) -> (PassType, Result<Vec<Value>, Error>) {
    let tpass: PassType = if let Some(pt) = pass_type {
        pt
    } else {
        get_pass_type(indv_p)
    };

    if tpass != PassType::Unknown {
        let res_badge = if tpass == PassType::Vehicle {
            let car_number = indv_p.get_first_literal("mnd-s:passVehicleRegistrationNumber").unwrap_or_default();

            ctx.pw_api_client.badging_api().badges_key_key_value("BADGE_FNAME", &format!("%25{}%25", &car_number))
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
                            first_name = employee.get_first_literal_with_lang("v-s:firstName", &[Lang::RU, Lang::NONE]).unwrap_or_default();
                            last_name = employee.get_first_literal_with_lang("v-s:lastName", &[Lang::RU, Lang::NONE]).unwrap_or_default();
                            middle_name = employee.get_first_literal_with_lang("v-s:middleName", &[Lang::RU, Lang::NONE]).unwrap_or_default();
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

pub fn get_literal_of_link(module: &mut Module, indv: &mut Individual, link: &str, field: &str) -> Option<String> {
    module.get_literal_of_link(indv, link, field, &mut Individual::default())
}

pub fn access_levels_to_json_for_add(access_levels: Vec<String>, is_tmp_update_access_levels: bool, date_to: Option<i64>) -> Vec<Value> {
    let mut sj: Vec<Value> = Vec::new();
    for lvl in access_levels {
        let sji = if is_tmp_update_access_levels {
            json!({
            "ClearCodeID": lvl,
            "ClearCodeType": 3,
            "ValidFrom": i64_to_str_date_ymdthms (Some(get_now_00_00_00().timestamp())),
            "ValidTo": i64_to_str_date_ymdthms (date_to)
            })
        } else {
            json!({ "ClearCodeID": lvl })
        };

        sj.push(sji);
    }
    sj
}

pub fn access_levels_to_json_for_new(access_levels: Vec<String>) -> Vec<Value> {
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
    fields
}

pub fn create_asc_record(el: &Value, backward_id: &str) -> Individual {
    let mut acs_record = Individual::default();
    acs_record.set_id(&("d:asc_".to_owned() + &Uuid::new_v4().to_string()));
    acs_record.set_uri("rdf:type", "mnd-s:ACSRecord");
    acs_record.set_uri("v-s:backwardProperty", "mnd-s:hasACSRecord");
    acs_record.set_uri("v-s:backwardTarget", backward_id);
    acs_record.set_bool("v-s:canRead", true);

    set_badge_to_indv(&el, &mut acs_record);

    acs_record
}

pub fn set_badge_to_indv(el: &Value, dest: &mut Individual) {
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

    let fields = get_custom_badge_as_list(el);
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

pub fn set_card_status(ctx: &mut Context, card_number: &str, card_status: i32) -> Result<(), (ResultCode, String)> {
    let cnj = json!({
        "CardNumber": card_number,
        "CardStatus": card_status,
    });
    if let Err(e) = ctx.pw_api_client.badging_api().badges_cards_put(cnj) {
        error!("block cards if exist temp card: badges_cards_card: err={:?}", e);
        return Err((ResultCode::FailStore, format!("{:?}", e)));
    }
    Ok(())
}

pub fn set_update_status(
    module: &mut Module,
    ctx: &mut Context,
    indv: &mut Individual,
    res: Result<(), (ResultCode, String)>,
    status_if_err: &str,
    status_if_ok: &str,
) -> ResultCode {
    indv.parse_all();
    if let Err((sync_res, info)) = res {
        if sync_res == ResultCode::ConnectError {
            return sync_res;
        }
        indv.set_uri("v-s:hasStatus", status_if_err);
        set_err(module, &ctx.sys_ticket, indv, &info);
        return sync_res;
    }

    indv.set_uri("v-s:lastEditor", "cfg:VedaSystemAppointment");
    indv.set_uri("v-s:hasStatus", status_if_ok);
    indv.clear("v-s:errorMessage");

    let res = module.api.update(&ctx.sys_ticket, IndvOp::Put, indv);
    if res.result != ResultCode::Ok {
        error!("fail update, uri={}, result_code={:?}", indv.get_id(), res.result);
    } else {
        info!("success update, uri={}", indv.get_id());
    }
    ResultCode::Ok
}

pub fn str_date_to_i64(value: &str) -> Option<i64> {
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
            if let Single(offset) = Local.offset_from_local_datetime(&v) {
                return Some(v.sub(offset).timestamp());
            } else {
                return Some(v.timestamp());
            }
        } else {
            error!("fail parse [{}] to datetime", value);
        }
    }
    return None;
}
