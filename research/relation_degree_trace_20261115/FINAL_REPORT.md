# Certified consequences: measured native component result

Five of six predeclared fresh four-variable cases pass the frozen per-case
tenfold rule. Their median native checked-wall ratios are **23.4–23.9×**, over
both 2147483647 and 9007199254740881. The remaining case fails its A/A control
and is excluded despite its large descriptive ratio. This is a local L0
producer observation, pending independent signed validation `[pcc:unverified]`.

| Field | Fresh seed | Median ratio | Paired bootstrap 95% | A/A admitted | Tenfold admitted |
|---|---:|---:|---|---|---|
| 2147483647 | 1009 | 24.0988 | [23.2628, 24.3499] | no | no |
| 2147483647 | 2003 | 23.4635 | [22.8825, 24.0137] | yes | yes |
| 2147483647 | 3001 | 23.4342 | [23.0972, 23.8768] | yes | yes |
| 9007199254740881 | 1009 | 23.8770 | [23.5896, 24.1804] | yes | yes |
| 9007199254740881 | 2003 | 23.8922 | [23.4087, 24.2329] | yes | yes |
| 9007199254740881 | 3001 | 23.6573 | [23.1318, 23.9062] | yes | yes |

All six cases completed 21 paired samples and seven A/A pairs. The failed
control's interval is [1.004619, 1.035816], excluding 1. No replacement or
green rerun was used. Among admitted cases, median baseline costs are
1137–1177 ms and candidate costs 48.5–50.0 ms. Ratios are medians of paired
ratios, not ratios of these separately computed medians `[pcc:unverified]`.

## Computation and accounting

The output is the complete reduced grevlex Gröbner basis of a controlled
four-variable cycle of paired Semaev f3 equations on y²=x³+2x+3. Shared x
variables model repeated point incidence. Fixture witnesses generate and
validate inputs; the solver and preprocessor receive only equations.

Baseline: native textbook Buchberger on raw inputs. Candidate: recognize
paired f3 equations, construct exact certified cubics, compute edge resultants
with Sylvester cofactors, combine incident projections by extended Euclidean
GCD, then use initial interreduction and the standard product criterion.
The common final checker checks original-input membership certificates,
every input normal form and every S-polynomial, including criterion-skipped
pairs. Exact reduced-basis equality is checked for every paired sample.

Charged wall time includes input cloning, all candidate construction and
identity checks, full basis computation and internal checking, deallocation,
and the timed inter-arm equality replay added to both arms. Both exclude
process launch, input parsing/primality testing and known-answer fixture
generation. One thread, CPU 0, 2 GiB and 30 seconds per worker. Alternating
AB/BA pairs; fixed 10000-draw paired bootstrap. Admission requires all 21
pairs, an admitted seven-pair A/A control, and lower 95% endpoint >10.

Measured workers are frozen from source
`2471e4c16bbcb85410017bf8ea5d05703f5ce215`. Confirmation seeds 1009/2003/3001
were committed in `4418b8da6ed9604d15ffce7034c208429000a131` before their
fixtures/timings were examined. Byte-identical copies of the measured workers
ran confirmation; analysis tools were built separately. Source, binary,
input and raw-output hashes are retained with the archives.

## Earlier attempts and interpretation

[The complete tables](results/final/REPORT.md) retain all exploratory cases
and incomplete attempts. Cubics alone did not approach tenfold; three-variable
cases regressed. Projections gave about 4.8–5.0× in completed four-variable
cases, then about 6.7–6.9× with initial interreduction. The final common-checker
and product-criterion comparison reached 22.4–23.0× in four admitted
exploratory cases. Raw-input engine-only arms were slower on that family.
Six-variable baselines were capped and have no numerical speed ratio.
Failed controls remain visible. These stages use different common checkers;
their timings are never spliced into a combined ratio `[pcc:unverified]`.

The measured gain belongs to **certified projections plus the candidate
engine**, not to sparsity or overcollection alone. The original two-variable
degree trace witnesses a generator-dependent closure threshold 5→4 and
different top-form Hilbert behavior for equally sparse cubics. That does not
establish a four-variable degree-of-regularity decrease. Projection membership
certificates have expanded product degree 15 in these fresh cases; no claim
of a lower discovery-inclusive maximum degree follows from their quartic
output or from the measured wall ratio `[pcc:unverified]`.

## Independent CAS algorithm checks and performance limits

All 24 supported external arm/algorithm runs returned exactly the native
reduced basis: Singular std on both fields (12), Singular slimgb on the
31-bit field (6), and msolve F4 on the 31-bit field (6). Twelve unsupported
arm/algorithm combinations are explicit: slimgb's 53-bit coefficient ring
and msolve's documented p<2^52 limit. These are independent CAS algorithms
invoked by this producer, not an independently signed execution verdict
`[pcc:unverified]`.

The external numbers limit the practical claim. Singular std's raw-input
100-solve CPU batches take 0.468–0.551 ms/solve on the 31-bit fixtures and
0.775–0.812 ms/solve on the 53-bit fixtures. The corresponding preprocessed
batches take 0.169–0.175 and 0.766–0.779 ms/solve, respectively. Those exclude
native preprocessing/certification and are solver-only CPU diagnostics, not
end-to-end wall ratios. msolve's single CLI wall times include startup, and
its printed CPU timer is too coarse here. No clock scopes are mixed into a
headline ratio. The native comparison establishes no advantage over these
mature solvers `[pcc:unverified]`.

Singular: 4.4.1.p5, exact prime coefficient rings. msolve: 0.11.0, source
`8667f956a2534a3b414a3cb77f82eb8e3ef204cc`. Raw outputs and exact CAS input
scripts are archived. [Machine-readable closeout](results/final/confirmation.json)
includes every fresh case and external status.

## Scope, evidence and decision

This accepts the frozen tenfold hypothesis for five specified local native
component cases. It does not supply P/Q coefficient tracking, factor-base
collection, relation yield, dependency-based logarithm recovery, whole-ECDLP
cost, scaling to six variables, GAE pricing, or a fastest-solver result.
There is no signed independent replay or matched-rho validation. PCC standing
remains unverified. The tested construction is usable instrumentation; a
production or asymptotic advantage remains unestablished `[pcc:unverified]`.

Dashboard exclusion: these controlled polynomial fixtures have no registered
whole-pipeline curve/reference, accepted performance row, or validated IC/rho
cost. This PR reports unvalidated instrumentation diagnostics and proposes no
leaderboard adoption. Existing frozen tables and validation standings remain
unchanged. A validated component promotion must integrate the generated
leaderboard diagnostic and affected browser views before landing that claim.

Reproduce from this standalone crate with `cargo test --release --locked`
and `cargo clippy --all-targets --locked -- -D warnings`. To inspect the
recorded campaign, hydrate and extract the archives as documented in
[artifacts/README.md](artifacts/README.md), then run
`cargo run --release --locked --bin report -- . /tmp/relation-report-new`.
The full root repository test suite was not run from the API-staged source
tree. The fresh protocols are [CRITERIA_PROTOCOL.md](CRITERIA_PROTOCOL.md) and
[CONFIRMATION_PROTOCOL.md](CONFIRMATION_PROTOCOL.md); earlier protocols remain
historical preregistrations, not current status pages.
