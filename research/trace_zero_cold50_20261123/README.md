# ECC2K-130: fixed-baseline 50% instruction goal met

The unchanged degree131 d12 uniform index-calculus relation query uses **21,360,373 complete cold simulated instructions**, **51.48% below the unchanged original44,023,404 count**. It clears the fixed22,011,702 ceiling. The simultaneous matched original records44,038,338 (51.50% removed); prior incumbent34,999,978 (38.97% removed). Complete oracle instructions fall from18,083,340 to13,811,466 (23.62%). All120 rows and all six frozen control criteria pass. Unsigned producer statements PCC-20261123-84a0ce and PCC-20261123-9a757d. [pcc:unverified]

[Final three-page report](candidate3/report.pdf), [editable source](candidate3/report.typ), [graph](candidate3/instructions.svg), [150dpi PNG](candidate3/instructions.png), [all six cases](candidate3/results-table.md).

## Request and evidence

| Requirement | Status and evidence |
|---|---|
| At least50% below the fixed original count | Pass, producer only: frozen goal ceiling22,011,702; final21,360,373. [Fixed-reference arithmetic](candidate3/frozen-reference.json). |
| Fresh matched comparison |120/120 complete, zero exclusions/retries;6cases*4arms*5repeats. [Raw rows](candidate3/rows.jsonl), [frozen Rust summary](candidate3/summary.json), [separate grid audit](candidate3/grid-audit.json). |
| Same inputs and every original check/order |65 cases on all3 executables,195 native rows. [Qualification](candidate3/equivalence-rows.jsonl), [input hashes](candidate3/input-index.json). |
| Exact primitive implementation |49 focused release tests. [Log](candidate3/checks/focused-tests.txt): full raw/fused basis products, parser grammar/boundaries, cache separation and existing context/projector/group checks. |
| Actual saved relations replay |80 primary and138 development tuples replay with separate schoolbook field/group arithmetic. [Receipt](candidate3/polynomial-replay.json). Same producer; independent signed admission pending. |
| Complete counted boundary |Setup+oracle+post, all cold cache construction, input parsing, replay and process exit charged. Three phased/unphased calibrations pass0.1%. |
| Canonical export |[Strict check](candidate3/checks/export-check.txt) has zero findings. [Export](candidate3/export.json). |
| Raw/binary custody |Both complete archives extract and match every member digest. [Manifest](archive-members.json), [receipt](candidate3/checks/archive-custody.json). Local custody only. |
| Full root integration |600second release attempts time out compiling dependencies before tests execute; [latest log](candidate2/checks/root-tests.txt), [exit124](candidate2/checks/root-tests.exit). Remains open. |
| Admission/storage/historical leaderboard |Separate gates remain open. Earlier authenticated HeadObject403 and historical E27/E30/E32 tolerance failures are preserved; no gate weakened and no admitted row added. |

All result/check assertions above are unsigned producer evidence. [pcc:unverified]

## Implementation

The original exact-half-trace constructor remains part of the final code. Ordinary ASCII hex now decodes directly into field words, with eight-digit SSE2 packing on x86-64 and portable decoding elsewhere. The original parser preserves Unicode whitespace, optional plus and underscore syntax as fallback; discarded high digits are still validated. Two/three-limb PCLMUL products use three/six Karatsuba products under the existing runtime ISA guard, retaining generic paths and XOR-accumulation semantics.

For the complete ECC2K131 modulus, the compact workspace fuses the same product with its existing exact two-fold reduction. Qualified complete83/131 polynomials have separate immutable trace-table OnceLocks; other polynomials retain the general cache. All tables are built on first use and charged. The relation solver makes one BTreeMap Entry search across the original sign/admission/insertion logic, preserving every supplied index and model order.

The trace-zero variety, factor base, target, all projected equations/13 coefficient bits, rank/limit behavior, roots/signs and original-group checks are unchanged. This does not establish a smaller summation-polynomial system or new Z[tau] relation-finding leverage. Warmed-collector throughput and native runtime are unmeasured.

