# Parallel EC experiment follow-up

Date: 2026-11-24. Scope: the [100 source proposals](IDEAS.md) and their [item-by-item assessment](README.md), with the owner's selected target `E(F_p)`. Three parallel workstreams examined items 1–34, 35–67, and 68–100. This report separates mathematical screens, historical records, three newly executed bounded EC checks, and proposals lacking an executable point-relation construction. No 100-way experiment was performed. Full-cost studies need separate frozen workloads.

![First-pass status diagram: mechanism counterexamples, existing baselines, and initial direct decomposition diagnostic.](PROGRESS.svg)

## Direct mathematical checks

These are **derived counterexamples to the literal proposed mechanism**, not impossibility theorems for every variant.

| Item | Check | Consequence |
|---:|---|---|
| 26 | A subfield of `F_p` has cardinality `p^d` for a divisor `d` of `1`; the only choice is `d=1`. | A proper-subfield sieve cannot run inside `F_p` itself. An extension and norm need a separate costed construction. |
| 35 | Let three edge rows on a triangle be `A = [[1,1,0],[0,1,1],[1,0,1]]`. The underlying graph has a one-cycle, while `det(A)=2`, so `A` has full rank modulo `5`. | Topological cycle detection alone does not identify a modular row dependency. A coefficient-preserving cycle-to-row map is required. |
| 46 | In `F_23^×`, `2^11 = 1` and `13^11 = 1`, while neither base is `1`; since `11` is prime, each element has order `11`. Multiplication by either element permutes the 22 nonzero field elements in two 11-cycles, giving the same spectrum. With integer smoothness bound `B=3`, canonical representatives `2` and `13` have different smoothness. | Phase information from the multiplication operator's spectrum cannot alone decide representative `B`-smoothness. A separate factorization predicate is needed. |
| 80 | The binary order-two de Bruijn cycle `0011` has length-two windows `00, 01, 11, 10`, interpreted as `0,1,3,2`. Reduction modulo `3` yields `0,1,0,2`. | Window uniqueness does not imply distinct field residues. A candidate-map injectivity proof is needed before claiming nonredundant coverage. |
| 83 | The one-entry matrix `[2]` has rank one over the reals and rank zero over `F_2`. | Real SVD cannot certify finite-field rank without an exact modular reconstruction step. |

The #35 check also shows why the repository's **coefficient-aware** large-prime cycle filtering matters: the graph topology alone omits row coefficients and the subgroup modulus. These arithmetic checks are short enough to replay directly from the displayed values. They are not general algebraic identity claims or performance measurements.

## Historical evidence checked against proposed work

