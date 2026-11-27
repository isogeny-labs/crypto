# Reproduce the isolated input-parser diagnostic

Use source `ae710f5660a91320c4e974d637af21e68bbaa3f4` in a separate checkout.
The [before-call identity](evidence/msolve_input_audit_v1/precall_identity.json)
at `5567b3ffdf3b7ca11e6f67545c90553421650276` pins 14 files. The original
scientific input JSONLs and prior residual audit are required byte-for-byte.
Use new output paths; do not overwrite any retained result.

From the repository root:

```sh
cc -std=c11 -O2 -Wall -Wextra -o /absolute/parser-probe \
  research/smooth_p_plus_one_20261123/native/diagnostics/msolve-parser-probe.c
CARGO_BUILD_JOBS=2 cargo +1.99.1 build --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/native/Cargo.toml \
  --example msolve-input-audit
/absolute/msolve-input-audit --self-test /absolute/parser-probe
```

The C probe requires C11/POSIX and standard headers. It contains attributed
upstream GPL parser excerpts and a bounded harness; no msolve installation
or algebra backend is needed. The Rust companion uses the locked existing
crate. Recorded executable SHA-256 values:

```text
Rust: 17c456f0acc7dbff3f9a1911b5d8ba7b7d04e404a22bee01285db41040f6c28b
C:    fd6137043702903eba6a6b8601c2b7922331ea91b6ebba9c12e32887032323ed
```

After controls pass, preserve the
[wrapper](evidence/msolve_input_audit_v1/command.sh) limits: one worker,
1 GiB address space, 30 seconds plus five seconds termination grace. Invoke:

```sh
/absolute/msolve-input-audit /absolute/parser-probe \
  research/smooth_p_plus_one_20261123/evidence/factorwise_extract_v1/f127_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/liftable_fibers_v1/f127_d4.jsonl \
  research/smooth_p_plus_one_20261123/evidence/rur_record_audit_v1/audit.jsonl
```

Capture stdout, stderr and exit in a new directory. The private stream
wrapper only copies complete JSONL records; a standalone reproduction can
capture directly. Both records are attempted, parsing/reconciliation errors
fail the command, and nonzero effective residuals remain descriptive output.
No solver or root-search call is made. The test has no timing conclusion.

Inspect retained evidence without rerunning:

```sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/MSOLVE_INPUT_AUDIT_SHA256SUMS
jq 'del(.roots,.parsed_equations,.parsed_form_terms)' \
  research/smooth_p_plus_one_20261123/evidence/msolve_input_audit_v1/audit.jsonl
```

From the study directory, render only recorded counts and report:

```sh
python3 plot-msolve-input-audit.py
pandoc MSOLVE_INPUT_AUDIT_RESULT.md --from markdown+tex_math_single_backslash \
  --pdf-engine=xelatex -V geometry:margin=0.6in -V fontsize=10pt \
  -o MSOLVE_INPUT_AUDIT_RESULT.pdf
```

All results are producer-local and unsigned. Upstream source version and
installed Debian package version are recorded separately; the excerpt
harness does not establish installed-binary provenance. Omission of zero
terms is a proposed integration change, not a replacement of any historical
solver input or result.
