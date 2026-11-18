# H116 first omitted-Frobenius boundary experiment

Protocol `H116-FROBENIUS-BOUNDARY-001` was frozen at commit `ec89b5d977`
before either result was generated. The p=31, r=37 excluded fixture and
precision 18 were fixed. The preregistered prediction was that the normalized
retained `V-1` map and normalized scalar `[t-1]=[-6]` map would agree through
coefficient 14 and first differ at coefficient
`(p-1)/2=15`, with `V-1 minus scalar` delta `2/(t-1)=10 mod 31`.
[pcc:unverified]

## Retained v1 failure and correction

The first producer result is retained unchanged in
`FROBENIUS_BOUNDARY_RESULT.tsv`. Its scalar-map curve identity was false, and
the independent division-polynomial verifier rejected it. The defect was a
single wrong sign in the rational duplication numerator: `A^2 D^4` was
subtracted rather than added. Complete diagnostics and hashes are in
`FROBENIUS_BOUNDARY_V1_ERRATA.md`; the failure was not interpreted as a test of
the mathematical prediction. [pcc:unverified]

The corrected producer was placed in a new source file and refrozen at commit
`61f650e0b9`. The fixture, precision, prediction, controls and independent
verifier were unchanged. The v2 protocol SHA-256 is
`8d8945085db40c418c41f01d28f76d4be1201b3f97a762538e95262ded4da371`;
producer SHA-256 is
`9d24f17c29b44e467c98edfaece4512a9da4f682ab395ce41634982b416c6949`;
and the byte-identical v1/v2 verifier SHA-256 is
`c4af847dd450b9ce1b2893fe35f1123cc02f407ccf5745011eca43607ea4a52d`.

## Result

```text
V-1:      1,0,20,0,13,7,2,4,8,14,5,25,12,2,6,18,7,11
scalar:   1,0,20,0,13,7,2,4,8,14,5,25,12,2,6, 8,7,20
first difference: 15
delta modulo 31:  10
```

The measured first difference and delta exactly equal the frozen prediction.
Both rational maps satisfy their cleared curve identities. The producer
records 96,757 additions, 96,881 multiplications and four inversions for this
fixed 18-coefficient calculation. Those counts characterize this instrument;
they are not an asymptotic or wall-time claim. [pcc:unverified]

The independent verifier does not reuse the producer's `[2]([3])`
construction. It builds the scalar map from the division-polynomial recurrence
for `psi5`, `psi6^2` and `psi7`, then reconstructs the expected normalized
prefix from the retained TSV. It reports:

```json
{"verified_cells":1,"first_difference":15,"delta":10,"equal_coefficients":15,"map_identities":2,"negative_controls_rejected":3}
```

The omitted-scale control first differs at coefficient zero; a wrong target
fails its curve identity; and a mutation at coefficient 15 fails the map
identity. Result SHA-256 is
`d2efcf9d8eb20962b057e079be04a17e6e64d70b22a067b293a58950fb0bcad2`;
independent-verifier SHA-256 is
`ce94c6fb0c81ba8301269a7b27d894d3f58460f4c4bce1feda0b2da74547ec8b`.
[pcc:unverified]

## Lean mechanism and evidence boundary

The tracked Lean module now imports Mathlib's actual `FormalGroup` and proves:

1. `Y` divides `F(X,Y)-X` for every one-dimensional formal group over a
   commutative ring;
2. substituting zero-constant `s,c` gives `F(s,c)=s+cq` for some series `q`;
3. if `c=X^m tail`, then `F(s,c)` and `s` have identical coefficients below
   degree `m`;
4. after factoring a shared leading `X`, an order-`n+1` correction preserves
   the normalized double-pole and regular-unit coordinate below degree `n`.

`lake build H116FiberTrace` passes at default limits. The repository's exact
axiom auditor reports 84 theorems, zero failures and zero trusted native
decisions. This proves the generic filtration mechanism, not the remaining
elliptic endpoint. The actual identity that the supplied rational `V-1` map is
the formal sum of `[t-1]` with the correctly signed/scaled Frobenius correction
still needs a curve-level Verschiebung/Frobenius instance. The single p=31 cell
does not establish a growing-family law or a subexponential ECDLP algorithm.
[pcc:unverified]

The inspected report is
[`reports/run-20261118T0645Z/REPORT.pdf`](reports/run-20261118T0645Z/REPORT.pdf).
Primary source context is recorded in `PRIOR_ART.md`.

