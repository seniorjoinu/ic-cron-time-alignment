use ic_cdk::api::time;
use ic_cdk::export::candid::export_service;
use ic_cdk::print;
use ic_cdk::storage::{stable_restore, stable_save};
use ic_cdk_macros::{heartbeat, post_upgrade, pre_upgrade, query, update};
use ic_cron::implement_cron;
use ic_cron::types::{Iterations, ScheduledTask, SchedulingInterval, TaskId};

use crate::common::{DayOfWeek, TimeNanos, NANOS_IN_WEEK};

pub mod common;

#[update]
pub fn greet_each(weekday: DayOfWeek, name: String) -> TaskId {
    cron_enqueue(
        name,
        SchedulingInterval {
            delay_nano: time().nanos_till_next(weekday),
            interval_nano: NANOS_IN_WEEK,
            iterations: Iterations::Infinite,
        },
    )
    .expect("Unable to enqueue a new cron task")
}

#[query]
pub fn list_tasks() -> Vec<ScheduledTask> {
    get_cron_state().get_tasks()
}

#[update]
pub fn dequeue_task(id: TaskId) -> Option<ScheduledTask> {
    cron_dequeue(id)
}

#[heartbeat]
pub fn tick() {
    for task in cron_ready_tasks() {
        let name: String = task.get_payload().expect("Unable to deserialize a name");

        print(format!("Hello, {}", name).as_str());
    }
}

#[pre_upgrade]
pub fn pre_upgrade_hook() {
    stable_save((get_cron_state().clone(),)).expect("Unable to save the state to stable memory");
}

#[post_upgrade]
pub fn post_upgrade_hook() {
    let (cron_state,) = stable_restore().expect("Unable to restore the state from stable memory");

    set_cron_state(cron_state);
}

implement_cron!();

// ---------------- CANDID -----------------------

export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
