# Ten-target trace +26 cover-degree bound

**Update, 16 November 2026:** Five exact targets (masks 30, 103, 139, 194, 233) now have independently checked degree-seven covers and nonzero subgroup transfers; see `DEGREE7_FINDING.md`. Their minimum degree for this fixed carrier is exactly 7. These five K-models and their covers are one Frobenius orbit; the other five masks form the second orbit and retain the verified `[7,20]` interval. A complete measured s=3 attack for mask 30 is recorded in `S3_FULL_ATTACK_RESULT.md`; the s=11 costs remain open.

15 November 2026. This is a finite-class statement for one explicit carrier,
not a new infinite family of vulnerable elliptic curves. The finite-search
records and geometric reduction remain `[pcc:unverified]` until an independent
PCC theorem verdict is signed.

Let `K=F3[a]/(a^5+2a+1)` and

`Y/F3: y²=x¹¹+x⁹+2x⁸+x⁷+2x⁵+x³+x²+x+1`.

The directly checked Weil numerator is `1−26T⁵+243T¹⁰`. Consider the ten
K-curves `E_b: v²=u³+2u²+b` in the table. The b-mask is the base-three encoding
of the five coefficients of b in the basis `1,a,a²,a³,a⁴`.

| b-mask | Exact b | Degree-four candidates | Degree-five interpolation survivors | Degree-five maps |
|---:|---|---:|---:|---:|
| 3 | a | 0 | 2 | 0 |
| 27 | a³ | 0 | 2 | 0 |
| 30 | a³+a | 0 | 1 | 0 |
| 103 | a⁴+2a²+a+1 | 0 | 1 | 0 |
| 139 | a⁴+2a³+a+1 | 0 | 1 | 0 |
| 167 | 2a⁴+a+2 | 0 | 2 | 0 |
| 182 | 2a⁴+2a²+2 | 0 | 2 | 0 |
| 194 | 2a⁴+a³+a+2 | 0 | 1 | 0 |
| 230 | 2a⁴+2a³+a²+a+2 | 0 | 2 | 0 |
| 233 | 2a⁴+2a³+a²+2a+2 | 0 | 1 | 0 |

Each E_b has `#E_b(K)=218=2·109` and trace +26. The ordinary trace
discriminant `26²−4·243=−296` is fundamental, so each has the maximal order
`Z[π]` as endomorphism ring. The ten entries are the ten normal-form
K-isomorphism classes in the trace +26 census `class_count.json`.

**Theorem.** For each E_b in the table, no K-morphism `Y_K→E_b` has degree
at most six. A K-morphism of degree at most twenty exists. Thus the minimum
degree is in **[7,20]** for each exact target. Any such cover has degree
coprime to 109, and `φ_*φ^*=[deg φ]` makes pullback injective on the chosen
order-109 subgroup of E_b(K). This gives an existence and subgroup-preservation
bound into `J(Y)(K)`, not an evaluable map into `J(Y)(F3)`.

## Proof and finite receipts

The earlier `CUBIC_PROTOCOL.md` and `cubic_geometric_all_magma.json` exclude
degree-two and degree-three maps from this Y to **any** elliptic curve over the
algebraic closure; degree one is impossible by genus. That certificate is
geometric, so it applies to all ten targets and all base changes. The complete
degree-four pole-shape proof is in `DEGREE4_ALL_PROTOCOL.md`: 249 denominators,
59,697 assignments. The b-mask 3 producer and independent direct-linear replay
are in `degree4_all.json` and `degree4_linear_replay.json`. For each of the
remaining nine, `degree4_isogeny_all.sage` and
`degree4_isogeny_linear_replay.sage` independently test the same complete
shapes. Their per-mask JSON/stderr and `degree4_isogeny_manifest.json` show
exactly zero degree-four candidates, with every count and source hash checked.

For degree five, the complete six-shape denominator proof is in
`DEGREE5_THEOREM.md`: 60,269 denominators and 14,446,377 logical assignments
per target. Since every E_b has exactly one K-rational nonzero 2-torsion point
and a conjugate quadratic pair, each unpoled quadratic branch pair has three
Galois-compatible x-coordinate choices. The rational branch point `x=-1`
maps to the unique rational 2-torsion coordinate. The b-mask 3 producer,
direct-linear replay and Magma quotient check are in `degree5_screen.json`,
`degree5_linear_replay.json` and `degree5_survivors_magma.json`. The nine other
targets have separate source-bound CRT producer and direct-linear replay
receipts plus manifests. Both methods agree on the exact unordered `(A,B)`
set for each target: thirteen alternate-target degree-five interpolation
survivors in total. Every survivor fails the exact equation
`f C²=B(A³+2A²B+bB³)`, so no degree-five morphism exists. The order and
complete search space were checked separately for every b-mask.
The frozen `degree5_isogeny_survivors.magma` submits all thirteen exact
alternate-target pairs to public Magma 2.30-1. Its retained response
`degree5_isogeny_survivors_magma.json` reports HTTP 200, successful exit,
untruncated stdout, and a degree-eight nonsquare quotient for every pair;
all listed irreducible factors have exponent one. Magma independently checks
the full-equation obstruction; the two Sage constructions check the complete
branch interpolation and exact candidate sets.

