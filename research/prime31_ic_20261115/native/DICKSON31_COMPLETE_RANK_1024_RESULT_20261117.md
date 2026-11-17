# Actual Prime31 Dickson31: a complete 1024-Q Rust relation/rank/log-recovery control

**Result dated 2026-11-17.** Producer development evidence [pcc:unverified]; independent original-field arithmetic replay is unsigned. Original curve and generator: p=2147483647, n=2147446831, `y²=x³+847514846x+2087444724`, `G=(625037715,2100627251)`, cofactor one. Pinned Rust 1.100.0, immutable arithmetic source `1df714645aa683cf975bc76fcacb9842cb4780fd`.

## Achieved, precisely scoped

A **complete index-calculus-style 31-factor-base logarithm pipeline** now exists as a tested Rust extension of the existing native Dickson31 oracle in `examples/prime31_s9_b31_half_sum_oracle.rs` (not a new Rust group-arithmetic implementation). It collects signed eight-leaf relations on known public scalar multiples `R=[a]G+[shift]G` and solves

`Σ_j c_ij · ell_j = a_i + shift_i (mod n)`

for all 31 factor-base logs `ell_j`, then applies a public-only target `Q` relation

`Σ_j d_j · ell_j = k + shift (mod n)`

to recover the previously unknown target log `k`. A separate cold generic BSGS fallback ensures correctness on targets unrepresented after nine public shifts; any use is labelled and charged separately. The factor-base logs were not given to the Rust solver: the 31-dimensional linear algebra recovered them from signed authentic relations.

**Scientific classification:** The relation extractor is still the explicitly enumerative, generic-equivalent signed `4A+4A` table (618,885 distinct half sums). Although this is a fully functioning, mathematically checked relation→modular rank→complete ECDLP pipeline, it is **not** a genuinely non-generic relation finder, a new exponent, or a cheaper solver than the fastest available shared generic baseline. No NIST-scale inference is warranted from a 31-bit curve.

## Exact-source implementation and evidence

