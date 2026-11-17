# Direct-index torsion pair probe: source and correctness report

Recorded 2026-11-16. This report describes source-bound checks and exact
storage arithmetic. It is not a timing run, a hypothesis review, or a
performance conclusion. The opt-in parameter is `layout=dense` on
`pdp3-binary:m=2|3,engine=rootfind-negfold-batch-harvest-torsion`; the
incumbent remains the default hash layout. [pcc:unverified]

## Question, scope, and provenance

The m=3 relation collector probes a negation-folded pair table once per
factor-base point on each target trial. The candidate converts the already
built first-witness hash map into a direct binary-key array. It changes
storage and lookup, while leaving factor-base points, collision order,
seed rows, target scan and final group re-addition intact. Source ancestor:
`0fa360d479`; frozen [protocol](PROTOCOL.md) commit `4ae60f1f92`;
implementation commit `de23b92b5e4929a4fa44926155c1e6d67f41ee26`.
The original pair-table oracle and its limits are documented in the
[program paper](../../papers/ecdlp-index-calculus-program/sections/rootfinding.tex)
(internal source); the prior [N83 dense packed-root protocol](../n83_dense_root_20261013/PROTOCOL.md)
uses a different table and workload. This is an ecbench layout integration,
with no novelty or larger-field transfer claimed. [pcc:unverified]

| Checked Koblitz E0 curve | Subgroup order | Full curve UID |
|:--|--:|:--|
| GF(2^13), `icv1-f2m13-t181-515ee569` | 2,003 | `urn:ec-record:1:sha256:1dfa32cdf623e34fb258128415cab472abaa51695380fa5d4f5d636ed5a3da60` |
| GF(2^19), `icv1-f2m19-t797-b6cf2467` | 130,873 | `urn:ec-record:1:sha256:1e2a8f558313fb5343fc2b0aa0999944c41cdd3a6fed3eef58c82c6e2565bfa2` |
| GF(2^23), `icv1-f2m23-t5197-69e76b73` | 2,095,853 | `urn:ec-record:1:sha256:27a979171a138afd2371cf229d26f042414a913f28fd274811c6bfc81390bb31` |

The identities above were read from `docs/curves/registry.json` at this
source revision; the tests construct `KoblitzCurve::new(0,n)` with standard
subspace dimensions 5, 7, 8 and signed Frobenius-orbit columns. All three
curves have cofactor four. [pcc:unverified]

## Observations and exact derivations

- **Producer correctness checks.** The focused release library test passed
  once (1 pass, 0 failures). It compared every stored key-to-first-witness
  pair on K13/K19/K23, every possible key on K13, sampled signed group
  probes on all three, collision order, insertion and group counters, and
  rejected a non-torsion table and n=24 conversion. The release ecbench
  binary suite passed 21 tests, 0 failures, 1 ignored. Its K13 pipeline
  test checked the same public scalar 641 under hash and dense layouts:
  answer check true, 23 target trials, 13 target-bearing rows, rank 18,
  and 2,262 group-addition equivalents in each. It also rejected `layout`
  on a different engine. These are source-local test assertions, not an
  independent replay or wall measurement. [pcc:unverified]
- **Test custody.** Commands, complete stdout and stderr are retained as
  [`test-lib.stdout`](test-lib.stdout), [`test-lib.stderr`](test-lib.stderr),
  [`test-bin.stdout`](test-bin.stdout), and [`test-bin.stderr`](test-bin.stderr).
  Both commands returned exit 0 under Rust 1.99.1 on macOS arm64.
  Test-binary SHA-256 values: library
  `ed7fa2143fd1a18dc6cef2264065616dc98c76107046fd714dc679e8bb07497e`,
  ecbench
  `cdee7746319377e8c51158810812ef6d6b841362d76c43685c38b2ff7652fd5d`.
  Source SHA-256 values: `ic_boundary.rs`
  `e76c46312b2eedce8f1530937c95c91364427204e3fbca496ad4290ca9eef4a9`,
  `pdp3_koblitz.rs`
  `f3358ee12560c1f7b882173f1b1dac87c48658d93a096792fd297ad64501bdac`.
  The `rust-objcopy` missing-`libLLVM.dylib` strip warning was nonfatal.
  The tracked source was clean at the rerun; untracked report artifacts and
  an unrelated local screen were present. [pcc:unverified]
- **CLI plan admission.** The release `ecbench` rebuilt from the pinned
  implementation commit has SHA-256
  `2d9307249c036f5744551035d88c72c85193fa96f72b0ba4d8863ece42c96fd9`;
  its [build stdout](build.stdout) and [stderr](build.stderr) are retained.
  `ecbench plan --doc` accepted [the plan-only spec](SPEC-K13-DENSE-PLAN.json)
  (SHA-256 `3cd1f7d849f6b598fe2509a94573dc5a7d4a592490c1ae27d520f1eaf9d`)
  and wrote [PLAN-K13-DENSE.json](PLAN-K13-DENSE.json) (SHA-256
  `d73733e20b9c36511b99fe154bef4ae1f6c42c6769e6971405b054917fd2a306`).
  Plan ID `ECS1h7bd090d1bd39` has 4 arms, 4 registered K13 public workloads
  and 144 scheduled executions. Within each target-round group, algorithm
  seeds match; its four target points and generator match the prior K13
  torsion plan. The dense and dense A/A arms name `layout=dense`, while the
  hash arm omits it. This is parser and identity admission only; none of
  those executions was launched. [pcc:unverified]
- **Derived allocation.** The array has `2^n+1` eight-byte words, exact
  bytes 65,544 at n=13, 4,194,312 at n=19 and 67,108,872 at n=23.
  It is limited to n<=23 and a 128 MiB cap. The candidate still incurs
  the incumbent hash build, then zero-fills the direct array and writes one
  witness per retained key. It records `table_dense_zero_words`,
  `table_dense_witness_writes` and `pair_table_dense_bytes`; these additions
  remain unpriced in the complete attack-cost model. The
  [editable graph source](make_visuals.py), [storage graph](dense_storage.svg),
  [diagram source](diagram.dot) and [diagram](diagram.svg) show the formula
  and path. No RSS or process-wall measurement was taken. [pcc:unverified]

## Prior mechanism and decision boundary

`dominated_by`: the same-source hash table is the reference. Output and
counted group work are equal in the fixed checks, while the dense layout
uses more initialized memory and conversion work. Whether lower lookup
latency repays that cost is unknown; if a frozen same-target complete-call
panel shows no gain beyond dense A/A noise, the hash layout dominates this
candidate on that scope. The N83 packed-root variant is an existing dense
engineering analogue, but its table structure and curve differ, so its
measurements do not decide this candidate. [pcc:unverified]

`sota_delta`: zero change to the relation-search exponent and to the tested
relation/rank trajectory; exact additional direct-array allocation above;
complete-call wall ratio, memory peak, F4/F5/F6 comparison and rho ratio all
unknown. The next discriminating check is an alternating same-binary
hash/dense/dense-A/A diagnostic on fixed public targets, charging cold setup,
scan, rank, verification, native writes and peak memory. The branch-specific
S3 resume record must first satisfy the project gate; Conductor T-6342 still
reserves `.resume`. The frozen EKS L2 F4/F5/F6 comparison and independent
replay remain distinct requirements. No canonical performance graph was
changed because this round supplies no timing result. [pcc:unverified]
