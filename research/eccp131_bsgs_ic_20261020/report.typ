#set page(paper: "a4", margin: (x: 18mm, y: 18mm))
#set text(font: "Libertinus Serif", size: 10pt)
#set par(justify: true)

= Certicom 130-bit challenge workflow integration

#text(size: 8pt)[Recorded 2026-10-20; validation updated 2026-10-21 · Darwin arm64 · Rust/Cargo 1.93.1]

== Exact identities and command paths

The published #link("https://www.certicom.com/en/curves-list")[Certicom curve list] supplies the prime-field ECCp-131 group and the distinct binary Koblitz ECC2K-130 group. The ECCp-131 field, curve, generator, subgroup order, cofactor, and published point are bound in the #link("../../src/ecc/curve.rs")[curve constructor] and #link("../../src/bin/ic/params.rs")[IC profile]. ECC2K-130 already has a fixed-parameter file in `aburan28/cryptanalysis`.

#figure(image("workflow.svg", width: 100%), caption: [Exact-parameter checks and bounded BSGS are separate from the scaled prime IC and staged binary IC paths. The red box is an open implementation gap. Source: `workflow.dot`.])

== Observed validation

- `cryptanalysis` commit `3ac3f59363`: two ECCp-131 release unit tests passed; the direct `ca-ic inspect` command reported `checks_passed` for both published points, including curve-equation and subgroup checks. General prime-field point counting was explicitly `not_checked`.
- `ca-icx inspect eccp-131` passed; its estimate identified a scaled IC regime. `crax ecdlp --analyze-only` accepted the published point for structural analysis.
- The ECC2K-130 fixed-parameter status command exited 0 with `status: stopped`, zero saved relations, and no logarithm database. Its #link("ecc2k130-status.json")[raw JSON] and #link("ecc2k130-status.sqlite3")[SQLite state] are retained. The command was a workflow check; its operation counters are not used as an attack-cost comparison.
- `crypto` implementation commit `07dd1d97fa`: the exact-group bounded BSGS release test passed. The first full release library run ended with 4,445 passed, 11 failed, and 119 ignored. Seven failures came from a sparse-checkout fixture omission; all affected tests passed once its tracked files were present. Four unchanged time-limited `jv_cover` cases passed in isolation after reruns, with one intermediate sample-count failure. The failed full run remains recorded in #link("REPORT.md")[the source report].

== Boundary and next check

The exact ECCp-131 relation-to-scalar chain has no validated full-parameter run. The prime IC CLI executes its relations and descent on a generated smaller curve; #link("../../problems/eccp131-full-parameter-index-calculus.md")[the problem record] states the required exact-curve relation, independent rank, target descent, reconstruction, and cold-cost checks. Bounded BSGS requires a justified interval and refuses a whole-subgroup table. No speed or full-parameter IC cost conclusion is made here.

The existing index-calculus scoreboard and progress timeline were checked. Neither has a new measured row from this integration, so their canonical data and rendered copies remain current. Full commands, revisions, initial sparse-checkout setup failures, raw artifact digests, and status distinctions are in #link("REPORT.md")[REPORT.md].
