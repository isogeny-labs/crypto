# Autoresearcher integration handoff

Latest continuation: NONSPLIT_MAP_CHECKPOINT.md / CHECKPOINT.md. The actual
fixed F17 non-split map is now implemented; earlier prospective paragraphs
below are historical snapshots. Vault state remains locked.

Destination: `crypto-autoresearcher` lead atlas, after configured vault access
is restored. Last inspected commit:
`8edec422a60d87b2cbc656fb02f99f42eba959c7`. Its `VAULT.md` requires encrypted
storage; `tools/vault/vaultctl.py status` reported locked and no configured
KEY binding was available. No plaintext research file was written there.
This handoff remains in the authorized crypto study and GitHub queue #3904.

## Evidence to reconcile, without repeating it

1. Original public fixture: F101, E:y²=x³+1, P=(4,41), r=17,
   C:y²=x⁶+1. Whole-fiber pullback and both norms admit all10404 rational
   Jacobian classes, including nonsplit, ramified and infinity support.
   An explicit Richelot map has kernel4, image2601, equal fibers and both
   dual norms[2]. This is local unsigned producer evidence. [pcc:unverified]
2. Model audit: its target C2:w²=41z⁶+9z⁴+9z²+41 is F101-isomorphic
   to C by x=(z−1)/(z+1), y=80w/(z+1)³. The Richelot map is a
   nontrivial endomorphism after identifying models. The complete elliptic
   endpoint equals the historical elliptic2-isogeny after[2] and a coordinate
   isomorphism, on all102 inputs. Preserve these two equivalences as negative
   controls, not closures of every computational transfer. [pcc:unverified]
3. Preselected successor: splitting (x²−1)(x²+x+4)(x²−x+4), determinant91,
   source/target sextic coefficient lists [85,0,9,0,6,0,1] and
   [21,0,83,0,81,0,20]. Each Jacobian has9216 rational classes, but rational
   2-torsion cardinalities4 vs16 obstruct F101-isomorphism. Both complete
   class sets, dual norms, native arithmetic and pre-existing BigUint
   arithmetic were checked. Source elliptic order96 has largest odd prime
   factor3; P=(48,20), first endpoint(2,72), second endpoint zero. Retain
   r=3 explicitly. Its complete96-point endpoint also equals a direct
   elliptic2-isogeny after doubling, followed by tau(X,V)=(100X+80,91V).
   No computational advantage is established. [pcc:unverified]
4. Eighteen core Lean lemmas compile, with standard axiom gates auditing
   22+8 declarations. They give conditional structural criteria and cleared
   coordinate identities, not a formal construction of Picard groups or
   Richelot geometry. Signed standing remains unverified. [pcc:unverified]
5. Prime-field boundary: ordinary proper-subfield Weil restriction does not
   apply to a prime field. The Lean characteristic obstruction applies to
   unital additive/field maps, not arbitrary encoded cyclic group transfers.
   Lifts and cross-prime reductions remain open mechanisms. [pcc:unverified]
6. The predeclared next pairing(2,18),(-2,28),(-18,-28), with leading scale20,
   gives determinant67 and C_u coefficients[38,9,29,62,86,17,8], base11.
   Both full9,216-class domains and both dual norms are checked. The next
   kernel meets the previous dual kernel trivially; rational composite kernel
   size8 with4 self-inverse elements differs from geometric degree16. The
   three rational2-torsion cardinalities4,16,8 distinguish all three
   Jacobians over F101. Original r3 maps to (u,v)=([27,86,1],[7,34]) and
   reversed-chain/source norm gives[8], invertible modulo3. BigUint third-model
   development checks agree. This is no measured DLP advantage; immutable
   replay is tracked separately. [pcc:unverified]

## Custody and next discriminating tasks

- Crypto PR3877 and issue3904 contain the current source checkpoint and queue.
  Cryptanalysis PR1469 initially pins crypto18db046; each later pin update
  must name its exact source digest and retain the baseline receipts.
- Read `REPORT_20261111.md`, `SOURCES_20261111.md`, `QUEUE.md` and the latest
  session log. Consult actual PCC standing; no signed verdict has been
  asserted by the producer.
- On proper vault unlock, use the repository's documented vault wrapper for
  atlas reads/writes. Run `lead_atlas.py --match` and research-KB retrieval
  before assigning existing lead IDs or claiming novelty. No lead ID was
  guessed while the atlas was unreadable.
