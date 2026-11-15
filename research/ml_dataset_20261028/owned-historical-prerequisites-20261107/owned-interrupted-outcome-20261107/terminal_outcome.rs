pub(super) fn terminal_outcome(
    worker: Value,
    stop: Option<&str>,
    exit_success: bool,
    exit_code: Option<i32>,
) -> Value {
    if let Some(reason) = stop {
        return serde_json::json!({"status":reason,"solver_completed":false});
    }
    if !exit_success {
        return serde_json::json!({"status":"worker_failed","exit_code":exit_code,"solver_completed":false});
    }
    worker
}
