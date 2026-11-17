# Streaming lifted character trace

See [DERIVATION.md](DERIVATION.md), [RESULTS.md](RESULTS.md), and [report.pdf](report.pdf).
recurrence.rs implements a degree-sized coefficient window with bounded precision
lifting, two selected trace sums, and an explicit capped sign-factor subset path.
The public wrapper constructs no dense phase-quotient polynomial; it retains the
previous calibrated source and scalar phase/correction construction.

From the repository root:

```sh
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-character-trace-recurrence/driver.rs -o /tmp/t1944-replay
timeout 60 /tmp/t1944-replay
```

Compare stdout with validation02/check.stdout; do not overwrite saved runs.
The 500000-step/4096-subset/native-modulus caps apply independently to each call.
Runtime prime/source compatibility assumptions are inherited. Scalar enumeration
and dense coefficient retention occur only in the finite correctness oracles.
