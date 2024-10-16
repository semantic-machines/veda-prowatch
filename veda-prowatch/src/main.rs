#[macro_use]
extern crate log;

mod common;
mod from_prowatch;
mod lock;
mod to_prowatch;

use crate::common::{clear_card_and_set_err, load_access_level_dict, set_update_status, Context, PassType};
use crate::from_prowatch::sync_data_from_prowatch;
use crate::lock::{lock_holder, lock_unlock_card};
use crate::to_prowatch::{delete_from_prowatch, insert_to_prowatch, update_prowatch_data};
use prowatch_client::apis::client::PWAPIClient;
use prowatch_client::apis::configuration::Configuration;
use std::process::exit;
use std::thread;
use std::time as std_time;
use url::Url;
use v_common::module::common::load_onto;
use v_common::module::info::ModuleInfo;
use v_common::module::module_impl::{get_cmd, get_info_of_module, get_inner_binobj_as_individual, init_log, wait_load_ontology, wait_module, Module, PrepareError};
use v_common::module::veda_backend::Backend;
use v_common::onto::individual::Individual;
use v_common::onto::onto_impl::Onto;
use v_common::storage::common::StorageMode;
use v_common::v_api::api_client::IndvOp;
use v_common::v_api::obj::ResultCode;
use v_queue::consumer::Consumer;

fn main() -> Result<(), i32> {
    init_log("VEDA-PROWATCH-CONNECTOR");

    if get_info_of_module("fulltext_indexer").unwrap_or((0, 0)).0 == 0 {
        wait_module("fulltext_indexer", wait_load_ontology());
    }

    listen_queue()
}

fn listen_queue<'a>() -> Result<(), i32> {
    let mut module = Module::default();
    let mut backend = Backend::create(StorageMode::ReadOnly, false);

    let sys_ticket;
    if let Ok(t) = backend.get_sys_ticket_id() {
        sys_ticket = t;
    } else {
        error!("fail get system ticket");
        return Ok(());
    }
    info!("ticket={}", sys_ticket);

    let mut onto = Onto::default();

    info!("load onto start");
    load_onto(&mut backend.storage, &mut onto);
    info!("load onto end");

    let module_info = ModuleInfo::new("./data", "prowatch-connector", true);
    if module_info.is_err() {
        error!("{:?}", module_info.err());
        return Err(-1);
    }

    //wait_load_ontology();

    let mut queue_consumer = Consumer::new("./data/queue", "prowatch_connector", "individuals-flow").expect("!!!!!!!!! FAIL QUEUE");

    let configuration;

    if let Some(s) = Module::get_property("prowatch_url") {
        if let Ok(conn) = Url::parse(&s) {
            let conn_str = format!("{}://{}:{}", conn.scheme(), conn.host().expect("invalid prowatch_uri"), conn.port().unwrap_or(0));
            info!("prowatch_url={}", conn_str);
            configuration = Configuration::new(&conn_str, conn.username(), conn.password().unwrap_or_default())
        } else {
            error!("invalid prowatch_url string {}", s);
            return Err(-1);
        }
    } else {
        error!("not found param prowatch_url");
        return Err(-1);
    }

    let dict = load_access_level_dict(&mut backend, &sys_ticket);
    if let Err(e) = dict {
        error!("{:?}", e);
        exit(-1);
    }

    let mut ctx = Context {
        sys_ticket: sys_ticket.to_owned(),
        onto,
        pw_api_client: PWAPIClient::new(configuration),
        access_level_dict: dict.unwrap(),
    };

    module.listen_queue(
        &mut queue_consumer,
        &mut ctx,
        &mut (before_batch as fn(&mut Backend, &mut Context, batch_size: u32) -> Option<u32>),
        &mut (prepare as fn(&mut Backend, &mut Context, &mut Individual, my_consumer: &Consumer) -> Result<bool, PrepareError>),
        &mut (after_batch as fn(&mut Backend, &mut Context, prepared_batch_size: u32) -> Result<bool, PrepareError>),
        &mut (heartbeat as fn(&mut Backend, &mut Context) -> Result<(), PrepareError>),
        &mut backend,
    );
    Ok(())
}

fn heartbeat(_module: &mut Backend, _ctx: &mut Context) -> Result<(), PrepareError> {
    Ok(())
}

fn before_batch(_module: &mut Backend, _ctx: &mut Context, _size_batch: u32) -> Option<u32> {
    None
}

fn after_batch(_module: &mut Backend, _ctx: &mut Context, _prepared_batch_size: u32) -> Result<bool, PrepareError> {
    Ok(false)
}

