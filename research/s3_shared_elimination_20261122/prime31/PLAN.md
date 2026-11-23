# Prime31 correctness follow-up

Requested field: Fp with p = 2^31 - 1 = 2147483647. Continue PR #4922.
This is a development correctness extension of the existing shared-S3 example,
not an admitted performance experiment or a replacement for its open benchmark.

## Fixed inputs and arithmetic contexts

- Curve: y^2 = x^3 + 847514846*x + 2087444724 over Fp.
- Generator: (625037715, 2100627251).
- Declared subgroup order: r = 2147446831; retain the existing order check.
- Factor-base x coordinates: those of G, 2G and 4G, as in the existing fixture.
- Target: the existing public Q = 7G known-answer fixture. Only the caller
  knows the expected answer; recovery receives the public point Q.
- Coordinate equations and Macaulay elimination use p; relation coefficients
  and recovery use r. Do not substitute Mersenne reduction modulo p for r.
- Keep three finite leaf domains and an unrestricted auxiliary coordinate.
- Keep the degree-four bound, matrix limits and ten-second per-call F4
  cancellation guard. No elapsed-time result is collected.

## Required development checks

1. Compare the original F4 system and fresh/shared preconditioned systems
   with an exhaustive reference over the tiny declared factor base. The
   reference enumerates signed finite point prefixes, not all p auxiliary
   coordinates, and is used only for validation.
2. Require identical complete polynomial solution sets and exact signed
   group replay. Preserve original equations in every solver call.
3. Exercise Integer, FMA and Split16 preconditioners on the requested field.
4. Recover the public Q through actual F4 relations for Q + aG, with a
   finite query cap, the existing explicit G anchor, and IncrementalGauss
   modulo r. Do not supply factor-base logarithms or the expected answer.
5. Check the recovered scalar against public Q using the original group
   arithmetic, then compare with the caller-only expected answer.
6. Preserve the earlier exhaustive F17 controls. They are regression tests;
   the requested principal field is Prime31.

Any solver interruption, incomplete reference comparison, failed lift,
inconsistent relation matrix or incorrect scalar fails the corresponding
check. A capped search is not evidence of unsatisfiability.

## Evidence and scope

Build the existing actual-production-source integration qualifier, copy its
executables into a separate read-only directory, and write distinct build,
pre-call and post-call records. Never replace a pre-call record or old evidence.
Use one local Rust build job and one test thread, with a 180-second outer
correctness-call guard. Retain raw outputs and failures. No paid execution,
timed comparison or leaderboard update is part of this follow-up.

The chosen target and three-point base are deliberately structured for a
known-answer correctness check. This does not sample uniform unknown targets
and does not establish practical full-cost recovery. Full chart coverage,
the full-root-library gate and the previously requested admitted benchmark
remain separate open work in PROTOCOL.md.

## Sources checked before implementation

- Existing examples/shared_s3_matrix.rs and its production F4, root-finding,
  group, factor-base and relation-matrix source at 5c1a5e0130.
- Existing REPORT.md, PROTOCOL.md and development-receipt.json in this study;
  PR #4920 metadata and the previously selected Prime31 fixture.
- Dumas, Giorgi and Pernet, Dense Linear Algebra over Finite Fields,
  https://arxiv.org/abs/cs/0601133; full PDF retrieved for the arithmetic
  context, with no new arithmetic kernel proposed here.
- van der Hoeven, Lecerf and Quintin, Modular SIMD arithmetic in Mathemagix,
  https://www.texmacs.org/joris/simd/simd.html; exact modular operations and
  rounding bounds remain required.
- Semaev, Summation polynomials and the discrete logarithm problem on
  elliptic curves, https://eprint.iacr.org/2004/031. The landing page was
  retrieved; the PDF endpoint returned HTTP 403. The existing explicit S3
  formula and independent point-addition controls supply the local source
  fallback; this follow-up makes no new summation-polynomial identity claim.
- KB unavailable: no loaded research-KB tools; the repository wrapper cannot
  find the research-kb executable. The exact source and PR pointers above
  replace the KB lookup for this continuation.
- Conductor executable unavailable. Work is isolated in a new worktree and
  one child owns the example file; no other checkout is edited.

No curve fact is inferred from the field size. The existing numeric fixture
is retained. The sibling lane's proposed registry slug is not present in this
revision's registry and is not promoted here.
