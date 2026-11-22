# Prime31 symmetric Semaev / Newton original-Rust CI and admission record
Date: 2026-11-22 UTC. Lane: PR #4924. Status: source-only until a completed source-specific CI receipt is inspected. **[pcc:unverified]**

## Exact registered problem and non-negotiable comparison

- Reuse the ordinary prime-order (cofactor-one) project fixture already frozen in `research/prime31_ic_20261115/README.md`: `p=2147483647`, `E:y²=x³+847514846x+2087444724`, `n=2147446831`, `G=(625037715,2100627251)`. Do not substitute a smooth-order example, a deliberately weak curve or an unrelated M31 fixture.
- Algebraic stage uses the existing Rust `crypto_lib::cryptanalysis::f4_fp`, `semaev_prime_poly` and original-crate examples. Same host, arithmetic, equation budget, honest cold input materialization and timeout accounting for competing arms.
- New CI instrument (triggering commit `917311f8d4c54d21245e9066873bbbedb51ea381`): `.github/workflows/prime31-trace-symmetric-rust-gate.yml`; first GitHub Actions run [#45082848679](https://github.com/isogeny-labs/crypto/actions/runs/45082848679). Whether it **passed** is determined only by the completed job receipt, not by the existence of this link.
- Exact arithmetic tests for trace–skew unordered pair image and m=2/3/4 symmetric trace-moment/Jacobian identity, followed by four original-Rust F4 modes: ordinary m2 S3, m2 quadratic power-sum circuit, m4 membership-only, and a genuine m4 chained-S3 positive-control identity. `--m4` alone is **not** an S5 relation solve.
- All positive-control F4 instances contain planted public group-verified relations. These cannot measure honest unknown-index relation yield and are **not** complete ECDLP recovery. Record setup time, F4 time, matrix rows/cols, field operations, rank witnesses where available, exceptions, and all timeouts. Do not translate a degree or stage win into a cold DLP speedup.

## Previously searched work / no redundant claims

| Lane | Primary evidence | Relevant constraint |
| --- | --- | --- |
| Radical DRL unordered pair (2026-09-21) | `research/radical_pair_image_20260921/RESULTS.md` | The previous `b+1` generator radical scheme already showed an F4 **stage** benefit on tiny GF(17)/GF(29), e.g. holdout `b=11` cold ratio `0.0967`; this is the incumbent, **not** a cold ECDLP result |
| Prime31 authentic public RHS / ranked target quotient | [PR #4901](https://github.com/isogeny-labs/crypto/pull/4901), `research/prime31_f4_admission_20261121/RUN72_RESULTS.json` | Three original-Rust 32-target K2/K3/K4 cohorts [CI #45077314037](https://github.com/isogeny-labs/crypto/actions/runs/45077314037) work with a **916,469-entry generic** pair table; they cannot be counted as qualifying non-generic extraction |
| Prime31 M6 mixed signed relation and frontier | [PR #4714](https://github.com/isogeny-labs/crypto/pull/4714) | 1,024 logs recovered, but M6 was `20.85–23.32×` slower than matched shared BSGS and slower than existing 4+4 IC |
| Prime31 S3 chain / signed x-root | [PR #4920](https://github.com/isogeny-labs/crypto/pull/4920), `research/chained_s3_prime31_20261122/README.md` | Existing S3 discriminant `16 f(x)f(u)` is square on rational factor-base points; a QR filter does not improve yield. Separate pilot has no native timing yet |
| Prime-field S4 pair-sum MITM | [PR #4911](https://github.com/isogeny-labs/crypto/pull/4911) | u64 optimized complete 32-bit fixture IC can enter the unoptimized rho time band; no matched optimized negation-rho win established |
| Binary selector CDCL(T) over prime fields | `isogeny-labs/cryptanalysis:crates/prime-field-m3-cdclt/README.md` | m3 binary selector avoids a variable blowup but measured online exponent about `1.33`, above the registered `<0.5` gate |
| Universal S3/S4 exact evaluator | `isogeny-labs/crypto-autoresearcher:notes/g31-004-wp2-evalcost-20261120/semaev_eval.py` | Reusable exact universal polynomial and grouped quartic evaluation; no evidence it lowers the complete index-calculus exponent |

Literature originality boundary: Semaev 2004 [ePrint 2004/031](https://eprint.iacr.org/2004/031), Faugère–Huot–Joux–Renault–Vitse 2014 (EUROCRYPT, DOI `10.1007/978-3-642-55220-5_3`), Amadori–Pintore–Sala 2018 (DOI `10.1016/j.ffa.2018.01.009`). Symmetrizing Semaev constraints and one-relation prime-field index calculus are existing techniques. The shared research-KB MCP and its CLI wrapper are unavailable in this execution environment; a GitHub primary-source audit is **not** proof of global novelty.


## Derived Run72 full-cold control ratios on the *same ordinary Prime31 curve*

This is **not** a new measurement: it is direct arithmetic using source-pinned `research/prime31_f4_admission_20261121/RUN72_RESULTS.json` from #4901 (CI #45077314037, three deterministic 32-public-Q cohorts, all native Rust outputs replaying 32/32 each), with `K=4` for every cohort. Full candidate time is `cold.cold_total_ms`; matched BSGS time is `bsgs.setup_ms+bsgs.online_32_ms` from the same JSON.

| Seed | K4 cold candidate ms | Shared BSGS ms | Candidate / BSGS | Genuine candidate shifts vs K4 readout-only |
| --- | ---: | ---: | ---: | ---: |
| `0x2c6478eb19d` | 1223.973 | 243.237 | **5.032× slower** | 1,668,210 vs 2,528,933 |
| `0x68b677a1d1` | 1271.430 | 235.344 | **5.402× slower** | 1,668,210 vs 2,528,933 |
| `0x9d3a6f5c127` | 1299.790 | 269.195 | **4.828× slower** | 1,825,641 vs 2,528,933 |

This *does* establish a useful cost-gate assessment for already completed **generic** original-Rust controls: target-certificate difference rows improve the public relation shift count, but do not offset the signed pair-table setup + certificate discovery + relation work even against shared BSGS on these three cohorts. This cannot qualify as a novel non-generic or index-calculus advantage; comparisons with strongest IC and optimized rho are also outstanding. Producer-reported CI and this arithmetic are **[pcc:unverified]**, not a signed independent certification.

## Specific mathematical wall and falsifiable next experiment

For `m=4`, explicit degree-`B` expansion in four elementary-symmetric coordinates has a worst-case `binomial(B+4,4)` monomial space; at `B=677` that is `8,882,684,790`. A preprocessing algorithm that actually constructs `Theta(B^4)` terms cannot be called sub-rho at the optimistic `B≈p^(1/5)` because its setup is already `p^(4/5)`. This is a bound on this representation, **not** a lower bound on every algorithm. The sparse Newton circuit has `O(B)` equations and variables; a generic F4 may still explode.

Next test, *after* original-Rust CI compiles: same genuine Prime31 public target set and 15s per F4 arm for `B=3,4` comparing ordered, raw moments, degree-`B` reduced moments and the quadratic circuit, plus m4 chained positive control. Inspect exact solution certificates, not only `F4Result::solving_degree`. Bring the earlier radical DRL basis into the same engine before claiming a faster algebraic stage. Next non-generic test must process **unplanted random public Q**, cover signed lift/O exceptional branches, create independent genuine RHS rank, and recover at least `1/32/128/1024` logs on this exact curve; compare cold setup + relation search + LA + individual descent to the *best existing* index-calculus, shared BSGS and optimized negation rho under identical wall/operation units.

### Decision gate
1. **PASS component:** original Rust compiles, tests and exact symmetric/power-circuit witness checks pass; every F4 arm returns a completed or explicitly censored record, with all attempts represented.
2. **PASS relation oracle:** no signed generic pair/half-sum dictionary or secret scalar feeds the algebraic candidate; correctly verified public-only roots with bounded memory and total field-work, including negative and incomplete cases.
3. **PASS solver:** full authentic labelled independent row system plus individual-log recovery, `[k]G=Q` independent verification and source-pinned replay for every target.
4. **PASS advantage:** fully charged complete cold single-target and declared amortized panels beat the best prior index-calculus frontier (and, for a generic-break claim, the strongest matched negation rho/BSGS) on identical random-target cohorts. A 31-bit constant-factor win alone cannot prove a better asymptotic exponent; several ordinary prime-field sizes and uncertainty bounds are needed.
5. **FAIL / censored:** any missing compiler result, timeout, rank deficit, generic dictionary, omitted setup or lack of matched strong baseline means no accepted speedup. The result remains a useful negative/partial record.

Never promote data to the leaderboard without `VALIDATION.json`, independent replay/validator and signed PCC standing. **No novel complete prime-field non-generic ECDLP advantage is claimed here. [pcc:unverified]**