## Retained development history

Candidate1's engineering pilot saves45.9443%, and candidate2's49.1996%, both below the unchanged50% target. Neither has attempted primary rows. [Candidate1 report](candidate1/report.pdf), [candidate2 report](candidate2/report.pdf), their source identities/drafts/raw pilots, and the inherited harness allowlist failure remain preserved. Candidate3 passes its engineering pilot before the unchanged primary protocol is frozen and published. [pcc:unverified]

[First incumbent profile](profile.pdf), [remaining-cost profile plan](candidate1.profile.v2.protocol.json), [recorded candidate2 index hotspot](candidate2/checks/pilot-oracle-annotate.txt). The first remaining-profile plan was unrun and superseded before data to correct a copied instrument label. Its benchmark metadata was honestly imported after the calls from actual framework manifests; no counter retry was used for bookkeeping.

## Reproduction and provenance

Final numerical source7aa3c43c5e23d24b386c15b83759a8924900a9af; primary protocol publication0a2e495ef2d5ec76f9a4c2a8ef5c200f93a6e7f4 before every primary row. [Protocol](candidate3/protocol.json) SHA2563a37ad5672ec53ecf9f856c124717cb719ce2224d9bd6c8ea8326f94a122cbd4. [Source inventory](candidate3/checks/source-inputs.json), [unchanged post-run check](candidate3/checks/source-final.json), [binary identities](candidate3/checks/binary-identities.json).

One [locked release workspace](native/Cargo.toml), with default candidate, legacy-halftrace original field and incumbent prior field features. Controls also select exact historical word/index snapshots; default imports actual production modules. Every other harness/dependency/flag is common, and runtime guards bind labels to variants. Use [build script](candidate3/checks/build-executables.sh); [preflight script](candidate3/checks/run-preflight.sh) and [primary script](candidate3/checks/run-primary.sh) preserve actual commands. For a new run, use new output paths and a new start epoch/record; do not overwrite original data.

Rust1.100.0; Callgrind3.24.0; Intel Xeon Platinum8573C. CPU0,2GiB child address space,600seconds combined calibration/pilot/freeze/primary per candidate panel, no paid resources or retries. Final sequence23:50:03–23:52:56 UTC. Native tally and tally-disabled Callgrind children are separate for each row (240 primary child processes). Compact multiplication/squaring tallies are informational; total Ir prices every optimization. [pcc:unverified]

[RAW-ALL.tar.xz](RAW-ALL.tar.xz) preserves every failed/corrected qualifier, all candidate calibrations/pilots/primary outputs, complete profiles, original payloads and full65-case workload. [FROZEN-EXECUTABLES.tar.xz](FROZEN-EXECUTABLES.tar.xz) preserves all21 executables, including initial failed-harness and all numerical variants plus analysis/checker tools. Each member is identified by [SHA256/length](archive-members.json) and independently extracted locally. This is not remote-store custody or signed scientific admission. [pcc:unverified]

The exact frozen-original percentage is produced by the separately documented [post-run Rust arithmetic](candidate3/postrun_frozen_reference.rs). Primary acceptance had already fixed the absolute ceiling and matched50% rule; no raw row or primary analysis changed. Both fixed and simultaneous baselines are explicitly retained.

## Sources and remaining gates

KB unavailable (wrapper127, no loaded MCP). Reviewed actual source/profile stacks, num-bigint0.4.8 parsing grammar, existing Karatsuba/two-fold reduction formulas and standard Rust Entry semantics. Methods are established; no novelty claim. Root/SAT integration, signed independent admission and [artifact-store upload](../../problems/ecc2k-adjoint-artifact-store-upload.md) remain separate. [Historical leaderboard tolerance obstruction](../../problems/ic-leaderboard-historical-tolerances.md) is preserved; no old evidence or tolerance is edited.
