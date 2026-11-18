# EXP-R31-K: exact natural four-summand coverage and an arity stop/go gate

**Date:** 2026-11-15. **Status:** producer-local exact finite-group diagnostic, **[pcc:unverified]**. This is a **generic enumeration NEGATIVE CONTROL**; it neither extracts algebraic relations faster than generic enumeration nor recovers a discrete logarithm. Checked into the existing [prime31 IC PR #4188](https://github.com/isogeny-labs/crypto/pull/4188).

## Previous leads and external primary context (reviewed before this gate)

- Our previous [four-summand quotient](M4_PAIR_SUPPORT_RESULTANT_20261115.md) reuses Rust S3 field polynomials and has a measured **106.916 ms** target-dependent algebraic norm/GCD stage at B16 on one *planted* target, with severe B8-to-B16 scaling. The 0.052 ms signed validation phase is not the bottleneck. No complete natural m4 rank or cold ECDLP success is established there.
- [S3 discriminant classification](S3_DISCRIMINANT_CLASSIFICATION_GATE_20261115.md) rules out treating already-known oriented point additions as a non-generic discovery speedup; [y-coordinate support gate](XY_COORDINATE_PROJECTION_PILOT_20261115.md) found the B31 depth-six base has 14 unique-x y-fibres and 17 three-x y-fibres, while a depth-seven selected B31 y-injective base is available.
- #4228 is the open certified quotient/subresultant experiment; #4275 is the closed dense F4 comparison; #4077 is a closed prime-field whole-call gate; #4311 explores one-relation and coordinate-join models; #3877 covers transfers. The cryptanalysis auxiliary-curve and crypto-autoresearcher lead-atlas threads are not completed prime31 cold non-generic solver results. Do not duplicate their native code.
- [Petit–Kosters–Messeng, PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1) is foundational prior prime-field algebraic index calculus; [Amadori–Pintore–Sala, Finite Fields and Their Applications 51 (2018)](https://doi.org/10.1016/j.ffa.2018.01.009) proposes a one-factor-base-relation variant. They prohibit framing this as a new prime-field index-calculus algorithm. Research-KB MCP / repository wrapper, Conductor, local cargo toolchain and EKS were unavailable in this current connector+Python runtime; the bounded fallback was live GitHub PR and primary repo file reads plus publisher paper records. No exhaustive novelty audit is claimed.

## Exact mathematics: a representation-independent natural-success ceiling

Let `F={P_1,...,P_B}` be the registered factor base. Allow at most four summands, with each signed and repeated points allowed. Any valid target has the form `R=sum_i c_i P_i` with integral coefficients satisfying `||c||_1 <= 4` and `||c||_1 == 4 (mod 2)`; padding with cancelling `+P-P` pairs covers all coefficient vectors with the requisite parity. The number of **distinct coefficient vectors** is exactly

    M4(B) = 1 + 4B + 16*binom(B,2) + 24*binom(B,3) + 16*binom(B,4).

The corresponding **distinct point support** has size at most `M4(B)` regardless of coordinate choices, solver, F4, resultant implementation, or arithmetic tricks. For a public query `R=aG+bQ` with uniform independent `a in Z/n`, the target is uniform in the prime-order group, hence

    Pr(a signed <=4 point decomposition) <= min(1, M4(B)/n).

This is a *hit-probability ceiling*, not a computational lower bound for arbitrary ECDLP or higher-arity IC; the actual factor base can have collisions making the success rate smaller.

The analogous combinatorial ceilings from sums with lengths ≤2,4,6,8 and the appropriate parity for B=31 are:

| Max parity-compatible arity | Coefficient vectors M_m(31) | M_m/n | Interpretation |
| ---: | ---: | ---: | --- |
| 2 | 1,923 | 8.95482e-7 | Upper bound, exact group support confirmed by previous gate |
| **4** | **618,885** | **0.000288196** | Exact coverage measured on canonical B31 |
| 6 | 80,329,991 | 0.0374072 | **Upper bound only**, not an m6 measured yield |
| 8 | 5,654,193,417 | 2.633 | Vacuous as a group-support bound; actual m8 coverage unmeasured |

The combinatorial count at even arity m is `M_m(B)=sum_{j=0,2,...,m} sum_{k=1}^{min(j,B)} binom(B,k)*binom(j-1,k-1)*2^k`, taking the j=0 term as one. This count is algebraically exact for coefficient vectors, not an assertion that they all map injectively into E(Fp).

## Executed on exactly the requested ordinary cofactor-one prime31 curve

`p=2147483647, a=847514846, b=2087444724, n=2147446831`. The producer-local Python program reconstructs the same `Base::build(depth=6, seed=3)` fixture and counts the full group support with explicit additions only **as a diagnostic**. It computes the signed two-sum set (including identity), then its unordered two-sum pair set to give the complete signed four-sum support including all repetitions and cancellations.

| Base (same canonical depth-six fibre) | Signed S2 including O | Signed S4 including O | Uniform hit probability | Expected queries/hit |
| --- | ---: | ---: | ---: | ---: |
| B4 | 33 | 225 | 1.04776e-7 | 9,544,208 |
| B8 | 129 | 2,945 | 1.37140e-6 | 729,184 |
| B16 | 513 | 44,545 | 2.07432e-5 | 48,208 |
| **B31** | **1,923** | **618,885** | **0.000288196** | **3,469.86** |
| Depth-seven **unique-y B31** | **1,923** | **618,625** | **0.000288075** | **3,471.32** |

The B31 canonical support attains all `M4(31)` coefficient-vector values with no collisions. Exact replay enumerated 1,848,003 nonidentity two-support pair additions plus 1,953 initial signed-base pair additions. Group additions and timing in this census are **not an index-calculus oracle benchmark**. Full support-set SHA256 fingerprints are frozen in [JSON](M4_EXACT_NATURAL_YIELD_20261115.json), and the source-bound replay is [Python](prime31_m4_natural_yield_gate.py).

For **injective-y B31** there are exactly **260 fewer distinct four-sum targets** than the combinatorial maximum, so selecting that base **does not improve raw natural m4 success rate**. We enumerated collision witnesses and found four distinct eight-summand signed zero relations with **rank four over Z/n** (all directly verified by independent point addition; coefficients are in the frozen JSON). The exact statement is only that these four rows are independent. They were discovered by generic enumeration; no non-generic search or asymptotic advantage follows. A random-scalar 31-point control yielded the maximal 618,885 in one separate producer run; one seed does not establish that the four collisions arise from special algebraic structure.

**Consequences:** For this specific canonical B31 factor base and uniformly randomized public-target queries, even a hypothetical zero-cost perfect natural m4 oracle needs on average **3,469.86 queries per positive**. Approximately 32 useful positives would require **about 111,036 queries in expectation even if every positive were independent**, and an actual full-rank requirement may be higher. This is a success-frequency calculation, not a rigorously measured runtime lower bound. It decisively motivates arity ≥6, and particularly an efficient *algebraic* m8 target-dependent oracle, over further micro-optimizing signed m4 verifier calls.

## Reused native Rust and evidence

New [Rust example](examples/prime31_m4_natural_yield_gate.rs) reuses `check_curve`, `Arithmetic`, `Base` and `PRIME31`; it verifies all five support counts, the combinatorial bound, the 4 frozen relations and their independent rank. [Separate Python program](prime31_m4_natural_yield_gate.py) uses the existing independent `y_fiber_probe.py` curve arithmetic and recomputes SHA256 support fingerprints.

Producer-local Python complete execution and direct witness verification took place before check-in. **No local cargo/rustc executable or full repository checkout was available, so the Rust source has been committed but has *not* been compiled or executed in this run.** No GitHub Actions run or independent signed PCC receipt has been observed for these new commits; do not claim green CI or add a leaderboard row. This source is a correctness/control test, not a claimed speed result.

## New pre-registered experiments (ranked)

1. **M8 natural-target quotient gate — main priority.** Implement a balanced target-dependent `S4` / `S3` polynomial elimination using the existing Rust quotient/subresultant engine from #4228. Do not substitute generic S4 group-sum tables for the algebraic oracle. Use the canonical B31 and the filtered depth-seven y-injective B31, with identical public `R=aG+bQ` inputs, frozen seeds and known-answer scalars inaccessible to the discovery stage. Price target-dependent algebraic work, root extraction, exception branches, preprocessing and all failed/censored trials.
2. **M6 intermediate viability test.** Its 3.7407% ceiling gives at least 26.73 uniform queries/positive even with perfect success. Compute *actual* coverage only as a generic negative control and benchmark natural Semaev S4/S5 algebraic extraction in Rust; use a sample-aware/censored cost cap before large enumeration.
3. **Certify oriented y-membership cost.** Retain x for Dickson low-degree tower, use y/slope selectively at target-dependent joins; compare certified membership, matrix dimensions, cubic lift branch counts, and signed witness multiplicity. The y-injective subset cannot be assumed to retain the original tower's low-degree equations without an exact certified filter. Confirm no false factor-base assignments.
4. **Try algebraic derivation of the four injective-y short relations.** Search the Dickson depth-seven tower and quotient polynomials for an expression that discovers these independent relations *without* generic group enumeration; compare against a random 31-point control and count full cost. Do **not** infer a special curve weakness just because four birthday-style collisions exist.
5. **Full cold solver gate.** Accumulate genuinely independent relation rank, run sparse LA, solve at least 12 held-out unknown prime31 scalars, verify `[k]G=Q`, and charge every unsuccessful query plus setup. Compare to same-host optimized negation-map Rust rho and BSGS, including 0/0.1/1 GAE opportunity cost and independent PCC replay. Test other ordinary prime-order curves before a general complexity claim.

**Decision:** Stop treating m4 sign-lift optimization or unqualified y-only reparameterization as the decisive speed lead. The algebraic target-dependent elimination bottleneck and arity-versus-density tradeoff are the next research priorities. No cheaper complete non-generic prime-field ECDLP solver was measured in this gate.
