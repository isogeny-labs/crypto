# New component: four whole rational Jacobian maps

Full37/3c1cb7 and six bound local Lean declarations/183written36modules
complete the actual Mumford rational evaluators, adjoints and recovery on
all12317 source classes for C10/C20/Cpi1/Cpi2. Writtenproof afd164 connects
them to global quotient maps. Mainreport pcc/claims/PCC-20261112-e4f934.json with editable figures and
inspected2pagePDF: [whole maps](reports/even-whole-maps/REPORT.md).
Geometric7/49 kernel certificates are next; no measured cost claim.
Consumer1513/748e ownCI43268001587 succeeds at fixed338d35-testscope;
new37-testownCI follows this commit. Scientificstandingunsigned. [pcc:unverified]

---
# New component: actual partial-Frobenius gluing transfers

Full35/f04008 andsixboundlocalLean177written35modulesacceptactualA24/A12,
Cpi1/Cpi2equations/covers/graphs andallrationalfactorchecks. Writtend67ccf
givespurelyinseparabledegree101^k/connectedkernelscheme/F101DLPpreservation
andprovesnormalizedrationalagreementwithexistingdirectproductbridge.
This component establishes no additionalfactorstageevaluationgainoverthat
bridge. Mainreportpcc/claims/PCC-20261112-644ced.json: [partial Frobenius](reports/partial-frobenius/REPORT.md),
editablefigures/recordedcounts/inspected2pagePDF;threefailedattemptsretained.
WholeMumfordmaps are nextgenericchart/normtask forbothCpiandC10/C20.
Consumer1509f656 exactCIbothjobsSUCCESS/f58004(root6236a7). Allscientific
standingunsigned;newrootownCIstillrequired. [pcc:unverified]

---
# New component: two actual unbalanced-transfer models

C10/C20 are constructed overF101 withoutretwist;fullnative34/925b02,
algebra23/sevencerts/f404e3 andtwoboundLean1bde23/d03d51 acceptthemodel/
cover/graphphase. Writtenquotientproof128eb1 givesglobaldegree7/49 and
nonuniformnorms. WholeJacobianmaps/duals/kernels/full12317corpus/T20remain
phaseB/C. Reportpcc/claims/PCC-20261112-7ae0d2.json,editablefiguresandinspectedtwo-pagePDF:
[reports/unbalanced-models/REPORT.md](reports/unbalanced-models/REPORT.md).
Allscientificstandingunsigned. Root51dCIgreen2c4744;consumerf656CIpending.
[pcc:unverified]

---
# New component: actual gluing cycle and fixed quotient catalog

Fullnative33/70a16a,algebra23/8d3d02,nineboundLean169written33modules;
actualgraphreturn36 versusellipticpair12. Writtenfixedcatalog120AV/84principal/
72Jac/36nonPP,f48205,V2standingannotationb7c7e7. Fullsource/evidence/failed
attempts/figures/two-pagePDF in reports/isogeny-cycles and versionedproof.
These are unsigned correctness/written results,not new72curveequations or
performance evidence. Consumerready1509fixed8f456314;ownCI pending.
[pcc:unverified]

---
# Chained Jacobian transfers

