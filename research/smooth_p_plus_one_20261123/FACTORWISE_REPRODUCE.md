# Reproduce the factorwise finite audit

[Frozen argument/protocol](FACTORWISE.md) · [results](FACTORWISE_RESULT.md).
This checks witnesses and actual equation Jacobians on the three public
fixtures, without a relation solver, recovery or timing comparison.

Use source `3747e09eae5702b7a8e743bc4ad930e475bab41b` in a separate checkout.
The [before-call identity](evidence/factorwise_v1/precall_identity.json),
published at `d0e5cc767be035f24cd2c76d9f9b0bae5398e129`, pins the exact
protocol, command, lockfile, native sources and executable. Driver SHA-256:
`be6de11f5c302642a148a8c5d211696a692433102e51c6a8a63ba758571cb2d3`.

```sh
CARGO_BUILD_JOBS=2 cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand --features production-controls
CARGO_BUILD_JOBS=2 cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand
CARGO_BUILD_JOBS=2 cargo +1.99.1 build --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand --features production-controls
```

Copy the executable to an immutable new path and record its actual digest.
Use a new evidence directory; never overwrite the retained audits. Preserve
the [exact wrapper](evidence/factorwise_v1/command.sh): one worker,2GiB address
space,120 seconds plus five seconds termination grace per fixture, all three
attempted even after failure, and separate stdout/stderr/exit capture.
The native interfaces are:

```sh
/absolute/driver 127 4 /unused factorwise-audit
/absolute/driver 8191 4 /unused factorwise-audit
/absolute/driver 8191 5 /unused factorwise-audit
```

The positional solver path is unused by this command. The audit generates
targets from signed leaf sums and reuses the catalog, RR5 interpolation and
field row reduction; it is not independent implementation evidence. Failure
of any equation, rank or accounting requirement aborts that fixture's audit.
The all-fixture wrapper preserves the remaining planned attempts.

For the recorded data, from the repository root:

```sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/FACTORWISE_SHA256SUMS
jq -s -f research/smooth_p_plus_one_20261123/evidence/factorwise_v1/summarize.jq \
  research/smooth_p_plus_one_20261123/evidence/factorwise_v1/f127_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/factorwise_v1/f8191_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/factorwise_v1/f8191_d5.jsonl
```

`reconcile.jq` consumes these three raw records and the corresponding three
records under `full_cutoff_v1/`. It groups by prime/depth and compares tuple,
sign, exclusion and per-chart old-rank records, while checking that every
new rank-seven count equals its chart's admissible-binding count. Require
all three `counts_match`, `charts_match` and `new_full_rank` fields to be true.
This comparison replays recorded metadata; it is separate from the actual
Rust equation/Jacobian checks.

Render the recorded-value graph and report from the study directory:

```sh
python3 plot-factorwise.py
dot -Tsvg factorwise-flow.dot -o factorwise-flow.svg
dot -Tpdf factorwise-flow.dot -o factorwise-flow.pdf
pandoc FACTORWISE_RESULT.md --from markdown+tex_math_single_backslash \
  --pdf-engine=xelatex -V geometry:margin=0.6in -V fontsize=10pt \
  -o FACTORWISE_RESULT.pdf
```

The results are producer-local and unsigned. A written proof, finite
Jacobian audit and current-head CI are separate from a checked Lean theorem,
global radical certificate, complete algebraic extraction and full-cost gain.