- [Passing source-exact Rust 1024-Q CI](https://github.com/isogeny-labs/crypto/actions/runs/44190401085): 40 original library tests plus 1 focused 1024-Q test (source `ce045bc388ffaabb1c953d2b97eb5af317af2763`, Rust 1.100). Source control/full tests recorded on the PR branch; later cold-BSGS corner-case regression is a separate source change and must be checked in its later Actions run.
- [Original raw Actions artifact](https://github.com/isogeny-labs/crypto/actions/runs/44190401085/artifacts/14376491440), GitHub-reported artifact ZIP SHA-256 `27cb86bf1075414d28a1b00f51dad877f909ea021bb83f84af98351bf71f3b4e`, *not independently ZIP-rehashed*.
- [Durable full original Rust JSON transcript](DICKSON31_COMPLETE_RANK_1024_RAW_20261117.json), with all 31 source rows, 31 derived logs, 1024 public Q, 1024 returned k, signed witness words, shifts and failed-attempt counts.
- [Independent BigInt replay](DICKSON31_COMPLETE_RANK_1024_INDEPENDENT_AUDIT_20261117.json), separate implementation of prime-field addition/multiplication/inversion and curve arithmetic. It independently re-solved rank 31, verified all 31 original-curve factor-base logs and every signed 8-leaf relation and [k]G=Q for all 1024 public Q, plus all 31 source rows, 76 counted failed shifts.
- [Independent historical 98-Q audit](DICKSON31_INDEPENDENT_98Q_RANK_AUDIT_20261117.json), reconstructing the true Dickson depth-six base from seed 3 and checking 98 public Q, 98 signed group relations, all 31 independently recovered base logarithms; the full B31 rank is reached by the first 31 successful relations.

A producer test is not an independently signed PCC result, and the separate BigInt auditor does not independently reproduce wall-clock timings or implement an independent ECDLP solver.

## Fully charged source-specific measurements, 1024 unique Q

| Component | Final replay-transcript CI wall |
|---|---:|
| Original curve validation, special factor base and 618,885-entry generic 4A index | 215.353 ms |
| 31 known-public-source relation equations, modular rank, 31 original-curve log checks | 164.229 ms |
| 16 further independent known-public-source equation controls | 135.216 ms |
| 1024 public-Q relation search, misses, recovered logs, [k]G=Q checks, fallback (not invoked) | 7,419.923 ms |
| **Whole solver (not including caller-only corpus generation)** | **7,934.720 ms** |

The 10.853 ms caller-only Q generation is separately instrumented as harness overhead rather than solver work. Exact-source warmup / CPU scheduling is not isolated. The earlier summary-only CI measured 10,028.812 ms on a different GitHub-hosted run; **7,934.720 ms is the transcript-instrumented run's own observed wall**, not a claim of repeatably lower runtime.

| Complete-control metric | Observed |
|---|---:|
| Independent factor-base equations / rank | 31 / 31 |
| Additional source equations verified | 16 |
| Blinded complete recovered target logarithms | 1024 / 1024 |
| Target hits at first public shift `a=0` / `a=1` / `a=2` | 951 / 70 / 3 |
| Target membership attempts / exhaustive unsuccessful public shifts | 1100 / 76 |
| Generic BSGS fallbacks on this corpus | 0 |
| 512-lane batched window operations (all setup+queries) | 328,566 |
| Processed complement lanes (all setup+queries) | 168,216,321 |
| Batch-custom finite-field multiplications / inversions | 1,029,333,564 / 328,566 |
| Half-table HashMap capacity | 917,504 entries (not bytes or RSS) |

The 31 basis logs exactly matched a separate reconstruction from an earlier independent 98-Q historical corpus, rather than merely satisfying the new input equations.

## Performance decision

Historical [shared-precomputation 1024-Q Rust BSGS](../mixed_base/SHARED_BSGS_1024Q_RESULT_20261117.md) completed with a million-entry table in **170.113 ms**, while the historical mixed-base generic-table IC took **2,221.474 ms** on the same older cohort and runner. This new Dickson31 complete control's **7,934.720 ms is from another run and implementation**, so the two absolute runtimes cannot be treated as a matched speed ratio. It does, however, decisively show the currently measured Dickson31 table-based extraction is not ready for a cost-advantage claim. Current setup alone is ~215 ms on the new run, before 1,024 target descents. A genuine new mechanism must remove both the 4A table and the slow repeated 8A scans.

The separate #4596 lane owns a matched 1024-Q IC vs shared BSGS benchmark and #4604 owns public-only RR9 D9 rank/certificates; neither was duplicated in this PR. The data here cannot show best-in-class against strong rho, external published IC, or an independent PCC gate.

## Prior leads and next falsifiable experiments

1. **Highest priority: target-conditioned, support-free signed RR9/S9 recovery.** Reuse native Rust RR9 norms/Hermite, quotient/F4 and exact rootfinding (earlier #4228/#4349). Candidate receives only public residual R and public Dickson H31; forbid 4A HashMap probes, signed seven-anchor advice, given-leaf coefficients and expanding all 48,903,492 degree-eight multisets. Search for `F=A(x)+yV(x)` satisfying the exact original-ideal norm and signed-fibre conditions; for repeated-root cases use Hermite membership and saturations. First exact B4/B8 known positive, negative, identity, repeated/cancellation controls, then the actual Dickson31 98-Q and new 1024-Q public data (witnesses only for independent grading). Collect negative unit certificates and positive original-curve witness replays. Terminate early if elimination has no measurable cost compression versus enumerative MITM.
2. **Compact K961–S5 target-conditioned join.** Existing 961 fully split pair-root quotient can generate the same generic support catalogue; that is *not* novelty. Test a compact target-dependent resultant/selected-root elimination without materializing `961^4` choices, replacing all roots with exact algebraic output rather than a group-table join. Verify original-system equivalence, sign orientations, exceptional fibres, memory cap and runtime. Use negative Dickson31 controls as mandatory falsifiers.
3. **Only after nonenumerative relations work:** feed them to the now measured 31-column rank and logarithm pipeline. On exactly identical 1024 Q, measure full cold setup+relation collection+rank+every attempted descent+verification+peak memory against shared BSGS with matched memory/advice, optimized WZ/rho and mature PKM/APS/F4 implementations. An illustrative absolute engineering gate from the historical generic batch is `<170 ms` total for 1024 Q, equivalent to at most ~0.166 ms per target if setup were free; this is not a cross-run statistical threshold or an asymptotic claim. Isolated repeated A/B order, independently signed PCC and validator replay are required for a real frontier claim.

Literature checked: Petit–Kosters–Messeng, PKC 2016, DOI [10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1) and Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51 (2018), DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009). Earlier internal negative tests #4548/#4549/#4550/#4575/#4581/#4612, #4594 39-lead portfolio, #4615/#4618 control, autoresearcher #3301/#3305 and cryptanalysis #1536 were reviewed. Knowledge base MCP/CLI unavailable; cannot claim an exhaustive novelty search.

**Disposition:** complete native Rust correctness/measurement control attained and independently checked on the actual prime31 curve. The cheaper **genuinely non-generic** prime-field solver goal is not established by this generic-equivalent candidate. Retain all null/negative evidence and do not update any security claims or performance leaderboards.
