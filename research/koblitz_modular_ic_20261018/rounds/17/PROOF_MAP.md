# Round17 proof map

| module | named results | faithful kernel scope |
|---|---:|---|
|KoblitzRound17|7|Smith kernel parameterization under additive equivalences; odd-group averaging; dimensions; T-translation identity|
|KoblitzRound17Integral|12|actual integer intertwiners, fixed11 source/nonzero images/reverse cancellation, root counts and derivative checks|
|KoblitzRound17Theta|10|actual integral Smith and two-sided inverses; kernel order; six actual finite pairing-exponent congruences|
|KoblitzRound17Survivors|24|sixteen scalar separators, four actual Q22 units and four source quartic gate bindings|
|KoblitzRound17Boundary|12|four complete finite Q24 models, cubic zeros and nonzero status|
|KoblitzRound17LiftProof|4|universal integer Pic2 correction residual, cleared/rational characteristic2 consequences, CM norm identity|

Total69 named results. Generated equation lemmas are counted separately
by the full project audit. All reported proofs must use only propext,
Classical.choice and Quot.sound; no sorry, custom axiom or trusted native
calculation is accepted. The receipt is lean-full-audit.log.

The root-count and derivative proofs are actual finite statements over
ZMod11; the Hensel/étale lattice classification and integral Tate theorem
are written arguments, not hidden hypotheses in a purported geometry proof.
The Smith kernel theorem exposes its additive equivalences and matrix
factorization hypotheses. Literal inverse certificates supply the algebraic
unimodularity data, but the action of the CM order on elliptic points remains
a native/written bridge.

The averaging theorem assumes a supplied representation. G18's geometric
construction of such a representation uses written Miller/divisor rules
and exact native function evaluations. Its canonical character and full
F32/F2 compatibility are not established by that Lean theorem.

Three new IDC records bind the universal Pic2 square residual, CM norm
identity and T-translation residual to their Lean sources; the checker
adds128 parsed points. The reused quartic residual retains frozen
IDC1h84c81bd7c932e9cf from Round16.

Open full formalization: finite-field/Miller implementation semantics,
Tate/abelian geometry, global modular normalization and factor embedding,
all function-divisor identifications, and the closed projective quotient.

Completed transitive project audit:553 theorems,0 failing,0 trusted.
The69 new named results are distinguished from generated equation lemmas.

| identity | source-bound certificate |
|---|---|
|Pic2 correction square|[IDC1h31bc950ebd40dee8](identity-Pic2-correction-square.json)|
|CM norm product|[IDC1h9a53998afa83f290](identity-CM-norm-product.json)|
|T translation|[IDC1h9e3d3e3bc4edfd8a](identity-T-translation-x.json)|
