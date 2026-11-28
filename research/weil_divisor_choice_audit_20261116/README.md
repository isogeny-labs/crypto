# Actual Weil divisor representative choice

[Report](reports/report.pdf), [protocol](PROTOCOL.md), [proof map](proof-map.json),
[source plan](sources.json), [source review](evidence/source-review.json),
[verification](verification.json).

Main: `WeilDivisorChoiceAudit.actual_weil_divisor_choice_certificate`.

The [actual Weil divisor-choice proof](README.md)
now proves that Galbraith26.1's quotient g(D)/f(E), on actual degree-zero
point divisors with disjoint supports and actual functions of divisors nD,nE,
is independent of both functions and both divisor representatives. It
covers every algebraically closed field/general Weierstrass model with
nonzero discriminant, every characteristic, and every integer n (hence
all natural levels). Only disjointness inside each pair is required;
old and new pairs need not be cross-disjoint. Actual root-of-unity
membership and inverse under swapping follow. At n=0 the quotient itself
is proved1, beyond vacuous zero-power membership. [pcc:unverified]

Actual local-unit evaluation gives addition/integer scaling/function powers.
Actual geometric Picard principality and point avoidance move any degree-zero
divisor outside any finite set, with an actual function witness. Changing
D to D+div(h) corrects f to f*h^n. Actual geometric reciprocity gives
 g(div(h))=h(div(g))=h(E)^n, so both quotient factors cancel. A common
intermediate divisor avoiding both right supports gives two-coordinate
independence. Actual degree-zero/torsion point-sum conditions derive
n-principal functions and proper representatives for every actual torsion
pair. No pairing-law, reciprocity or abstract population premise is assumed.
[pcc:unverified]

On the actual nonsingular F2 curve y²+y=x³, the rational points are exactly
O, P=(0,0), and -P, with both nonzero points proved distinct. No rational
S makes (P+S)-(S) disjoint from {O,P}. This disproves deletion of the
geometric-field condition from that shifted moving construction and agrees
with the source footnote; it does not refute all possible rational divisor
representations or the qualified source theorem. [pcc:unverified]

Local **PCC-20261116-c356c4 PASS at 2026-11-16T06:26:06Z**, signed status unverified.
Wholegate34/9 modules(8 concrete, one official bridge)/zero failures or
trusted native evaluations;337 formal/18 artifact pins match. All342
predecessor pins match before/after15 states and both gates;12 failed
source states include default200000-heartbeat failures05/06/07. Initial
FAIL_no_sources gate from sparse staging is retained; explicit sparse
staging and actual gate02 fix it. Final new-source warnings0;32 packages,
no copies/hydration/update. Full1289-character header SHA-256
`2c2c650e7341720a2c8e4e3a290e87ee62224df0b4af1ea50f1b412d8249d270`;
complete binding after completed gate, zero prefix/unresolved/empty suffix;
full normalized and parsed headers read separately. Lossless324 files/
111 objects/65704-byte XZ/two actual parts64000+1704/round-trip and actual
restore equal. First-valid one-page47158-byte report/all4471 chars read/
fullpage visual/inbounds/no drafts. Sourceplan65362 bytes/full inline
statement;265 custom modules pinned,253 imported (244 credited+9 new).
Same host/prover and procedural binding are not independent compiler,
parser or physical-host assurance. No holes/custom axioms/native proof
execution/raised limits/new flags/waivers/frozen edits/mathPython/spend.
[pcc:unverified]

Sources: fresh Galbraith chapter26 PDF1–5/printed573–577 text and all five
individual visuals, Exercise26.1.1/formula26.1/reciprocity sketch/complete
displayed26.3.2 proof/footnotes2–3. A truncated five-image batch was not
counted; every page was then viewed separately. External nondegeneracy and
multiplication-isogeny comparison references were not read in full. Six
KB calls and initial/expanded Conductor unavailable127; bounded actual/
official/primary fallback. No novelty or qualified-source refutation.
[pcc:unverified]

Arbitrary geometric divisor/function choice is now proved at these guards.
Multiplication-isogeny comparison, full canonical nondegeneracy, descent,
nonclosed residue fields, canonical Tate, general MOV/Frey–Rück and backend
runtime remain open. All fourteen requested targets remain active.
[pcc:unverified]

Publication: ready [PR #4389](https://github.com/isogeny-labs/crypto/pull/4389). Remote proof `256270fb73306a7e292cb0f9f3c3eec35bb36e20` and local receipt `b86740581c2c104c5318a92b78ba46f98671939c` share tree `6f08f3db619d6b28f9cac457caf4a4e3fe46c2ab`. Anonymous fetch completed before exact fresh head/title/body/ready confirmation.44 pinsets/5404 entries match. Scheduled06:30 check:2success/5running/1signskipped, no reviews or threads. Signed status remains unverified. [pcc:unverified]
