# General Green bounds, stationarity guard and literal equality counter

[Report](reports/report.pdf), [protocol](PROTOCOL.md), [proof map](proof-map.json),
[source plan](sources.json), [source review](evidence/source-review.json),
[verification](verification.json).

Main: `MarkovGreenBoundAudit.actual_green_bound_and_counter_certificate`.

The [actual general Green-bound audit](README.md)
proves the finite inequality in KMPT2010 Lemma3.3 for actual transition
kernels with uniform stationary distribution. From an official row-
stochastic matrix, Lean constructs actual PMFs with those real entries,
derives stochasticity of every power, and defines the inclusive Green
sum and actual finite maxima. Row normalization proves the sharper
bound sum(2j+1)*max(P^j), hence the source2*sum(j+1)*max(P^j).
Column normalization gives the column half through transposition.
Uniform invariance of normalized1/card(S) is proved equivalent to that
column condition. No desired Green moment is assumed. [pcc:unverified]

An actual positive reset chain on ZMod11 has q(0)=1/2 and all ten other
weights1/20. Its normalized PMF is irreducible/period1 and q is stationary,
but the uniform law is not. AtT1 column energy at0 equals19/4 while the
source right-hand side is4, so the actual column maximum violates the
bound if uniform stationarity is deleted. Qualified Lemma3.3 survives.
[pcc:unverified]

A separate actual uniform prime-state family refutes the literal printed
asymptotic equality following Lemma3.3 on reprintp8. For every nonzero N,
actual Green maxima equal1+(T²+2T)/N. Choose actual primes N_t>=3^(t+1),
take T=t, and error1/N_t. Lean proves error=o(1/t²), and the actual
pointwise envelope P_t^j(x,y)<=error+(1/2)^j for ALL powers j including0.
Both actual Green maxima tend to1, so the literal displayed equality
(2+o(1))/(1-1/2)², which would tend to8, is false. Even an eventual
representation with an actual vanishing remainder is negated. Every
member still satisfies the finite source bound; zero-time and singleton
controls pass. This counter concerns the printed equality. The source
finite upper bound, collision asymptotic order and qualified larger-
partition adding constant remain separately scoped. [pcc:unverified]

Local **PCC-20261116-7bb318 PASS at 2026-11-16T09:55:43Z**, signed unverified. Wholegate78 declarations
including generated equations,0failures/0trustednative. Seven modules,
six concrete and one official bridge; final new-source warnings0. Nine
states retain failures01–08 and pass09. All6163 entries/48initial plans
match around every state/gate/replay:6127 published preservation entries
plus36 StaticHash dependency entries. Producer6211records across
1874 unique committed paths all match.
Fourteen packages/no copies/hydration/update. Full172-character main
header SHA`79c651c05bc20361d063307cb87fcbfdacebef115c99371791ee0e6eda340318`
with four transparent pinned bundles/full supporting headers; separate
complete binding after actual gate1complete/0prefix/0unresolved/empty
suffix, normalized/parsed headers read separately.44formal/19artifact
pins, full inline statement/sourceplan17614B;26pinned custom modules,
19actually imported(12credited+7new),7additional frozen modules retained
as pins. Lossless295files126objects172612XZ/three actual parts, roundtrip
and actualrestore equal. Firstvalid one-page47052Breport/all4409textchars
read/complete individual visual/inbounds. Same-host checks are not
independent compiler/parser/host assurance. Standard foundations/default
limits only; no holes/custom axioms/native proof evaluation/new flags/
waivers/frozen edits/mathPython/spend. [pcc:unverified]

Source: Kim–Montenegro–Peres–Tetali2010/DOI10.1214/09-AAP625; actual
arXiv0712.0220v2 electronic reprint27Sep2010,300644B/SHA
`67bd4313585101da1138a8e08bf3a3045b6347fe289d4bd076594b9d57f91b07`;
its pagination/typography differ. Bounded text1–8/start9 and individual
visuals6–8 credited; current full lemma proof text and complete page8
visual reviewed, not all29pages. Twelve actual KB receipts and three
source-review Conductor receipts127; initial protocol’s count10 corrected
to actual12 in source review. Canonical/official/primary fallback; no
novelty/absence claim. All14 remain active, including general Markov
collision integer-block repair, qualified adding-walk limiting constants,
practical hashes, scalar-recovery runtime and recorded geometry/solver/
counting obligations. [pcc:unverified]

Publication: ready [PR #4389](https://github.com/isogeny-labs/crypto/pull/4389). Remote proof `0a9af6e9c78769248a64a9b6456f58052d786492` and local receipt `cda80aee08156b11e722bd6536d8a51a694d3514` share tree `abfec0a55101c0304f9dc4722e3e4aa9617cd55f`. Anonymous fetch completed before exact fresh head/title/body/ready confirmation. All48 pinsets/6190 entries match. New-head CI/review/signing pending; next scheduled owner10:30UTC. [pcc:unverified]
