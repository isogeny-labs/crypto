# Independent journal fault qualification

Prepared by /root/materializer_qualification under T-1955. Execution has not occurred.

## Scope and sources

Read the materializer-source-successor-review-20261107 and materializer-journal-reader-review-20261107 review.json records, actual candidate record_cleanup/checkpoint source, and reviewed receipt_reader.verify implementation. KB MCP and wrapper are unavailable in this runtime; these exact primary repository sources are the fallback. The producer's source review leaves storage failure behavior open. This qualification tests deterministic software fault behavior; it does not simulate power loss, establish physical durability, authenticate cleanup semantics or terminal custody, or measure materialization performance.

The candidate is SHA256 6bfc278cdfb8cd5f2adad13f5e920a2baee131be9f7dedc4b472329f2f0da4f7. The reader is SHA256 021215e4b46f84c63e0b297acc427fce244bbe675647fe13b88eb60a0326e2e9. The harness additionally requires exact AST equivalence of its two extracted functions to reviewed source085234b563a2c766040706ed7d805b2971074efba14da022265a6aa6db33ca63. No materializer top-level code or child command is executed.

## Frozen cases and decision

Fifteen cases run in fixed order: normal intermediate/final; partial unacknowledged tail; first-file fsync failure; first-directory fsync failure; append fsync failure; append write exception; append short write; append flush exception; checkpoint replace exception; acknowledged-prefix corruption rejection; normal A/A repeat; before-first-append interruption; durable append before checkpoint; checkpoint directory fsync failure; pointer-count mismatch rejection. Fixed order is appropriate for independent deterministic directories, with no elapsed-time comparative claim. The normal final is the positive storage control and corrupted prefix is the required negative. A/A is an exact repetition control, and statistical power is inapplicable to exact assertions; every fixed case must pass and the expected count must equal15. Each fixture uses two tiny synthetic cleanup dictionaries. Result rows preserve actual failures; no retries. Failure injection affects only the extracted function environment or a narrowly scoped Path.open wrapper. Reader execution follows removal of the injection.

## Resource and publication gate

Reserve and conservatively charge60 active worker-seconds from campaign-renewal-20261109, not120. One invocation only, serial with every other payload. Publish this exact harness, plan and manifest and record GitHub readback before root release. Internal work alarm50s; outer timeout55s followed by KILL5s. Use one CPU thread, at most512MiB memory and a fresh local /var/tmp output directory; all fixture files together are expected below64KiB, with an outer1MiB file-size limit. This is local bounded source qualification for the authorized local preparation path, not a scientific cohort or cloud experiment. Retain stderr/stdout, exit, runtime limits and result.json; if alarm or external timeout prevents a result, retain that as incomplete/failed and charge60s. No source test has yet executed.

Command after published/released exact bindings (working directory repository root):

```
ulimit -v 524288
ulimit -f 1024
/usr/bin/timeout --signal=TERM --kill-after=5s 55s python3 -B research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/materializer-qualification-20261109/qualify.py --output /var/tmp/pencil-materializer-qualification-20261109
```

Each ulimit is confined to the qualification subshell; do not propagate limits into materializer/native compiler execution. No process group or materializer service is created by the harness. Root owns capture, timeout and accounting. Result contains scoped booleans for materializer/compiler execution, power-loss durability and terminal custody, all false. A successful run supports only the fifteen frozen software storage cases; complete materialization and native custody require the separate admitted attempt.
