# Prime31 Run71 — 34.035% fewer genuine-RHS relation shifts from independent target-certificate differences

**2026-11-22 UTC · Result: original Rust PASS twice, exact-source SHA `0f75ec4572b99a119189a33508b494b512eed64a`, source blob `282af2f3163014f62ea436dd2587c7eba3268f45`.** Current prototype is still an intentionally **GENERIC pair-dictionary negative control**. No qualifying non-generic/index-calculus computational advantage or novel complexity result is asserted. Signed PCC unverified.

## What was demonstrated

For public, unknown-index target Q on the *unchanged ordinary cofactor-one* curve p=2147483647, subgroup n=2147446831, G=(625037715,2100627251), a=847514846, b=2087444724, each signed triple identity `Q+[j_r]G=Σ_i c_(r,i)P_i` yields the authentic labelled difference relation

`Σ_i (c_(r,i)-c_(0,i)) log_G(P_i) = j_r-j_0 (mod n)`.

It is computed from **public shifts and verified signed EC points alone**; no unknown target scalar or oracle witness index enters the relation. Duplicate, dependent and inconsistent rows are checked by exact modular RREF, not counted as independent by fiat.

From 32 fixed public targets with K=4 independently group-verified signed triple certificates each:

| Measured item | K4 readout-only control | K4 plus true-RHS difference rows |
| --- | ---: | ---: |
| Real public [k]G relation shifts | 2,528,933 | **1,668,210** |
| Genuine signed-pair RHS hits | 1,088 | **736** |
| Independent authentic pair DSU rows | 546 | **454** |
| Complete correct public target logs | 32/32 | **32/32** |
| Additional verified difference identities | 0 | **96** |

Reduction = **860,723 real shifts = 34.0350%** relative to exact matched K4 on the **same relation stream**. Relative to the previous K1 control at 5,378,231 shifts, the combined K4 + differences strategy uses 68.9822% fewer shifts, but incurs additional certificate discovery and algebra.

Initially the 88 verified homogeneous equations have exact rank 88. The 96 new group-verified difference rows increase it to **184** — independently contributing all 96 rows for this frozen public cohort. At the augmented stopping point, after the 454 true pair relations are specialized, rank of the free rows remains 88 and rank including differences is **178**, a surviving **90 independent extra constraints**. The projected base-log system still has nonzero nullity (677-454-178=45), but every target affine form is already determined. No full factor-base rank was presumed.

## Rust evidence and cost cautions

