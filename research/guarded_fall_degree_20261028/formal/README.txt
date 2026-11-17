Current update, 2026-10-29: complete-exact-pair.md records the complete
GuardedExactPair.exact_pair_prime theorem for the original prime-field model.
The degree-five upper construction, rationality/unit-ideal bridge and actual
weighted top-syzygy/Koszul first-fall equality are now proved. Exact current
sources and checks are bound by complete-exact-pair-validation/. The final
theorem is a default Lake target. Growing-base, binary and Boolean results
and numerical fixture provenance are separately scoped in PROOF_MAP.json.

The following two updates are historical partial-formalization records.

Historical update, 2026-10-29: exact-pair-progress.md records a checked ordinary
filtered-closure definition, complete uniform degree-four obstruction for the
actual six inputs, degree-four divided-difference derivation, and first-fall
dependencies. The full exact pair (4,5), including the degree-five upper
construction and weighted syzygy/Koszul definition, remains open. Current
source hashes, build, axiom audit and fresh module replays are in
exact-pair-validation/. PROOF_MAP.json is the current proof map.

The remainder of this file is the historical identity-only scope record.

Partial Lean formalization of fall-degree proof identities
Date: 2026-10-28

Addition, 2026-10-29: PrimeBitLocal.lean is a second default Lake target with
eight checked local dependency lemmas. Its separate receipt is
prime-bit-verification.json and its scope is in prime-bit-readme.txt. The
original twelve-theorem source and four existing ProofRefs are unchanged.
The complete prime-field bit-restricted degree bound is not formalized.

Status
------
The module FallDegreeIdentities.lean has twelve checked theorems. It proves
actual symbolic identities over arbitrary commutative rings, with an explicit
CharP R 2 assumption where required, and the structural implication that a
common root makes the scalar resultant expression zero.

It DOES NOT formalize the universal last-fall-degree theorem. The filtered
V_(F,D) definition, staged total-degree bounds, monic normal-form reductions,
scalar-grid interpolation/unit argument, converse resultant criterion, and
elliptic-curve point interpretation remain missing bridges. The lower-bound
argument and growing-factor-base extensions are not formalized either.
PROOF_MAP.json records the exact declaration-level boundary. In particular,
the custom scalar resultant expression has not been connected to Mathlib's
Polynomial.resultant; no such bridge is silently assumed as a theorem here.

Pinned dependencies
-------------------
Lean: leanprover/lean4:v4.34.1, compiler commit
5045d0056413266e57c625dcd7c365b10e377c52.
Mathlib: d13f23b723b8a846827a245b89c10fc7d3f11612.
The exact transitive package revisions are in lake-manifest.json.

Reproduce
---------
With the pinned Lean toolchain installed and its bin directory on PATH:
  MATHLIB_NO_CACHE_ON_UPDATE=1 lake update
  lake exe cache get Mathlib.Tactic.Ring Mathlib.Algebra.CharP.Basic
  lake build
  lake env leanchecker --fresh --verbose FallDegreeIdentities

The successful `lake build` compiled the actual module and printed every
reported theorem's transitive axioms. Only propext and Quot.sound occur;
there is no sorryAx, custom axiom, Lean.ofReduceBool, or native_decide use.
The source contains #print axioms commands so the audit is reproducible.

The direct compile log records `lake env lean FallDegreeIdentities.lean`;
the Lake build checks the same actual module. The cache and task-local
compiler are development dependencies, ignored rather than committed.

Sources and relation to the written proof
----------------------------------------
The polynomial expressions are those in universal_five_symmetric_audit.txt
and extensions/characteristic_two/normalized_resultant_char2.txt. Relevant
primary-source assumptions were checked by the parent task in Semaev,
https://arxiv.org/html/1504.01175 (Section 2, S3 formulas), Kosters-Yeo,
https://arxiv.org/html/1503.08001 (Proposition 2.1 and Section 4),
Huang-Kosters-Yang-Yeo, https://arxiv.org/html/1505.02532 (Section 2), and
Caminata-Gorla, https://arxiv.org/html/2112.05579v2 (degree conventions).
No literature priority or novelty claim is made by these formalizations.

Development errors retained as facts
-----------------------------------
The initial direct compile was invoked from the Mathlib package without
-R and exited 1 because the input lay outside that package root. Adding
-R /workspace/fall-degree-check/formal produced the successful compile.log.
An exploratory `leanchecker --help` without the compiler on PATH could not
launch lean; with PATH but no module it inferred the package name rather than
the library name. The explicit module invocation succeeds. These were setup
invocation errors, not failed theorem proofs.

Unconditional integer-lift addition
----------------------------------
The declaration char_two_resultant_integer_lift states the full cancellation
identity over EVERY commutative ring, without a CharP premise. The right side
retains the explicit integer factor 2. This named declaration can be linked to
a native identity certificate checked in odd characteristic without claiming
that a characteristic-two-only theorem holds there. The specialization theorem
char_two_resultant_cancellation separately uses CharP R 2 to remove that term.
