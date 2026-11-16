# Actual Frobenius check of the acknowledged factorization guard

[Report](reports/report.pdf), [protocol](PROTOCOL.md), [proof map](proof-map.json),
[source plan](sources.json), [verification](verification.json),
[source review](evidence/source-review.json).

Main: `IsogenyFactorizationGuardAudit.actual_isogeny_factorization_guard_certificate`.

The [actual Frobenius factorization-guard audit](README.md)
formalizes an acknowledged Galbraith9.6.18 correction. The downloaded
version2 chapter says kernel(phi)subsetkernel(psi) andPSIseparable imply
psi=lambda composedphi. Author errata explicitly requiresPHIseparable,
crediting Pierre Chretien. This is a known correction, not a novel source
error or a refutation of the corrected general theorem. [pcc:unverified]

For every actual nonsingular Weierstrass model over every finite fieldK,
q=#K>=2, Lean constructs the actual coordinate-ring and full function-field
q-Frobenius maps and proves localization/coefficient/genericpoint x/y
naturality. The actual infinity parameter has order1, so f^q=pi would
force q*ord(f)=1. Thus actual field Frobenius is injective but notsurjective,
and no arbitrary ring-map lambda* can satisfy phi*composedlambda*=identity.
The extension over its ACTUAL image field is purely inseparable and
notseparable. This verifies that the author-correctedPHI guard excludes
the example. The image field is distinct from the quadratic projection
baseK(x). [pcc:unverified]

The actual q-Frobenius on actual elliptic points over AlgebraicClosureK
is bijective with trivial kernel. Identitypsi has trivial kernel and
actual separable identity field extension, satisfying the wrong reading's
premises. Lean negates that explicit actual-function-field/rational-
coordinate factorization statement. Geometric point-set kernels, as in
source9.6.1, are distinguished from group-scheme kernels; scheme-kernel
inclusion would fail here. A full Mathlib Scheme/isogeny-category comparison
is not built. Healthy identity-left/Frobenius-square factorizations and
actual nonsingular F2/F3 examples pass. Corrected general factorization
and general Weil nondegeneracy remain open. [pcc:unverified]

Local **PCC-20261116-bc1b4e PASS at 2026-11-16T17:08:35Z**, signedunverified. Wholegate23 declarations,
0failures/0trustednative;6roots(5concrete+1officialbridge),newwarnings0.
Ten states retain failures01–08/passes09/10 and a separate source-interface
query1. All6385 predecessor/dependency entries52initialplans match around
builds/gate/binding/replay. Producer6437records/2005unique committed paths
match.25packages/no copied importmodules/hydration/update. Full172-character
main/SHAaa3cc46970752b0e498ea28e956816cea287f5920a3904fa2dced71ed034d996,
four transparent pinned bundles/full supporting headers/context;
1complete/0prefix/0unresolved/empty suffix after completedgate.
255formal20artifact275pins/204pinned186actually imported custommodules
(180credited+6new),18extra frozen modules retained as pins. Lossless
312files135objects188300XZ/3parts/roundtrip/actualrestore. Firstvalid
onepage47097B/all5055textchars/fullindividual1500pxvisual/inbounds.
Standard foundations/default limits, no holes/custom axioms/native proof
evaluation/newflags/waivers/frozenedits. Samehost checking does not establish
independent compiler/parser/host or signed assurance. [pcc:unverified]

Primary ch9.pdf387389B/SHA
f9a3272af43e74723accaed2e5e1d19405779141d1ed7f516d822203a40cc495;
complete bounded text printed165–167,172–176/individual visuals175/176.
Actual author errata HTML SHAbdba99cb397a95b1bd6b37d2f672d23b487038dd7a3737de76c6ccee8edf825c;
its printed150 reference differs from downloaded PDF176, no pagination
equivalence claimed. Full Silverman/Washington external proofs unread.
9KB/two source Conductor/publication Conductor127; bounded primary/
canonical/official fallback, no absence/novelty claim. No native experiment
or paid spend. All14 remain active, including the recorded geometric,
counting, walk and implementation obligations. [pcc:unverified]

## Publication

Published in ready [PR4389](https://github.com/isogeny-labs/crypto/pull/4389).
Local proof receipt `ebcca480340a10d20ac0a8d407afb18db982add8` and remote proof
`447ab839cbd093de3a7e28d2aa3ba236010481c4` share tree
`366b4ea6adb0e4b2ff63d8e8c603e9a61d183c82`. Completed anonymous fetch and fresh exact
head/title/body/open/ready confirmation are recorded in
[publication.json](publication.json). Current-head CI/review/signing remain
pending. Local replay PASS does not establish signed standing. The author
correction is acknowledged; general corrected factorization/Weil nondegeneracy
and Scheme API comparison remain open. All14 targets stay active. [pcc:unverified]
