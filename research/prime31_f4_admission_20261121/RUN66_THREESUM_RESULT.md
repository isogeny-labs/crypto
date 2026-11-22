# Prime31 run66: exact signed-triple capacity and public-target three-sum census

**Date:** 2026-11-22 UTC. **Branch / draft:** [#4901](https://github.com/isogeny-labs/crypto/pull/4901). **Validation:** independent C++ exhaustive *generic* grader + independent Python big-integer replay PASS; native original-Rust replication [see CI status in RUN66_RESULTS.json]. **Research claim:** finite-group support and admissibility barrier, **not** a new non-generic relation oracle, index-calculus solving time or an ECDLP log.

## Mathematical bound that holds without a sampling heuristic

For B distinct nonsymmetric canonical points of an ordinary elliptic curve of order n, admit all 2B signed points and repetition. Every sum of three signed elements corresponds to at least one unordered signed multiset, hence

\[
\left|3(\pm\mathcal B)\right| \le \min\left(n,\binom{2B+2}{3}\right).
\]

The bound is purely combinatorial, independent of presumed random sumset behavior. On **unchanged** E/F_(2^31−1), p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251):

| Base | B | Exact signed-pair distinct support | Absolute three-sum capacity | Capacity / n |
|---|---:|---:|---:|---:|
| Original Dickson depth10 | 509 | 518,085 | 176,348,140 | 8.2120% |
| Dickson plus y^7 cubic-fiber | 677 | 916,469 | 414,635,420 | 19.3083% |

A capacity count is **not** a count of distinct three-sum targets: collisions can only reduce it. It is an unconditional bound for this fixed-base three-summand family, not for larger bases, higher arities or all non-generic index calculus.

## Independent reference measurement

Build a 518671/917335-record unordered signed **two-sum hash dictionary** with independent C++ affine EC arithmetic. For each public Q, exhaust all signed c and test Q−c against the dictionary; group-verify the returned triple. This is the exact generic ground-truth **grader**, deliberately *not an admissible algebraic candidate*. Public inputs: a deterministic set of 4096 SHA-256 hash-to-curve points whose scalar logarithms were never supplied, plus the **distinct** frozen 4096 W8 constructed public targets from run55 (not claimed uniform).

| Base | SHA-256 Q: two-sum / three-sum | W8 Q: two-sum / three-sum |
|---|---:|---:|
| B509 Dickson | 2 / **304 of 4096 (7.42%)** | 2 / 353 (8.62%) |
| B677 hybrid | 4 / **711 of 4096 (17.36%)** | 5 / 763 (18.63%) |

On the SHA-256 cohort, the B677 three-sum sample fraction has descriptive Wilson 95% interval [16.23%,18.55%]; B509 [6.66%,8.27%]. The independent **Python big-integer group law verified all 2131 positive triples** across both bases and both cohorts, regenerated all 4096 SHA-256 points, checked nG=O, and confirmed the B509 positive set is included in the B677 set (407 additional SHA-256 and 410 additional W8 positives). Negative classifications rely on the complete independent C++ generic lookup, not the Python positive-witness checker. Full population three-sum support was **not** enumerated.

The B677/B509 SHA-256 coverage ratio (711/304=2.339×) closely matches the **triple cardinality ratio** (414635420/176348140=2.351×). Empirical support / capacity equals ~89.9% for B677 and ~90.4% for B509. Thus this cohort gives **no evidence of a special three-sum structure beyond factor-base size**. The extra 75 known free homogeneous equations (13→88) remain a separate rank advantage conditional on generating usable non-generic relations.

## Performance admission, not a solver timing

The hybrid still has 589 residual unknown factor-base logs after its previously verified 88 free rows, so a full-rank three-term relation strategy needs at least 589 **independent** useful relation rows. If every uniform random public probe independently samples this fixed three-sum support, the sample 711/4096 implies an optimistic 589/(711/4096) = **3393** probes even with one fresh rank per success. The exact capacity ceiling implies at least **3049** expected uniform probes under the same sampling hypothesis. Those estimates exclude negative-F4 cost, algebraic root decoding, retries, matrix rank stalls, setup, scalar shifting, target logs, fallback and verification.

Against the **historic different-runner** optimized shared-BSGS 170.113ms/1024 target result, the empirical optimistic count implies a sensitivity ceiling of **~50.1 microseconds per full public-only algebraic oracle attempt** before all omitted costs. This is not an end-to-end comparable runtime or a generic ECDLP lower bound. The existing [#4891](https://github.com/isogeny-labs/crypto/pull/4891) S4/F4 micro-base experiment reported **~1.7–10.7 seconds per F4 call** at membership degree k=6/8 (different system, device and input). Those are not directly comparable, but they strongly caution against spending the next iteration on blind S4 based solely on a generic support gain.

## Rust source-linked control

Original source reuse: [native example](../prime31_ic_20261115/native/examples/prime31_run66_three_sum_support.rs), [frozen 168 public y^7 leaves](RUN66_Y7_EXTRA168.csv), and [workflow](../../.github/workflows/prime31-run66-three-sum.yml), pinned to Rust 1.100.0. The native code uses only original \`Arithmetic\` and \`Base::build\`, reconstructs 509/677 canonical points and 518085/916469 two-sum supports, then checks **two different deterministic 256-point public x-walks**, independent of the larger SHA-256/W8 cohorts, against separately frozen C++ expected counts: Dickson (0,15)/(1,16) and hybrid (0,43)/(1,41) for two-sum/three-sum. It verifies each recovered witness against original-Rust EC group law. See the protocol's amendment for why these are additional replication cohorts rather than the 4096-member holdouts.

## Prior-work / no-duplication audit and prioritized next experiment

Checked current repo: [#4692](https://github.com/isogeny-labs/crypto/pull/4692) registered blind B8 panel, [#4822](https://github.com/isogeny-labs/crypto/pull/4822) RR9/Krylov, [#4812](https://github.com/isogeny-labs/crypto/pull/4812) 14-variable Horner/F3, [#4809](https://github.com/isogeny-labs/crypto/pull/4809) public-only S3, [#4891](https://github.com/isogeny-labs/crypto/pull/4891) S4/F4 tiny-base cost and existing \`prime31_anchor2_m6_support_validator.rs\` generic three-sum grader on B31. Related prior art: Amadori–Pintore–Sala, *Finite Fields Appl.* 51 (2018), DOI 10.1016/j.ffa.2018.01.009; Semaev 2004/031. Repository research-KB MCP was unavailable; novelty coverage is bounded to these sources.

**P0:** invest in actual unknown-index F3/RR9 extraction in the owning PRs. Run existing 46-positive/18-negative public-only corpus with exact ordinary-curve signed verification; charge all failed, timeout, OOM and extension-root branches; no hashed pair table permitted in candidate custody. **P1:** feed only successful independently checked roots to the run59 quotient-rank system, with scalar right-hand sides; verify full target logs [k]G=Q. **P2:** if extraction succeeds, repeat cold 1/32/128/1024 complete tasks on the same Rust runner versus strongest existing IC, negation-BSGS and optimized rho, counting setup and memory. A standalone 3-sum generic lookup cannot qualify as a non-generic solver.

**Current bounded conclusion:** the full 4096-target support experiment and its 2131 positive certificates are reproducible and materially constrain a higher-arity lead. The sampled rise in three-sum yield is largely cardinality-driven; no cheaper complete ordinary Prime31 ECDLP solver is established by this experiment.
