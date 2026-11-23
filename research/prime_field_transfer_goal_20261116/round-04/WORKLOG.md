# Round04 reproduction and preservation record

This is local native development correctness qualification under the
prewritten protocol and three amendments. No timing, performance, EKS,
leaderboard or independent execution experiment is reported.

## Native calls and outputs

- Compiler01/02 fail from a pre-existing helper type-name collision;
  compiler03/04 pass. Version02 passes. Version03 passes. Version04 build01
  fails fatal indentation, build02 passes. Root issuer build01 fails fatal
  indentation, build02 passes; both subsequent issuer versions build.
  Every compiler uses fatal warnings. Failed source snapshots and diagnostics
  are preserved. Full first two74783-byte compiler logs are local only, with
  published hashes/excerpts under large-build-log-pins-01.json.
- Qualifier01: setupu3 exits0; setupu5/u7 exit124 at60seconds. u3 all18
  point qualification exits0; complete coverage at tested16/32 digits.
- Qualifier02: u3/u5 setup exits0; u7 setup exits124. u3 model/output
  compare byte-identically to01. Four u5 public-input calls exit0 with
  only index1 accepted at4/8/16/32 digits.
- Qualifier03: direct division-polynomial u3/u5 models and every qualification
  output compare byte-identically to02. u7 setup exits124 at60seconds.
- Qualifier04: rational Padé representation, same models/input/precision/call
  caps. All five qualification calls exit0. u3 coverage6/18 at1/2/4 and18/18
  at8/16/32. u5 coverage1/4 at4/8/16 and3/4 at32; index3 remains uncovered.
  Claim PCC-20261116-fb2ab0 is unsigned. [pcc:unverified]

Nine setups include five model-producing regressions and four timeouts.
All16 qualification processes complete; a zero-acceptance process is not
a successful map. No u7 public-input qualification runs without its model.
The actual commands and timeout bounds are in the immutable pre-call
records, with separate post-call output hashes and exit files.
[pcc:unverified]

The later recorder version checks copied qualifier sources and executable
before and after calls. Earlier01/02 executable pins are in the pins array
and checked before calls;03 uses a separate executable field not checked by
the old recorder. None of these producer manifests is an execution proof.

## Certificates and Lean

Root issuer01 computes two exact quotients, then exits1 on an artificially
inflated expression-degree bound. Issuer02 and03 each produce four native
IDC1 records and reject four mutations. The final03 records point to the
actual V3 source. Earlier generated Lean source01 lacks noncomputable/scoped
settings;02 misses an unnamed section end. Exact failed bytes are archived
as text with relocation receipts. [pcc:unverified]

The corrected root/residue modules pass focused compilation. Mathlib's
`Polynomial.funext` resolves rational coefficient equalities without new
ring laws; finite coefficient projection resolves the residue identities.
One preparation command used the wrong cwd and fails before mutation;
its diagnostic receipt is retained.

Final gate02, after adding the explicit axiom audit client, audits55
theorems with zero failures and zero trusted native exceptions. Nine named
literal endpoints have normalized statement bindings in lean-claims-01.json.
All explicit axiom outputs are standard foundations. Gate claim
PCC-20261116-be69df is unsigned. Earlier passing gate01 is preserved.
No native_decide, custom axiom or limit increase. lean4checker and Cargo
are unavailable. [pcc:unverified]

## Report and next obligation

The two-page round04 PDF is inspected page by page from rendered images;
editable SVGs, source Markdown/HTML/CSS and recorded count-plot inputs
accompany it. No PNG preview is committed. All37 frozen round02/03 inputs
are checked unchanged; original GOAL.md and automatic PNG migration remain
untouched. No canonical-ledger rewrite or direct artifact upload.

The implemented component selects actual compatible global classes from
coordinates on these fixtures. A compact evaluator, uniform height/precision
bound, full costs, independent checking and arbitrary-curve applicability
remain the broad goal's next obligations. Dense degree is not a lower bound
against a compressed algorithm. See RESULT.md and the final proof-map
supplement for precise written/native/formal boundaries.
