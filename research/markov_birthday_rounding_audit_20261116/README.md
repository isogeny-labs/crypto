# Literal finite Markov birthday bound and valid integer-time controls

[Report](reports/report.pdf), [protocol](PROTOCOL.md), [proof map](proof-map.json),
[source plan](sources.json), [source review](evidence/source-review.json),
[verification](verification.json).

Main: `MarkovBirthdayRoundingAudit.actual_markov_birthday_rounding_certificate`.

The [actual Markov birthday finite-bound audit](README.md)
now refutes the literal finite all-c formula of KMPT2010 Theorem3.2.
The actual uniform-reset chain on47 states has a normalized transition
PMF, doubly stochastic matrix, uniform stationary law, positive one-step
reachability and period1. Its actual time1 Green sum includes the identity,
and both actual maximum row/column square sums equal50/47. With T=1,
m=M=1 and c=1/44, the displayed time equals exactly1. The actual
least-repeat collision probability is1/47, strictly below1-exp(-1/44).
Taking the ceiling of the displayed time also gives1. Lean directly
negates the all-c statement on these actual objects. [pcc:unverified]

The first-collision law is connected to the actual chronological Markov
word/path PMF by credited deferred decisions only until the first repeat.
In one step a repeat means the actual uniform next state equals the start,
so its probability is1/p. The source p8 proof uses integer k-block
amplification and then substitutes k=2cM²/m², which is1/22 for this
witness. The finite formula needs an integer-time qualification. This
counterexample does not refute the source asymptotic square-root order
or the qualified larger-partition adding-walk constant. A full general
Markov second-moment/block-ceiling repair is still open. [pcc:unverified]

For the actual uniform-reset chain, Lean proves a healthy exact law.
Selfavoiding n-label words are genuinely bijective to embeddings into
the complement of the initial state, so their number is
(p-1).descFactorial(n). Actual word mass is(1/p)^n, giving that exact
no-repeat probability, including zero once n>=p. Its recurrence proves
survival<=exp(-n(n+1)/(2p)). Every INTEGER n with n(n+1)>=2pc therefore
gives collision probability>=1-exp(-c); ceil(sqrt(2pc)) is a valid
choice for all c>=0. The same47-state/c1/44 instance passes a two-step
control, with zero-time/confidence and singleton controls. Nonzero p is
required; no empty uniform law is assumed. [pcc:unverified]

Local **PCC-20261116-0afecf PASS at 2026-11-16T09:11:41Z**, signed status unverified. Wholegate60
declarations including generated equation helpers, zero failures/trusted
native proof evaluations. Nine modules(eight concrete, one official
bridge), final new warnings0. Nine states retain failures01–08 and pass09,
including the actual default-recursion-limit failure. All6110 entries
from47 frozen plans match around every state/gate/replay:6074 published
preservation entries and36 static-hash dependency pins. Producer check
matches all6157 source/artifact/manifest entries across1843 unique
committed paths. Thirteen packages/no copies/hydration/update.
Full149-character header SHA
`b09028b7712726177e63155f7f154897daced7cef1b86748882864e4ebd7027a`
names four transparent proposition bundles in pinned source; full
supporting headers include section-variable context. Separate complete
binding after completed gate:1complete/0prefix/0unresolved/empty suffix;
normalized and parsed headers read separately.34 formal/19 artifact
pins/sourceplan15832B/full inline statement;19 custom modules all
imported(10 credited+9 new). Lossless315 files/144 objects/171640-byte
XZ/three actual parts/round-trip and actual restore equal. First-valid
one-page47307-byte report/all4946 text chars read/fullpage visual/inbounds.
Same-host prover/header checking is not independent compiler/parser/host
assurance. Standard foundations/default limits only; no holes/custom
axioms/native proof evaluation/new flags/waivers/frozen edits/mathPython/
spend. [pcc:unverified]

Source: Kim–Montenegro–Peres–Tetali, Annals of Applied Probability20(2),
495–521(2010), DOI10.1214/09-AAP625. Actual arXiv0712.0220v2 electronic
reprint,300644B/SHA
`67bd4313585101da1138a8e08bf3a3045b6347fe289d4bd076594b9d57f91b07`;
its pagination/typography differ. Complete bounded text1–8/start9 and
complete individual visuals6–8 read, not full29 pages. Publisher access
failed; Crossref/OpenAlex located the actual reprint. Twelve KB calls
and Conductor unavailable127; canonical prior/official/primary fallback,
known methods credited and no novelty/absence claim. Original Teske
remains unread. All fourteen targets remain active, including the
qualified adding-walk limiting constants/practicalhash/scalar-recovery
runtime and the other recorded geometry/solver/counting obligations.
[pcc:unverified]

Publication: ready [PR #4389](https://github.com/isogeny-labs/crypto/pull/4389). Remote proof `8703dc5ebdc43d7a49b12ca0b7890f4328e2601c` and local receipt `88e347f23c54fd947598118c43401ce6e46564f6` share tree `22d96c9bb002b005c89413320e3ae2072f7d7f94`. Anonymous fetch completed before exact fresh head/title/body/ready confirmation. All47 pinsets/6127 entries match. New-head CI, review and signing remain pending; next scheduled owner09:30UTC. [pcc:unverified]
