# Post-Jacobian unequal APS prime31 Rust profile — source-exact fresh confirmation of incumbent (2026-11-16)

**Disposition: negative tuning result.** The prior `[384,384,16384]` incumbent remains fastest in this prospective, source-pinned 32-public-Q panel. The first source-specific CI finished **PASS** on 2026-11-16, with 160/160 complete, independently verified logs. The preregistered exploratory candidate-speed gate **FAILS**, because no challenger beats the incumbent at full cold cost. Scientific standing `[pcc:unverified]`: developer Rust Actions is not independent PCC or a mature external solver comparison.

- Precommitted protocol: [APS_POST_JAC_PROFILE_PROTOCOL_20261116.md](APS_POST_JAC_PROFILE_PROTOCOL_20261116.md), commit `0f3a85cd8357129154662c4563e2bb3143a989de`.
- Executed Rust 1.100 locked release source: commit `d2c31e6a95dcfa0ec8de828aab661e98284d8b19`, [GitHub Actions run 43858844508](https://github.com/isogeny-labs/crypto/actions/runs/43858844508), job `133270049272`, **PASS**. The existing `prime31-s4-split-20261116.yml` ran focused Rust APS example tests, all retained earlier Rust experiments and this new CLI step successfully. No paid compute.
- [Actual extracted JSON stdout](APS_POST_JAC_PROFILE_CI_20261116.json) retains **32 target records × 5 arms**, all public Q, known scalars *only for caller verification*, original signed relation equations, all round transcripts, failures, setup work, S3 pair counts, call order and complete cold time. Extraction strips GitHub log timestamps and an intermittent U+FEFF log marker; the program JSON values are not rewritten. No dropped or resampled targets.
- Source is [native/examples/prime31_aps_one_relation.rs](examples/prime31_aps_one_relation.rs), `--post-jac-profile`. The only changed component is partition sizes around the current exact-source Jacobian+x-fibre solver, not a new algorithm.

## 32 fresh public prime31 targets, all fully cold

```
p = 2147483647 = 2^31-1
E: y^2 = x^3 + 847514846*x + 2087444724 (mod p)
#E(Fp) = 2147446831 prime; cofactor 1
G = (625037715,2100627251)
```

All arms regenerate their own public sources, charge setup, all failed rounds, S3 root trials, candidate verification and full verified scalar recovery. Five cyclic arm orders across the 32 *new* Q; no known scalar enters the solver. These are **sums of the charged in-process cold durations**, not external process or compiler startup time.

| Arm | Three public labelled-list sizes | Verified scalars | Cold total, ms | Paid source, ms | S3 pair quadratics | Per-Q wins vs incumbent |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| A | 256 / 256 / 32768 | 32/32 | 306.802452 | 112.470648 | 557,568 | 2/32 |
| B | 320 / 320 / 24576 | 32/32 | 252.978062 | 102.566974 | 725,760 | 15/32 |
| **C — incumbent** | **384 / 384 / 16384** | **32/32** | **226.238457** | **87.365618** | **1,088,256** | — |
| D | 512 / 512 / 8192 | 32/32 | 287.947830 | 75.963672 | 2,416,128 | 15/32 |
| C′ — incumbent repeat | 384 / 384 / 16384 | 32/32 | 234.468467 | 86.862467 | 1,088,256 | — |

The C/C′ mean is **230.353462 ms** and their aggregate difference is **3.64%** relative to the initial C, within the prespecified 5% A/A gate. The best challenger is B, **1.0982× the C/C′ mean cost** (9.82% slower), not the 0.90× or below needed for the claimed 10% tuning gain. C alone is 226.238 ms, while B is 252.978 ms (11.82% slower). The preregistered development improvement gate is therefore **NOT met** even though all 160 unknown scalar solves passed.

Unselected negative arms and all attempts are retained. A naive S3-pair-count proxy would have incorrectly preferred A or B: A uses 49% as many pair quadratics as C but is 36% slower than C alone. Larger third partitions raise paid public-source setup, index construction and x-fibre precomputation, and change time-to-first-hit in ways not captured by raw pair counts. The experiment **measures** total costs; attributing exact field versus hash/precompute shares would require separate profiling, not inference from these aggregates.

A deterministic 10,000-resample paired-target bootstrap of each candidate's ratio against the within-target average C/C′ yielded exploratory 95% percentile intervals: **A 1.163–1.503**, **B 0.887–1.376**, **D 0.988–1.539**. These intervals are descriptive only; no isolation, host randomization, external audit or signed PCC is claimed. B and D intervals straddle 1 despite losing in summed wall time. Incumbent C has one multi-round target; B has one, D two, A none; all are charged.

## Prior-lead disposition (reviewed first)

- [#4391](https://github.com/isogeny-labs/crypto/pull/4391) verified full Rust APS improvements: walk-labelled sources, Mersenne arithmetic, asymmetric lists, x-fibre precompute and Jacobian batch normalization. Those are *engineering* gains, not proof of non-generic exponent improvement. The current finite-list S3 pair oracle is **generic meet-in-the-middle equivalent**.
- [#4383](https://github.com/isogeny-labs/crypto/pull/4383) 12-target T1024 exploratory apparent gain was invalidated by its own fresh 24-target confirmation, reinforcing the need for prospectively separated panels. This post-Jac panel independently finds that an aggressive unequal profile change is *not* a win.
- [#4373](https://github.com/isogeny-labs/crypto/pull/4373) original S4/three chained S3 F4 m8 arms were **resource-censored**, not unsatisfiable or solved. The separate two-anchor m6 formulation has only a 3.74% algebraic coverage upper bound on the 31-point base and failed on 16 natural targets; don't repeat those forms without a different mechanism.
- [#4228](https://github.com/isogeny-labs/crypto/pull/4228) / [#4349](https://github.com/isogeny-labs/crypto/pull/4349) provide exact quotient/certificate infrastructure and selected-root optimizations, but their tested small-system speed ratios cannot be transposed to B31 m8. The exhaustive B8 symmetric gate demonstrates correctness of **checking a supplied f**, not efficient *finding* f from Q.
- Publication baseline: Christophe Petit, Michiel Kosters, Ange Messeng, *Algebraic Approaches for ECDLP over Prime Fields*, PKC 2016, DOI `10.1007/978-3-662-49387-8_1`; Alessandro Amadori, Federico Pintore, Massimiliano Sala, *On the DLP for Prime-Field Elliptic Curves*, FFA 51 (2018), DOI `10.1016/j.ffa.2018.01.009`, [preprint](https://eprint.iacr.org/2017/609). APS explicitly uses one target-labelled relation and partitioned factor-base polynomial constraints. Its published complexity analysis does **not** prove a subgeneric algorithm. The new work claims no method novelty. Shared KB tool is unavailable here; checked committed source, prior leads and primary paper, not a complete bibliography survey.

## Decision and next experiment

**Keep [384,384,16384] as the reference APS configuration. Do not launch a fresh confirmation of a challenger that failed this prospective 32-Q screen, and do not promote a leaderboard/scientific claim from this result.**

The highest-value unresolved scientific experiment is **target-conditioned, original-ideal-certified symmetric m8 root extraction** on the same ordinary curve, with nonenumerative selection of `f(T)` satisfying `f | H^8` and the S9 group relation, not a scan of `binom(38,8)=48,903,492` possible B31 multisets. Concrete gates:

1. Build a B4/B8 native Rust original-ideal quotient interface using existing #4228/#4349 modular polynomial and certificate machinery. Return rational roots from **public Q and public base only**, with proof obligations covering multiplicities, sign choices, infinity/cancellation and repeated x; no known witness passed to the oracle. Test both true natural positives and negatives against the independent existing B8 S3/EC validator *after* extraction. Reject any root that fails the original equation or full point replay.
2. Prototype block-Vieta 3+3+2 elimination or selected-root subresultant sequences on B4 and B8, recording total root-set completeness/certificates and all caps. Treat F4 over-budget as **undetermined**, never negative.
3. Scale to B31 with 24 novel natural Q, measure useful signed rows, rank and (if available) complete unknown target logarithms. Charge all factor-base, quotient, root extraction, failed queries, certificate validation and linear algebra.
4. Pair **completed equal-output** algebraic relation oracles against the best available PKM/APS, native F4 and mature CAS controls on a shared runner. Only after full non-generic solver completion compare against optimized negation-map rho. Independent PCC and ordinary-curve replication required for any security/comparative frontier assertion.

This run establishes a useful negative benchmark and rejects a misleading raw pair-work proxy; it does **not** establish a genuinely non-generic or faster-than-rho algorithm.


## Exact-source automatic replay on separate CI run — same negative disposition

After publishing the first raw receipt and report, the unchanged Rust source was run again by the same focused workflow on the documentation/evidence head: [Actions run **43859993988**](https://github.com/isogeny-labs/crypto/actions/runs/43859993988), job `133273990689`, **PASS**. All 160/160 fresh-panel public scalars were again recovered and independently checked. Same 32 Q, same five program arms and same recorded S3 pair counts — **not** 32 new held-out targets and **not** independent verifier/PCC. [All replay records](APS_POST_JAC_PROFILE_REPLAY_CI_20261116.json) are frozen separately, preserving both timing observations.

| Complete 32-Q cold Rust duration | First source-exact run, ms | Later repeat on same source, ms |
| --- | ---: | ---: |
| A [256,256,32768] | 306.802452 | 258.882749 |
| B [320,320,24576] | 252.978062 | 236.378185 |
| **C [384,384,16384]** | **226.238457** | **220.701207** |
| D [512,512,8192] | 287.947830 | 279.037128 |
| C repeat A/A | 234.468467 | 220.349463 |

On this second run, the best challenger B is **1.07189× the C/C′ mean** (7.19% more cold time), with **14/32** direct wins versus C; A/C′ drift is **0.159%**. All 160 complete solves passed and the preregistered candidate improvement gate failed again. Pair queries are identically `557568/725760/1088256/2416128/1088256` across the five arms in both executions.

**Decision unchanged:** retain C; no challenger confirmation or speedup/leaderboard promotion. A repeat on the same Q/source helps falsify a fragile one-off timing winner but does not replace a fresh independent signed PCC campaign.
