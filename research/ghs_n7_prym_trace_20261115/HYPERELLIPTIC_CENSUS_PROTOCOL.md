# Complete F2 genus-seven hyperelliptic isogeny-class screen

Frozen before the run, 2026-11-15 UTC. This extends the earlier necessary trace screen. It does not assert a GHS transfer map or a discrete-logarithm cost.

## Literature and coverage

Borodin–May, *On Weil Polynomials of Hyperelliptic Curves over Finite Fields of Characteristic 2* (2025), Lemmas 2.1–2.2 and 4.1, and Theorem 4.5: <https://arxiv.org/pdf/2508.16886>. The authors' source is <https://github.com/bmatvey/char_2_hyperelliptics>. Our branch-support argument is in `BRANCH_SUPPORT_BOUND.md`. The internal lead `LEAD-TRF-weil-restriction-jacobian-class` previously proposed searching for genus-n Jacobians in Weil-restriction isogeny classes. The 2004 Menezes–Teske paper already defines the W1 source family.

For ordinary E/F128 with odd trace t and a genus-seven **hyperelliptic** Y/F2 satisfying J_Y ~ Res E, the branch support has one F2 point and one degree-seven orbit. Move the rational point to infinity. Write

`Y: y^2 + v(x)y = u(x)`

where v is monic irreducible of degree seven, deg u=15, and the coefficient of x^15 is one. The 18 monic irreducible septics are paired freely by x -> x+1, leaving nine representatives. A more general degree-16 right-hand side has coefficients a15 != a16 at infinity (Borodin–May Lemma 4.1); shifting y by x^8 sets a16=0 and a15=1. Thus all target isomorphism classes occur among the nine supports and all 2^15 values of the lower coefficients of u. We deliberately retain isomorphic duplicates for a simple completeness argument: exactly 294,912 equations are screened.

For each equation, test the exact point counts N_m=2^m+1 for m=1,...,6. At m=7, define t=(129-N_7)/7 when integral and compare with W1 trace classes {-17,-15,-9,-7,-1,1,7,9,15,17}. For every full point-count hit, test smoothness by Borodin–May Lemma 4.1: `gcd(v, u'^2 + v'^2 u)=1` and the infinity condition. For irreducible v the gcd condition is equivalent to a nonzero remainder modulo v. Count every hit by trace; print every v,u hit and an exact witness for each trace. A count-vector match alone is not an algebraic transfer map.

Independent replay: use a second finite-field implementation that enumerates all y for every x for each reported hit and checks N_1,...,N_7. The producer uses the trace criterion for `z^2+z`. Verify 18 septics, nine translation pairs, and 294,912 equations. A zero hit for a trace is an exhaustive obstruction only to genus-seven hyperelliptic Jacobians over F2 with this exact Weil polynomial, not to nonhyperelliptic targets or higher genus.

Fields F_(2^m) use monic irreducible moduli `0x3,0x7,0xb,0x13,0x25,0x43,0x83` for m=1,...,7. All field arithmetic and polynomial calculations are native Rust; these moduli are checked by polynomial-factor tests in the program. The output directory is `reports/hyp-census-20261115T*/` with source hash, command, checkpoint ID, raw counts, and replay receipt.

Stop on any failed field, irreducibility, pairing, equation-count, point-count, or replay assertion. A positive geometry hit survives only as a candidate isogeny class. A vulnerability claim would additionally require an explicit subgroup-preserving algebraic map with nonzero action on the chosen source subgroup, complete target recovery, and measured total attack cost below matched rho.

## Amendment A, before corrected run

The first local run at `reports/hyp-census-20261115T0851Z/result.txt` completed the same 294,912-equation search, but its display-only W1 trace list omitted classical traces ±15. The numerical stage counts and witnesses were unaffected. Preserve that raw file. Correct the list to include ±15 and emit every hit for independent replay; re-run after a fresh checkpoint. No search boundary or mathematical hypothesis changed.

## Amendment B, orbit certificate

The corrected census and direct-y replay agreed on every stage count and trace. An added y-shift orbit assertion stalled at r=0 because its polynomial-squaring helper did not handle zero; that interrupted run is preserved at `reports/hyp-census-20261115T0854Z-result.txt` (no final summary). Guard zero inputs, checkpoint, and re-run. This affects only the new orbit assertion, not the completed census or replay. The assertion requires the 128 equations for each trace to equal one orbit under `u -> u+r^2+vr` for deg r≤7.