Current continuation: [report](REPORT_20261111.md), [PDF](report.pdf),
[small-prime supplement](SUPPLEMENT_20261111.md), [supplement PDF](small-prime-report.pdf),
[persistent queue](QUEUE.md), [sources](SOURCES_20261111.md),
[GitHub #3904](https://github.com/isogeny-labs/crypto/issues/3904).

`native/` is the execution path for new development. It is a standalone,
dependency-free Rust crate for public F101 fixtures and model-tagged quadratic
splittings. Its default API uses a fixed field. The opt-in `small-prime`
feature adds checked odd primes≤4096, with enumeration capped at101; see
the supplement and external-source input manifest. The historical
Python files, protocol and 2026-11-10 receipt are retained evidence; their
old status paragraphs do not describe the continuation. Do not extend or use
those Python paths for new comparisons.

```sh
cargo test --release --locked --offline --manifest-path research/chained_jacobian_transfers_20261110/native/Cargo.toml -- --nocapture
cargo clippy --locked --offline --manifest-path research/chained_jacobian_transfers_20261110/native/Cargo.toml --all-targets -- -D warnings
tools/lean/proof-gate.sh research/chained_jacobian_transfers_20261110/lean/ChainTransfer.lean
tools/lean/proof-gate.sh research/chained_jacobian_transfers_20261110/lean/PolynomialIdentities.lean
```

The cross-repository reference is
`cryptanalysis:experiments/chained-transfer-reference/`. The comparison uses
pre-existing BigUint arithmetic after a faithful monic chart change; both
implementations use Cantor and the runs are by this producer. This is
differential development evidence, not signed independent verification.
All current claim standings must be read through `tools/pcc/pcc.py standing`.
[pcc:unverified]

The fixed fixture now has full rational source maps, a genuine Richelot hop,
its sign-corrected dual, nonzero and zero endpoint controls, and a complete
finite-group deduplication against an elliptic shortcut. Its target model is
isomorphic to the source. A second preselected fixture has nonisomorphic
Jacobians over F101 (rational2-torsion4 vs16, each order9216), but its complete
96-point elliptic endpoint also matches a direct2-isogeny after doubling.
Its preserved prime subgroup has order3. A predeclared second Richelot hop
now reaches a third Jacobian with rational2-torsion8; the reverse-kernel
intersection is trivial and all full-domain dual checks pass. The three
counts4,16,8 distinguish the three Jacobians over F101. The report gives the precise scope
and open geometric proof bridge. No performance run or speed claim is present.
[pcc:unverified]

Pinned oracle baseline: [cryptanalysis PR #1469](https://github.com/isogeny-labs/cryptanalysis/pull/1469),
merged; [successor PR #1471](https://github.com/isogeny-labs/cryptanalysis/pull/1471)
pins b280efc with27 passing tests/1 ignored benchmark. The third model's
independent BigUint development checks pass; its immutable replay is tracked
separately from those retained checkpoints. [pcc:unverified]

Benchmark/integration blockers have a
[named record](../../problems/chained-transfer-admission-and-ledger-sync.md).
Root-library checks and the new focused CI are tracked in the session log and
PR. A source checkpoint does not waive their gates.

The next bounded prerequisite is implemented in the sibling reference:
[cubic arbitrary-class norm](CUBIC_NORM_20261111.md),
[diagram and PDF](cubic-report.pdf). Whole-fiber cubic pullbacks preserve
full prime-order109/19 sources through genera4/13; an actual genus4 norm
handles closed support through degree4. All130 fixed norm witnesses pass a
separate root arithmetic replay. Genus13 recovery is restricted to the exact
pullback image. No computational advantage or prime-field descent is claimed.
The active queue now moves to balanced even-sextic arithmetic, then possible
Frobenius-stable non-split Richelot kernels. [pcc:unverified]

The [balanced prototype checkpoint](BALANCED_CHECKPOINT.md) now supplies the
bounded prime5..101 monic split-sextic representation and quadratic induced
maps. Original and translated10,404-class controls match the existing engine;
E109 transfers without rational Weierstrass points. Latest consumer suite
passes native25/reference37 tests(1 old benchmark ignored). The production
backend/catalog gap remains open; CT13 next assesses actual non-split kernel
descent. No cost or cross-prime result. [pcc:unverified]

[Non-split kernel/codomain assessment](NONSPLIT_KERNEL_CHECKPOINT.md) now
classifies all15 pairings on E19/F17:1 degenerate and2 equivalent smooth
stable targets. The E109 unshifted cover has only a degenerate stable kernel.
These are unsigned fixture-specific results; actual non-split correspondence
is CT13b, then a predeclared shifted E109 cover is CT15. No cost result or
unpolarized-Jacobian nonisomorphism is inferred. [pcc:unverified]

The [operational map checkpoint](NONSPLIT_MAP_CHECKPOINT.md) now supplies the
actual F17 non-split correspondence and dual. Every285 source/target class
passes descent, both dual norms[2], bijection and paired additivity; all19
elliptic scalar/recovery relations pass. Latest native25/reference43 guard
passes(1 old benchmark ignored). A separate pre-existing flat Fp6 engine
replays128 field witnesses; that checks the extension layer, not independently
the whole Jacobian map. The subsequent CT15 checkpoint is linked below.
Standing unsigned, no demonstrated cost advantage or cross-prime descent.
[pcc:unverified]

The [shifted109 checkpoint](SHIFTED109_CHECKPOINT.md) now evaluates the
predeclared shift2 source: determinant63/raw scale22, all12,317 classes in
each direction satisfy both dual norms[2], descent, bijection/additivity;
all109 scalar/recovery[4]/inverse82 controls pass. Independent flat F101⁶
field replay128 and four shifted-cover Lean lemmas pass. Current full46-test
consumer CI is pending; CT16 next assesses target Frobenius2-torsion and
nonbacktracking kernels. Unshifted negative/equivalent targets retained;
standing unsigned, no cost or cross-prime descent result. [pcc:unverified]

The [target Frobenius checkpoint](TARGET_FROBENIUS_CHECKPOINT.md) proves the
stronger unpolarized Fp-Jacobian distinction via2-torsion over Fp³:16/4.
Both accepted17/101 targets have6-cycle branch action and only the previous
dual as a stable Richelot kernel,so the next rational step must backtrack.
All256 torsion additions/Frobenius branch-image controls and seven Lean
finite/conditional lemmas pass. Prior shifted source full46-test CI passes;
CT16 source63cf80560/PR1480 full47-test CI pending. Next CT17 implements
both elliptic quotient norms/full rational-group reconstruction. Standing
unsigned,no universal transfer impossibility or cost advantage. [pcc:unverified]

The [split-quotient checkpoint](SPLIT_QUOTIENT_CHECKPOINT.md) now implements
both elliptic norms and full rational-group reconstruction on all285/12317
classes,including reciprocal zero/infinity completion. Target source-scalar
coordinates are([4]P,0),with all19/109 controls passing. Seven cleared/
conditional Lean lemmas compile,total62. Prior CT16 exact-head47-test CI
passes/PR1480 merged;currentda66ab9d9/PR1481 full49-test CI running. These
are rational-group bijections,not variety-product isomorphisms. CT18 next
assesses rational odd-prime isotropic kernels; no solver/cost benefit or
cross-prime descent accepted. [pcc:unverified]

The [odd-kernel checkpoint](ODD_KERNEL_CHECKPOINT.md) closes rational3/5
maximal-isotropic kernels for both accepted examples,also7 for17;shifted101
has four abstract7 candidates. Complete finite-module controls and six new
Lean algebra/root certificates pass,total68. A separately authored polynomial
Rabin/Frobenius oracle from closedPR3918 compiles unchanged and passes;
formatting-only consumer integration enters CI. Source75e5064d1/PR1482
full50-unit-plus1-integration CI running. CT19_PLAN.md freezes actual7-map/
dual construction and a later compatible genuine Jacobian7-hop. No such map
or cost advantage pre-accepted;standing unsigned. [pcc:unverified]

Current actual quartic-gluing extension: [written proof](QUARTIC_GLUING_PROOF.md)
(933d35),[two-page report](reports/quartic-gluing/report.pdf),
[native source](successor-preflight/src/quartic_gluing.rs),
[formal structural lemmas](lean/QuarticGluing.lean) and
[identity adapter](algebra-certificates/src/main.rs). Full31+23 tests and
154written31modules;actualTateindex16/sixgeometricJacobian distinction
uses explicit written global dependencies. See CHECKPOINT/QUEUE for the
next full4-primary quotient evaluator. [pcc:unverified]

Full4-primary quotient extension: [proof](TWO_PRIMARY_QUOTIENT_PROOF.md)/492496,
[two-page report](reports/two-primary/report.pdf)/04e86b,
[native](successor-preflight/src/two_primary_quotient.rs) and
[formal consequences](lean/TwoPrimaryQuotient.lean). Full32+23 tests/
160written32modules;all256A4classes/all16lifts/all6directions,Frobenius
fielddegree12 and a conditionalseparablecrossfamilydegree4 obstruction.
Literalclearedmapzero isdistinctfromthenativedoublingcontrol;V2explains.
Allscientificstandingunsigned. See currentCHECKPOINT/QUEUE. [pcc:unverified]
