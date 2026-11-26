# Reproduce liftable-fiber qualification

[Frozen construction/protocol](LIFTABLE_FIBERS.md) · [results](LIFTABLE_FIBERS_RESULT.md).
This qualification contains an F127 comparator mismatch and a missing query
suffix. Reproduction must preserve them rather than overwrite the evidence.

Use source `3b7cd7422da55ceaf49ff76e23b18d1914fe9183` in a separate checkout.
The [before-call identity](evidence/liftable_fibers_v1/precall_identity.json),
published at `c2d6a4c3184cd7d8716833ee6184d13f115e25dc`, pins18 files,
including protocol, exact wrapper, audit reconciliation, Cargo files, all
native sources and executable/solver. Driver SHA-256:
`bb1b949b2c393ea1ea2ccfe2e1d99054293cff4e744f269be3321f6fb7d4ea55`.
Msolve SHA-256:
`74088d6775c82845faedcbdc24a82d3709e39ee7ebf3e3e4aabde080f9ab3c28`.

```sh
CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=1 cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand --features production-controls
CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=1 cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand
CARGO_BUILD_JOBS=2 cargo +1.99.1 build --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --bin dickson-four-summand --features production-controls
```

Copy the driver to an immutable new path and record its actual digest. Use a
new output directory and preserve the [wrapper](evidence/liftable_fibers_v1/command.sh)
semantics. It first attempts all three finite audits, each with one worker,
2GiB virtual memory and120 seconds plus five seconds termination grace:

```sh
/absolute/driver 127 4 /absolute/msolve liftable-audit
/absolute/driver 8191 4 /absolute/msolve liftable-audit
/absolute/driver 8191 5 /absolute/msolve liftable-audit
```

These audits do not call msolve. Native equation/rank checks must pass and
`audit_reconcile.jq` must match all prior counts and rank distributions before
any solver panel starts. Only then run these panels, each with the same
worker/memory constraints and60 seconds plus five seconds grace:

```sh
/absolute/driver 127 4 /absolute/msolve liftable-cutoff cofactor-msolve
/absolute/driver 8191 4 /absolute/msolve liftable-cutoff cofactor-msolve
/absolute/driver 8191 5 /absolute/msolve liftable-cutoff cofactor-msolve
```

All three panels are attempted even if one fails. A mismatch stops only its
panel and its missing suffix remains unattempted. The native caller fixes the
target order, three-second shared query deadline, forms, caps and full-answer
comparison. Membership construction is inside that deadline; base/reference
setup and final JSON emission are outside. The wrapper preserves stdout,
stderr and exits and copies complete JSONL records to the live spool.

For the retained records, from the repository root:

```sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/LIFTABLE_FIBERS_SHA256SUMS
jq -s -f research/smooth_p_plus_one_20261123/evidence/liftable_fibers_v1/summarize.jq \
  research/smooth_p_plus_one_20261123/evidence/liftable_fibers_v1/f*.jsonl
jq -s -f research/smooth_p_plus_one_20261123/evidence/liftable_fibers_v1/audit_summary.jq \
  research/smooth_p_plus_one_20261123/evidence/liftable_fibers_v1/audit_f*.jsonl
jq -s -f research/smooth_p_plus_one_20261123/evidence/liftable_fibers_v1/audit_reconcile.jq \
  research/smooth_p_plus_one_20261123/evidence/liftable_fibers_v1/audit_f*.jsonl \
  research/smooth_p_plus_one_20261123/evidence/factorwise_v1/f*.jsonl
```

Compare these administrative outputs with their recorded JSON companions.
`comparator_difference.json` compares the current F127 (41,95) cofactor
record to `factorwise_extract_v1/f127_d4.jsonl`, extracting prepared equations,
input prefix and first-form RUR. It reports byte equality and RUR digests,
without a new solver invocation or mathematical validation of that RUR.

Render recorded-value figures and report from the study directory:

```sh
python3 plot-liftable-fibers.py
dot -Tsvg liftable-fiber-flow.dot -o liftable-fiber-flow.svg
dot -Tpdf liftable-fiber-flow.dot -o liftable-fiber-flow.pdf
pandoc LIFTABLE_FIBERS_RESULT.md --from markdown+tex_math_single_backslash \
  --pdf-engine=xelatex -V geometry:margin=0.6in -V fontsize=10pt \
  -o LIFTABLE_FIBERS_RESULT.pdf
```

The evidence is producer-local and unsigned. Required artifact upload and byte
replay remain unfinished for three new and eight older raw files. Preserve
the raw records when the supported artifact workflow fails; Git publication
does not establish object-store availability. No admitted cost comparison,
recovery run, checked Lean theorem or canonical experiment export is supplied.
