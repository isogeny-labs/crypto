# Prime31 signed W8 B4/B8: exact 4+4 grading oracle and relation multiplicity census

**2026-11-18 UTC** · existing [RR9 weighted-slot lane PR #4728](https://github.com/isogeny-labs/crypto/pull/4728) · [frozen experiment](W8_B4_B8_SIGNED_ORACLE_PROTOCOL_20261118.md) · [native Rust original arithmetic](../native/examples/prime31_w8_signed_oracle.rs) · [independent BigInt verifier](../native/verify_prime31_w8_signed_oracle.py).

**Scope:** an independent *generic* enumerator used only as an exact truth oracle for evaluating the proposed target-conditioned RR9 polynomial extractor. This is not the extractor and not a cheaper ECDLP solver. The original prime-order, ordinary, cofactor-one Prime31 curve is unchanged:
`p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`. No weak/adversarial curve substituted. All verification and cost claims remain `[pcc:unverified]` pending independently signed validation.

## New exact finite support check

For eight signed public weights `1,3,5,7,11,13,17,19`, split four weights per half. Each slot contains positive and negative copies of the original first-`B` rational-x base. A half has `(2B)^4` **ordered** signed words; preserve multiplicities rather than parity-reducing them.

| Original base | Four-slot words / half | Distinct formal coefficient vectors / half | Distinct EC sums / half | Excess words |
|---|---:|---:|---:|---:|
| B4 | 4,096 | **3,921** | **3,921** | 175 |
| B8 | 65,536 | **64,737** | **64,737** | 799 |

These exact counts were independently produced with Python big-integer group arithmetic before the new Rust CI check. In both halves, every observed EC collision is already forced by equality of formal signed coefficient vectors; there were **zero additional EC-only collisions**. The new original-crate Rust code contains assertions reproducing both formal and EC cardinalities.

**Exact first moment, not a Poisson prediction.** If `C_B(R)` counts *all* signed eight-slot words summing to group element `R`, then

`sum_{R in E(F_p)} C_B(R) = (2B)^8`.

Thus the exact mean over all `n` elements is `(2B)^8/n`: B4 `0.00781263394...`; B8 `2.00003428909...`. This group-wide arithmetic identity is unconditional; it does not imply independent words, Poisson-distributed multiplicities or any runtime advantage. The prospective 64-point panel is sampled uniformly from *nonidentity* group elements, so this group-wide mean is a nearby reference rather than literally its exact nonidentity expectation.

## Frozen 64-target pilot, separate Python BigInt computation

After committing the new protocol, an independent Python implementation executed its **fresh specified 64 public residuals per base**, with the exact seeded rejection-sampling rule. This is a *local source-independent pilot*; CI compilation/replay of the new Rust source is a separate gate, and no Rust runtime value is claimed here.

| Metric | B4 signed W8 | B8 signed W8 |
|---|---:|---:|
| Public targets tested | 64 | 64 |
| Exact positive-relation targets | **0** | **46** |
| Exact zero-relation targets | **64** | **18** |
| Total signed witnesses, multiplicities counted | **0** | **115** |
| Mean witnesses / public residual | 0.000 | **1.796875** |
| Exact group-wide first-moment reference | 0.007813 | 2.000034 |
| Maximum witnesses for one public residual | 0 | **7** |
| Planted target multiplicities (8 targets) | 38, 5, 38, 5, 38, 5, 38, 5 | 3, 9, 7, 10, 2, 6, 3, 7 |

B8 observed yield of **46/64 = 71.875%** is a panel diagnostic, not a statistical confidence claim for the entire group. In particular, *planted* B4 targets are highly multiply represented while **all 64 natural targets had no witness**: planted-chart positives cannot stand in for natural relation-yield evidence. B8's 18/64 misses must not be silently converted to hits by root/resultant or parity assumptions.

The new source-specific Rust runner measures actual field/group operations and timing of building the generic 4+4 oracle; the experiment is intentionally an oracle for grading, not an allowed component of a competitive RR9 extractor. The independent checker re-derives all half supports, all 64 public target points (from the frozen PRNG), exact counts for the first 16, all eight planted target counts, and their emitted witness group replay per B. It will reject mutated witnesses and counts. A full 64-count transcript must be read from the Rust JSON artifact when CI finishes. Until that workflow reports success, its metrics and compiled Rust result are **not** verified.

## Leads audited and ranked actions

- **#4728 weighted RR9**: promising *equation encoding*, mathematically sound only for disjoint signed slot x sets, but no solver for unknown seven-chart coefficients. The new B4/B8 truth oracle provides correct zero/nonzero/multiplicity labels. Recommended next: solve the factorized resultant system on the frozen 64 B8 targets **without reading four-sum tables**, record every no-solution and timeout, compare to the grader.
- **#4692 algebraic RR9/F4 and #4717 low-power strata**: reuse rather than fork. At B8, dense polynomial expansion of eight products of 31 quadratics is inappropriate; compare low-degree auxiliary incidence equations against factored/F4 with measured memory, ranks, saturation/exceptional-root handling. For B4, check planted positives and explicit negatives before moving to B8.
- **#4714 M6 complete Rust control / #4722 exact bounds**: existing complete 1,024-Q logs and exact collision ceilings are valuable controls, but 3+3 enumeration remained substantially more expensive than matched shared BSGS; do not label the generic grading oracle non-generic.
- **#4720 signed two-leaf projector**: reuse for certified finishing leaves and root-sign checks; no independent B31 solve result.
- **#4698 theoretical subexponential and #4746 round-7 correction**: do not treat any asymptotic speed exponent as established until the revised hypotheses and full cost model are independently proved and compiled. #4746 explicitly retracts an earlier 0.43 exponent in favor of a losing 0.621 estimate for one setting.

Primary prior art checked: [Amadori–Pintore–Sala (2018)](https://doi.org/10.1016/j.ffa.2018.01.009), Semaev summation-polynomial methods, and [Bochum's October 2026 conditional subexponential proposal](https://github.com/bochum-crypto/subexponential-ecdl). None certifies a practical cheaper 31-bit target-conditioned RR9 extractor. KB MCP and repository wrapper were unavailable in this runtime; GitHub PRs and primary literature were used. No novelty priority claim.

### Gate for any actual solver advantage

The candidate must take **unseen public residuals** and produce valid eight-slot signed relations without the grader's 4+4 tables, fixture scalars, or planted chart coefficients. B31 must attain rank 31 and solve **1,024 blinded ECDLP targets** through the unchanged native Rust pipeline. On the same machine and identical targets, include setup, unsuccessful target-conditioned solves, F4/rootfinding, relation learning, sparse/dense LA, final extraction and verification, plus observed memory, against best mixed-IC, cold/cached shared BSGS and optimized rho. Report both single-target and amortized 1,024-target costs. Exact-source and signed-independent validation are distinct gates.

**Standing result:** mathematically exact B4/B8 truth labels and a reproducible new native-Rust grading experiment are checked in. No lower complete ECDLP solving cost is claimed.
