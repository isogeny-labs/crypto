# SAT validation recovery, 2026-11-07

The original full-library run reached its 3,600-second timeout (exit 124), with no final test summary and unchanged source. It did not pass the publication gate. The later corrected CLI/native release build completed successfully, but its ephemeral binary and source directory were missing after interruption. The runtime preflight stopped before native execution.

All 1,442 inputs were reconstructed with identical hashes from the previously archived source and the reviewed V4 overlay. A bounded local rebuild is in progress; its eventual result is not asserted here. No new scientific cloud job, budget, deadline extension or dataset measurement is represented.

To reconstruct: download and verify the archive named in base-source-s3.receipt.json, then use its tested-source directory as the base and overlay sat-cli-native-reviewed-composition-20261105-v4/proposed. Select exactly the relative paths in sat-cli-native-recovery-20261107/source-hashes.json and verify every SHA-256. That manifest is identical to the successful build input manifest. The recovery source directory contains no additional source files.

## Subsequent migration interruption

The recovered-source rebuild was interrupted before a completion receipt. Its runner and Cargo PIDs were absent and the tool session was unknown; the source and partial target cache remained. The saved attempt allowed zero retries. One additional local build attempt is prepared with execution disabled pending owner approval; no retry is claimed here. See rebuild-interruption.receipt.json and migration-reconciliation.json. No EKS job or Diem generator was restarted.