- Source: `research/prime31_ic_20261115/native/examples/prime31_run71_difference_rhs.rs`. It **includes Run69 original Rust source** and reuses exact `Arithmetic`, `Base`, true-RHS signed DSU, free-identity verification and quotient row-space logic. The 916,469-key pair dictionary is explicitly GENERIC and forbidden for non-generic performance claims. Public certificate leaf probes: **933,657** in both arms.
- [Original Rust Actions attempt 1](https://github.com/isogeny-labs/crypto/actions/runs/45071857479/attempts/1): **PASS 41/41 library**, 96 group proofs, 32/32 actual target logs, group law, secret-only post-solve and separate BSGS; generic setup 224.853 ms, witness discovery 186.773 ms, **joint two-arm** relation walk 636.634 ms, **joint including reference BSGS** 1,215.642 ms.
- [Original Rust Actions attempt 2](https://github.com/isogeny-labs/crypto/actions/runs/45071857479/attempts/2): **PASS 41/41**, identical ranks/queries/logs on another runner; setup 352.151 ms, certificates 297.095 ms, joint walk 927.554 ms, joint including reference BSGS 1,819.654 ms.
- Paired reference shared BSGS setup+32-target online: **163.989 ms** and **237.763 ms**, respectively. Both whole **joint experiments** include baseline-RREF overhead and BSGS verification, so their 7.41×/7.65× ratios must **not** be called cold difference-only solver costs.
- **New isolated cold-difference source** `9fef5061a562e3b1daf1eb1d96fdec5d69fbe42c`, original Rust source blob `eee78a17c02e789195c6777ece70452a5f5c3d45`, captures full cold K4+RHS elapsed *at the 32/32 verified stop*, **before** the control-arm RREF or BSGS reference. Its two source-identical passing Actions attempts ([attempt 1](https://github.com/isogeny-labs/crypto/actions/runs/45072185961/attempts/1), [attempt 2](https://github.com/isogeny-labs/crypto/actions/runs/45072185961/attempts/2)) are the qualified cost measurements below.

## Previous-lead and novelty audit

Run65 shift-correlation optimization was negligible; the authentic rank mechanism in Run69 saved 34.95% of Run68 shifts; Run70 K4 multi-readout saved 52.98% against K1 but stayed 7.2× slower than reference BSGS under its generic table. Run71 is an additional source of real known-RHS rank, **not** a new extractor for finding the three leaves.

No duplication of other active lanes: [#4692](https://github.com/isogeny-labs/crypto/pull/4692) owns the 46-positive/18-negative blind selector/RR9/F4 gate (checkpoint transport currently blocked); [#4822](https://github.com/isogeny-labs/crypto/pull/4822) owns sparse four-variable RR9/Krylov; [#4812](https://github.com/isogeny-labs/crypto/pull/4812) owns 14-variable Horner/F3 polynomial identities and root recovery; [#4809](https://github.com/isogeny-labs/crypto/pull/4809) owns small quadratic S3. New [#4919](https://github.com/isogeny-labs/crypto/pull/4919) is the B8 coefficient-extractor protocol; [#4921](https://github.com/isogeny-labs/crypto/pull/4921) is only an unmeasured symmetric S3/FASS proposal. APS one-relation prime-field method predates this work (Amadori, Pintore & Sala, 2018, https://doi.org/10.1016/j.ffa.2018.01.009); foundational polynomial approach is Semaev (2004, https://eprint.iacr.org/2004/031). Knowledge-base MCP unavailable; primary GitHub and bibliographic sources checked.

## Next decisive experiment

1. Freeze the new **cold K4 difference-only** original-Rust result with separate setup, 933,657 certificate probes, actual 1,668,210 authentic relation shifts, modular rank and target verification, **excluding BSGS and readout-only control work**.
2. Repeat on independent public seeds and T=1/32/128/1024, K=1/4/16/32; sweep memory, exact rank, success and cost, including censored cases and amortization.
3. Replace both generic pair/triple enumeration stages with a genuine public-only **nonenumerative algebraic** RR9/Horner/F3 oracle from #4692/#4822/#4812. Require the frozen blind positive/negative corpus, unknown-index witness extraction, all failed attempts, authentic RHS and verified complete logs.
4. Compare same-host fully charged index calculus frontier, optimized shared BSGS and negation-map rho. A lower query count or faster component **alone** does not meet the user's complete cheaper solver criterion.

The PR stays **draft, PCC unverified** pending blind polynomial extraction and full-cost win.


## New fully charged cold K4 difference-only control — original Rust PASS twice

Both source-identical GitHub Actions executions passed **41/41 original Rust library tests**, independently checked **96/96 public signed group difference identities**, reproduced the **same exact 1,668,210 / 736 / 454** query/hit/pair-row stop and **all 32/32 actual public target logs**. The timer is recorded *before* optional baseline rank and before the independent reference BSGS, so unlike the prior joint two-arm figures this is a real cold difference-only cost accounting gate.

| Matched original Rust (lower is better) | [CI attempt 1](https://github.com/isogeny-labs/crypto/actions/runs/45072185961/attempts/1) | [CI attempt 2](https://github.com/isogeny-labs/crypto/actions/runs/45072185961/attempts/2) |
|---|---:|---:|
| Generic B677 / 916,469-key setup | 288.887 ms | 355.968 ms |
| Four verified certificates per target | 247.014 ms | 298.757 ms |
| Authentic RHS collection, modular rank and verification | 534.314 ms | 614.763 ms |
| **Cold K4 true-RHS difference solver** | **1,074.583 ms** | **1,274.441 ms** |
| Matched same-process shared BSGS including setup | 200.234 ms | 236.674 ms |
| **Cold difference solver / BSGS** | **5.367× slower** | **5.384× slower** |

The two runs use the same deterministic public-target cohort on different GitHub Actions runners: the identical exact row rank and shift counts are reproducible, but the timings are not independent statistical samples. This is still a complete, genuine-RHS, **generic-table** solver, not a qualifying polynomial non-generic extractor. The best *prior* K4 whole-time metric labelled 7.2× slower included the BSGS reference in its numerator; the [Run70B erratum](RUN70B_K4_COLD_RESULT.md#cost-accounting-erratum-run71-source-audit-2026-11-22) corrects the source-structure estimate to ~6.20–6.25× slower, not a directly timed cold stop. Ratios from different workflows/runners must not be used as controlled solver speedups.

The strongest next lever is public-only algebraic extraction (#4692/#4812/#4822), using these 90 extra surviving RHS constraints. Next experiment should jointly track marginal row rank per additional root, negative extraction attempts and cold memory/time, with the generic dictionary removed from BOTH relation and target-certificate discovery.
