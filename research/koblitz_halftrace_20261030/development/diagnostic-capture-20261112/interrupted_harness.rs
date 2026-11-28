// Synthetic libtest output fixture, not an index-calculus workload.
#[test]
fn fails_before_completion() {
    panic!("diagnostic-fixture-panic");
}

#[test]
fn waits_for_external_termination() {
    loop {
        std::thread::park();
    }
}
