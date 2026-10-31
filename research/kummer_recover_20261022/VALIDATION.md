# Reproduction

From the repository root:

```sh
cd research/kummer_recover_20261022
TMPDIR=/var/tmp cargo build --release --locked --offline --bins
cd ../..
research/kummer_recover_20261022/target/release/kummer-recover-gate
research/kummer_recover_20261022/target/release/recover
python3 research/kummer_recover_20261022/validate.py
MPLCONFIGDIR=/var/tmp/kummer-mpl python3 research/kummer_recover_20261022/plot.py
```

The first native program replays all 175 quotient zero-output rows. The second
replays the full target cohort and checks each signed output against the
independent source-derived Jacobian target. The Python validator recomputes
the source joins and polynomial identities from the archived rows. The
archived literal-equation failures are input to the validator, not overwritten
by the corrected producer.
