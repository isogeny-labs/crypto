# Scope and pins

Lean v4.35.0; shared CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e;
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Transitive revisions are
fixed in lake-manifest.json. Only standard foundations are permitted.

`JetFlattening.value` proves, for any rational polynomial p and rational t
with p(t)=1, that the actual defined flattening has value1 at t.
`JetFlattening.first_derivative` proves its actual derivative evaluates to0
under the same hypotheses. There is no assumed desired derivative value.

Other-node values, higher derivatives, full chart integrality, valuation
and rank observations, and matrix-completion proposals are not formalized.
The generic identities do not prove either proposed improvement. Initial
successful source/linter output is preserved; final source removes an
unreachable tactic. No sorry, custom axiom or native evaluator is used.

Final clean build, actual-repository shared proof gate (2 declarations,
0 trusted), and official import-inclusive fresh replay all exited0.
`fresh.log` records `replaying JetFlattening with --fresh`.
