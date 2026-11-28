# Z3primary source review

Versioned primary sources for4.15.4:
- https://github.com/Z3Prover/z3/blob/z3-4.15.4/src/params/sat_params.pyg
- https://github.com/Z3Prover/z3/blob/z3-4.15.4/src/sat/sat_cut_simplifier.cpp
- https://github.com/Z3Prover/z3/blob/z3-4.15.4/src/sat/sat_anf_simplifier.cpp

Parameter source identifies cut,cut.xor,cut.aig as optional and ANF as a separate setting. Actual pinned binary metadata/readback and hard statistics corroborate the selected SAT-cut experiment. ANF's implementation references the Groebner/PDDsolver header; it stayed disabled. No claim is made that Boolean cut extraction is a new generalized rootfinding algorithm, or that source review independently proves the hard search result. Numeric14residue/old-q/full-support model checks remain required if a model appears.
