# Requirement-to-evidence map

Final local run:504/504 primary plus72/72 separate calibration calls completed/replayed, no measurement active. Canonical seed registration fails; see REGISTRATION_ERRATA.md.

| Requested work | Current status | Evidence and precise remaining obligation |
|---|---|---|
| Replace long classical Euclidean GCD with half-GCD; keep schoolbook | Implemented, scoped correctness checked | Existing shared recursive transformation-matrix backend is reused unchanged; schoolbook and Euclid/Newton remain explicit comparison arms. Native matrix/GCD tests and matched relation-set tests pass. No default switch is justified merely by implementing an option. |
| Independently validate the degree bound, including projectors, repeated roots and splitting | Partial | Lean kernel/proof gate checks21 theorems with0trusted exceptions: generic actual-degree NF law, bridge to existing last-fall spaces, finite literal projector/full-sibling/pole/coprimality certificates and nilpotent controls. Nonempty native order19 diagonal test passes. The arbitrary-depth tower NF instance and independent signed standing remain unfinished. See ASSUMPTION_AUDIT.md and PROOF_MAP.md. |
| General rational tower evaluator in Rust, denominators, certified split fibers, charged construction | Implemented, scoped correctness checked | Existing shared homogeneous evaluator, coprime/degree/pole/full-distinct-sibling certification, empty/identity/diagonal branches and point lifting are reused unchanged. Exhaustive small prime-field targets and wide-field comparisons pass. Construction is separately measured and fully charged to all new arms. |
| New frozen complete-query comparison against lookup, followed by measured speedup | Measured descriptively; strict qualification fails | Calibration72/72 and primary504/504 completed/replayed. All12 controls groups pass. Full cold/query counts and local elapsed ratios reported. Canonical seed-table mismatch360/60 findings remains; no L2 admission or protocol pass. Half stays opt-in after slower Euclid/Newton elapsed results. |
| Larger sizes and scaling | Fixed matrix completed | Word62 cosets8192/16384, P224 coset2048, rational word62/2048 and P224/128,512. No exponent inference or scaling beyond measured points; P224 uses its standard curve while the word-field component makes no prime-order assertion. |

Execution boundary: isolated local/Git-only, no paid/cloud experiment or local
S3 operation. The host does not earn L2 timing admission. Local elapsed ratios
can be descriptive observations only if the frozen controls pass; an admitted
speedup remains unavailable under this host/authorization. No protected
benchmark index, frontier or leaderboard promotion is requested or performed.

External publication prerequisite observed05:20UTC: shared crypto-lean PR26's
selftest and build-and-audit jobs did not start because GitHub reports an
account billing lock. The exact check annotations are pinned in
`evidence/shared-library-ci-blocker.json`. Focused local kernel checks passed;
CI did not execute them. No spending or bypass was attempted.

Publication/record blocker: official censored-close aborted on unchanged main historical record20261105T0828Z-icb-large-prime-n53-s1-archive command1 lacking a successful integer exit. Our1301 raw pointers/6 native count metrics are registered, but the record stays open; no historical exclusion or bypass was added. See evidence/record-close-blocker.json.

Official artifact migration failed S3 HeadObject403 at head5ac77b08ee. Nine policy files remain pending; object absence versus access denial is undetermined. Git-only compression/repacking of committed logs is prohibited, so no storage workaround or local S3 operation was attempted. Evidence/artifact-migration-blocker.json retains the actual job and failure.