- Next: immutable second-hop replay, then field parameterization with fixed
  inputs and overflow/nonresidue validation. A genuinely different Jacobian
  is insufficient for a cost win.
- Measurements remain blocked by the failed S3 admission gate and unknown
  live IsoLab budget/capacity. Future total-cost comparisons include direct
  source/destination solvers and both collapsed elliptic routes. No benchmark
  result or canonical leaderboard edge is ready for promotion.
- Current-main crypto PCC ledger divergence and historical missing benchmark
  receipts are named in
  `problems/chained-transfer-admission-and-ledger-sync.md`. Do not rewrite
  frozen claims, archived evidence or either ledger history to merge.

## Small-prime/global endpoint continuation
Feature small-prime uses unchanged checked Fp2 context(p≤4096), enumeration capped101. Fixed13/29/101 plus separate nonresidue control31 give #J180/960/9216/1008, source/target2-torsion4/4,4/16,4/16,16/4, source r5/5/3/7. Native25 tests and Clippy pass; BigUint development comparison covers all eight models and independent elliptic counts. No signed standing or performance claim. [pcc:unverified]

GLOBAL_SUCCESSOR_COLLAPSE.md and seven new core Lean ring lemmas give a written first-elliptic-endpoint argument n R f*=tau phi[2] in characteristic≠2,3,5, tau=(X/100+1/125,-V/1000). Actual base translation by(1/125,0) is essential. Total25 written lemmas,22+8+7 audited declarations. Geometric completion remains written, not Lean formalized. This is no claim about the final Jacobian-valued two-hop map. [pcc:unverified]

Cryptanalysis1469/1471 merged;1473 at73d6d5849 pinsaabae197, before this feature. New immutable feature pin/replay is next. Preserve every older pin/receipt. Next construction assessment: scope of the simple quadratic cover for odd full elliptic groups, then alternative odd-degree covers if warranted. Do not invent lead IDs or write plaintext into the locked atlas.

## Cubic arbitrary-class norm checkpoint

Cryptanalysis1469/1471/1473/1474 are merged;1474 merge852a464a7 pinscrypto75a737b8,
with exact-head focused replay42999610365 succeeding. Next sourcef91a5e061
implements the bounded arbitrary-class genus4 norm on the existing Enge
cover family. Whole cubic pullbacks preserve E109/F101 and E19/F17 into
genera4/13;130 fixed norms with irreducible support through degree4 pass an
independent root arithmetic replay. The second inverse/norm is restricted
to the exact image. All standing remains unsigned, no cost result.
[pcc:unverified]

## Non-split kernel/codomain continuation

Cryptanalysis0348e90f0654255917c0d6878168ebbda782ffd5, PR1477, classifies
the predeclared E19/F17 source:3 stable pairings among15,1 degenerate and2
smooth descended targets, deltas11/6, leading9 andJac285. Targets are
reflection-equivalent; full PGL2 audit finds no F17 curve isomorphism to
source. E109 unshifted quadratic cover instead has only its degenerate stable
kernel. Both are fixture-specific, without unpolarized-Jacobian inference or
operational non-split map. Claims42713c/3c51f5 and crypto five-lemma465142 are
unsigned. Native25/reference40 guard passes,1 old benchmark ignored.
[pcc:unverified]

NONSPLIT_KERNEL_CHECKPOINT.md and CHECKPOINT.md preserve exact source, report,
diagram/PDF and F17⁶ correspondence/dual acceptance plan. Next CT13b actually
evaluates determinant11 map with w=3Y and both dual norms; then CT15 tests
predeclared shift2 of the existing E109 cover. Preserve unshifted negative
and equivalent-target deduplication. No plaintext atlas write or guessed lead
ID, benchmark, solver gain or prime-field descent result. [pcc:unverified]

CUBIC_NORM_20261111.md, cubic-checkpoint.json and CHECKPOINT.md bind the source,
witnesses, proofs and resume instructions. Four new Lean lemmas bring the
written total to29, gates22+8+7+12 declarations; finite-field/Picard geometry
remains written. The odd-full-group restriction concerns individually rational
quadratic factors, not every Frobenius-stable unordered splitting.
Next CT12 supplies balanced genus2 arithmetic; CT13 then assesses actual
non-split descent. Do not assign a new atlas lead or performance edge before
the configured vault/KB and independent verification become available.
[pcc:unverified]

## Balanced genus2 continuation