fn prepare(backend: &mut Backend, ctx: &mut Context, queue_element: &mut Individual, _my_consumer: &Consumer) -> Result<bool, PrepareError> {
    if let Err(e) = prepare_queue_element(backend, ctx, queue_element) {
        error!("fail prepare queue element, err={:?}", e);
        if e == ResultCode::ConnectError {
            error!("sleep and repeate...");
            thread::sleep(std_time::Duration::from_millis(10000));
            return Err(PrepareError::Recoverable);
        }
    }
    Ok(true)
}

fn prepare_queue_element(backend: &mut Backend, ctx: &mut Context, queue_element: &mut Individual) -> Result<(), ResultCode> {
    let cmd = get_cmd(queue_element).unwrap_or(IndvOp::None);
    if cmd == IndvOp::None {
        error!("cmd is none");
        return Ok(());
    }

    let mut new_state_indv = Individual::default();
    get_inner_binobj_as_individual(queue_element, "new_state", &mut new_state_indv);

    let itypes = new_state_indv.get_literals("rdf:type").unwrap_or_default();

    if cmd != IndvOp::Remove && new_state_indv.is_empty() {
        return Ok(());
    }

    if let Some(counter) = new_state_indv.get_first_integer("v-s:updateCounter") {
        if counter > 1 {
            return Ok(());
        }
    }

    for itype in itypes {
        if itype == "mnd-s:SourceDataRequestForPass" {
            if let Err((res, err_text)) = sync_data_from_prowatch(backend, ctx, &mut new_state_indv) {
                clear_card_and_set_err(backend, &ctx.sys_ticket, &mut new_state_indv, &err_text);
                return Err(res);
            }
        } else if itype == "mnd-s:SourceDataRequestForPassByNames" {
            // ПРОВЕРКА НАЛИЧИЯ ДЕРЖАТЕЛЕЙ В СКУД
            if let Some(tag) = new_state_indv.get_first_literal("v-s:tag") {
                if tag == "Auto" || tag == "Human" {
                    if let Err((res, err_text)) = sync_data_from_prowatch(backend, ctx, &mut new_state_indv) {
                        clear_card_and_set_err(backend, &ctx.sys_ticket, &mut new_state_indv, &err_text);
                        return Err(res);
                    }
                } else {
                    let upd_res = if tag == "AutoWithCompany" {
                        lock_holder(backend, ctx, PassType::Vehicle, &mut new_state_indv)
                    } else if tag == "HumanWithCompany" {
                        lock_holder(backend, ctx, PassType::Human, &mut new_state_indv)
                    } else {
                        Err((ResultCode::Ok, "unknown v-s:tag".to_owned()))
                    };

                    let res = set_update_status(backend, ctx, &mut new_state_indv, &upd_res, "v-s:StatusRejected", "v-s:StatusAccepted");
                    if res == ResultCode::ConnectError {
                        return Err(res);
                    }
                }
            }
        } else if itype == "v-s:ExternalModuleHandler" {
            let module_label = new_state_indv.get_first_literal("v-s:moduleLabel").unwrap_or_default();

            if module_label == "winpak pe44 create" {
                let upd_res = insert_to_prowatch(backend, ctx, &mut new_state_indv);
                let res = set_update_status(backend, ctx, &mut new_state_indv, &upd_res, "v-s:StatusRejected", "v-s:StatusAccepted");
                if res == ResultCode::ConnectError {
                    return Err(res);
                }
            }

            if module_label == "winpak pe44 update" {
                // ДОБАВЛЕНИЕ НОВОЙ КАРТЫ ДЕРЖАТЕЛЮ
                let upd_res = update_prowatch_data(backend, ctx, &mut new_state_indv);
                let res = set_update_status(backend, ctx, &mut new_state_indv, &upd_res, "v-s:StatusRejected", "v-s:StatusAccepted");
                if res == ResultCode::ConnectError {
                    return Err(res);
                }
            }

            if module_label == "winpak pe44 delete" {
                let res = delete_from_prowatch(backend, ctx, &mut new_state_indv);
                if res == ResultCode::ConnectError {
                    return Err(res);
                }
            }
            if module_label == "prowatch lock" {
                let upd_res = lock_unlock_card(backend, ctx, &mut new_state_indv, true);
                let res = set_update_status(backend, ctx, &mut new_state_indv, &upd_res, "v-s:StatusRejected", "v-s:StatusAccepted");
                if res == ResultCode::ConnectError {
                    return Err(res);
                }
            }
            if module_label == "prowatch unlock" {
                let upd_res = lock_unlock_card(backend, ctx, &mut new_state_indv, false);
                let res = set_update_status(backend, ctx, &mut new_state_indv, &upd_res, "v-s:StatusRejected", "v-s:StatusAccepted");
                if res == ResultCode::ConnectError {
                    return Err(res);
                }
            }
        }
    }

    Ok(())
}
