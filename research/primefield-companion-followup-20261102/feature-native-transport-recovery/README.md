# Adaptive normalized transport recovery

lib.rs preserves the quartic program and binds it to the normalized consumer.
Source::new takes public curve/base parameters and explicit max_lanes/max_window.
The total window limit includes the initial request and permitted extensions.
source-extra.rs and transport-extra.rs contain the consuming cursor extensions.
DERIVATION.md states the invariant, assumptions, lazy-skip behavior and costs.

From the repository root:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-transport-recovery/generate.py --check
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-transport-recovery/lib.rs -o /tmp/transport-recovery-tests
/tmp/transport-recovery-tests --test-threads=1 --nocapture
rustc --edition=2021 -O -C overflow-checks=yes research/primefield-companion-followup-20261102/feature-native-transport-recovery/check.rs -o /tmp/transport-recovery-check
/tmp/transport-recovery-check research/primefield-companion-followup-20261102/feature-native-quartic-recovery/program.tsv research/primefield-companion-followup-20261102/feature-native-quartic-recovery/corrections.tsv research/primefield-companion-followup-20261102/feature-native-transport-recovery/recovery.tsv
```

Substitute negative-scalar.tsv or negative-initialization.tsv as the last argument
to reproduce intentional failures. Native build cap90seconds; tests/checker cap60.
Clear inherited export variables, including TRANSPORT_RECOVERY_OUT,
QUARTIC_RECOVERY_OUT, MULTISEED_OUT and NORMALIZED_TRANSPORT_OUT, on read-only replay.
Packaging scripts write artifacts; run only in a scratch copy.

The initial build failure, receipt and dependency lock are retained with initial-
prefixes. initial-source.rs matches the failed source hash; restore it to source.rs
in a scratch copy to reproduce the missing module-path error. The corrected
source uses an explicit transport.rs path. Final source-lock.json includes Rust
include! dependencies and contains36 hashes. The separate checker pins its three
inputs in checker-receipt.json and imports no producer code. Claims remain unsigned.
No comparative timing or cloud job; original4563-case cohort unchanged.
