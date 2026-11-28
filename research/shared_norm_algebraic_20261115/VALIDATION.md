# Validation, scope and custody

All software observations are local to the producer; PCC-20261115-be28ec remains
unverified with no signed main-verifier verdict. The supporting Lean declarations
are separately compiled and axiom-audited. They establish their exact polynomial
statements, not this whole program or the original subexponential criterion.
[pcc:unverified]

## Native controls

- Build: `build.sh`, g++ C++17, `-O2 -Wall -Wextra -Wpedantic -Werror`.
  No warning suppressions. Shell orchestration passes `bash -n`.
- Exact quotient controls: F5 grid25 (>p); nonreduced quotient dimension4
  with one point; extension-only quotient; unit ideal; cap rejection; malformed
  operators rejected. Rank-free recovery controls also pass.
- Retained system: all28 assignments satisfy every original equation; decoded
  models match the native reference; regenerated variable list and coefficient
  maps equal the retained canonical input exactly.
- Final original encoding, models-002:90 complete model sets,400 models.
  Independent OpenSSL complete enumeration agrees, including zero-model cases.
- Final original encoding, cold-002:18 unique nonzero point-only targets;
  72 accepted signed relations and18 kernel/scalar recoveries replay in OpenSSL.
- Witness-pruned mode: same90 complete model controls and18 cold targets;
  OpenSSL agrees with all400 models,72 relations and18 recoveries.
- Each encoding's fixed three arity controls: m5,t3,c16 has2 models and
  quotient8; m5/t4 and m7/t4 remain unknown at20s, with reference counts88,56.
- Wrong-scalar and duplicate-case tamper controls exit2; pruned c=0 admission
  is rejected by the native selftest. No disagreement was relabelled unknown.

These are deterministic finite software controls. There is no scientific
performance measurement, fitted exponent, comparative speed claim, random-target
population estimate or scientific export. Caps are configured resource limits,
not reliable timing observations on this shared development host. [pcc:unverified]

The separately compiled replayer imports no candidate code. It uses OpenSSL
curve admission, point operations and membership arithmetic to enumerate the
complete m3 model sets and replay mixed-query labels, accepted rows, left-null
equations, denominator retries and recovered scalars. It checks the target and
model-control censuses. Its printed rank field is not independently recomputed;
no rank certificate is claimed. Arity5/7 complete sets use the bounded native
reference, not the m3-only OpenSSL model replayer. [pcc:unverified]

The candidate reuses the historical canonical parser and group/linear utilities
by including their C++ sources under renamed CLI entry points. It does not call
the old coefficient-census solver. Final candidate base setup factors membership
and lift polynomials; only the external bounded references enumerate field
points. Development RNG seed1234 is deterministic test input, not a claim about
cryptographic RNG quality. The oracle receives R only, never a,b,Q, a hidden
scalar, factor-base logs or a decomposition witness. [pcc:unverified]

## Full raw records

Each xz archive is an entire software panel, with every backend input, printed
basis, stdout/stderr, exit status, model file and final trace. Sizes are naturally
below64KiB; no records were split or dropped to meet artifact limits. Native
binaries and Debian packages are not committed. `evidence/SHA256SUMS` binds all
archives. Their SHA256 values are:

| Archive | Bytes | SHA256 |
| --- | ---: | --- |
| records-001.tar.xz | 63332 | `95a22645526048bc12c88fcb30a4e5b7f136376e754fb4122a68ffe692b1bb0e` |
| records-final002.tar.xz | 62044 | `bbda6ce0d06d7dc354759e83bc3a5f3fd6d6acfc97b2b5452c2855143a196b14` |
| records-pruned001.tar.xz | 54928 | `010dae853cb5cd9de7542fb88482ba5fbe79ad3fa878c24a9bec105c332a27e4` |

