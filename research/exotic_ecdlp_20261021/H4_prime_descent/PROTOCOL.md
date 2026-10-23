# H4 — FPPR prime-degree descent to F_2: first-fall degree trend toward n=131 (ECC2-131)

Pre-registration. Tests C4. This is the one route that actually exists on ECC2-131.

| Field | Value |
|---|---|
| Object | `E/F_{2^n}`, `n` prime `∈ {11,13,17,19,23,29,31,37,41}` (as far as time allows; ≥5 points for a fit), random ordinary curves, plus the real ECC2-131 model for `n=131` structure only. Factor base = an `F_2`-linear subspace `V ⊂ F_{2^n}` of dimension `l ≈ ⌈n/m⌉`, `m=3`. Semaev `S_{m+1}` decomposition "`x(R) = x(P_1)+..` with `x(P_i)∈V`" expanded via Weil restriction into `n` Boolean equations (FPPR), with the 2-/4-torsion (FGHR) symmetry where the curve has rational 4-torsion. |
| Claim (C4 counterexample) | The first-fall / solving degree `d_ff(n)` of the Boolean system stays **bounded** (constant) as `n→131`, so one `m=3` decomposition costs `< 2^{21}` bit-ops and the whole IC attack is subexponential. |
| Observable | For each `n`: the first-fall degree `d_ff` (degree at which the first non-trivial degree drop occurs in F4/F5), the solving degree `d_solv`, the Macaulay-matrix size at `d_solv`, wall-clock and estimated bit-ops of one decomposition (planted-solution arm for correctness + random arm for cost). Fit `d_ff(n)` and `d_solv(n)`; extrapolate to 131. |
| Null (C4) | `d_ff` and `d_solv` grow with `n` (consistent with Bardet–Faugère–Salvy / the known "first-fall degree grows" results for these systems), giving extrapolated cost `≫ 2^{21}` at `n=131`. |
| Win | `d_ff(n)` flat (slope ≤ 0 within error) and extrapolated solve cost `< 2^{21}`. |
| Kill | `d_ff(n)` increasing; extrapolated `n=131` solve cost `≫ 2^{21}` (report the gap in bits to the `2^{21}` gate and to the `2^{65}` rho floor). |
| Scale | The whole experiment IS the scale test: the `d_ff(n)` and `log2(ops)(n)` trends and their extrapolation to 131. |

## Method
1. For each `n`: build `F_{2^n}` (poly basis), a random ordinary `E`, a subspace `V=⟨v_1..v_l⟩`, `l=⌈n/3⌉`. Build the chained-`S_3` system for `x(R)=x(P_1)+x(P_2)+x(P_3)`, Weil-restrict to `n` Boolean eqs in `3l` variables; add field eqs `v_i^2=v_i`. Apply FGHR symmetrization if `E[4]` rational (else note raw).
2. Two arms: **planted** (R = sum of 3 known V-points; must solve, correctness control) and **random** (R uniform; relation-collection cost).
3. Solve with Gröbner over `F_2` (Sage/Singular `std` with `degrevlex`, or PolyBoRi/`BooleanPolynomialRing` + `groebner_basis`). Record the step degrees to extract `d_ff` (first degree with a fall) and `d_solv` (max degree reached), the largest Macaulay matrix, time.
4. Repeat `≥ 5` random instances per `n`; report medians. Censor (not fail) any instance exceeding the per-instance time/memory cap.
5. Fit `d_ff(n)=a+b·n` (and a sublinear alt `a+b·log n`); extrapolate to 131 with a band. Convert `d_solv` to bit-ops via `(Macaulay cols)^ω`, `ω=2.37`, and to group-ops for the `2^{21}` gate; report the bit gap at `n=131`.
6. Output JSON: per-`n` `(d_ff,d_solv,cols,time,arm)`, the fits, the extrapolated `n=131` cost, the gap to `2^{21}` and to `2^{65}` rho.

## Decision rule
Kill if `b>0` (first-fall degree increasing) with the fit's 95% band excluding 0, and extrapolated `n=131` cost `> 2^{21}`. Win if the band includes a bounded `d_ff` and cost `< 2^{21}`.

## Isolab
One job, `cpu 16 mem 32Gi seconds 7200`. Expect `n ≤ ~31–37` to finish per the Shantz–Teske regime; larger `n` likely censored, which is itself the trend evidence. Boolean Gröbner is the bottleneck.
