#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= The unspecialised (22, 7) system does not refute at degree 4

These are the results of `research/dreg_closure_null22_7_20261028/`; the full
text is in `RESULTS.md`. *Class:* stage diagnostic.

#figure(image("reports/run-20261028T0335Z/null.svg", width: 100%),
  caption: [Draws on which the degree-4 closure refutes, by cell. Run 2026-10-28T03:35Z.])

- *Registered verdict: N0 does not refute on at least 3 draws, the pure ℓ = 7
  reading.* The unspecialised (22, 7) system, with the same 43 unknowns and
  136,698 columns as the three-guess set, saturates degree 4 without the constant
  `1` on 4 of 4 draws. Controls valid (identity, dimension-6 positive,
  satisfiable soundness).
- *With the earlier cells,* 12 of 12 unspecialised ℓ = 7 draws do not refute, over
  N = 40, 42, 43 and surplus −2, 0, +1. The surplus confound is not what
  produced the earlier ℓ = 7 failure.
- *Relation to the three-guess set:* it failed on 3 of 4 draws and refuted on one.
  With four draws each, 3 of 4 and 4 of 4 are not distinguishable. The two
  cells are an analogy, not an identity.
- *ECC2K-130 (derived):* the three-guess failure is what ℓ = 7 does. The pattern
  fits `m·(ℓ−7)+1` guesses, 4 at ℓ = 8, and 105 at the priced m = 4, ℓ = 33
  cell against a break-even of 6. That rests on ℓ = 7 and ℓ = 8 and is an
  extrapolation. No row moves, and none beats rho.
- *Next:* a three-variable set that leaves one summand at dimension 8, which this
  reading predicts will fail; a degree-5 closure at ℓ = 7 needs a larger host.
