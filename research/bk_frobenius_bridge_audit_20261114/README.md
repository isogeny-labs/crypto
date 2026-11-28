# Actual Frobenius representation and finite-field descent at prime three

Lean constructs a basis of the actual geometric three-torsion group for
every nonsingular general Weierstrass model over every algebraically closed
field of characteristic other than three, including two. In that basis it
represents actual coordinate q-Frobenius by an actual two-by-two F3 matrix.
Its powers are the identity exactly when all actual three-torsion points lie
in the image of a chosen compatible field of size q^k. Whole gate:
**31 declarations, zero failures, zero trusted native exceptions**.
Exact local **PCC-20261114-e203ff** replay PASS at2026-11-14T03:36:21Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

The actual subgroup/module/basis comes from the preceding
[general rank packet](../three_torsion_rank_audit_20261114/README.md).
Existing [finite-extension descent](../theorem_census_20261109/lean/FrobeniusFiniteExtension.lean)
and [generic linear step](../bk_linear_audit_20261113/README.md) are reused.
This packet connects those components: the generic determinant formula
`det(Frobenius)=q mod3` is still an explicit hypothesis. It is not inferred
from having a nonzero determinant, and the full qualified BK theorem remains
open. The constructed matrix's determinant is nonzero and basis independent.
Three dividing the actual rational point count gives a nonzero fixed vector
through Cauchy and injective actual point inclusion. [pcc:unverified]

For the actual curve **E/F2: y²+y=x³+1**, Lean proves three rational points,
nine geometric three-torsion points, actual Frobenius squared equal to the
identity and actual Frobenius different from the identity. The negative
control uses actual descent: an identity Frobenius would inject nine points
into three. The literal division quartic and curve equation give x^4=x and
y^4=y. Kernel-checked finite matrix classification then proves the actual
determinant is2. This supplies the all-extension criterion for every chosen
field of size2^k and compatible embedding **without a determinant premise**.
No new construction of finite fields/embeddings for every k is claimed.
[pcc:unverified]

The actual F7 excluded-boundary curve y²=x³+3x+2 has a constructed geometric
basis and a Frobenius matrix different from identity despite7≡1mod3.
This reuses the earlier nonfixed actual point; its stronger all-k boundary
was already proved in the [F7 packet](../bk_unipotent_audit_20261113/README.md).
No computed Jordan coordinates or general determinant theorem is claimed.
Earlier actual F5/F25 matrices and F5/F7 all-k results, the short-only
three-torsion count and general chosen-field descent are credited in
[source review](evidence/source-review.json). [pcc:unverified]

The complete selected declaration is
`BKFrobeniusBridgeAudit.actual_frobenius_bridge_certificate`, in
[FrobeniusCertificate.lean](lean/FrobeniusCertificate.lean).
[Proof map](proof-map.json) and [source plan](sources.json) bind its full
1,867-character type, all24 recursive custom modules and15 configuration
files across five projects, plus16 artifacts. [pcc:unverified]

Reproduce with pinned Lean4.35.0 and
Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679:

```sh
tools/lean/proof-gate.sh research/bk_frobenius_bridge_audit_20261114/lean
python3 research/bk_frobenius_bridge_audit_20261114/restore_attempts.py /tmp/bk-frobenius-builds
```

All135 build-provenance files are retained losslessly using41 unique
content-addressed objects in the34,320B archive; restoration checks every
hash and refuses conflicting overwrites. Failed drafts and the interrupted
build with no final exit are preserved. Five whole-gate records and the
inspected one-page [PDF report](reports/report.pdf)/[SVG](reports/frobenius-bridge.svg)
are included. Scalar supplement **IDC1h6d1cab18310b048b** has a universal
CommRing Lean identity and the unchanged official checker's32+32-point
check; it does not certify geometry. Same producer/host, no independent-host
or signed verification claim. [pcc:unverified]

No holes, custom axioms, native evaluation, raised limits, new global
elaboration flags or waivers are introduced. Standard foundations are
propext/Classical.choice/Quot.sound. No native experiment, paid compute or
agent storage upload. Original BK1998 Theorem1/proof/Remarks pp142–143
were re-read; its qualifications are retained. General determinantq and
canonical Weil-pairing necessity, other odd primes≥5, the full all-k p<500
census and the broader original fourteen-item obligations remain open.
The qualified source is not refuted; no full-paper/book read, corpus
promotion or novelty claim. KB/Conductor were unavailable; bounded
primary/repository/pinned-library fallback is recorded. [pcc:unverified]
