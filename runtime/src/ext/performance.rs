use std::time::Instant;

use andromeda_core::{Extension, ExtensionOp, HostData, OpsStorage};
use nova_vm::ecmascript::{
    builtins::ArgumentsList,
    execution::{Agent, JsResult},
    types::Value,
};

use crate::RuntimeMacroTask;

pub struct PerformanceResource {
    pub start_time: Instant,
}

#[derive(Default)]
pub struct PerformanceExt;

impl PerformanceExt {
    pub fn new_extension() -> Extension {
        Extension {
            name: "performance",
            ops: vec![ExtensionOp::new("internal_now", Self::internal_now, 0)],
            storage: Some(Box::new(|storage: &mut OpsStorage| {
                storage.insert(PerformanceResource {
                    start_time: Instant::now(),
                });
            })),
        }
    }

    /// Returns the number of milliseconds since the start of the program.
    fn internal_now(agent: &mut Agent, _this: Value, _args: ArgumentsList) -> JsResult<Value> {
        let host_data = agent.get_host_data();
        let host_data: &HostData<RuntimeMacroTask> = host_data.downcast_ref().unwrap();
        let storage = host_data.storage.borrow();
        let state = storage.get::<PerformanceResource>().unwrap();
        let start_time = state.start_time;
        let elapsed = start_time.elapsed();
        let seconds = elapsed.as_secs();
        let subsec_nanos = elapsed.subsec_nanos();

        // If the permission is not enabled
        // Round the nano result on 2 milliseconds
        // see: https://developer.mozilla.org/en-US/docs/Web/API/DOMHighResTimeStamp#Reduced_time_precision
        // TODO: Implement a way to enable/disable this behavior

        let _ms = (seconds as f64 * 1000.0) + (subsec_nanos as f64 / 1_000_000.0);
        
        Ok(Value::pos_zero())
    }
}
