# EXP-R31-ANCHOR2 — completed Rust prime31 F4 and exact support audit (2026-11-16)

**Outcome: verified execution of a negative algebraic lead**, exact-source GitHub Actions development CI, **[pcc:unverified]**. The target is the same ordinary full-prime-order curve over p=2^31−1, not a weak-order substitute. There is **no new complete non-generic ECDLP solver or speedup** from this test.

## Frozen inputs, code and execution evidence

- `p=2147483647`; `E:y²=x³+847514846x+2087444724`; `#E=n=2147446831` prime, cofactor one; G=(625037715,2100627251).
- Use registered B31 depth-six Dickson factor base, `P[0],P[1]` fixed as positive anchors, and all 16 original natural mixed-mask public target points `R=[a]G+[b]Q` without changing labels/seeds. No known log enters F4; positive-control full point is constructed separately.
- [Source-pinned Rust F4 CI #43809984651](https://github.com/isogeny-labs/crypto/actions/runs/43809984651), job 133109279191, tested head `28f38c1390c8a83bc9559cc63d037c4ad3b0ca44`, **PASS**, with [original full JSON](ANCHOR2_M6_F4_CI_EVIDENCE_20261116.json).
- [Source-pinned independent Rust support-control CI #43810252441](https://github.com/isogeny-labs/crypto/actions/runs/43810252441), job 133110113485, tested head `21d9d45c24cea860cdb12ef9b9a9f7ebd863619e`, **PASS**, with [original full JSON](ANCHOR2_M6_SIGNED_SUPPORT_CI_EVIDENCE_20261116.json). This support-control is **generic exhaustive MITM**, used only as an independently constructed ground-truth diagnostic and NEVER as algebraic relation discovery.

## Algebraic anchor2 experiment: six residual leaves instead of eight

Exact signed equation: `R=A+B+sum_{j=0..5} eps_j P_ij` iff `R'=R-A-B=sum_{j=0..5} eps_j P_ij`, with both A and B fixed public factor-base points.

The reused native Rust `six_ring`, `six_input`, `s4`, `s3` and `f4_tower` represent six depth-six fibres plus U,V in 38 tower/free variables, with three polynomials:

    S4(x0,x1,x2,U), S4(x3,x4,x5,V), S3(U,V,x(R')).

One **independently checked positive six-leaf witness** `P[2]+...+P[7]` was evaluated against all three input equations and every partial F4 basis element. This checks soundness but does not supply F4 with a known root.

| Bounded Rust F4, 15,000 reducer-entry cap | Calls | Outcome | Degree reached | Max cols | Max NNZ | Max dense B' | Partial muladds |
|---|---:|---|---:|---:|---:|---:|---:|
| Frozen natural anchored R' | 16 | **16/16 oversize / undetermined** | 10 | 2,577 | 10,372 | 10,229 | 56,211 per call |
| Independent known-positive six-leaf R' | 1 | **oversize / undetermined** | 10 | 2,577 | 10,372 | 10,229 | 56,211 |

All 16 natural calls have the same structural counters and **zero completed rational-root extractions**, just like the valid positive control. The total engine-internal F4 reported time for 16 was 181.576005 ms (≈11.35 ms per call), **not** completed oracle time, complete scalar-solving time, or a valid speed ratio. The two fixed S4 triples still dominate work at this cap; removing the m8 pair S3 and two leaves does not solve them. It is unsafe to call the censored 16 ideal systems unsatisfiable.

## Crucial independent negative finding: anchored m6 has sparse actual support

The second Rust program built a complete table for the **62³=238,328 ordered signed triple-sum sequences**, retaining infinity, repetitions, cancellations and all 31 signed base points. There were **39,804 distinct triple-sum group elements**, consistent with the reduced canonical three-summand combinatorics. On each frozen target it tested all 238,328 complementary signed triples, counting exact full-point ordered six-summand solutions. This **is intentionally generic exhaustive enumeration and never feeds the algebraic F4 oracle**.

| Independent full-point diagnostic | Exact measured |
|---|---:|
| Natural frozen R' with ≥1 anchored signed m6 witness | **0/16** |
| Total natural signed m6 ordered witness sequences | **0** |
| Separately constructed positive six-leaf control | **720** ordered signed witnesses |
| Distinct signed three-sum points (from 238,328 ordered sequences) | **39,804** |
| End-to-end generic validator wall (including table construction) | 2.062599151 s |

This result is a **real absence of the fixed-anchor six-summand group representations** for those 16 registered public R', not an assertion about other anchors or the full m8 ideal. The 720 positive-control representations come from permutations of six distinct points; they are not 720 different algebraic relations.

### Correcting the preregistered ordered-count intuition

The protocol's naive mean `(62)^6/n=26.450124` is the exact group-average **ordered representation multiplicity** under uniform targets, but it is **not** a measure of natural-target coverage. The abelian group operation makes the six summands commute; signs and P/−P cancellation create even more duplicates. A rigorous upper bound on **distinct six-sum targets** is the number of canonical reduced signed multisets of length 0,2,4,6:

    C_t = sum_{d=1}^t binom(31,d) binom(t-1,d-1) 2^d,  C_0=1.
    |six-sum support| <= C_0+C_2+C_4+C_6
                      = 1+1,922+616,962+79,711,106
                      = 80,329,991.
    max uniform target coverage <= 80,329,991/2,147,446,831
                                = 3.7407208%.

Each signed multiset can be reduced by removing P/−P pairs; after cancellation it has parity-consistent length 0,2,4 or 6. Each canonical reduced multiset contributes at most one group element. Thus the bound applies to any abelian group and *this exact 31-point base*, without heuristic independence assumptions. The cruder unordered-multiset bound `binom(67,6)/n≈4.647%` is weaker.

On 16 *independent uniformly random* group targets the probability of seeing **no** fixed-anchor witness would be **at least 0.54335**, even with maximal support. The registered public masks are deterministic, so this is a sampling interpretation, not an exact probability statement about this panel. The observed 0/16 should **not** be treated as anomalous. This closes the misleading ordered-tuple yield argument for trying arbitrary single fixed two-anchor m6 targets.

A single fixed anchor leaves seven unknown signed summands; the analogous canonical coverage **ceiling** is `719,208,184/n≈33.4913%` (not a guarantee). Unanchored eight-sum has no useful sub-100% ceiling from this count. This makes aggressive anchoring a poor route to a high-natural-yield IC oracle despite a smaller F4 ring.

## Prior-work and frontier disposition

- **Stop this configuration:** naive fixed-2-point m6 F4 at registered caps. The known-positive and all 16 natural calls are censored, and the actual frozen uniform-style targets have no anchored six-sum witnesses.
- **Do not retry generic-root-index reductions as novel IC:** #4374/#4391 APS S3 pair scanning already completes unknown prime31 logs faster as Rust engineering but is generic meet-in-the-middle equivalent. #4383's apparent T1024 gain was falsified by fresh confirmation. #4188's factorized S4 microstage improvement does not cure B31 four-sum support sparsity. M8 chained S3 #4373 and oriented discriminant/F4 reports already tested straightforward rewrites.
- **Advance the genuinely different target-dependent mechanism:** on original unanchored eight-sum, try block-symmetric quotient coordinates (old `M8_ORDERED_ROOT_SYMMETRY_GATE.md` already finds 144-fold ordered ideal symmetry), certified `K/S5` elimination and a first-root/subresultant strategy using #4228/#4349. Require exact root equivalence and signed y/repeated/infinity branches, not a group-hash fallback.
- **Next admission:** keep same 16 frozen public targets plus fresh holdouts, build matched *completed* IC algebraic oracle and one-relation PKM/APS reference calls with real original-ideal certificates, rank or one-relation reconstruction, full cold 12-log verification and complete cost. Later compare optimized rho as a separate security baseline. Censored internal F4 operations are not timing wins.
- Relevant publication prior art: [Petit–Kosters–Messeng PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1) and [Amadori–Pintore–Sala FFA 2018](https://doi.org/10.1016/j.ffa.2018.01.009). Sources consulted also include repo #4188/#4373/#4374/#4383/#4391/#4228/#4349 and prior symmetry/discriminant reports. **KB unavailable:** MCP not loaded and local repo wrapper not executable in connector-only runtime; this is a bounded prior-work audit, not an exhaustive novelty clearance.

**Validation boundary:** frozen development CI PASS for source-specific Rust program execution and original-source evidence; not independent EKS/PCC, not an external mature CAS comparison, no new target scalar recovered by m6, no new claimed security break, and no leaderboard promotion or policy alteration.
