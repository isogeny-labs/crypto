# Reproduce the auxiliary RR5 software adapter

Check out `1ab4844db046a65dabec9ce962c070ac18709e6a`, which contains the exact
Cargo.lock used by the retained invocations. The preceding source commit
c6b870aa038488048ac691c9107ad74b0ba2ed40 omitted that ignored nested lockfile;
the correction adds its exact bytes without resolving dependencies again.

From the repository root, using Rust 1.99.1:

```sh
CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=1 cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/auxiliary-rr5/Cargo.toml
CARGO_BUILD_JOBS=2 cargo +1.99.1 build --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/auxiliary-rr5/Cargo.toml
```

The component has six unit tests and zero doc tests. It calls the original
native cofactor preparation boundary, which builds equations without running
F4 or root extraction. Dependency tests are not run by this command. Unit
controls use F31 only; no msolve executable or scientific job is required.
Use fresh output paths for reproduction, and retain any failure instead of
overwriting the qualification logs.

`evidence/auxiliary_adapter_v1/commands.txt` records local environment/cache
paths. `development_flow.json` records invocation and exclusion counts.
`build_identity.json` binds 34 specified source/input files and two artifacts;
it is a producer-local identity record, not a signed execution certificate or
a claim of an independently rebuilt hermetic dependency closure.

At the result-publication revision, check the committed report/evidence bytes:

```sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/AUXILIARY_ADAPTER_SHA256SUMS
```

Render from `research/smooth_p_plus_one_20261123`:

```sh
dot -Tsvg auxiliary-adapter-flow.dot -o auxiliary-adapter-flow.svg
dot -Tpdf auxiliary-adapter-flow.dot -o auxiliary-adapter-flow.pdf
pandoc AUXILIARY_ADAPTER_RESULT.md --from markdown+tex_math_single_backslash \
  --pdf-engine=xelatex -V geometry:margin=0.6in -V fontsize=10pt \
  -o AUXILIARY_ADAPTER_RESULT.pdf
```

The matched predicates are checked on Fp assignments. They are not asserted
to define the same ideal over the algebraic closure. The native preparation
check does not certify subgroup metadata or complete a relation-collection,
recovery or performance comparison. Those remain separate integration and
scientific execution work.
