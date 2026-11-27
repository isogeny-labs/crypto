# Reproduce canonical-form and semantic-guard qualification

Build source `55f8d14f3b5fb1c997740ce8952360c2a31535e6` in a separate checkout.
The result publication adds the build identity and report; inspect its
manifest at the publication revision. Preserve existing logs and use new
output paths for any test reproduction.

The native Cargo.lock and repository dependency closure are required. Follow
the existing [production adapter setup](PRODUCTION_ADAPTER.md); no msolve
installation is needed for the new synthetic/saved-output/stub controls.
The feature suite also retains the existing production-library unit controls.

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

The source qualification records 60 feature tests, 32 overlapping default tests
and a successful release build, with no rerun. Exact local environment and
commands are in `evidence/semantic_guard_v1/commands.txt` and
`build_identity.json`. Recorded executable SHA-256:

```text
5d25b56793ff93865eeb130fd30466d651911e95bdbb8b0f49aff42faa5a9ffe
```

The saved-data regressions read the original F127 factorwise and liftable
JSONLs. Do not substitute a new solver output. The guard tests distinguish
the old effective form from the intended canonical form; the caller stubs
check exact input-file text, retained bad output, absence of extra semantic
retries and explicit unchecked-empty status. No new scientific solver panel
belongs to these test commands.

For the retained publication:

```sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/SEMANTIC_GUARD_SHA256SUMS
```

Render the implementation diagram and report from the study directory:

```sh
dot -Tsvg semantic-guard-flow.dot -o semantic-guard-flow.svg
dot -Tpdf semantic-guard-flow.dot -o semantic-guard-flow.pdf
pandoc SEMANTIC_GUARD_RESULT.md --from markdown+tex_math_single_backslash \
  --pdf-engine=xelatex -V geometry:margin=0.6in -V fontsize=10pt \
  -o SEMANTIC_GUARD_RESULT.pdf
```

Local claims remain unsigned. Historical fixed-form policies, source
attestations, failures and missing queries are preserved. New solver panels
must use a separately frozen corrected policy and the normal execution gates;
these software tests provide no performance comparison or general solver
completeness certificate.
