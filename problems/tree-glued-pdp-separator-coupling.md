# Separator coupling sets the solving degree of tree-glued prime-field PDP systems

- **Status:** `investigating`
- **Kind:** empirical bottleneck; resource limit (the deciding cells need more than 15 GB)
- **Affected work:** `research/ic_tree_gluing_20261103/` (rounds 1–6, `STATUS.md`); the symmetric PKM line (`symmetric-pkm-complete-cost.md`)
- **Next action owner:** unassigned. The big-host run needs lab AWS access and an msolve-capable job (`research/ic_tree_gluing_20261103/round4/BIGHOST.md`)
- **Last updated:** 2026-11-04

## Problem

We cannot yet show that any point-decomposition encoding over a prime field
has an F4 solving degree D with slope at most 1 per summand and flat in the
tower depth. Without that, the dense-Macaulay cost law does not beat rho at
cryptographic sizes [pcc:unverified].

## Evidence and uncertainty

- **Observed** [pcc:unverified]:
  - With every partial sum ("separator") fixed, D = 4 in every cell up to
    m = 6.
  - The cherry tree's excess D − 4 is 0, 2 and 3 at m = 2, 3, 4, at least 5
    at m = 5 and at least 6 at m = 6, at t = 4–5.
  - Its depth slope is about +0.5 per level at m = 3.
  - Recorded with msolve 0.6.5 at p = 786433; logs are in
    `research/ic_tree_gluing_20261103/round*/logs/`.
- **Derived** [pcc:unverified]:
  - Enumeration-type solvers pay at least √p (counting bound, `round5/SOLVERS.md` §2a).
  - Character or rationality branching is invisible to algebraic solvers.
  - The cost law needs ω ≲ 1.3 and β = 0 to beat rho at 2^256.
- **Suspected explanation:** the excess comes from coupling through elliptic
  partial sums. These have no low-complexity description (an Elekes–Szabó-type
  heuristic, `THEORY.md` §2). This is a conjecture.
- **Missing capability:** a host with at least 64 GiB, or a memory-lean F4,
  for m5 t4 and m6 t5.
- **What this evidence does not establish:**
  - Laws beyond m = 6 or t = 7.
  - Other primes or curves; one curve was measured.
  - That no node relation of degree < 4 exists outside x-only coordinates.

## What would unblock it

- [ ] m5 t4 and m6 t5 complete with exact D, on the pinned inputs in
  `round3/SYSTEMS.sha256`. Then P2 is decided: E(5) ≤ 5 and E(6) ≤ 6, or not.
- [ ] A node relation with input degree < 4 whose tree has D below the
  cherry tree's in a completed, planted cell.