Cryptanalysis28ba661017cdba4b132c2dcbc1615a3fd92a2a48 now implements the
bounded monic split-sextic representation at checked primes5..101. Both
10,404-class original/translated controls match existing arithmetic;
E109/F101 transfers without rational Weierstrass points, with arbitrary-class
norm image109/equal107 fibers on11,663 classes. Full native25/reference37
guard passes,1 old benchmark ignored. Producer PCC8b7ca0 and crypto six-lemma
PCC1cadd0 remain unsigned. BALANCED_CHECKPOINT.md links source, diagram and
inspected PDF. [pcc:unverified]

The production backend and1,940-row catalog remain unchanged; CT14 covers
large primes, extension fields, normalizations and replay. CT13 now assesses
actual Frobenius-stable non-split kernels/codomains/maps on E19/F17. No
non-split correspondence, cost gain or cross-prime descent is supplied by
this checkpoint. Atlas/KB/vault and benchmark gates remain unchanged.
[pcc:unverified]

## Shifted109 operational continuation

Cryptanalysis767c945a0a832c4f9d2827add8aefce3a16d2371,ready PR1479,now
implements the frozen shift2 E109/F101 source. All12,317 classes each way
pass both Richelot dual norms[2],descent,bijection/additivity; all109 source
coordinate/scalar/recovery[4]/inverse82 controls pass. Stable kernels:
1 degenerate/2 smooth reflection-equivalent,delta63/38,raw scale22.
SHIFTED109_CHECKPOINT.md binds digests and inspected two-page report.
Independent flat F101⁶ replay128 plus four shifted-cover Lean lemmas pass;
field layer alone independently cross-checked,not whole Jacobian map.
Current full46-test guarded CI pending. Prior PR1478 mergedf387be120,
exact-head focused43014776243 succeeds. All standings unsigned.
[pcc:unverified]

CT16 next computes target Frobenius action/geometric2-torsion and stable
next kernels; no nonbacktracking continuation or stronger nonisomorphism
result assumed. Retain unshifted negative/equivalent targets. Atlas remains
locked,no plaintext/guessed lead ID; no benchmark,cost or cross-prime result.
[pcc:unverified]

## Target Frobenius continuation and closed neighbors

Cryptanalysis63cf80560b8339b9ea67629746131f5c6f722299,ready PR1480,now
checks full geometric2-torsion/Frobenius for accepted17/101 targets. Both
have6-cycle,only previous-dual rational Richelot kernel,source/targetFp3-fixed
torsion16/4. Written standard branch-pair geometry plus Frobenius invariance
obstructs unpolarizedFp-Jacobian isomorphism; no rational next nonbacktracking
Richelot hop. All256 torsion additions/equivariance checks and seven Lean
certificates pass. Prior1479 merged3e76a274f,exact-head43022364261 passes
native25/reference46. CT16 full47-test CI pending. [pcc:unverified]

TARGET_FROBENIUS_CHECKPOINT.md binds exact source,report/inspected PDF and
limits. Retain forced-backtrack negative with positive genuine maps; other
degrees/products/extensions/covers remain open. CT17 next makes both elliptic
quotients/full odd rational-group reconstruction explicit for direct cost
controls. No split implementation or solver advantage accepted yet. Vault/KB
still blocked,no plaintext/new guessed lead ID. [pcc:unverified]

## Explicit split-quotient continuation

Producerda66ab9d953d2ba96c4ac9d6e8379c3478b3f277,ready PR1481,implements
both quotient norms/completed reciprocal maps. All285/12317 classes pass
inverses,bijection,paired additions,cross0,sum[2] and odd reconstruction;
all19/109 transferred source scalars have coordinates([4]P,0). Seven core
Lean reciprocal/split/recovery lemmas compile,total62;geometry remains
written and standing unsigned. SPLIT_QUOTIENT_CHECKPOINT.md binds exact
sources/report/inspectedPDF. Current full49-test focused43026964101 runs;
prior CT16 exact-head43024621372 passes47 tests/PR1480 merged57ab3ad05.
[pcc:unverified]

This exposes direct original-source controls,no extra DLP equation from the
second elliptic factor and no demonstrated solver gain. CT18 next assesses
rational odd-prime maximal-isotropic kernels at3/5,prospective7 control.
No kernel/codomain/map pre-accepted. Atlas still locked,no plaintext/new
invented lead ID. Root S3 gate rechecked after exact-format resume update;
transport still unavailable,so no benchmark/paid allocation. [pcc:unverified]
