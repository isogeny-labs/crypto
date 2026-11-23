# Run77 — native paired non-table S3 extraction replay, Prime31

**Observation date:** 2026-11-22 18:15 PST / 2026-11-23 UTC  
**Scope:** original-Rust x-only target-dependent Semaev S3 extraction component, *not* complete discrete-log recovery.  
**Source:** frozen per-cell and per-size logs in [RUN77_RUST_PAIRED_REPLAY_20261123.json](RUN77_RUST_PAIRED_REPLAY_20261123.json).

## Verification receipt

Two completed original-Rust GitHub Actions jobs both passed the crate's checks, x-only extraction correctness controls, Run75 paired comparison, Run76 cached comparison, and all post-hoc oracle validations:

| workflow | source commit | original Rust job | verdict |
|---|---|---|---|
| [45095390200](https://github.com/isogeny-labs/crypto/actions/runs/45095390200) | `36b9af2ae331cce3404e17ae06642e907642f3e9` | 137433959174 | success |
| [45095316844](https://github.com/isogeny-labs/crypto/actions/runs/45095316844) | `3bc5e60f269c2360e6fb2b0a9240f9161dd9cabb` | 137433737548 | success |

The later source commit includes a cached/uncached equivalence unit test. Both execute the unchanged ordinary cofactor-one curve over p=2,147,483,647. Each comparison has 4 sizes B=64/128/256/677, eight public target x-values per size, three one-pair positives and five exact negatives, five alternating/rotating repeats per arm and full result-set equality to an independently generated **post-solve-only** exhaustive reference. The fixed deterministic corpus is identical between runners, so the two passing jobs constitute **source replay, not independent target sampling**. Their runner CPU speeds differ; compare arms *within* a job, never pool absolute times across jobs.

## Run76: balanced tree beats cached Newton at B=256 and B=677

Each table entry is the sum of eight per-target median millisecond durations. `cached+` includes the factor-base-membership polynomial computed **once** for eight queries, not eight times. Ratios are `(cached+) / tree`; >1 favors the balanced tree.

| B | Job 137433959174: tree / cached+ (ms) | Ratio | Job 137433737548: tree / cached+ (ms) | Ratio |
|---:|---:|---:|---:|---:|
| 64 | 2.435 / 2.026 | 0.832 | 3.237 / 2.856 | 0.882 |
| 128 | 7.997 / 7.671 | 0.959 | 10.344 / 10.275 | 0.993 |
| 256 | 27.804 / 31.876 | **1.146** | 36.509 / 40.594 | **1.112** |
| **677** | **180.048 / 228.122** | **1.267** | **230.012 / 279.649** | **1.216** |

At B=677 the balanced extractor takes **22.506–28.752 ms/query** across the two runners, versus setup-charged cached Newton **28.515–34.956 ms/query**. The balanced-tree advantage **survives** caching the fixed factor-base polynomial. At B=64 and 128, cached Newton wins or is essentially tied. This is a reproducible size-dependent **component-stage performance improvement**, not a new solution asymptotic; no peak RSS or field-operation counter for these new arms was captured.

Run75 uncached Newton versus balanced tree independently passed in both jobs (and the earlier [45094536545](https://github.com/isogeny-labs/crypto/actions/runs/45094536545)), including all 32 x-only controls. B677 uncached ratios are approximately 1.309 and 1.265. The Run76 cache correction is the stronger engineering comparator.

## Important limitations / lead assessment

1. **The oracle remains expensive against enumerative reference.** Prior original Rust post-hoc exhaustive B677 pair verifier was approximately 4.8–5.5 ms/query on its runner, whereas Run76's balanced extractor is 22.5–28.8 ms/query on different runners; this is a *directional warning* and **not** a matched same-host speed ratio, nor a valid non-generic solver replacement.
2. **No signed group law or authenticated RHS in Run77.** The older [November 15 signed S3 quotient/GCD](../prime31_ic_20261115/native/examples/prime31_s3_quotient_gcd.rs) already has signed lifting and independent point-addition validation; this is prior art that the new code must be compared against on identical W and target Q. Raw x-only S3 root counts alone do not give unknown-index relations.
3. **Complete cold control PR #4901 merged.** Stop maintaining that PR. Its frozen [Run72](../prime31_f4_admission_20261121/RUN72_RESULTS.json) already verifies 32/32 complete Q on nine producer cohorts, but uses a **916,469-key generic pair lookup**, a disallowed extractor. Its K4 fully charged cold cost is 4.83–5.40× same-process shared BSGS. Reuse the merged original Rust rank/target solver as infrastructure but do not claim an IC breakthrough from the generic reference.
4. **No asymptotic escape here.** The current dense arithmetic still has schoolbook polynomial multiplication and dense GCD/division over polynomials of O(B) degree. Empirical scaling resembles quadratic, not sublinear in B. The distinct four-leaf Semaev strategy's conditional uniform-yield criterion requires amortized o(B) work per genuine public relation probe (plus affordable preprocessing and rank recovery); a better two-leaf constant cannot meet that assumption.
5. **Prior investigations:** September radical-pair-image DRL, signed S3 quotient/GCD above, #4920 chained signed S3, #4921 FASS, #4922 elimination, and independent cross-repo `cryptanalysis/crates/prime-field-m3-cdclt/README.md` compact m3 CDCL(T) and `crypto-autoresearcher/notes/g31-004-wp2-evalcost-20261120/` evaluator modes. Symmetrized Semaev polynomials were studied by Faugère–Huot–Joux–Renault–Vitse (EUROCRYPT 2014). KB MCP wasn't available in this tool session: this is a bounded GitHub primary-source review, not a complete research-KB survey.

## Next implementation gate

1. In #4924, add a **third existing signed-November-15 quotient/GCD** arm via reuse/adaptation, fixing identical public Q, actual factor-base W and source; verify all ± combinations (including diagonal, infinity and x(Q) collision) via group law. Accumulate group operations, memory and setup. Unmatched factor-base families must remain separate experiments, not silently compared.
2. Measure where B677 elapsed time goes: balanced polynomial construction, D/N elimination, univariate GCD, `Fp`-rootfinding and signed verification. Add per-stage field counters and peak RSS before replacing schoolbook polynomial multiplication with subquadratic fast univariate arithmetic. Quantify whether that changes the slope rather than only the constant.
3. Work through an existing public-only **m4** extraction lane (especially #4692/#4812/#4822), **feed actual recovered sign-verified RHS rows** into the merged Rust rank/quotient solver, and freeze 1/32/128/1024 cold log workloads over the same Prime31 curve. Include all rejected probes, factor-base generation, polynomial precomputation, rank/sparse LA and target recovery, then matched best-project IC, negation BSGS and optimized negation-map rho. Hold out new unknown-index public Q, and independently check `[k]G=Q`. Do not mix biased planted-positive controls into uniform-yield estimates.

**Decision:** retain Run77 as a positive reproducible **large-B extraction-component** result; do not advance to a complete ECDLP or asymptotic performance claim. PR #4924 remains draft for incomplete integration.
