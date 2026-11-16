# Prime31 prospective complete APS versus **unfolded** batched rho: fresh result

**Source-specific GitHub Actions**: [run 43919923606](https://github.com/isogeny-labs/crypto/actions/runs/43919923606), job 133484136190, source commit [b1b0339937233b2048f88de8246d001090a37329](https://github.com/isogeny-labs/crypto/commit/b1b0339937233b2048f88de8246d001090a37329), Rust 1.100.0 Ubuntu 24.04 release. **SUCCESS**: touched Rust tests, 24x4 original rho pilot, 24x4 watchdog follow-up and new 24x4 unfolded-rho command. This report covers only the last prospective panel. [pcc:unverified]

Protocol committed **before source**: [APS_VS_UNFOLDED_RHO_PROTOCOL_20261116.md](APS_VS_UNFOLDED_RHO_PROTOCOL_20261116.md) at `d376525132f8977a4e929c7a3a81974e1057c27b`. Implementation `097533a63b7fa60d1014a6db809c6733e5ae2ed3`; source-specific workflow `b1b0339937233b2048f88de8246d001090a37329`. [All original 24x4 Rust program JSON records with timestamp wrappers removed](APS_VS_UNFOLDED_RHO_EXACT_CI_20261116.json), committed at `edce6c98342bb04f970717148b18fd925734d752`.

## Scope and complete cold measurements

Identical ordinary cofactor-one curve `p=2147483647`, `E:y²=x³+847514846x+2087444724`, `n=2147446831` prime, same `PRIME31.g`. Fresh independent public `Q=[k]G` generated from `0x52484f5f4e455733`; `k` was available only to the caller-verifier, not any solver. All four arms independently pay public setup, point generation, memory/hash lookup, group arithmetic, failed attempts, collision recovery and `[k]G=Q` replay. Arm order rotated by target.

| Same-target fully cold Rust method | Complete logs | Total over 24 Q (ms) | Paired APS wins |
| --- | ---: | ---: | ---: |
| Published APS S3 one-relation (joint-Jacobian source, [384,384,16384]) | **24/24** | **94.771239** | — |
| **Unfolded 32-lane DP4 r-adding rho**, correct signed x-collision | **24/24** | **86.369557** | **13/24** |
| **Unfolded 64-lane DP5 r-adding rho**, correct signed x-collision | **24/24** | **85.930974** | **13/24** |
| Unfolded 32-lane A/A repeat (identical per-Q seeds) | **24/24** | **85.074349** | — |

- `APS/unfolded32=1.097275965` and `APS/unfolded64=1.102876350`. On **this one new development panel**, unfolded rho has approximately **8.9%** and **9.3%** lower aggregate cold wall, respectively. This is **not** an independently supported general ranking or a fastest-rho conclusion.
- The 32-lane A/A total drift is **1.5224%**, within the prospective <=5% development stability gate. **96/96** complete calls recovered the correct scalar in source; the caller independently replayed each.
- Rho work charged: **1,403,392** 32-lane and **1,519,360** 64-lane walk steps. **Zero** watchdog restarts, unlike the previously folded map; folded and unfolded were **not** run on the same Q in this test. All arms completed below the 2,000,000-step cap.
- Independent report-side readback recomputed 96 individual cold sums and 13/24 paired APS wins in each comparator, 96 matching caller-known scalars, and all **24** APS signed congruences `alpha+beta*k=0 (mod n)` with zero discrepancies.
- **Uncertainty is material**: paired-target bootstrap (10,000 deterministic resamples of the 24 case pairs, ratio of sums) gives rough 95% intervals **[0.748,1.607]** for APS/unfolded32 and **[0.802,1.491]** for APS/unfolded64. Both cross 1. This panel cannot establish that one method generally dominates. The A/A repeat is deterministically identical walks, not independent target/collision sampling.

## Correctness repair that makes this rho control meaningful

The original generic rho control canonicalizes `{P,-P}` at every step. The new opt-in **unfolded** mode preserves signed affine points, signed `(a,b)` labels and the same precomputed 16 public jumps, batched Jacobian and distinguished-point infrastructure. Because x-only distinguished keys identify either P or -P, an x collision with opposing y must negate the *stored* `(a0,b0)` before solving `(b-b0)k=a0-a (mod n)`. We explicitly check both branches; otherwise the supposed generic control can yield false collisions. The plain variant disables x-only near-cycle detection that presumes canonical points; it still charges the 512-step stagnation watchdog (which did not trigger on these runs). The folded path remains a separate unchanged callable.

The two earlier folded-only panels remain frozen: without long-cycle escape **12/24** and **20/24** rho calls completed under the same cap; after the fresh watchdog repair **24/24**, with an aggregate APS advantage against **those particular** folded configurations. Those are different target panels. Their ratio cannot be transferred to the present unfolded panel.

## Previous leads, external evidence and next tests

1. **Generic control validity — changed:** the optimized folded map was not the only plausible denominator. The new unfolded variant completes every call and is faster in aggregate than APS on this fresh 24-Q panel. Test folded and unfolded **on the same fresh Q** and isolate CPU clock; then directly run the repository's tuned `src/cryptanalysis/ic_boundary.rs::rho.negation` Wiener–Zuccherato look-ahead/cycle escape. Only then propose a generic-reference ranking.
2. **Published APS engineering — retained:** full algebraic m3 S3 one-relation solver on a prime-order curve, complete scalar recovery and existing 64-jump source optimizations. Explicit finite S3 pair enumeration is mathematically meet-in-the-middle equivalent. Prior joint-Jacobian batch improvement and matched BSGS win are separate valid producer-development comparisons, not a security extrapolation.
3. **Novel non-generic m8 path — open:** use merged [#4228](https://github.com/isogeny-labs/crypto/pull/4228) quotient groundwork and open [#4349](https://github.com/isogeny-labs/crypto/pull/4349) exact Rust selected-root/FGLM infrastructure for target-conditioned symmetric S9/K–S5 extraction. Certification must start from the original unsaturated ideal, include signs/infinity and prove relation extraction **without scanning the 48,903,492 B31 multisets**. First B4/B8 positive, negative and naturally sampled targets, then B31 complete cold scalar recovery. Ordered m8 F4 and anchored-six F4 were censored; latter natural support has an upper bound **3.7407208%**, so do not rerun unchanged.
4. **Other competing IC leads:** [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) auxiliary-vs-random-base Rust timing preflight is blocked by compute/transport; correctness-only fixtures cannot establish an advantage. Vieta work [#4410](https://github.com/isogeny-labs/crypto/pull/4410) is separate; neither replaces the prime31 complete-index-calculus benchmark.
5. **Primary sources:** [PKM 2016](https://doi.org/10.1007/978-3-662-49387-8_1); [APS 2018](https://doi.org/10.1016/j.ffa.2018.01.009); [Wiener–Zuccherato](https://doi.org/10.1007/3-540-48892-8_15); [Bos–Kleinjung–Lenstra ANTS 2010](https://doi.org/10.1007/978-3-642-14518-6_9). Internal `research/rho_walk_leads_20261014/README.md` already documented fruitless cycles and the difference between iteration count and real runtime.

**Evidence gate:** producer CI and report-side calculation checks only; source-bound [pcc:unverified], not signed PCC, independent replay, isolated host, fastest external rho/PKM/APS/CAS, or a new non-generic complexity proof. Research-KB MCP, local Rust/Conductor and EKS transport unavailable in this execution. No paid compute, merge, relaxed gate, or leaderboard promotion.
