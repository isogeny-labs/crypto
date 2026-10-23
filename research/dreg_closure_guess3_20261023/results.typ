#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Three guessed variables do not suffice at ℓ = 8 for this set

These are the results of `research/dreg_closure_guess3_20261023/`; the full
text is in `RESULTS.md`. *Class:* stage diagnostic.

#figure(image("reports/run-20261023T0516Z/guess3.svg", width: 100%),
  caption: [4 draws × (G6 + 8 G3 branches), run 2026-10-23T05:16Z.])

- *Registered verdict: three guesses do not suffice at ℓ = 8 (for this guess
  set).* 1 draw suffices, 3 do not. On draws 0, 3 and 5 the first branch
  saturates degree 4 without refuting (the other 7 were not run, by the
  registered stop rule); on draw 6 all 8 refute. Controls valid: N7 equals
  the committed record, G6 refutes on 4 of 4.
- *What it says:* three do not suffice for variables 7, 15, 23; four
  (7, 15, 23, 24) did in the previous study. One cell does not by itself kill
  the guess-then-degree-4 route or fix a growth law; another three-variable
  set is unmeasured.
- *ECC2K-130 (derived):* at the priced m = 4, ℓ = 33 cell (N = 394) the
  ratio `C(394, ≤5) / (2^k · C(394−k, ≤4))` is 78.2, 39.5, 19.95, 10.08, 5.09
  for k = 0..4; the hybrid beats degree 5 only if k ≤ 6. No row moves, and no
  row beats rho.
- *Next:* another three-variable set at ℓ = 8; ℓ = 9, m = 4 at ℓ = 6 and
  degree 5 past ℓ = 7 need a larger host.
