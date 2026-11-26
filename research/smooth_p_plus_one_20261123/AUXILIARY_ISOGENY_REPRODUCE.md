# Reproduce auxiliary two-isogeny software qualification

Use source `2cb1f120c5e5bd3665a844cdaa1fec46d2b3bcd7` in a separate checkout.
The dependency-free library is its own Cargo workspace. Cargo.lock is tracked;
do not remove it or overwrite the retained qualification evidence.

```sh
CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=1 cargo +1.99.1 test --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/auxiliary-isogeny/Cargo.toml
CARGO_BUILD_JOBS=2 cargo +1.99.1 build --release --locked \
  --manifest-path research/smooth_p_plus_one_20261123/auxiliary-isogeny/Cargo.toml
```

The retained qualification has one actual suite: nine passing unit tests and
zero doc tests. tests_1 records the earlier exit101 before compilation because
the lockfile was missing. tests_2 records the completed suite. The release
build succeeds. Exact local toolchain/cache paths and the setup failure are
in evidence/auxiliary_isogeny_v1/commands.txt; build_identity.json binds the
source, library and unit-test executable. Fresh reproductions should use new
output paths and retain any failure.

Controls use the fixed F7/F31 points and compare independent representations:
the group-addition law against the isogeny map; chained rational evaluation
against dense composition; coset enumeration against a product polynomial;
and exponentiation-based target lifting against direct tiny-field y checks.
They share field arithmetic and are not an independent external audit. No
Sage installation, msolve call, large-prime selector run, relation collection
or benchmark is needed for these tests.

At the result-publication revision, from the repository root:

```sh
sha256sum -c research/smooth_p_plus_one_20261123/evidence/AUXILIARY_ISOGENY_SHA256SUMS
```

Render the report from the study directory:

```sh
dot -Tsvg auxiliary-isogeny-flow.dot -o auxiliary-isogeny-flow.svg
dot -Tpdf auxiliary-isogeny-flow.dot -o auxiliary-isogeny-flow.pdf
pandoc AUXILIARY_ISOGENY_RESULT.md --from markdown+tex_math_single_backslash \
  --pdf-engine=xelatex -V geometry:margin=0.6in -V fontsize=10pt \
  -o AUXILIARY_ISOGENY_RESULT.pdf
```

The allowed-prime/depth limits and pole/fiber/target-lifting behavior are
defined by the source and AUXILIARY_ISOGENY.md. New scientific panels require
their own frozen workload, exact image/launcher binding and normal execution
gates. These unit controls establish no performance ratio or recovery result.
