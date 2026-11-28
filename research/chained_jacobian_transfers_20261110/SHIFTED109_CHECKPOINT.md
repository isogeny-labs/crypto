# Shifted full-order109 transfer checkpoint

Producer cryptanalysis767c945a0a832c4f9d2827add8aefce3a16d2371,
branch research/chained-transfer-shifted109-20261111. Source digests:

|Source|SHA-256|
|---|---|
|src/nonsplit.rs|258440b5859be491fe4c6b17f1988e4bbd12c032e587138e6eb217af176598e7|
|src/nonsplit_map.rs|795d52766132a06f480d846c638142ec060adccb630f6f780ea53d4ef58b28f0|
|src/balanced.rs|5a40813acf7f600ee2631c6a2db3c5c8d8bd924d9161595d3d5462b76b554597|

The native runtime pin remains crypto75a737b8366ec6c78ed8672886423fcbe69853a2;
its seven guarded inputs are unchanged. Source-linked
[report](https://github.com/isogeny-labs/cryptanalysis/blob/767c945a0a832c4f9d2827add8aefce3a16d2371/experiments/chained-transfer-reference/SHIFTED109.md),
[diagram/PDF](https://github.com/isogeny-labs/cryptanalysis/blob/767c945a0a832c4f9d2827add8aefce3a16d2371/experiments/chained-transfer-reference/shifted109-report.pdf).
Both PDF pages inspected. All scientific standing remains unsigned.
[pcc:unverified]

## Actual fixed construction

Original E109/F101 is y²=X³+X+46,G=(2,37). Frozen proposal8774df
sets U=X−2, obtaining y²=U³+6U²+13U+56,G'=(0,37), and quadratic cover
C:y²=t⁶+6t⁴+13t²+56. All109 original/shifted coordinate, pullback and
norm[2] controls pass. Raw point counts N1=120,N2=10436 give
#Jac(F101)=12317=109*113. Reciprocal elliptic quotient
y²=x³+13x²+33x+5 has order113. [pcc:unverified]

The checked cubic quotient has coefficients[56,13,6,1]. Of all15 root
matchings,3 are Frobenius stable:1 degenerate and2 smooth targets, with
determinants63/38. Their raw coefficients are
[56,41,62,78,24,57,80]/[56,60,62,23,24,44,80]. Both have the same
point counts/Jacobian order; leading80=22² makes raw ordinate w=22Y an
explicit F101 normalization. First monic target[31,75,74,54,71,55,1];
the second is its reflection. Unshifted6-cycle negative and target
equivalence remain retained. No novelty claim for the prior Enge family.
[pcc:unverified]

The actual determinant63 correspondence and sign-corrected dual pass both
norms[2] on all12,317 rational source/target classes, coefficient descent,
paired additivity and a12,317-image bijection. All109 elliptic scalar
relations/source norm[4]/inverse82 recovery pass. F101⁶ intermediates,
whole quadratic fibers and balanced infinities are explicit. No map reads
scalar labels. Producer claims e580c9/4c335c bind inventory/full targeted
replay; three retained F17 map controls pass after parameterization.
Formatting/strict Clippy pass. Current full46-test guarded CI is pending;
the immutable earlier43-test receipt is not relabeled. [pcc:unverified]

## Independent field and compiled algebra

The pre-existing crypto flat Fp6 engine/Rabin checker replays128 frozen wide
F101 tower witnesses, with exact seed coordinates, products,inverses and
Frobenius powers. Let gamma=T+s,s²=2:

    s=(gamma³+6gamma²+19gamma+68)/(3gamma²+12gamma+15),
    gamma⁶+12gamma⁵+56gamma⁴+18gamma³+2gamma²+46gamma+33=0.

The F17 fixture remains byte-identical and both field tests pass.
PCC5dbc9d binds independent-shifted-field6-01.txt and the new immutable TSV.
Only test/fixture code changed; existing engine arithmetic is untouched.
This validates the bounded field layer, not independently the whole
Jacobian map. Both group layers use Cantor and harnesses are producer-authored;
unsigned attestations remain not_covered by the local PCC verifier.
[pcc:unverified]

Four ShiftedCover.lean lemmas pass gate4/local PCC7c396d: general cover
expansion, exact translation, root-product sign and inverse82 under109-torsion.
Total48 written lemmas, gates22+8+7+12+6+5+7+4 declarations. Local proof pass
does not create signed scientific standing; finite-field/Picard/global-dual
geometry remains written primary input. [pcc:unverified]

## Reproduce changed inputs and continue

Consumer check.sh with Rust1.99.1 and the seven unchanged upstream runtime
files; root `cargo test --release --bin icprog chained_ -- --nocapture`;
proof-gate.sh on lean/ShiftedCover.lean. The existing minimal sparse profile
automatically includes both tracked TSVs. The workflow proof loop now
includes ShiftedCover. Initial inventory compile failure and raw successful
targeted replay are retained separately. [pcc:unverified]

CT16 next computes actual target branch Frobenius action, extension2-torsion
and all stable next kernels. No unpolarized-Jacobian distinction or further
nonbacktracking hop is assumed. CT14 production/catalog/large-prime coverage
and all solver-cost obligations remain open. No benchmark, paid job, cost
ratio or cross-prime descent result. S3/live-budget,KB/vault and crypto
frozen-ledger/archive blockers remain; canonical graphs are unchanged.
[pcc:unverified]
