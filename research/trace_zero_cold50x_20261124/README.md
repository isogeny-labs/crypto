# ECC2K-130: another6.28% fewer instructions;50x goal remains open

The complete fixed n131 d12 uniform index-calculus query uses **20,006,773 simulated instructions**, versus matched previous best **21,346,948**: **1.066986x**, or **6.28% fewer**. Complete oracle instructions fall9.71% to12,458,426. All six control guards pass. Unsigned producer claim PCC-20261124-bc5973. [pcc:unverified]

The user requested **another50x from the fixed21,360,373Ir baseline**, so the original ceiling stays **427,207 complete Ir**, together with the frozen2%-of-contemporaneous-baseline criterion. That goal is **not met**. It is not renamed a50% target or moved to a warmed/batched denominator.

[Two-page report](report.pdf), [all-six-case exact ranges](results-table.md), [plot](instructions.svg), [editable report](report.typ).

| Requirement | Evidence/status |
|---|---|
| Another50x, same full cold query | Unmet.20,006,773Ir is above427,207. [Frozen audit](inline/grid-audit.json). |
| Actual additional improvement |1.066986x/6.28% whole,9.71% oracle on retained n131 uniform; all six control guards pass. [Rust summary](inline/summary.json). |
| Exact implementation and fallback |52 focused release tests, including actual8192-slot/1994-point boundary, arbitrary spill order and tree fallback. [Log](inline/checks/focused-tests.txt). |
| Same relation semantics |65 cases ×3 executables,195 matching native rows, including m83 and malformed-input/limit fixtures. [Qualification](inline/equivalence.jsonl). |
| Full controlled comparison |120/120 primary rows per candidate, four rotated arms, six cases, five repeats; both complete panels retained. [Accepted component rows](inline/rows.jsonl), [rejected first rows](v2-rows.jsonl). |
| Original arithmetic replay |80 tuples per primary replay using separate schoolbook arithmetic. [Inline replay](inline/polynomial-replay.json), [first replay](v2-polynomial-replay.json). Same producer, not independent admission. |
| Strict exports |Both final exports pass with zero findings. [Inline](inline/export.json), [first](v2-export.json), [check](inline/checks/export-check.txt). |
| Source identity during primary |Unchanged frozen inventories for both completed panels. [Inline](inline/checks/source-final.json), [first](checks/v2-source-final.json). |
| Complete local raw/binary custody |Whole archives, per-member hashes/lengths and extraction checks below. Remote custody remains open. |
| Global S4 filter |Proposed relaxation is consistent on all six actual d12 targets; uniform ranks83/131 are full. No rejection or speed gain from that relaxation. [Lead](leads/S4-ENVELOPE.md), [corrected exact grid](leads/s4-actual-window-rows.jsonl). |

All result/check assertions are unsigned producer observations. [pcc:unverified]

## Production change

For at least64 supplied points with low-coordinate x-values, select a direct-address index only if its power-of-two bucket count is<=65,536 and<=8 times the point count. Other shapes retain BTreeMap. The common first two point indices stay inline; additional duplicates spill without changing order. Exposed keys are sorted exactly as before, so the basis, model order and returned indices stay equal. Per-input field-width/sign/curve admission and all root/group/replay checks remain. Every table/allocation is built and charged inside the cold query.

This is an index representation change. It does not shrink the summation-polynomial system, change the factor base or establish additional Z[tau] leverage. Native runtime and warmed-collector throughput remain unmeasured.

## Failed attempts and corrections

The first complete index candidate required at most4 buckets per supplied point. The actual n131 base has1994 points and needs8192 slots, so that candidate fell back to its tree and added selection overhead. Its entire120-row panel regressed about1.02% whole and1.56% oracle; all six regression guards failed. [Audit](v2-grid-audit.json), [exact source snapshot](dense_v2_snapshot.rs). The revision fixes the bounded eligibility test and removes common per-key heap allocations. No first-panel row was excluded. [pcc:unverified]

