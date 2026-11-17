# Quartic conditional recovery

Read DERIVATION.md for assumptions and scope; report.pdf gives the visual summary.
Source: lib.rs and routed/source.rs. Frozen artifacts: program.tsv, corrections.tsv,
recovery.tsv, source-lock.json, receipts and raw logs. Claims in claims.json are unsigned.

Replay from the repository root using a fresh output binary (omit export environment
variables to preserve the committed evidence):

```sh
rustc --edition=2021 -O -C overflow-checks=yes --test research/primefield-companion-followup-20261102/feature-native-quartic-recovery/lib.rs -o /tmp/quartic-recovery-tests
/tmp/quartic-recovery-tests --test-threads=1 --nocapture
rustc --edition=2021 -O -C overflow-checks=yes research/primefield-companion-followup-20261102/feature-native-quartic-recovery/check.rs -o /tmp/quartic-recovery-check
/tmp/quartic-recovery-check research/primefield-companion-followup-20261102/feature-native-quartic-recovery/program.tsv research/primefield-companion-followup-20261102/feature-native-quartic-recovery/corrections.tsv research/primefield-companion-followup-20261102/feature-native-quartic-recovery/recovery.tsv
```

Build limits90seconds, test/check60seconds, single test thread. For negative controls
substitute negative-corrections.tsv or negative-recovery.tsv; each should exit nonzero.
The saved runners document original commands; rerun only in a scratch copy because
those packaging scripts write logs and artifacts. This is development validation,
not a comparative benchmark. direct_phases in recovery.tsv counts extension work only.
