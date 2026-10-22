#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Four guessed variables bring every ℓ = 8 draw back to degree 4

These are the results of `research/dreg_closure_guess_20261022/`; the full
text is in `RESULTS.md`. *Class:* stage diagnostic.

#figure(image("reports/run-20261022T0555Z/guess.svg", width: 100%),
  caption: [4 draws × (G6 + 16 G4 branches), run 2026-10-22T05:55Z.])

- *Registered verdict: K_marginal favoured, four guesses suffice at
  ℓ = 8.* All 64 branches of the four-variable guess refute at degree 4;
  the dimension-6 positive control refutes on 4 of 4. K_excess, which
  predicted 6 guesses, is rejected for this guess set.
- *An upper bound:* k = 4 is the fewest guesses this container can measure
  at N = 46. A k that stays bounded as ℓ grows is not excluded.
- *ECC2K-130 (derived):*
  - The guess-then-degree-4 oracle `2^k · C(N−k, ≤4)^w` beats degree 5 at
    the priced m = 4, ℓ = 33 cell only if k ≤ 6. Both linear rules give
    k ≈ 105–108 there.
  - Its best case is the degree-4 row itself, `2^68.43` (`2^+7.62` over
    rho), still contradicted until k is shown to stay bounded.
  - No row moves, and no row beats rho.
- *Next:* k = 3 at ℓ = 8 (136,698 columns, possibly within this host at
  `--max-cols 140000`), then ℓ = 9 on a larger host.
