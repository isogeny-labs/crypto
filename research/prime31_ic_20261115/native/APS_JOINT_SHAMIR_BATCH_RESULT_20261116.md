# EXP-R31-APS-JOINT64 — complete prime31 index-calculus Rust result

Date: **2026-11-16 UTC**. **Development/source-specific producer evidence, [pcc:unverified].** This is an actual measured complete-solver improvement of the *published APS-style single-relation index calculus* code on the requested ordinary, cofactor-one prime-field elliptic curve. It does **not** establish a non-generic complexity exponent, independent PCC standing, or superiority to optimized rho and the fastest external PKM/APS/CAS programs.

## Frozen protocol, source, execution and receipts

- **Protocol committed first**: [APS_JOINT_SHAMIR_BATCH_PROTOCOL_20261116.md](APS_JOINT_SHAMIR_BATCH_PROTOCOL_20261116.md), protocol commit `c7be1af3f5e87275dc9c281fc69b7fcf1510d852`.
- **Existing Rust APS solver reused and extended**, not reimplemented: [examples/prime31_aps_one_relation.rs](examples/prime31_aps_one_relation.rs). Candidate commits `b3417047144eac5967f1172ed6156cd044aa06ca` and `b779cf15d72b3e3d0330c33fa5fed5bc6893cf22`, workflow commit `9e9f8c5dff0848b6e34c225b3d7822d433872a34`.
- **Source-bound successful Rust 1.100.0 release GitHub Actions**: [run 43897531857](https://github.com/isogeny-labs/crypto/actions/runs/43897531857), [job 133405033404](https://github.com/isogeny-labs/crypto/actions/runs/43897531857/job/133405033404), exact workflow/source head `9e9f8c5dff0848b6e34c225b3d7822d433872a34`. Focused Rust library/example unit tests and `--joint-jumps-confirm` passed. This does not certify an independent signature or a whole-repository test pass.
- **Raw unmodified program data (GitHub timestamp prefixes removed)**: [APS_JOINT_SHAMIR_BATCH_CI_20261116.json](APS_JOINT_SHAMIR_BATCH_CI_20261116.json), evidence commit `e63e66c238796ca7860cea7b13c9a8a6998e8f47`. Contains every target, its independent-caller-known answer, chronological arm ordering, full cold solver traces, signed witnesses and failed rounds. No headline is substituted for these individual rows.

Curve: `p=2147483647=2^31-1`; `E: y^2=x^3+847514846*x+2087444724 mod p`; `n=2147446831` prime and cofactor one. Same existing `PRIME31.g` used in every arm. Public target SplitMix seed `0x4a5333434f4c4431`, 24 distinct fresh nontrivial `Q=[k]G`, `k` accessible to **caller-side verifier only**, not either solver. APS partition sizes `[384,384,16384]`, maximum six complete rounds. Each arm pays all public generation, rootfinding, misses, verification and cold setup.

## New public-jump construction (scoped optimization)

The incumbent builds **64 labelled public additive-walk jumps** `J_i=[a_i]G+[b_i]Q` by two scalar multiplications and an affine sum for *each* jump, then generates its public partitions by mixed-Jacobian walk with batch normalization. The candidate computes the same 64 public jumps via left-to-right binary Shamir joint multiplication, reusing `G+Q`, keeps them as Jacobian triples, and normalizes all 64 with **one Montgomery batch inversion**. It preserves each public `(x,y,a,b)` label, the exact PRNG transcript, the original `S_3` factorized field-root oracle, the packed third-partition abscissa lookup, S3 pair counts and signed witness recovery. The candidate is an opt-in variant; the old source remains an independent matched control.

**Not a new cryptographic primitive.** Joint double-scalar Jacobian multiplication already appears in this repository's [RESULTS_shamir.md](../../ic_e222_m221_20261106/RESULTS_shamir.md); that study showed a winning *Jacobian*, but losing *affine*, Shamir implementation. This experiment reuses that arithmetic insight in a previously unmeasured APS 64-jump bottleneck, with whole-solver costs. PKM 2016 (DOI 10.1007/978-3-662-49387-8_1) and APS 2018 (DOI 10.1016/j.ffa.2018.01.009) are established index-calculus prior art. Explicit S3 root enumeration of pair supports is generic meet-in-the-middle equivalent.

## Matched 24-Q fully cold results

| Same-host, same public Q, 24 separate cold complete solver calls | Complete and caller-replayed | Total cold wall (ms) |
| --- | ---: | ---: |
| Existing packed-index APS, old independently produced 64 public jumps | 24/24 | 134.118157 |
| **Candidate APS, joint-Jacobian + 64-point batch-normalized jumps** | **24/24** | **87.218164** |
| Candidate independent A/A repeat | 24/24 | 85.745937 |
| Fresh full-table batched-Jacobian BSGS generic control | 24/24 | 106.240928 |

All **96/96** completed, correctly recovered the caller-known scalar, and performed `[k]G=Q` verification in the Rust example. The successful `S_3` algebraic signed relations were identical across APS arms, as were attempt counts and pair query records. The 72 APS signed relations additionally pass independent report-side modular-congruence recomputation; `(alpha + beta * k) mod n == 0` in all 72. This self-executed and report-side checking is **not independent signed PCC**.

- **Candidate/old APS = 0.6503083993**, i.e. **34.9692% lower fully cold cost** or **1.53773× throughput speedup** on the matched sample; candidate wins **24/24** paired targets.
- **Candidate/BSGS = 0.8209469330**, i.e. **17.9053% lower fully cold cost** or **1.21810× speedup** on this matched sample; candidate wins **19/24** paired targets. This is a measured same-run win over the included BSGS implementation, *not* optimized rho or the fastest generic solver.
- Candidate/independent A/A drift = **1.71696%**; same implementation only, not cross-host reproduction.
- Predeclared development gate requires candidate/old <= 0.90, >=18/24 target wins, A/A drift <= 5%, full complete correctness: **PASS**.
- Chargeable APS source setup across 24 Q fell from **74.377654 ms to 26.795911 ms** (A/A **26.280107 ms**), roughly **2.7758×** source-setup speedup. This is a classical arithmetic-source optimization, not a relation-search complexity improvement.

The matched BSGS arm builds its own new 46,340-baby table per target. All four arms are in-process Rust release on GitHub Actions ubuntu-24.04, unisolated producer measurements; the ratios should be remeasured on an isolated identical host before any externally claimed ranking.

## Independently checked raw-data invariants

Report-side readback of the source-exact program JSON enumerated all 24 targets × four arms. Recomputed the four total cold nanosecond sums from **individual cases**, 24 candidate/old and 19 candidate/BSGS paired wins, 96 recovered scalars, 72 signed congruences, 24 matched APS signed-witness arrays and exact round-query transcripts. These recomputations all agree with program's summary. Rust focused unit tests also cover joint-Jacobian versus classical two-scalar arithmetic on public points including `Q=-G` and zero-scalar paths and confirm equality of every source-walk labelled point between old and new for two partition size triples. No independent keyring-signed PCC verdict has been obtained: **[pcc:unverified]**.

## Prior leads and next experiments

| Lead | Established scope or negative evidence | Technical gate |
| --- | --- | --- |
| APS 64-jump source arithmetic | **PASS:** full cold 96-arm measured improvement; 24/24 paired wins vs old, 19/24 vs included BSGS | Fresh separately seeded second panel with isolated CPU and independent replay; compare optimized rho + mature PKM/APS/CAS |
| Earlier packed unique-x index | 5.81% faster than compact index but **failed** its preregistered >=10% gate; prior 24-Q BSGS was faster | Keep frozen negative record; don't overwrite by the current different-sample result |
| Post-Jacobian APS partition sizes | 384/384/16384 held off 256/320/512 alternatives in two 32-Q panels | Keep incumbent until changed mathematical mechanism |
| Anchored two-point m6 F4 | Natural support rigorously <=3.7407% of targets; 0/16 natural supports, and F4 censored even planted-positive | Do not repeat unchanged anchored m6 |
| Ordered m8/chained F4 | Prior `#4373` and followups censored, not completed rational-root extraction | Change algebraic quotient/elimination, not only tuning |
| Symmetric m8 finite signed-leaf validator | All 6,435 B8 unordered eight-multisets / 25,740 four-target decisions agreed with exact signed full-point check, but **given f** | The missing component is recovering **f from Q without scanning candidates** |
| Certified selected-root quotient | `#4228` merged / `#4349` open, useful smaller-arity Rust basis/root certificates but no completed B31 target-conditioned m8 | Compose original-ideal equivalence with **S9 / K-S5** target-conditioned coefficient elimination |

**Highest-priority new non-generic scientific experiment:** construct an actual *target-conditioned*, original-ideal-equivalent symmetric eight-leaf S9 system in elementary coefficients `f(T)=T^8-e1*T^7+...+e8`, with `f | H(T)^8` enforcing repeated factor-base x roots, and the full signed/infinity branches from the B8 oracle. Use the existing Rust #4228/#4349 selected-root/subresultant machinery to **extract candidate f without enumerating unordered supports**. B4 should have 165 possible multisets; B8 has 6,435 and is exhaustively validated, allowing exact solution-set checks. A B31 computation scans 48,903,492 candidate multisets if implemented naively, so that is a stop condition rather than progress. Freeze a pre-registered cap, test natural positives, negatives and constructed-positive targets; only an actual completed root extractor advances to B31 full cold unknown scalars.

Follow with full completed *one-logarithm* solver timings, independent signed PCC, optimized negation-map rho and mature external PKM/APS/native F4/CAS on the exact same curve, inputs, host and units. The existing result is meaningful engineering progress but does **not** claim a new asymptotic attack. No paid compute, merge, policy bypass or leaderboard row was created.

## Access / evidence limits

Research-KB MCP and local wrapper, Conductor and local Cargo/clone were not attached in this GitHub connector runtime. Fallback review used committed GitHub source, logs, prior experiments, #4228/#4349, and primary PKM/APS publisher records. Tests are from exact-source GitHub Actions, not local or separately signed machine; no novelty claim can rely on a bounded search being exhaustive. No external CAS, independent PCC verifier or cost-equivalent optimized rho was run in this panel.
