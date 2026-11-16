# AUDIT MP-symmetry-1 (2026-11-01)
audit: partially_valid. corrected_outcome: inconclusive (stated: supports). reran: true.

## Reran
n=11 A0, k=4, cmax=12, in $TMPDIR (so raw files not overwritten): xr=1493,2024,119,1082 identical; cstar_full=cstar_triv=9, span_stable=true, dim_span=11. Wall 68s. Deterministic, reproduces.
RESULTS.json vs raw_*.json: counts, c*, rank ratios, wordops ranges (22.8-24.68) consistent. Moduli for n=11,13,17 irreducible (sympy checked).

## Code vs protocol: correct
- gmap v -> ((v//l+1)%3)*l + v%l is the cyclic block shift. A0 uses identical basis in all 3 slots, so Z/3 is an exact symmetry.
- Triv map: coordinate = parity of coefficient sum over each orbit = Reynolds (1+g+g^2) in the orbit-sum basis (3=1 mod 2, |G| odd, valid over F_2). Rows only over orbit reps m: valid because the basis is degree-filtered (leading-term echelon) and span is G-stable.
- contains_one: reducing e_0 against highest-bit pivots is correct (1 in span iff pivot at bit 0 exists). Constant has index 0 in both arms.
- No off-by-one found in the column set (4017 = 4096-79 squarefree monomials deg<=9 in 12 vars).

## Problems
1. Control arm has no power, and the protocol itself flagged this as a falsification clause ("A-null agreement 100% with identical ranks -> inconclusive on power"). That clause fired, so the pre-registered verdict is not "true".
2. Null arm is not a matched control. It changes the polynomial system (independent V_i), not just the group action, so "c* rises 9 -> 10 under symmetry breaking" is confounded with the loss of structure; the Z/3 reduction applied to a non-stable span is mathematically meaningless. A real control would keep the system and apply a non-symmetry gmap (e.g. shift on j-index), or perturb one generator, and show triv gives wrong c* or wrong rank.
3. Null arm deviates from protocol: 4 instances at n=11 only; protocol says 8 targets at n=11,13. README says 4/4 honestly but does not flag it as a deviation. A1 also not run (disclosed).
4. Seeds deviate: protocol says seed 1000n+k per target; code uses one Random(1000n+l) stream across instances. Reproducible, but not as registered.
5. Saturated regime: N=12 variables, squarefree columns max deg 12; c*=9 is near the trivial ceiling and equal for n=11,13,17 (dim_span=n). Agreement of c* is almost forced; it says nothing about d_F growth (README admits).
6. W_c stability check covers only every 7th multiplier row and only at c*; "0 new dims" is a sample, not a proof (immaterial given theorem).
7. Wordops ratio 22.8-24.7 compares two python incremental echelons with different sparsity/fill; it is not evidence for F4/M4RI. "Memory ~1/9" is cols*rows dense bound (0.336*0.345=0.116, ~1/8.6), not measured memory.
8. 18 CPU-min figure plausible (A0 ~27s*24 + null ~70s*4), under 20.

## README overclaim
Header "Outcome: SUPPORTS" is overstated: only the narrow A0 prediction holds, and it is a textbook identity. README caveats are otherwise honest (null no power, no novelty, engineering). Recommend outcome "inconclusive on power; implementation check passes".
