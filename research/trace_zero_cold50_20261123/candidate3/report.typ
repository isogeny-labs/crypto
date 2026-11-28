#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.5em)
= ECC2K-130: more than 50% fewer cold-query instructions
The unchanged degree 131 d12 uniform relation query uses *21,360,373 complete cold simulated instructions*: *51.48%* below the fixed original count of 44,023,404, meeting its unchanged 22,011,702 ceiling. Against the simultaneous matched original implementation (44,038,338), the reduction is 51.50%. All 120 primary rows and every frozen control criterion pass. Unsigned producer statements PCC-20261123-84a0ce / PCC-20261123-9a757d. [pcc:unverified]
#image("instructions.svg", width: 100%)
#table(columns: (1.4fr,1.2fr,1.2fr,1fr,1fr), inset: 4pt, [Fixed d12 query], [Original cold Ir], [Final cold Ir], [Cold saved], [Oracle saved],
[83/planted], [28,516,367], [14,444,017], [49.35%], [14.29%],
[83/uniform], [33,348,936], [19,080,235], [42.79%], [10.70%],
[83/cancellation], [24,706,931], [10,781,796], [56.36%], [21.41%],
[131/planted], [36,262,501], [15,297,142], [57.82%], [25.19%],
[131/uniform], [44,038,338], [21,360,373], [51.50%], [23.62%],
[131/cancellation], [32,399,888], [12,224,404], [62.27%], [28.13%],)
The table compares simultaneous matched implementations. Five repeats per cell; whiskers show observed ranges, not population intervals. The fixed historical denominator remains separate and unchanged. [pcc:unverified]
#pagebreak()
= What changed, and what remains the same
The prior exact half-trace constructor remains: invert the trace-normalized Artin–Schreier map and build the same byte tables, with the exact conventional half-trace root. The final candidate adds four concrete implementation changes:

+ Parse ordinary ASCII hex directly into field words, using baseline SSE2 for eight-digit validation/packing on x86-64 and portable decoding elsewhere. The complete legacy Unicode-whitespace/plus/underscore grammar remains the fallback; truncated high digits are still validated.
+ Use three/six carry-less Karatsuba products for two/three limbs under the existing runtime PCLMUL guard, preserving XOR accumulation and generic/ISA fallbacks.
+ Fuse the canonical 131-bit product with the existing exact two-fold sparse reduction, inline compact reduction, and use separate immutable OnceLocks for the two completely qualified field polynomials. All table construction occurs at runtime and is fully charged.
+ Retain one BTreeMap Entry search across the same original sign/admission checks and insertion. The original supplied index order is retained.

The existing trace-zero variety, factor base, d12 query, projected equations, 13 coefficient bits, signs, model order, rank/limit behavior, every root check and original-group replay remain unchanged. No relation-yield or system-size reduction is inferred.

= Validation and accounting
The final focused release suite passes 49 checks. These include every byte at each SIMD position, legacy parsing and word/inline/heap boundaries, all 53,248 raw two/three-limb basis products, all 17,161 fused canonical 131-bit basis products against independent bitwise reduction, qualified cache separation, and existing context/projector/field/group checks. Three exact source variants agree on every semantic/witness/model/order/group/projected diagnostic across all 65 retained inputs (195 rows). These remain producer-written checks. [pcc:unverified]

A separate schoolbook polynomial/group implementation replays all 80 primary tuples and 138 development tuples. Primary flow: 120 planned, attempted and completed; zero excluded, zero retries. Recorded compact multiplication/squaring counts agree; those tallies exclude general setup, and total Ir prices all parsing, allocation, arithmetic and index changes. All six A/A and no-regression guards pass. [pcc:unverified]

For the fixed n131 uniform query, complete oracle instructions fall from 18,083,340 to 13,811,466 (23.62%). The prior incumbent cold count is 34,999,978; final code removes 38.97% of that matched cost. Warmed relation-collector throughput and native runtime need their own measured boundary. [pcc:unverified]
#pagebreak()
= Reproduction and retained attempts
The two earlier engineering pilots are retained: candidate 1 saves 45.9443% and candidate 2 saves 49.1996% against their matched originals, both below the unchanged goal. Neither has an attempted primary row. Their complete source/binary identities, calibration, raw pilot data and reports remain under candidate1/ and candidate2/. The inherited missing arm allowlist was caught and corrected before comparative data; the failed qualifier prefix is preserved. [pcc:unverified]

Final numerical source: 7aa3c43c5e23d24b386c15b83759a8924900a9af. Primary protocol publication: 0a2e495ef2d5ec76f9a4c2a8ef5c200f93a6e7f4, before every primary row. Protocol SHA256: 3a37ad5672ec53ecf9f856c124717cb719ce2224d9bd6c8ea8326f94a122cbd4. Source inventory is unchanged after the panel. The fixed-reference percentage is secondary post-run Rust arithmetic; the original absolute ceiling and contemporaneous 50% acceptance rule were already frozen and recomputed by a separately implemented Rust grid checker from the same producer. [pcc:unverified]

One release harness, three exact numerical source variants and separate frozen executables. Controls select exact original/prior field modules plus unchanged old word/index source snapshots; the final candidate selects actual production modules. Compiler/layout effects are charged. A/A uses the identical original executable. Runtime label guards reject source/executable mismatches. Qualified unexecuted SAT/chain interfaces support compilation; numerical field/curve/context/oracle modules are actual production sources.

Rust 1.100.0, Callgrind 3.24.0, Intel Xeon Platinum 8573C. CPU0, 2 GiB child address space, zero paid resources or retries. The final calibration/pilot/freeze/primary sequence spans 23:50:03–23:52:56 UTC, within its 600-second guard. Each full cold count includes input/fixture/context setup, first-use tables, all oracle preparation/search/roots/signs, replay, output, destruction and exit. There is no uncharged offline lookup-table construction. [pcc:unverified]

The broader release root-library attempts time out while compiling dependencies, before tests execute. Final root integration, independent signed scientific admission, artifact-store custody and historical leaderboard-validator reconciliation remain separate gates. No admitted leaderboard row is added. [pcc:unverified]

= Sources and evidence
KB unavailable (wrapper 127, no MCP). Reviewed actual field, word-workspace and index code, the num-bigint parser grammar, existing Karatsuba/reduction formulas, current source-bound profiles and Rust Entry semantics. The algebra and data-structure methods are established; no novelty claim.

Evidence: protocol.json, rows.jsonl, export.json, summary.json, grid-audit.json, polynomial-replay.json, frozen-reference.json, checks/source-inputs.json and raw/executable archives described in the parent README. Plots only render the recorded Rust summaries; no Python scientific analysis or solver computation is used. All observations remain unsigned producer evidence pending independent review.