| Items | Existing record and bounded reading | Next discriminating step |
|---|---|---|
| 24 | The [technique inventory](../../docs/ic/ECDLP_RESEARCH_TECHNIQUES.md) reports dense `S4`/`S5` construction and a 40-bit diagnostic with 125/125 and 6561/6561 nonzero coefficients; [KN-OPEN-004](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/knowledge/open-problems/KN-OPEN-004.md) already poses the sparse-support question. A bounded exact GP diagnostic below found a special-coordinate support reduction on a prime-order curve. | Freeze a matched elimination and relation-collection protocol before any performance claim. Test whether the support reduction survives coordinate changes and lowers exact solver cost, then check independent relation rank. |
| 53–60 | The [A100 kernel record](../../benchmarks/records/20261112T2313Z-ic-gpu-cards.json) reports SpMV 0.008175 → 0.0005398 s, `n=5`, equal CPU digests. The [complete-solve record](../../benchmarks/records/20261115T2002Z-ic-gpu-rootfind-la-r3.json) reports a 39-bit, factor-base-size-4096 GPU solve 1.541 → 0.305 s, `n=6`, with a 0.0866 s matched rho reference. Both records are historical and `[pcc:unverified]`; the former is a kernel result, the latter is a complete solver result on its recorded workload. The complete candidate time is about 3.52 times that reference. | Profile the actual limiting stage in a current, frozen run, then compare exact GPU queue, arithmetic, and memory-layout variants with the same input and transfer accounting. No new GPU run was made here. |
| 75–76, 84 | The [technique inventory](../../docs/ic/ECDLP_RESEARCH_TECHNIQUES.md) identifies an existing coefficient-aware multi-large-prime hypergraph filter, CADO-style filtering, and block Wiedemann. The [graph-theory study](https://github.com/isogeny-labs/workspace/pull/59) reports that structured Gauss changed little in its tested prime cells, where linear algebra was 0.03–0.14% of recorded total GAE `[pcc:unverified]`. | Replay current verified partial-relation streams through the existing filter and a proposed separator/rank variant, retaining source-row witnesses, modular rank, target column, and complete cost. |
| 96–97 | The [graph-theory study](https://github.com/isogeny-labs/workspace/pull/59) reports a 24-repeat frozen base-size sweep on prime 16-bit and 18-bit instances. Its recorded `S` for `m=2` is 13.460 at base size 16 versus 19.620 at size 40, and 12.839 at size 32 versus 20.389 at size 63 `[pcc:unverified]`. These are **fixed-base** comparisons; they do not test a dynamic controller or target descent. | Compare a predeclared online resizing policy with the best frozen base on held-out, time-ordered relation streams, including tuning, setup, rank, solve, and target descent. No suitable interval trace was found in the selected study for #97. |

The observed GPU ratios and frozen-base values belong to their source workloads. We made no new performance measurement and did not transfer those numbers to a different curve or hardware. The [GPU audit](../../docs/perf/20261112-ic-gpu-audit.md) and the source records contain the original methods and limits.

## #24 exact support diagnostic

The committed [PARI/GP source](S4_SUPPORT_GP.gp) and [raw output](S4_SUPPORT_GP.txt) use GP 2.17.3 on arm64. The source computes the standard short-Weierstrass `S3`, takes `Res_t(S3(x,y,t), S3(t,w,z))` modulo 1009, and enumerates every coefficient in the degree-four box. The fixed `z` values are listed in the source; no random seed or timing is involved. The wrapper separately checks group cardinality, primality, and the order of the point with `x=0`.

Reproduce with `gp -q < S4_SUPPORT_GP.gp`; compare stdout byte-for-byte with `S4_SUPPORT_GP.txt` and reject any GP `***` error line. This source and raw output are a structural diagnostic, not a frozen performance protocol.

**Native corroboration protocol, frozen before invocation.** Run the existing `semaev_prime_structure` example from the checked-out commit with Rust 1.99.1, `--p 1009 --a 1 --b 14 --m 3 --zs 3 --lines 0 --seed 20261124`. Its three seeded `z` values are not selected by hand. Accept only parseable JSON rows with `p=1009`, `m=3`, and `support_max=125`; independently recompute their support counts with GP before interpreting agreement. Record the command, toolchain, exact rows, and any build failure. Do not use elapsed time from this local correctness check as a benchmark. The tracked `panel.json` dependency was materialized from `isogeny/main` after a first sparse-checkout build could not find it; its bytes were not edited. The final release build succeeded and the committed [native output](S4_SUPPORT_RUST.txt) has `z=750,620,320`, each with `125/125` support and nonzero box corners. The GP source was extended to check those exact returned values independently; its output agrees on all three counts. `--lines 0` means this run did not test irreducibility. The example printed a short execution time, which is excluded from cost comparisons.

| Curve over `F_1009` | Fixed `z` | Nonzero coefficients / 125 | Nonzero box corners / 8 | `#E`; order of point at `x=0` |
|---|---:|---:|---:|---|
| `y²=x³+2x+3` | 7, 17, 29 | 125 each | 8 each | 1068; 267 |
| `y²=x³+2x+3` | 0 | 88 | 4 | 1068; 267 |
| `y²=x³+3` | 0 | 37 | 1 | 948; 3 |
| `y²=x³+2x` | 0 | 35 | 4 | 980; 2 |
| `y²=x³+x+14` | 0 | 88 | 4 | **1013 prime; 1013** |
| `y²=x³+x+14` | 7 | 125 | 8 | 1013 prime; 1013 |
| `y²=x³+x+14` | 320, 620, 750 | 125 each | 8 each | 1013 prime; 1013 |

![Exact nonzero coefficient counts for the fixed S4 specializations.](S4_SUPPORT.svg)

The last pair shows that a special-coordinate support reduction occurs even when the whole curve group is prime-order. All specialized resultants in this panel retain degree four in each free variable. This is a **structural component observation**: fewer nonzero coefficients need not imply lower elimination degree, lower decomposition time, more independent rows, or lower full solve cost. The 40-bit inventory result and this 1009-field panel have different curves and specializations. The original Rust example was attempted first but could not compile on this merged checkout because `research/ecbench_phased_panel_20261117/panel.json` was missing; the GP calculation supplies an independent bounded diagnostic, not a Rust performance comparison. An initial version of the GP script also had a multiline `for` parse error after printing support rows; the point-order block was rewritten as a function and the final raw output was regenerated without GP error lines.

## Target-domain boundary for the requested EC scope

The owner selected discrete logarithms in `E(F_p)` as the target for this pursuit. Many source proposals describe multiplicative smoothness in `F_p^×`; those screens are assessments of the attachment's literal premise, not EC point-relation results. For the #24 diagnostic curve over `F_1009`, the exact group count recorded in `S4_SUPPORT_GP.txt` is prime `1013`, while `|F_1009^×|=1008` and `gcd(1008,1013)=1`. Every group homomorphism between these two particular finite groups is therefore trivial, because its image order divides both group orders. A proposal using multiplicative relations needs a separate, explicit construction that yields checked point relations for this EC target. This arithmetic boundary is specific to the diagnostic curve and does not rule out other constructions on other curves. `[pcc:unverified]`

## Second parallel pass: exact checks and execution boundaries

Each of the [1–34](WORKSTREAM_001_034.md), [35–67](WORKSTREAM_35_67.md), and [68–100](WORKSTREAM_68_100.md) workstreams now has one item-specific outcome or missing construction per proposal. These are bounded producer analyses `[pcc:unverified]`; an item with a stated gate has not thereby been experimentally validated.

| Workstream | New bounded result | Execution boundary |
|---|---|---|
| 1–34 | Derived exact gates for the direct EC translations, including trivial valuations on `F_p^×` (#1), the absence of exceptional divisors when blowing up a smooth point of `P¹` (#4), and the character-evaluation circularity on a prime-order EC subgroup (#11). A matched #24 protocol fixes the prime-order curve `y²=x³+x+14` over `F_1009`, on-curve `z=0` and `z=320`, one 16-coordinate factor base, and all 16³ ordered triples per target. | The required S3 checkpoint upload returned HTTP 403 with `InvalidAccessKeyId` on token and profile paths. The #24 decomposition/rank/cost comparison was not launched. |
| 35–67 | The same triangle support gives rank 3 or 2 over `F_5` depending on coefficients (#35–39). The point `P=(0,425)` on the diagnostic EC doubles to `Q=(991,641)`, so a direct group check accepts `Q−2P=O` and rejects `Q−P=O` (#61–62). Abstract quantum and matrix checks specify missing exact EC output maps; historical A100 numbers remain tied to their source workloads. | No matching GPU or specialized hardware was available for a fresh device comparison. Scratch Python outputs were excluded under the repository's native-language rule; the displayed arithmetic is hand-checkable. |
| 68–100 | A coefficient-aware three-row example cancels all large-prime columns while retaining source rank 3 and only one completed small row (#75–76); two additional independent small rows are needed to pin its target. The prior fixed-base sweep gives controls for #96, not an adaptive result. | Selected historical records lack locally accessible immutable partial relation rows, and the native filter test build stopped on missing sparse-checkout fixtures. The aggregate base-size CSV has no interval-order trace for #97. |

The 35–67 and 68–100 small matrices are algebraic controls, not recorded EC relation streams. The #61–62 point identity is an EC group check; it does not establish a new collector. No new benchmark timing or full-cost comparison is claimed from this pass.

## Coverage of all 100 items

Every numbered item remains in the [assessment ledger](README.md), with its stated decisive check. The three parallel workstreams refined those checks by range: 1–34 require a relation-preserving construction or a costed sieve comparison; 35–67 require exact modular output or an exactly checked relation predicate before hardware or learning cost can be compared; 68–100 require exact rank/coverage or a sound map from proposed representations to point relations. The five arithmetic counterexamples above close only their **literal premises**. The source-list, ledger, and this report together distinguish `screened`, `historical evidence inspected`, `diagnostic completed`, and `future experiment` statuses. No result is silently promoted by a screen.

## Next controlled batch

### Executed bounded native runs and admission status

Following the request to execute experiments, three distinct public-EC checks were frozen and launched in parallel. #24 uses [PARI/GP source and report](S4_MATCHED_REPORT.md) for `z=0` versus `z=320` on `E/F_1009`; #35–39 use a [same-support rank protocol](EC_GRAPH_RANK_PROTOCOL.md) with six checked point equations; #75/#76/#84 use a [native Rust filter protocol](WS68_FILTER_PROTOCOL.md) on `E/F_17`. The raw receipts, corrected source revisions, and independent verification artifacts are linked from those records. All results below are producer claims `[pcc:unverified]` pending the repository's signed verification process.

The required `scripts/checkpoint.sh gate` returned HTTP 403, and all three configured AWS profiles failed identity checks. After this was disclosed, the owner explicitly authorized these bounded local runs without the S3 checkpoint. Each protocol records that exception. The first GP attempts for #24 and #35–39 failed because multiline top-level input did not parse; their stdout, stderr, exit files and original sources are preserved. A separately amended V2 GP source for each kept the mathematical inputs and acceptance checks fixed, and the V2 stderr files are empty. The first Rust compile attempt failed on an inferred crate name containing a dot; the second added an explicit crate name without changing the source. These failed attempts remain part of the record.

| Item(s) | Bounded outcome | Interpretation limit |
|---|---|---|
| 24 | On the same prime-order `E/F_1009` and 16-point base, exact `S4` zero checks agreed with signed EC sums for all 4096 triples at each target. At `z=0`, 24 triples yielded 24 signed rows of rank 4/17; at `z=320`, 33 yielded 33 rows of rank 6/17. Support was 88/125 versus 125/125, while the pair-specialized Horner proxy was 16,320 versus 16,352 multiplications and 15,728 versus 16,320 additions. [Raw V2 output](S4_MATCHED_V2_STDOUT.txt) and a [distinct row/rank verifier](EC_S4_V2_INDEPENDENT.stdout.txt) agree on the emitted rows and ranks. | The 29.6% support reduction corresponded to only about 0.20% fewer proxy multiplications and 3.63% fewer proxy additions. Resultant construction and specialization, wall time, and full collection cost were not measured. The two target identities differ, so their yield and rank differences cannot be attributed to support alone. |
| 35–39 | Two matrices with identical triangle support had ranks 3 and 2 modulo the EC subgroup order 1013. All six row equations held for the recorded EC points. [GP V2 output](EC_GRAPH_RANK_GP_V2.stdout.txt) and a [distinct native verifier](WS68_EC_GRAPH_V2_VERIFY.stdout) agree. | This falsifies support-only rank inference on these planted matrices; it does not measure a relation collector, topology-guided filter, or complete solve. |
| 75/76/84 | A native exact control on `E/F_17` checked three partial EC point rows, coefficient-aware cancellation to one small row, source rank 3, small rank 3 only after two additional known rows, target log 12, and a disjoint block of rank 2. [Native stdout](WS68_FILTER_REPLAY.attempt2.stdout) and a separate read-only point/rank audit agree. | The rows and two unit equations are planted with known logs. This measures exactness and target coverage in a fixed control, not naturally collected relation yield, filter speed, or scale. |

#### Independent readback

The #24 [independent verifier](EC_S4_V2_INDEPENDENT.md) checked all 57 emitted point rows and recomputed both ranks; it did not independently enumerate `S4` zeros or support counts. Separate checks reproduced the six #35–39 point rows and the #75/#76/#84 fixed rows and hashes. None is a signed PCC verdict.

1. Restore valid checkpoint credentials for subsequent experiments. For #24, freeze a comparison that separates target identity and resultant construction cost from coefficient support; the present two-target arithmetic proxy does not isolate those effects.
2. Obtain one immutable set of independently checked EC full and partial relations for #75/#76/#84, including source-row witnesses and target column; then freeze a matched filter replay. Stop if source-row combinations or target-column recovery fail.
3. Obtain time-ordered relation and descent traces, then freeze the #96/#97 policy panel against the best fixed controls. Include tuning, setup, rank, solve, and artifact budgets.
4. Profile a current exact GPU end-to-end run before allocating #54/#55/#57 engineering, since the historical kernel and complete-solve records identify different boundaries.

These bounded passes are incomplete relative to experimental pursuit of all 100 proposals. Many descriptions need a defined EC point-relation mechanism before an executable experiment exists; their exact missing obligations are retained in the assessment ledger and the three workstream reports.
