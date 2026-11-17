# Prime F4 chain software validation

Source: `69e4c9763bed508f0ab0d2f424873d2e748a7229`. These are software correctness and delivery checks, not a cryptanalytic experiment or generated cloud corpus.

The full release library suite passed **5,153 tests, zero failures, 123 ignored**. Three focused native trace tests and eight focused prime adapter tests passed. The separate endpoint test passed all 360 combinations of n23/61, three families, two twist settings, m2/4/15 and ten adapter names; unsupported and capped outcomes remain permitted, so this is not 360 solved instances. Release CLI compilation and four actual-binary capability/help checks passed.

Strict maintained Clippy failed on exactly three unused startup helpers: `ActivationPhase`, `validate_launch_delivery`, and `BudgetSession::activation_phase`. The reviewed startup module and central prime-chain replay registration await coordination with T766, which still holds `src/ml_dataset/mod.rs`. Neither proposed registration is counted as compiled or integrated. No lint warning was suppressed.

The earlier compile failure, initial full-suite failure (5,128 passed, 25 failed, 123 ignored), lint launch failure and missing-target lint failure are retained. Missing tracked fixtures and Cargo targets were restored from Git without changing their content; all 25 failed runtime tests then passed, followed by the clean full-suite run above. See the restoration inventories and per-command receipts. Source readback was checked before and after each validation command.

Raw logs, receipts and the release binary are content-addressed in `.artifacts.json`. `remote-readback-initial.json` describes the first 21 objects; `remote-readback.json` describes the complete 36-object fresh GET, size and SHA-256 verification. The latter includes the actual release binary. The binary was built from the stated source, before later documentation-only commits.

Native F4 still requires p<2^32. Wide-prime equations are preserved but are not passed to the narrow backend. Positive fixture coverage, m15 equation checks, restricted recursive traces and generation-time input verification do not establish every requested solve, complete native intermediate tracing, exact first/last fall degrees, trained ML models or novel SAT algorithms. **Verified native cloud dataset cells: 0.** Controller S3 archive access does not establish worker IAM.
