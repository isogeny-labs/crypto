# Frozen adjacent degree-eight L-cubed quadratic-square screen

16 November 2026. This extends, rather than changes, `PROTOCOL.md`. It uses the same eight exact representative source/carrier pairs and the same finite-field model. Source `search_L3.sage` SHA256: `4e4105547f3396c3c762b81c577d37b9751a4f47b23b35ecc25205e82971c0cc`.

The new denominator is `B=L³ H²`, with L the unique rational branch factor and H monic squarefree quadratic coprime to f. It has degree seven and the required odd branch-pole multiplicity. As before, CRT fixes `A=(Bh) mod (Q1...Q5)`; degree eight requires its x^9 coefficient to vanish. The script searches every compatible branch assignment and every `(u,v)` root, including zero-polynomial fallback, then checks the exact identity `f C²=B(A³+A²B+bB³)`. Point-square evaluations are necessary-condition filters followed by full polynomial verification. A zero-hit result excludes only this denominator shape.

Run `sage search_L3.sage <fmask> <bmask> > l3cell_<fmask>_<bmask>.stdout 2> l3cell_<fmask>_<bmask>.stderr`; the script writes immutable `l3cell_<fmask>_<bmask>.json` and refuses overwrites. A hit requires independent identity and degree replay, subgroup pullback, nonzero subfield norm, and full same-target attack-cost comparison. The original `B=L H²` receipts remain separate.
