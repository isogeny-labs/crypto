# Reproduce the saved-output residual audit

Use source `b05d4d85b0943feb1276fa3c61fd0769c50ded9b` in a separate checkout.
The [before-call identity](evidence/rur_record_audit_v1/precall_identity.json)
at `c7ffa2f548eb88c0d69ca9a82c374b9bfdc5321e` pins all eight original inputs.
The immutable auditor SHA-256 is
`873ed24f493834ac807269907c0ea8a9be039adef6b9e7b00014c7e92012147c`.

```sh
CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=1 cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --example rur-audit
CARGO_BUILD_JOBS=2 cargo +1.99.1 build --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --example rur-audit
```

Use a new output directory; preserve the existing audit. The
[frozen wrapper](evidence/rur_record_audit_v1/command.sh) limits execution to
one worker, 1 GiB address space and 30 seconds plus five seconds grace, invoking:

```sh
/absolute/rur-audit \
  research/smooth_p_plus_one_20261123/evidence/factorwise_extract_v1/f127_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/liftable_fibers_v1/f127_d4.jsonl
```

The private capture wrapper copies complete JSONL stdout to the local spool
and preserves stdout/stderr/exit; it supplies no solver, credentials or
scientific input. A standalone reproduction can capture stdout and stderr
directly to new files. No msolve installation or call is needed.

The auditor selects exactly F127 depth 4 target (41,95), mode
ProductionCofactorMsolve, from each file. It parses the serialized equations
and fixed form, decodes the documented RUR coordinates, then evaluates every
equation at every eliminant root in F127. Invalid residuals are descriptive
records and do not make the command fail; parsing failures do. This is not a
general completeness test or relation oracle.

Inspect already recorded evidence without rerunning:

```sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/RUR_RECORD_AUDIT_SHA256SUMS
jq 'del(.roots)' research/smooth_p_plus_one_20261123/evidence/rur_record_audit_v1/audit.jsonl
jq '{source_path,roots}' research/smooth_p_plus_one_20261123/evidence/rur_record_audit_v1/audit.jsonl
```

Render recorded counts and the report from the study directory:

```sh
python3 plot-rur-record-audit.py
pandoc RUR_RECORD_AUDIT_RESULT.md --from markdown+tex_math_single_backslash \
  --pdf-engine=xelatex -V geometry:margin=0.6in -V fontsize=10pt \
  -o RUR_RECORD_AUDIT_RESULT.pdf
```

Both results and local PCC checks remain unsigned. The retained old answer's
13 valid signed rows do not repair its fixed-form residual failure, and the
new empty answer is not a correct replacement. Keep original records,
budgets, missing queries and acceptance policies unchanged.
