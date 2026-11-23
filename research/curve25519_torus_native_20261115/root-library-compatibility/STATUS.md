# Broad library compatibility check: incomplete

The development run of `cargo test --release --lib` was stopped after 51
minutes (including compilation); it did not complete and is not a passing check.
Its partial raw output is preserved in incomplete-run.log. The focused pricing
failure is missing the embedded records.jsonl path shown in pricing-failure.log.
No library source was changed: both the initial base 84fd610de0727b15cb009951392f7b21fea585df
and the feature head have src tree 279a88e7c2c4b5c6dfcbd376bcff261778966434.
Other reported failures were not fully diagnosed. Do not call this a clean
baseline reproduction or infer that the missing fixture explains all failures.
The standalone Rust compiler and consumer controls are separate checks.
All observations are producer evidence. [pcc:unverified]