For degree six, `DEGREE6_PROTOCOL.md` proves the ten complete denominator
shapes after translating the image of the rational point at infinity to the
elliptic origin. There are 60,518 monic denominators and 14,506,074 logical
branch assignments per target. The `LQiQj` shape additionally tests all 242
nonzero free numerator coefficients per assignment. The dominant `LH²` shape
solves three high-coefficient equations in two field parameters, including
the rank-deficient resultant case. The source-bound producer receipts for all
ten b-masks contain zero full-equation hits. `DEGREE6_REPLAY_PROTOCOL.md`
specifies a second implementation: it scans all first field parameters
directly, solves a quadratic for the second, and reconstructs polynomial
square roots coefficient by coefficient. `degree6_replay_manifest.json`
checks exact agreement of both methods for all ten shapes and all five
counters for each b-mask, with zero hits in every target. This is a
computer-assisted exclusion of K-defined degree-six morphisms under the
denominator-completeness argument, not a Lean formalization of that geometry.
An inseparable degree-six map would factor Frobenius of degree three and a
degree-two map from a geometric twist of Y, contradicting the earlier
geometric degree-two exclusion.

[Howe, Theorem 4.17 and Remark 4.18](https://arxiv.org/pdf/2110.04221)
give a K-map of degree at most twenty to each E_b, since `J(Y)_K` lies in
the isogeny class of `E_b^5` and `End(E_b)=Z[π]`. This completes the interval.

## Prior work and limits

The isogeny-assisted extension of GHS is established by
[Galbraith and Smart](https://www.cryptrec.go.jp/exreport/cryptrec-ex-1029-2001.pdf)
and discussed by [Karabina et al.](https://math.dartmouth.edu/~carlp/WeilDescent-Product.pdf).
[Tian](https://arxiv.org/pdf/2012.07173) gives a distinct explicit isogeny
transfer in degree three, including image recovery on the Jacobian.
[Tian--Li--Wang--Yu 2018](https://doi.org/10.1007/s10623-018-0457-6)
already constructs odd-characteristic degree-three covers for cofactor-two
elliptic curves. [Iijima--Momose--Chao 2015, full text](https://citeseerx.ist.psu.edu/document?doi=233a0bbe6b36d25753b1d8383abdc768922dd8d8&repid=rep1&type=pdf)
states a complete classification of direct `(2,...,2)` covers in odd
characteristic for prime extension degrees two, three, five and seven. Its
degree-five genus-five Case 13 requires four rational branch roots, whereas
the order-218 source orbit has only one rational 2-torsion point.
Those works limit claims about general cofactor-two and direct-cover families,
but do not imply a classification of all Jacobian isogeny carriers. The earlier repository
`research/ghs_cover_census_n5_20261013/README.md` excludes low-genus direct
`(2,...,2)` covers for the relevant order class, but does not give these exact
maps. The local LMFDB snapshot already lists the Weil-polynomial isogeny class
with uncertain Jacobian status; the explicit Y identification is a potential
catalog correction, not established publication priority. The KB revision
`9141e386254340a9bdba3036f2bb706a` predates this branch. These checks
support a new exact finite-class cover obstruction relative to the inspected
records; they do not certify a new attack family or method.
All ten targets share the same LMFDB isogeny-class flag, so that catalog check
applies to every table row; targeted individual-equation searches returned
irrelevant matches and cannot establish priority for any row.

The theorem covers this one Y, these ten E_b, and K-defined curve morphisms.
It leaves degrees seven to twenty, other carriers in the same Weil-restriction
class, correspondences, maps over larger fields, explicit E-to-E isogenies,
an F3-rational subgroup transfer, and full same-target attack costs open.
The 21 axiom-free declarations in `Odd5Ledger.lean` check arithmetic counts
and degree ceilings only; they do not formalize geometry or exhaustiveness.