The first copied-workspace control builds reused stale44M/35M executables from a shared Cargo target. Those pilot comparisons are invalid and retained under invalid-preflight-v1, including every original executable in the binary archive. A fresh isolated target rebuilt all three variants and repeated qualification before any primary. The relocated helper also briefly wrote old protocol/planning metadata; erroneous bytes are retained and original files were restored exactly from committed Git. No old numerical row changed. Initial schema/export drafts and the missing instance_id mapping are retained and corrected without altering counts.

The symmetric S4 identity matches its production resultant on all4096 F8 tuples. The first rank grid used degree d-1, while this fixture's trace-zero d-dimensional window reaches degree d. Both grids/sources are retained; the corrected d12 relaxation has318 columns, full83/131 uniform ranks and consistency on all six targets. The algebraic identity is valid, but this relaxation cannot reject these targets. [pcc:unverified]

## Reproduction and provenance

Inline numeric source **2e9a7b71ff**; protocol publication **bd2b17ed59** precedes every inline primary row. [Protocol](inline/protocol.json) SHA906243b747bf6c0f8f158704fbb2b12070a40401904c40cbe1047e6e9c741de9. Both exact previous-best controls use baseline_f2m_snapshot.rs, baseline_word_field_snapshot.rs and baseline_linearized_snapshot.rs; the default candidate imports production modules. Every other harness/compiler/dependency/flag matches. Use a separate target directory: the initial shared-target cache problem is retained, not silently reused.

At numeric source2e9a7b71ff, build the locked release [workspace](native/Cargo.toml) with default candidate, legacy-halftrace baseline, and incumbent duplicate-control features. Save each executable separately before the next feature build. The [isolated build script](checks/build-isolated.sh) gives the three-variant recipe; adapt output names in a fresh checkout. Do not overwrite existing immutable output directories or restart original epochs. The [inline preflight](inline/checks/run-preflight.sh) and [primary](inline/checks/run-primary.sh) show the exact measured orchestration. A new run needs new artifact paths, a new record and a newly frozen protocol. [Binary identities](inline/checks/binary-identities.json), [input index](input-index.json), [source inventory](inline/checks/source-inputs.json).

Rust1.100.0, Callgrind3.24.0, Intel Xeon Platinum8573C, CPU0,2GiB child address-space ceiling. Each fixed candidate panel has600seconds, zero paid resources and zero infrastructure retries. The first corrected panel retained its original epoch, including the invalid attempt and rebuilds. The inline candidate has its separately declared fixed panel. Fresh native and tally-disabled Callgrind children per row:240 per120-row primary. No native clock claim. [pcc:unverified]

[RAW-ALL.tar.xz](RAW-ALL.tar.xz), [FROZEN-EXECUTABLES.tar.xz](FROZEN-EXECUTABLES.tar.xz), [whole/member hashes and lengths](archive-members.json), [extraction receipt](checks/archive-custody.json). Local producer custody only. [pcc:unverified]

## Sources and next work

Sources checked (KB): wrapper127, no loaded MCP. Read actual production admission/index/projector/field code, prior frozen profiles/panels, standard direct-address/ordered dictionary behavior and the implemented Semaev quadratic-resultant recurrence. Reviewed profiling, controlled-experiment and report-evidence skills. No novelty claim.

The current decoder alone records710,124 self Ir, above the whole-query427,207 target. Solver-only edits retaining that observed ingestion path cannot meet50x; this is conditional on that path, not a general impossibility result. Further candidates need both less parsing/setup and a stronger algebraic method preserving nonlinear dependencies lost by the full-rank S4 relaxation. Those are open leads, not measured gains. [pcc:unverified]

Full root-library integration (prior600second dependency-build timeouts), independent signed admission, native timing, original artifact-store403 and missing historical leaderboard source remain separate. Prior generation receipts are preserved in the earlier study; no identical denied upload or historical gate is retried, no threshold weakened and no admitted row added. Conductor executable unavailable; edits remain scoped to this lane. The requested50x goal stays unchecked and PR5070 stays draft.
