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
  (7, 15, 23, 24) did in the previous study. Another three-variable set is
  unmeasured.
- *Reading (not registered):* the 3-set leaves every summand in dimension 7
  at N = 43, structurally the ℓ = 7 systems that did not refute at degree 4
  on 8 of 8 draws, so its failure is what that result predicts. The pattern
  fits `m·(ℓ−7)+1` guesses, 4 at ℓ = 8. Extended linearly that is 105 at the
  priced m = 4, ℓ = 33 cell against a break-even of 6. It rests on two
  values of ℓ and is an extrapolation, not a result.
- *ECC2K-130 (derived):* at the priced m = 4, ℓ = 33 cell (N = 394) the
  ratio `C(394, ≤5) / (2^k · C(394−k, ≤4))` is 78.2, 39.5, 19.95, 10.08, 5.09
  for k = 0..4; the hybrid beats degree 5 only if k ≤ 6. No row moves, and no
  row beats rho.
- *Next:* the matched null (unspecialised (22, 7), same matrix size) and a
  3-set that leaves one summand at dimension 8, both within this container;
  ℓ = 9, m = 4 at ℓ = 6 and degree 5 past ℓ = 7 need a larger host.
