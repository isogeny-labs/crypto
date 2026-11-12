# Results: chasing the five top-tier leads (2026-11-12)

Protocol: `PROTOCOL.md` (frozen at commit dccf37b6, amended at e50d4f23 before the S1 cells ran). All runs local
(4 vCPU), raw records in this directory with the engines' commit hashes; every number `[pcc:unverified]` (one
engine, one machine, no independent replay) unless a line says otherwise. Engines: ml-cryptanalysis at 27a67b9
(`scripts/ic_bounds_growth_lib.py`, `csd/ic_bounds_big.py`), N4v2 `v2.rs` built here (`j2/BUILD.txt`).

## P1 quotient law: stage 0 reproduced, stage 1 (fresh primes) — see table below

Stage 0 (`tools/p1_stage0.txt`): the twelve committed cells give q/2^k = 2.500, 4.375, 5.250, 5.156 (D = 3);
2.953, 5.016, 5.688, 5.365 (D = 4); 3.058, 5.083, 5.712, 5.374 (D = 5); 3.067 (D = 6): lane P's table to the digit.
Side finding: the N8 main instance (12289, 4021, 9917, 4408) is unit at k ≤ 6 and proper at k = 8, 9 (2, 16 zeros).

Stage 1 (`p1/`, `tools/p1_score.py`): STAGE1_PLACEHOLDER

## S1 crossing: the record's characteristic cells are void; rescoped cells — see below

**Structural result (exact, `tools/unit_test.py`).** Over F_l the chain ideal at depth k is proper whenever the
Semaev curve has an F_{l^m}-point with both coordinates in μ_{2^{v_2(l^m − 1)}} ⊂ μ_N; for the Fermat primes
F_l^× itself lies in μ_N (17 at k ≥ 4, 257 at k ≥ 8) and for 97 a third of F_97^× does, so unit instances do not
exist in practice: 0 of 60 seeded draws at k = 6 and 10 for each of l = 17, 97, 257. The record's cells C-c17,
C-c97, C-c257 (which needed 1 ∈ V_D) cannot be run. Primes with v_2(l − 1) = 1 admit unit instances freely
(19: 50/60, 23: 40/60, 43: 50/60, 107: 57/60, 227: 59/60), so the characteristic arm was rescoped to
l ∈ {19, 43, 107, 227} with one generator (the Frobenius images σ_l(g) of these l have weight ≥ 6, so the record's
"F-links" mechanism has no admissible element at D ≤ 5). This is also a statement about the binary analogy the
record draws: the l = 2 case is the extreme where F_2^× = {1} ⊂ μ_N trivially, and binary systems are never
single-equation chains of this form.

S1_PLACEHOLDER

## J2 percolation factor-base size — see table below

J2_PLACEHOLDER

## M1 three-summand arity grid (desk, from `ml-cryptanalysis/experiments/ic-bounds-m3-depth/`, exact records)

For m = 3 (variables 3k, generator NF(S_4) of degree 6): LFD = 7 at every k ≤ 5 (V_D = 0 for D ≤ 5 and V_6 = span g,
so LFD ≥ 7 is a proof for k ≥ 3; exact LFD 7 on 12 instances at k = 5, and V_7 = I ∩ M_≤7 at k = 2..5), and
LFD ≥ 8 at k = 6 on the one instance run (V_7 closed at dim 19,958 against 62,956 for the ideal part). Against the
§1 bar of `research/leads_20261112/README.md` (m = 3: LFD/k < 0.522 for the columns alone, < 0.184 with sparse
linear algebra): LFD/k = 1.4 at k = 5 and ≥ 1.33 at k = 6; columns at the LFD against the enumeration 2^{2k}:
16,384 vs 1,024 at k = 5 (16×) and ≥ 106,762 vs 4,096 at k = 6 (26×). The m = 3 Gröbner route is above
enumeration at every measured depth, with the gap widening. The only measurement that would add information is
D = 8 at k = 6 (106,762 columns) and k = 7 (401,930): GPU-class, not run. **M1 closes at the measured depths.**

## J1 quartic solver on cofactor-one Jacobians: chased to its obstruction (no run)

`J1_adapter_report.md` (code reading of N4v2): the D family reads only q, ℓ and the 15 quartic coefficients plus
the GHS curve E0 for the target, the three descent start classes and the final check; nothing in relations,
descent, linear algebra or verification uses 4 | #E0. A generic adapter (`v2 icx <inst.json>`) is about 140 lines
(a `Src` abstraction for start classes and the check, a forced base point, Tonelli–Shanks for q ≡ 1 mod 4, which
the GHS choice q ≡ 7 mod 12 hid). The obstruction is the instance side: the lab has no point counter for a
generic genus-3 curve (brute force is O(q^{2k}), unusable beyond q ≈ 20; no Sage/PARI/Magma on the host), GHS never
counts #J (it counts #E0 by BSGS and trusts the cover), and ℓ < 2^64 with #J ≈ q³ caps prime-order instances at
q ≈ 2^21. So the preregistered k = 12..18 grid needs a genus-3 point-counting instrument first (weeks), or a
family with #J known by construction, which reintroduces structure. Recorded as blocked, with the adapter sized.
