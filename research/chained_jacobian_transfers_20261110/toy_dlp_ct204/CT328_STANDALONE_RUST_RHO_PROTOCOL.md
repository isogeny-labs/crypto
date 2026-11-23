# CT328 — source-bound native Rust rho qualification without the broken root crate

Status: **pre-registration only**; no native CT328 execution or complete ECDLP speedup is claimed. [pcc:unverified]

## Ownership and independent limits

Follow-up on draft [crypto #4973](https://github.com/isogeny-labs/crypto/pull/4973), not a new branch. CT327 owns only `tests/ct238_native_cubic_norm_compression.rs` and this new isolated source-only/native qualification. The original 695/701 norm-point reuse fixture, physical `C169/C13a/C13b/N2_actual/D`, S3 custody, cryptanalysis matched full-cost panel and CT241 frozen records are **not** modified. No paid compute, EKS, S3, external private data, full-root build or concurrent source lane is authorized by this protocol. Keep the existing failed CT327 native job and logs.

## Failure motivating this narrower gate

CT327 [native Actions #45120587976](https://github.com/isogeny-labs/crypto/actions/runs/45120587976/job/137513188983) reached Rust compilation on an exact checkout, then failed before tests because `src/cryptanalysis/binary_hyperelliptic_index_calculus.rs` at root and branch has an unrelated unclosed-delimiter syntax error (reported near line 640; Git blob `f001a1b641abd48bb0130c1a9a246e37c28bcc3a`). The CT327 source itself is not yet compiled by that CI. Existing CT205 native also failed in a different checkout before Rust because its sparse checkout omitted `ChainTransfer.lean`. Preserve those failures; do not repair either competing owner file or change the original frozen scientific model.

## Fixed experiment, single inexpensive hosted job

Read **only** the original CT327 Rust test file, pinning the original Git blob `20ff945271d98415f077a74db13bcb18a14e4730`. Fail closed unless this blob matches. Generate a temporary, dependency-free Rust test translation unit from **unchanged exact textual source functions** `m`, `power`, `ec_add`, `ec_mul`, `inverse` and the original nested `ct327` Rust module up to, but excluding, the cold-triplet test, which depends on the broken root library and Jacobian. The generator may add only the `P/R/Ec` declarations imported from the source and a closing module brace; it must not alter arithmetic or test bodies. No copy of original sources is edited. Print original blob and generated SHA-256.

Compile the generated translation unit with `rustc --edition=2021 -O --test` and run the **existing unchanged Rust 109-target correctness and wrong-scalar/off-curve test**. Require exactly one `CT327_CORRECTNESS_PASS logs=109 mutations=110` marker and zero failed tests. Add one deliberately changed-source negative control, requiring refusal before writing an output source. A generated standalone binary is a correctness-only control on the source EC/rho path; this does not establish root-crate compilation, the missing IC-versus-rho cold triplets, matched resource isolation or an independent validator. [pcc:unverified]

Hosted runner: `ubuntu-latest`, hard job cap 10 minutes, one job, no packages beyond hosted Python/rustc. Do not dispatch a competing timing job. Capture exact commit, generated-source SHA, compiler exit and original native test stdout; do not optimize/tune the algorithm or the target after observing results.

## Acceptance and decisions

- PASS only means the **identical source** Rust EC/rho diagnostic returns and independently checks all 109 public F103 logs; report compiler/test logs and producer-only standing. A failure remains a failure and should be fixed only within the CT327-owned generator/test scope.
- It is **not** a full cold transfer solve, a Rust Cantor/Jacobian execution, 695/701 replay, a physical norm-reuse price, a Prime31 result, an optimized production rho, an independent signed PCC, or a complete ECDLP advantage.
- After this scoped gate, the root-crate syntax failure must be resolved by the responsible owner before the original CT327 full cold triplets can be qualified; do not silently build against a different source revision. The actual original-model scientific priority remains intact.

## Roles and evidence

Native child-agent slots were not exposed in this runtime. The coordinator directly performed disjoint source reviews: mathematics (F103 DLP-preserving injectivity and non-universal F101 Hom obstruction), Rust (unmodified CT327 source extraction), and validation (source hashes, 109 known answers, failure controls, full-cost exclusions). These are **not actual separate independent workers** and provide no signed verification. The coordinator alone performs authorized GitHub writes. All evidence is [pcc:unverified].