# Autoresearcher integration handoff

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
