# XP-20261024-published-ic-reference-arm

**Status: blocked** (the arm is specified, not built; one smoke run exists, see
[`SMOKE_20261024.md`](SMOKE_20261024.md)). Labels: *recorded* (cited file), *derived*
(arithmetic shown), *predicted* (stated before a run). Nothing here is a measurement of a
full reference-arm solve.

Files: this README, [`PROTOCOL.md`](PROTOCOL.md) (**DRAFT**, not frozen),
[`INVENTORY.md`](INVENTORY.md) (what exists, what must be built, costs),
[`RECONCILIATION.md`](RECONCILIATION.md) (reporting proposal, no edit of `AGENTS.md`),
`card.json`.

## 1. Question

At matched (curve family, n, m), what does the best *published* index-calculus pipeline cost,
cold, whole-solve, one target, answer verified, in the leaderboard's group-addition-equivalent
(GAE) unit, and how far is every lab producer from it?

## 2. Why

The stated external goal is to beat the best published index calculus for ECDLP
(owner rule in `AGENTS.md` "Success criterion", lines 74-110).
`ic-regime-review-20261024/REVIEW.md` §1 item 4 records that no experiment runs a standard
FGHR-style symmetric Gröbner decomposition with the standard base as reference; §3 item 5
records that the twist program compares only to negation rho; §5.1 F2 asks for this protocol.
The leaderboard's own references are rho only: Table B "Reference: Matched Pollard rho on the
same instance" (`docs/ic/LEADERBOARD.md`, Table B header). Every "x times" in the repo is
against rho or the lab's previous producer.

## 3. The arm, per regime (summary; `PROTOCOL.md` is normative)

| regime | descent / system | factor base | symmetry | solver | m / n |
|:--|:--|:--|:--|:--|:--|
| binary `E(F_{2^n})`, random and Koblitz | Weil descent of Semaev `S_{m+1}` (Diem; Petit-Quisquater; Gaudry) | standard `{P : x(P) in V}`, `V` an `l`-dim `F_2`-subspace, `l ~ n/m`; for Koblitz also the Frobenius-invariant subspace base (same dimension), as a labelled second base | FGHR 2-torsion `T` (invariants in the frame with `T: t -> t+1`) + Galbraith-Gebregiyorgis variable choice; Frobenius invariants only on Koblitz | msolve F4; WDSat (SAT + XOR, pinned commit) | m in {3, 4} |
| prime extension `E(F_{p^n})` | Gaudry/Diem, `S_{n+1}` over `F_p` | `{P : x(P) in F_p}` | FGHR `D_n` (n=4: `D_4`) | msolve | n in {4, 5} |

## 4. Frozen sizes (the existing ladders)

- Binary, GAE Table B ids present in `docs/ic/LEADERBOARD.md`: `icv1-f2m{13,15,18,19,21,23,24,27,37}`
  (random and K_0; Table B rows 15..27 at m = 2; 13, 19 in the F4/F5/F6 panel; 37 in the n37 section).
- Koblitz, Table A ids: `icv1-f2m{41,43,45,47,53,57,59,61}`. **Table A's unit is a clock-priced
  batched affine addition, not GAE** (Table A header), and `docs/ic/calibration.json` has no Koblitz
  instance above n = 41 (instances list), so these sizes fall back to host-measured factors until
  pinned (calibration `fallback` field). The protocol pins them first (M6).
- Prime `p^4`: `E(F_{29^4})` is the only ecbench end-to-end instance
  (`research/gaudry_quartic_d4_ecbench_20261013/README.md`); the twist ladder p = 37, 73, 97 is
  in REVIEW §1 item 2; oracle-only cost ladder p = 269, 1009 in `research/gaudry_quartic_d4_cost_20261014/README.md`.
  `n = 5` has no instance, no calibration and no code in `src/cryptanalysis/gaudry_*`.

## 5. Decision value

A row of ratios against the reference arm turns the program's "x times rho" into "x times
published", or shows that the published arm is itself far above rho at these n (the likely
outcome at toy sizes: the reference arm's own cost is what the lab's ratchet should be measured
against). It also prices the (4,6) Weil-descent cell against a published floor (REVIEW §5.1 F1/F2).

## 6. Cost

Spec: done here. Build: ~4-6 engineer-weeks (INVENTORY §3). Run: one week on the existing
ladders after build (REVIEW §5.1 F2: 2-3 days to specify, one week to run).
