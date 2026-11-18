# Valid finite and asymptotic Green upper-envelope bound

[Report](reports/report.pdf), [protocol](PROTOCOL.md), [proof map](proof-map.json),
[source plan](sources.json), [source review](evidence/source-review.json),
[verification](verification.json).

Main: `MarkovGreenUpperAudit.actual_green_upper_envelope_certificate`.

The [actual Green-envelope upper-bound audit](README.md)
formalizes the valid upper-bound conclusion supported by the KMPT2010
p8 argument. For actual doubly stochastic transition kernels, the
pointwise envelope P^j(x,y)<=c+d^j with0<=d<1 implies BOTH actual
inclusive Green-square maxima are at most
c*(T+1)*(T+2)+2/(1-d)². A finite weighted-geometric identity is proved
by induction, with exact constant contribution and zero-term/one-term/
zero-ratio controls. No desired Green bound is assumed. [pcc:unverified]

For varying actual finite nonempty state spaces and kernels, a fixed d
and actual IsLittleO error e=o(1/t²), the pointwise envelope is required
only at powers0<=j<=t. Lean constructs the nonnegative explicit error
R_t=|e_t|*(t+1)*(t+2), proves R_t tends to0, and bounds both actual
Green maxima by2/(1-d)²+R_t. For every epsilon>0 they are eventually
at most2/(1-d)²+epsilon on the same eventual set. This is a one-sided
conclusion. The credited actual uniform prime-state family with d=1/2
satisfies the bound8+R_t and has Green limits1, retaining the literal
printed-equality counter. Qualified source Lemma3.3 and all earlier
counterexamples/controls stay frozen. General Markov collision second-
moment/integer-block repair remains open. [pcc:unverified]

Local **PCC-20261116-02e783 PASS at 2026-11-16T10:18:06Z**, signed unverified. Wholegate17 declarations
including generated helpers,0failures/0trustednative. Four concrete new
modules, final new warnings0. Eight states retain failures01–07/pass08,
including the |* lexical error and rejected partial-definition target.
All6226 entries/49initial plans match around every state/gate/replay:
6190 published preservation entries plus36StaticHash dependencies.
Producer6275records/1903unique committed paths allmatch.
Fifteen packages/no copies/hydration/update. Full154-character header
SHA`28f396a4cd16557abd05803143f203b1bd0c6aee358a5177ce995f27bda734b0`,
four transparent pinned bundles/full supporting headers; separate complete
binding after completedgate1complete/0prefix/0unresolved/emptysuffix,
normalized/parsedheaders read separately.51formal19artifact70pins;
30pinned/23actually imported custommodules(19credited+4new), seven extra
frozen modules retained as pins. Lossless229files116objects168468XZ/
3actualparts/roundtrip/actualrestorematch. Firstvalidonepage46956B/all4376
textchars/fullindividual1x and1.4xvisuals/titlebboxchecked/inbounds.
Samehost proof/header check is not independent compiler/parser/host
assurance. Standardfoundations/defaultlimits, no holes/customaxioms/
native proof evaluation/newflags/waivers/frozenedits/mathPython/spend.
[pcc:unverified]

Source: KMPT2010/DOI10.1214/09-AAP625, actual arXiv0712.0220v2 electronic
reprint27Sep2010/300644B/SHA
`67bd4313585101da1138a8e08bf3a3045b6347fe289d4bd076594b9d57f91b07`.
Previous bounded text1–8/start9 and individual visuals6–8 credited;
current complete bounded page8 calculation/paragraph text re-read.
No full29-page reading or journal/reprint-identical claim. Six actual
KB calls and source-review Conductor127; canonical/official/primary
fallback, no novelty/absence claim. All14 remain active, including
general collision block repair, qualified adding-walk limiting constants,
practical hashes/scalar recovery and recorded geometry/counting/solver
obligations. [pcc:unverified]

Publication: ready [PR #4389](https://github.com/isogeny-labs/crypto/pull/4389). Remote proof `4c25dbcb345dee13b18d314d16eb76f334be74da` and local receipt `3bca64e9c28728f6d3b4a170239ba55f290d836b` share tree `846250eb6dc9e01d3f8c7e3cc7b29c719064a257`. Anonymous fetch completed before exact fresh head/title/body/ready confirmation. All49 pinsets/6260 entries match. New-head CI/review/signing pending; next scheduled owner10:30UTC. [pcc:unverified]
