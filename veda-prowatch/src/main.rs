#[macro_use]
extern crate log;

mod common;
mod from_prowatch;
mod to_prowatch;

use crate::common::{load_access_level_dir, Context};
use crate::from_prowatch::sync_data_from_prowatch;
use crate::to_prowatch::{delete_from_prowatch, insert_to_prowatch, set_update_status, update_prowatch_data};
use prowatch_client::apis::client::PWAPIClient;
use prowatch_client::apis::configuration::Configuration;
use std::thread;
use std::time as std_time;
use url::Url;
use v_module::common::load_onto;
use v_module::info::ModuleInfo;
use v_module::module::{get_cmd, get_info_of_module, get_inner_binobj_as_individual, init_log, wait_load_ontology, wait_module, Module, PrepareError};
use v_module::v_api::app::ResultCode;
use v_module::v_api::IndvOp;
use v_module::v_onto::individual::Individual;
use v_module::v_onto::onto::Onto;
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
    let sys_ticket;
    if let Ok(t) = module.get_sys_ticket_id() {
        sys_ticket = t;
    } else {
        error!("fail get system ticket");
        return Ok(());
    }

    let mut onto = Onto::default();

    info!("load onto start");
    load_onto(&mut module.storage, &mut onto);
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

    let mut ctx = Context {
        sys_ticket: sys_ticket.to_owned(),
        onto,
        pw_api_client: PWAPIClient::new(configuration),
        access_level_dict: load_access_level_dir(&mut module, &sys_ticket),
    };

    module.listen_queue(
        &mut queue_consumer,
        &mut module_info.unwrap(),
        &mut ctx,
        &mut (before_batch as fn(&mut Module, &mut Context, batch_size: u32) -> Option<u32>),
        &mut (prepare as fn(&mut Module, &mut ModuleInfo, &mut Context, &mut Individual, my_consumer: &Consumer) -> Result<bool, PrepareError>),
        &mut (after_batch as fn(&mut Module, &mut ModuleInfo, &mut Context, prepared_batch_size: u32) -> Result<bool, PrepareError>),
        &mut (heartbeat as fn(&mut Module, &mut ModuleInfo, &mut Context) -> Result<(), PrepareError>),
    );
    Ok(())
}

fn heartbeat(_module: &mut Module, _module_info: &mut ModuleInfo, _ctx: &mut Context) -> Result<(), PrepareError> {
    Ok(())
}

fn before_batch(_module: &mut Module, _ctx: &mut Context, _size_batch: u32) -> Option<u32> {
    None
}

fn after_batch(_module: &mut Module, _module_info: &mut ModuleInfo, _ctx: &mut Context, _prepared_batch_size: u32) -> Result<bool, PrepareError> {
    Ok(false)
}

fn prepare(module: &mut Module, _module_info: &mut ModuleInfo, ctx: &mut Context, queue_element: &mut Individual, _my_consumer: &Consumer) -> Result<bool, PrepareError> {
    if let Err(e) = prepare_queue_element(module, ctx, queue_element) {
        error!("fail prepare queue element, err={:?}", e);
        if e == ResultCode::ConnectError {
            error!("sleep and repeate...");
            thread::sleep(std_time::Duration::from_millis(10000));
            return Err(PrepareError::Recoverable);
        }
    }
    Ok(true)
}

fn prepare_queue_element(module: &mut Module, ctx: &mut Context, queue_element: &mut Individual) -> Result<(), ResultCode> {
    let cmd = get_cmd(queue_element).unwrap_or(IndvOp::None);
    if cmd == IndvOp::None {
        error!("cmd is none");
        return Ok(());
    }

    //let signal = queue_element.get_first_literal("src").unwrap_or_default();

    let mut new_state_indv = Individual::default();
    get_inner_binobj_as_individual(queue_element, "new_state", &mut new_state_indv);

    let itypes = new_state_indv.get_literals("rdf:type").unwrap_or_default();

    if cmd != IndvOp::Remove && new_state_indv.is_empty() {
        return Ok(());
    }

    if let Some(v) = new_state_indv.get_first_literal("v-s:lastEditor") {
        if v == "cfg:VedaSystemAppointment" {
            return Ok(());
        }
    }

    for itype in itypes {
        if itype == "mnd-s:SourceDataRequestForPass" {
            let res = sync_data_from_prowatch(module, ctx, &mut new_state_indv);
            if res == ResultCode::ConnectError {
                return Err(res);
            }
        } else if itype == "mnd-s:SourceDataRequestForPassByNames" {
            // ПРОВЕРКА НАЛИЧИЯ ДЕРЖАТЕЛЕЙ В СКУД ?
            let res = sync_data_from_prowatch(module, ctx, &mut new_state_indv);
            if res == ResultCode::ConnectError {
                return Err(res);
            }
        } else if itype == "v-s:ExternalModuleHandler" {
            let module_label = new_state_indv.get_first_literal("v-s:moduleLabel").unwrap_or_default();

            if module_label == "winpak pe44 create" {
                let upd_res = insert_to_prowatch(module, ctx, &mut new_state_indv);
                let res = set_update_status(module, ctx, &mut new_state_indv, upd_res);
                if res == ResultCode::ConnectError {
                    return Err(res);
                }
            }

            if module_label == "winpak pe44 update" {
                // ДОБАВЛЕНИЕ НОВОЙ КАРТЫ ДЕРЖАТЕЛЮ
                let upd_res = update_prowatch_data(module, ctx, &mut new_state_indv);
                let res = set_update_status(module, ctx, &mut new_state_indv, upd_res);
                if res == ResultCode::ConnectError {
                    return Err(res);
                }
            }

            if module_label == "winpak pe44 delete" {
                let res = delete_from_prowatch(module, ctx, &mut new_state_indv);
                if res == ResultCode::ConnectError {
                    return Err(res);
                }
            }
        }
    }

    Ok(())
}
