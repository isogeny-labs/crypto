# Focused actual-source recovery tests

This package imports the maintained SAT and initial-event codec files directly. It does not stub arithmetic or change their source tests. Run cargo test --release --lib here. The47existing tests pass after source-history recovery. This scope does not substitute for the full crypto_lib library build, which is still blocked elsewhere. No timing or15x result follows. [pcc:unverified]
