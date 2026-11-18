#set page(paper: "a4", margin: 14mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set heading(numbering: none)
= Vieta Rust: native NTT and exact quadratic fibers
Producer observations — unverified (PCC-20261117-b4ee50). Below-L2 local clocks.

Full cold charges native field setup, full certified point base, required lookup
index and one relation query. Fixture/oracle preparation, output encoding and
independent replay are outside worker clocks. Native field setup operations
are unpriced in counters; no complete fast cold-count claim.

#image("cold-ratios.svg", width: 100%)

#table(columns: (2.8fr,1fr,1fr,1fr,1fr,1.6fr,1fr), inset: 3pt,
  table.header([Workload/population],[Cohort],[Newton s],[Candidate s],[Paired ratio],[Conditional95%],[Lookup s]),
[p224 m2048 coset / conditioned], [Original4b], [9.7343], [0.8055], [12.09x], [[12.08, 12.84]], [0.2770],
[p224 m2048 coset / random abscissa], [Original4b], [9.7582], [0.7741], [12.61x], [[12.32, 12.70]], [0.2845],
[word62 m16384 coset / conditioned], [Original4b], [5.3044], [0.6830], [7.56x], [[7.31, 8.24]], [0.1099],
[word62 m16384 coset / random abscissa], [Original4b], [5.3917], [0.6941], [7.61x], [[7.42, 8.20]], [0.1126],
[word62 m32768 coset / conditioned], [Original4b], [censored], [1.4371], [censored], [unavailable], [0.2232],
[word62 m32768 coset / random abscissa], [Original4b], [censored], [1.5034], [censored], [unavailable], [0.2370],
[p224 m512 rational / conditioned], [Original4b], [11.2796], [0.3109], [36.47x], [[36.03, 36.66]], [0.1954],
[p224 m512 rational / random abscissa], [Original4b], [11.5918], [0.3006], [38.28x], [[37.50, 40.00]], [0.1983],
[word62 m32768 coset / conditioned], [Fresh8b], [20.4201], [1.4677], [13.83x], [[13.78, 14.21]], [0.2159],
[word62 m32768 coset / random abscissa], [Fresh8b], [20.2707], [1.5344], [13.82x], [[11.82, 13.99]], [0.2193],
)

Three retained targets per population. Medians/intervals copied from the frozen
Rust analyzer; no pooling. Optimized lookup is cheaper in every subgroup.

#pagebreak()
= Method and retained failures
The fast arm combines native Montgomery fields, NTT128 products, unchanged
half-GCD and exact linear/quadratic roots. Every sibling fiber, pole,
coprimality and separability/full-splitting check remains. Repeated roots are
deduplicated; incomplete splitting fails certification. Higher-degree roots
use the original generic finder. Root order and all transform twiddle/setup
multiplications are checked/counted per Arithmetic stage. No global cache.

Original cohort:192 calls,176 successful replays,16 references stopped by the
frozen4b multiplication cap at32768; the full cohort is inconclusive. The new
8b word32768 protocol has fresh targets and48/48 calls/replays. It does not
replace those failures. A/A and positive control groups pass;40 candidate/lookup
construction comparisons (including warmups) have equal primitive counts.

= Validation and limits
Native core15 tests plus7 library and2 worker groups pass with strict Clippy.
Exhaustive small-field quadratics cover repeated, nonsplit and zero cases.
BigUint scalar complete-set replay is algorithmically separate but unsigned and
producer-owned. Final exports for every cohort pass file checks; the original invalid
outcome-class export is preserved beside its corrected derivative and exact
difference audit. Canonical terminal completed includes failed, never success.

The root library attempt hit180s and emitted unchanged-main hashed-ecdl-tensor
metrics-module errors; no full-root pass. Profiling reported timer undercoverage
and supports no quantitative CPU or throughput claim. Word62 is synthetic with
no prime-order subgroup assertion. P224 uses the standard-curve fixture;
rational maps are certified conjugated-squaring fixtures. Transform acceleration
uses the specific primes' power-of-two roots. Arbitrary prime moduli need an
appropriate product backend.

The general degree-bound argument and arbitrary-depth formal instantiation
remain unverified. No independent signed standing, L2 admission, fastest-external
claim, local S3 availability assertion, or protected leaderboard promotion.

= Reproducibility
First source: #raw("149fd91f33844390511f6310a21ffa50ad607542")
New8b source: #raw("2595478e2ac478055c7526f19af6dfc1fe604f8f")

Protocols: #raw("protocol-primary-v2.json") and #raw("protocol-budget8b.json").
Raw records, native summaries, scalar replays, control audit, failed export,
repair audit and all preregistration corrections are preserved in this study.
See RESULTS.md for commands, exact paths and cost boundaries. Git-only/local;
no paid/cloud experiment, local S3 operation or protected promotion.
