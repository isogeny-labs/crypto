# Prime31 B31 negation-quotient paired complements: complete 256-Q Rust index-calculus control (2026-11-18)

**Status:** Original pinned Rust 1.100.0 CI **PASS**; independent producer-side BigInt arithmetic replay **PASS**; **[pcc:unverified]** (no independent signed verifier). [PR #4647](https://github.com/isogeny-labs/crypto/pull/4647). A **generic-equivalent index-calculus control** is 20.3–21.0% faster in a matched complete 256-target comparison. This does **not** demonstrate a new non-generic ECDLP algorithm nor beat the best matched generic BSGS frontier.

## Curve, frozen scope and source reuse

Original ordinary cofactor-one curve over F_p, \`p=2147483647 = 2^31−1\`:

- \`y²=x³+847514846x+2087444724\`
- \`n=2147446831\`, prime-order subgroup, generator \`G=(625037715,2100627251)\`.
- Same first 31 rational x-coordinate base points (both arms), 31-column modular rank, eight signed leaves, and the previously frozen first 256 public targets of #4623/#4635 (corpus seed \`0x4d49583450333131\`).
- Caller-only unknown target scalars never reach relation search or rank learning. Offline known-source a values are used only for relation RHS; 16 additional public source relation checks, all failed shifts and original-curve final checks are fully charged. Zero fallback in either arm.
- Reuse source \`research/prime31_ic_20261115/native/examples/prime31_s9_b31_half_sum_oracle.rs\` and **original native Rust arithmetic pinned at \`1df714645aa683cf975bc76fcacb9842cb4780fd\`**. Protocol committed before implementation: [B31_NEGATION_QUOTIENT_PROTOCOL_20261118.md](B31_NEGATION_QUOTIENT_PROTOCOL_20261118.md).

## Algebraic exactness and classification

The generic four-sum set \`S=4A\` over the signed 31-point base is closed under negation. It has 618,885 distinct original-curve points, with identity as its unique fixed point under \`P↦−P\` in the odd-prime-order group. Quotienting by negation yields exactly **309,443** representative points (including identity). Store **both** four-leaf words for \`+P\` and \`−P\`, then for any public residual \`R\`, check whether either \`R−P∈S\` or \`R+P∈S\`. If \`R=L+H\` with \`L,H∈S\`, then \`L=±P\` for a representative \`P\`, so a successful branch must be present. Every recovered eight-leaf word is replayed using the original group law; a negative exhausts every representative and both branches.

Both affine additions have denominator \`x_R−x_P\`, with slopes \`(y_R+y_P)/(x_R−x_P)\` and \`(y_R−y_P)/(x_R−x_P)\`, so one inverse can serve both. The existing 512-lane Montgomery batch inversion is extended to pairs. Identity, inverse, equal-x and tangent cases use the **original Rust curve arithmetic**, not dropped or assumed away. These are exact finite-group arithmetic identities.

**Interpretation:** This exploits generic elliptic-curve negation, not a secret or exploitable NIST curve selection property. It is a faster **generic-equivalent signed four-sum relation oracle**, with the preexisting genuine relation/rank/target-recovery plumbing. It is not a sign-independent RR9/S9 elimination algorithm, and must not be promoted as a novel non-generic ECDLP speedup.

## First matched component benchmark, 256 public first-shift residuals

All numbers from the final [exact-source Actions run #44224597718](https://github.com/isogeny-labs/crypto/actions/runs/44224597718) on GitHub Ubuntu 24.04 Rust 1.100.0, one sequential A/B interleaved panel (diagnostic timings, not independently host-calibrated):

| Scope | Original batch-complement | Negation-paired variant |
|---|---:|---:|
| 256 public membership query wall | 2,161.949 ms | **1,584.961 ms** |
| Extra quotient preparation | none | **89.824 ms** |
| Query + candidate extra preparation | 2,161.949 ms | **1,674.786 ms** |
| First-shift hit/miss results | 244 / 12 | **244 / 12, identical mask** |
| Exhaustive negatives | 12 | 12 |
| Stored four-sum points / reps | 618,885 | 309,443 |
| Original batch field multiplications | see raw | see raw |

Previous successful source run #44224128962 (its test passed, but a CI shell grep gate failed due quoting) independently measured 1,899.587 ms baseline query, 1,375.302 ms paired query, and 69.422 ms extra setup, or **1.381× component query speed ratio** and **1.315× including extra setup**. The final run supersedes those timings for its exact-source record; host variance limits generalization.

All original-curve signed witnesses replayed, 12 negative lookups exhausted, and four explicit exceptional/identity controls passed. See full stored [machine-readable component record and full unknown-log transcripts](B31_NEGATION_QUOTIENT_RAW_20261118.json).

## Primary experiment: fully charged complete 256-log solving, randomized arm order

These **same-runner** AB and BA trials each ran both algorithms on the same 256 blinded public points, with factor-base setup, relation collection/rank 31, 16 independent controls, unsuccessful shifts, all 256 unknown-log recoveries and verifier. Both had zero BSGS fallback.

| Full 256 unknown logs | Original first-x31 (ms) | Paired first-x31 (ms) | Savings |
|---|---:|---:|---:|
| AB: old first, paired second | 3,117.049 | **2,461.078** | **21.04% (1.267×)** |
| BA: paired first, old second | 3,117.007 | **2,484.184** | **20.30% (1.255×)** |

All four full panels: rank **31/31**, **256/256 blinded unknown logarithms** independently verified by original Rust \`[k]G=Q\`, 16/16 extra known-source checks, first-shift 244/256, 12 exhaustive negative target lookups, 268 total target attempts and **0** generic fallbacks. Each candidate setup includes building the unmodified original FourSet **plus** creating the canonical ± representation. The setup cost therefore increased (AB old 355.105 ms vs paired 445.110 ms), while target solve/replay declined (AB old 2,316.413 ms vs paired 1,692.627 ms); relation rank and controls were also lower in the paired arm. Both AB and BA produced the same scalars and public points for their corresponding 256 targets.

**Independent mathematical replay performed for AB panels:** a separately written BigInt short-Weierstrass implementation checked the exact curve equation of all 62 base points, 62 base logarithms, 62 modular relation rows, **512 recovered unknown-target scalar products** and **512 signed eight-point witness sums**, plus **512 AB-versus-BA target point/scalar correspondences**. Those checks were run independently of the candidate Rust arithmetic and succeeded. This is producer-operated independent arithmetic, **not** independent signed PCC validation.

The run's source-specific CI reports **40/40 original Rust library tests**, component 1/1, full-cost AB 1/1, full-cost BA 1/1 PASS, including all success gates. Raw run artifact [#14391903238](https://github.com/isogeny-labs/crypto/actions/runs/44224597718/artifacts/14391903238), GitHub-reported ZIP SHA-256 \`b5110f3979f88d08ea95529a05f64db6c544f95deeb25085dff9bc9b226ecd6b\` (not independently downloaded/rehashed). All original JSONs plus provenance and replay receipt are committed in [B31_NEGATION_QUOTIENT_RAW_20261118.json](B31_NEGATION_QUOTIENT_RAW_20261118.json). Old source #4635 remains the historical baseline; this report uses the new same-runner costs, not copied historical 2.7s as a ratio.

## Lead audit and nonduplicate research queue

Previously tried / already owned:

- [#4623](https://github.com/isogeny-labs/crypto/pull/4623): first complete Dickson31 generic-equivalent 1,024-Q rank-and-log Rust baseline; [#4618](https://github.com/isogeny-labs/crypto/pull/4618) 512-lane field batch complement; [#4626](https://github.com/isogeny-labs/crypto/pull/4626) random/Dickson/first-x B31 coverage; [#4629](https://github.com/isogeny-labs/crypto/pull/4629) B27/29/31 complete cost; [#4635](https://github.com/isogeny-labs/crypto/pull/4635) first-x complete baseline. This PR builds on rather than duplicates them.
- [#4575](https://github.com/isogeny-labs/crypto/pull/4575) B8 generic 4+4 comparator, [#4581](https://github.com/isogeny-labs/crypto/pull/4581) seven-quadratic quotient splitting into 128 signed charts, [#4550](https://github.com/isogeny-labs/crypto/pull/4550) B8 D9 separator. These did **not** achieve sign-independent relation extraction.
- [#4604](https://github.com/isogeny-labs/crypto/pull/4604) actively owns sparse RR9 D9 negative certificates. [#4596](https://github.com/isogeny-labs/crypto/pull/4596) owns the same-runner 1,024-Q optimized BSGS vs pair-table IC frontier. Separate autoresearcher [#3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301), [#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305), cryptanalysis [#1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) also cover prior algebraic approaches.
- Published prime-field IC research: Petit–Kosters–Messeng (PKC 2016, https://doi.org/10.1007/978-3-662-49387-8_1), Amadori–Pintore–Sala (FFA 2018, https://doi.org/10.1016/j.ffa.2018.01.009). Basic group negation and batched inversion are prior art; this is a new measured implementation control only. Research-KB MCP and CLI unavailable in this connector-only runtime, so this audit is bounded and not a novelty clearance.

**Prioritized experiments:**

1. **Primary mathematical attack:** actual-B31 public-residual-only *sign-independent* RR9/S9 sparse F4/quotient/rootfinding. No FourSet, no support enumeration, no known target scalar, no paid computation. Positive B8 and existing negative D9 certificates first; then repeated-root/exceptional B31, independent signed group replay and full-rank natural relation rows. Coordinate #4604 before implementation to avoid duplicate D9 work.
2. **Alternative nonenumerative elimination:** K961/S5 target-conditioned resultant / selected-root factoring in existing Rust, with exact certified failure cases, bounded complexity and no hidden \`961^4\` join. Compare operations and memory to the newly measured paired generic control, rather than the slower old control.
3. **Full 1,024 target frontier:** Only after a genuine algebraic extractor produces natural relation rows, swap it into existing full rank/recovery harness and do source-identical 1,024 blinded Q with setup, failures, rootfinding, linear algebra, verification, exceptional points and fallbacks charged. Bench against matched shared BSGS, negation rho and published IC frontier; check efficiency for multiple logarithms and other nonspecial prime curves, including the existing n≈2^31 control.
4. **Independent validation:** independent signed PCC of source/artifact, host-calibrated ABBA repeat and integration of paired arithmetic into a reusable Rust module once its incremental cost remains positive.

No claim of a new non-generic algorithm, of sub-square-root exponent, or of beating the best generic solver is supported by these measurements.
