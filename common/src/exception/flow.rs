use crate::entity::error::DispatchErr;

// 流调度错误统一处理器
pub fn flow_dispatch_err_handler(err: DispatchErr) -> Result<(), DispatchErr> {
    match err {
        DispatchErr::FlowFailed(_) => { Ok(()) }
        DispatchErr::RedressFailed => { Ok(()) }
        DispatchErr::RunOverLimit => { Ok(()) }
        _ => { Ok(()) }
    }
}