The initial001 panel is development history. Exact execution-binary custody is
incomplete across its instrumentation changes; do not assign it the later002
digest. The original final source is4930ca5e0d20dd30b6418cdc5456d6371cdafa0b;
models-002 and cold-002 use the same retained final binary. Its digest was
observed after cold-002 and captured before models-002, as Amendment A3 states.
The pruned source is574cc3b24 (full revision in its source-commit.txt); source
and all program/backend hashes were captured before its model/arity/cold calls.
All captured hashes still match. Full001 and002 records remain unchanged.
[pcc:unverified]

To inspect and replay the final original panel (from the repository root):

```sh
mkdir /tmp/norm-records002
tar -xJf research/shared_norm_algebraic_20261115/evidence/records-final002.tar.xz -C /tmp/norm-records002
/tmp/norm-build/replay --models /tmp/norm-records002/models-002 /tmp/norm-records002/models-002.tsv
/tmp/norm-build/replay --cold /tmp/norm-records002/cold-002 /tmp/norm-records002/cold-002.tsv
```

The pruned panel unpacks directly to models/,cold/,arity/ and matching TSVs.
Its source-binary.sha256 contains observed scratch absolute binary paths;
source hashes are repository-relative. Rebuilding produces a new execution
identity; do not reinterpret the old record as that new execution.

## Shared Lean support

Library: aburan28/crypto-lean, branch codex/norm-chart-laws-20261115, source
ad93dea (exact dependency pin in proof-dependencies.json). Mathlib and Lean pins
are unchanged: cfc55fc58714a59dc06e6e79edc6d1d95140d3c2,
leanprover/lean4:v4.35.0.

- `CryptoLean.Polynomial.isRelPrime_of_squarefree_norm`: squarefree divisor U
  of squarefree N=A^2-f B^2 implies `IsRelPrime U B`.
- `CryptoLean.Polynomial.kummer_norm_charts`: actual nonzero Kummer membership,
  target separation and the norm identity imply U separable and both declared
  coprimality predicates. The conclusion is not assumed as a hypothesis.
- Focused module build, actual `#print axioms`, module audit and the unchanged
  full shared-library proof gate pass:98 theorems, zero failing, zero trusted
  native proofs. New laws use only propext,Classical.choice,Quot.sound.
  Receipts are in checks/. `lean4checker` was not installed.
- First focused build failed on a multiline parser/layout issue; the corrected
  law then built. Both initial failure and final focused receipts are preserved
  in the pruned archive. The assembly probe had already type-checked.

The [proof map](chart-pruning.md) records the inheritance and remaining endpoint:
retained-chain divisibility and degree-bounded Bezout witness lifting are not
assembled in Lean here. There is no formal backend source-ideal provenance,
program verification, solver-degree/yield theorem or end-to-end ECDLP theorem.
[pcc:unverified]

## Review boundaries

The independent primary/code review and arithmetic replay are from the producing
session. No independent reviewer identity or signed certificate is invented.
Small quotient dimension does not bound basis construction; the two denser caps
remain unknown. Failed attempts, setup, extraction, linear algebra and recovery
must all be charged by a future growing-family analysis. The auxiliary arbitrary-
prime rational-isogeny membership pipeline is still unimplemented here.
[pcc:unverified]

Canonical diagram/leaderboard checked: this study has no admitted performance
result, so no performance leaderboard, progress chart or benchmark index changes.
The source-linked pipeline diagram explains actual component flow. report.pdf
was rendered from report.typ and both pages visually inspected. Metadata/prose
and artifact checks are recorded in PUBLICATION.md after their commands finish.

Subsequent source note: core sourceb5e2b31d42 changes only the baseline oracle
adapter to a template and adds an include guard around the CLI. The rebuilt
baseline selftests and retained28-model check pass under that source. Earlier
hash receipts bind their exact recorded commits/binaries; they must not be
checked against or assigned to this later execution identity. The separate
core study preserves its own pre-run hashes and complete qualification.
[pcc:unverified]
