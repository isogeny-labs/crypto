# Hashed ECDL development validation — 2026-10-29

Build base: `a55def20e6809e7160f98cd54db62607f2c686cb`.
Unpublished branch refreshed to `4eb4eb45d7cb104b85735c521e656a1fba112f13`
before publication. The upstream delta contains no Rust, Cargo or `src/`
changes; the only overlapping local files were the unsealed session log and
problem index, which Git merged cleanly. The algorithm source remained identical.
Host: Linux x86-64 cloud development container. No performance isolation or
performance claim. Compiler: rustc 1.99.1 (58cf8f9c3 2026-10-13).

Shared algorithm source SHA-256:
`9c8bf104089521179a7f830e79e9bc226083dd9d26897677f4a52d8a32bb4d3b`.
The module is byte-identical to cryptanalysis PR
[1296](https://github.com/isogeny-labs/cryptanalysis/pull/1296), head
`ff11fd7a653f064d1fc6f85b62f5b8a7494caa14`.

| Command / check | Observed outcome |
| --- | --- |
| `rustc --edition 2021 --test -O -D warnings -D missing-docs src/cryptanalysis/hashed_ecdl.rs -o /tmp/hashed-ecdl-tests`; execute binary | 9 passed, no failures |
| Same standalone compile/tests with `rustc +1.78.0` | 9 passed on the Rust binding's minimum version |
| `rustc --edition 2021 --crate-type lib -D warnings -D missing-docs src/cryptanalysis/hashed_ecdl.rs` | exit 0 |
| `rustfmt --edition 2021 --check src/cryptanalysis/hashed_ecdl.rs` | exit 0 |
| Cryptanalysis release library at the identical shared source | 19 passed, including independent C arithmetic over all 199 scalars; [receipt](https://github.com/isogeny-labs/cryptanalysis/blob/ff11fd7a653f064d1fc6f85b62f5b8a7494caa14/docs/hashed-ecdl-validation.md) |
| Cryptanalysis strict clippy / workspace format / API-doc build | exit 0 for each; same receipt |
| `cmp` between the two algorithm modules | identical |
| `git diff --check` | exit 0 |
| Typst 0.15.0 report build; inspect both rendered pages | 2 pages; diagram and text inspected |

## Root integration check

The initial root invocation lacked the sparse `isogeny_algos` path dependency.
After materializing it, `cargo test --release --lib` reported 41 missing
tracked compile-time fixtures. The full diagnostic is retained as
[hashed-ecdl-initial-build.txt](hashed-ecdl-initial-build.txt). These errors
were checkout omissions, not missing upstream source. No include or test was
deleted. Restored them with:

```sh
scripts/sparse-checkout.sh apply --profile minimal \
  --path isogeny_algos/ --path crates/ \
  --path research/pencil_large_prime_20261029/ --path problems/
timeout 600 cargo test --release --lib
```

The rerun compiled the release library successfully and began 5512 tests.
It reached the 600-second build-plus-test cap with exit 124; there is **no
full-root suite pass**. All nine new tests passed inside that library. The
same release test binary's focused nine-test rerun passed after the metadata-only
main refresh; [transcript](hashed-ecdl-tests.txt).

Six observed failures were additional sparse runtime fixtures: five Certicom
device tests lacked `gpu/certicom/emulator.cpp`, and the version-1 bounds test
lacked `research/ecbench_bounds_challenges_20261006/sessions/prime-toy-reference-e1/`.
Materialized both tracked directories with `scripts/sparse-checkout.sh add`.
All six device tests then passed ([transcript](hashed-ecdl-device-recheck.txt)),
and the exact bounds test passed ([transcript](hashed-ecdl-bounds-recheck.txt)).
No source, test, frozen session, stop criterion or validation gate was edited
to turn these failures into passes. No further root run is running.

The full bounded trace is retained locally at
`/workspace/scratch/subexponential-ecdl/root-library-bounded.txt`, 206183 bytes,
SHA-256 `f13b52fae7663f26dcebab83cce89b8ef2c6c809659dee04ccf54da678fb3534`.
It is **not uploaded**: it exceeds the 64 KiB raw-log policy and this runtime
has no configured artifact-store identity. This local path is not a durable
remote artifact. The initial compile diagnostic and compact focused reruns
are committed; the capped suite is not offered as admitted complete evidence.

## Other setup observations

Rust and rustfmt were not preinstalled; installed the official toolchain and
components. Conductor check exited 127 (`conductor: command not found`);
used `docs/CONDUCTOR.md`'s documented warn-and-allow fallback without claiming
a shared reservation. Development builds/tests are not a research experiment;
no benchmark, EKS job, S3 checkpoint, performance measurement, deployed-curve
run or scoreboard promotion was attempted.

The compact encoder, fast contraction backend, general-size fields and fixed
cofactor extension remain unimplemented. Full conditional asymptotic
replication is unresolved in the [canonical problem record](../problems/hashed-ecdl-tensor-backend.md).
Hosted gates and merge status are reported by the PR separately.

## Historical receipt; tensor follow-up

The source hashes and outcomes above describe the first bounded reference.
PR2878 merged as `055c5035be0e36ccf557a0b887dd51c08155e795` at
2026-10-29T20:56:14Z. Its optional crypto clippy check identified two
manual-is-multiple-of warnings; the follow-up preserves Rust1.78 with supported
outer MSRV annotations and rechecks strict lint. The new compact encoder,
contraction backend and tensor reduction have a separate
[current validation receipt](hashed-ecdl-tensor-validation.md). The earlier
capped trace remains incomplete evidence, with its original hash and scope.
