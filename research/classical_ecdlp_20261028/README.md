# Classical ECDLP followups and independent replication archive

This study preserves the classical APS investigation and the independent
guess-and-determine replication recorded on 2026-10-26/27. It adds a native
Rust replay of the public fixtures and saved witnesses, reusing the repository's
existing [W12 implementation](../../examples/lit2026_w12_zero_minor.rs).
It does not establish an improvement over the repository's rho reference.

The original files are immutable historical evidence. Their Python code and
timings predate this import; the active verification path below executes Rust.
The repository already has a broader
[W12 audit](../lit2026_w12_flagged_claims_20261011/README.md), including a
stronger operation-counted rho comparison. This archive supplements that audit
with independent fixtures and CAS receipts; it does not replace its conclusions.

## Request and evidence

| Requested work | Evidence and status |
| --- | --- |
| Investigate APS's one-relation method and classical followups | Fourteen source records in [sources.json](historical/aps/sources.json), with individual access limits; contribution map below. |
| Pursue the direct evaluation and signed-point-sum implementations | Original McGuire–Mueller definitions executed with our driver in six small Magma checks; [receipt](historical/aps/magma-author-code-check.json). |
| Try the guess-and-determine algorithm | Independent historical implementation: 60 public known-answer cases, all solved; six complete saved witnesses. Native replay checks all recorded answers and all six witnesses. |
| Keep the work classical | No quantum method, quantum implementation, or quantum experiment is included. |
| Check the work into the crypto repository | Original evidence, native replay/tests, source-linked report, editable visuals and PDF are included together. |
| Establish a practical advantage over generic methods | Not established. Historical Python measurements lack the required environment and strongest-reference evidence. No new performance experiment was requested for this import. |

## Classical leads

| Lead | Contribution, evidence and remaining limit |
| --- | --- |
| [Amadori–Pintore–Sala, FFA 2018](https://doi.org/10.1016/j.ffa.2018.01.009), [2017 preprint](https://eprint.iacr.org/2017/609) | Labelled points permit recovery from one usable relation, avoiding the usual factor-base discrete-log linear-system stage. Finding that relation still requires work. The accepted manuscript was read. |
| [McGuire–Mueller, ePrint 2017/1262](https://eprint.iacr.org/2017/1262), [author code](https://math.ie/ecdlp/ecdlp.html) | Retains labelled bases; replaces Gröbner solving by direct summation-polynomial evaluation, and then signed-point sums. The earlier title is *A few more index calculus algorithms for the ECDLP*. Complete public Magma definitions were inspected and run unchanged with an independent driver. Direct PDF retrieval failed. |
| [Kudo–Yokota–Takahashi–Yasuda, CANS 2018, 377–393](https://doi.org/10.1007/978-3-030-00434-7_19) | *Acceleration of Index Calculus for Solving ECDLP over Prime Fields and Its Limitation*: hybrid exhaustive search, Gröbner techniques and summation-polynomial symmetries. Publisher/precursor abstracts were available; the subscription chapter and its exact experimental tables were not independently audited. |
| [Yokota–Kudo–Yasuda, WCC 2017](https://drive.google.com/file/d/1I8t-2nEMqzTr8ueppHIO3xWHe1khWiAb/view) | Author-linked extended abstract was read. Its earlier experiments concern PKM; they must not be presented as APS experiments. |
| [Yokoyama–Yasuda–Takahashi–Kogure, JMC 2020, 460–485](https://doi.org/10.1515/jmc-2019-0029) | Reports the lack of an experimental win and develops conditional lower bounds for a specified naive Gröbner-based setting. Full text and stated assumptions were checked. This is not an unconditional impossibility result for every APS solver or coordinate-based method. |
| [Lanteri, Genoa master's thesis, 2023](https://unire.unige.it/handle/123456789/6368?show=full) | Public abstracts document independent C++/CoCoA APS and PKM implementations. The thesis file is restricted; a separate public implementation was not found. |
| [Bigatti et al., F4/FGLM tutorial, 2025](https://arxiv.org/html/2509.03346v1), [teaching source](https://github.com/tor-kristian/f4_genova) | Public tutorial and educational F4 code are available. They are not the restricted thesis implementation. |
| [La Scala–Pintore–Tiwari–Visconti, MultiSolve, 2024](https://doi.org/10.1016/j.ffa.2024.102452) | Multistep hybrid solving technology; experiments concern Trivium. No demonstrated APS-over-rho improvement was found. |
| [La Scala–Tiwari, oracle-based MultiSolve, 2026](https://doi.org/10.3934/amc.2026024) | Followup solver technology applied to Aradi. Full preprint was read; no APS benchmark is supplied by this investigation. |
| [La Scala–Marchesin–Tiwari, Hamming ideals, 2026](https://arxiv.org/abs/2609.18866) | Abstract-level newer lead concerns syndrome decoding, not a demonstrated ECDLP application. |
| [Abdullah–Mahalanobis, zero minors, 2025](https://doi.org/10.1080/10586458.2025.2525844) | Adjacent classical matrix/minor route; publisher record and opening text checked. It uses matrix algebra and does not satisfy the original linear-algebra-avoidance objective. |
| [Mahalanobis, guess and determine, 2026, v2](https://arxiv.org/html/2607.09814v2) | Algorithms 1/2 and the probability model motivate the independent replication. This is an adjacent zero-minor method, rather than an APS acceleration. The historical source-catalogue access note predates the subsequent replication. |
| [Shoup, generic lower bounds](https://www.shoup.net/papers/dlbounds1.pdf) | Relevant context with a generic-group hypothesis; it does not automatically exclude coordinate-based algorithms. |
| [Magma public calculator](https://magma-maths.org/calc/) | Independent CAS checks returned successful responses from version 2.30-1. Saved responses and self-contained witness code are preserved. |

The source catalogue is a dated access snapshot, not a claim that all full texts
or every possible classical lead have been obtained. Third-party papers and the
McGuire–Mueller source are linked rather than redistributed.

## What was tried

The APS archive contains tiny fixed-base comparisons of direct S3 evaluation,
signed-point sums, univariate GCD, full Gröbner and hybrid Gröbner solving.
The unfiltered population has 16 bases; the deliberately stratified population
has eight. Completed solution sets agree; capped Gröbner timeouts remain in the
records. These are correctness diagnostics, not end-to-end rho benchmarks.
The author-code receipt records all three McGuire–Mueller modes for m=3 and m=4
on the order-991 curve. Different search spaces prevent interpreting their
trial counts as a controlled performance comparison.

The guess-and-determine archive contains ten trials per curve, seed 20261027.
The attack receives public curve data, P and Q; the known scalar is used for
fixture checking. The historical implementation has no exhaustive-log fallback.
All 60 recorded successes used signature collisions, and no recorded case hit
its 15-second or search-budget limit. Six trial-zero cases preserve complete
matrix, label, signature, minor and point-sum witnesses.

All six curves have prime full group order, cofactor one and p different from r.
They are public synthetic known-answer fixtures, not deployed-curve challenges.

| Field p | a | b | Group order r | Generator P |
| ---: | ---: | ---: | ---: | --- |
| 251 | 1 | 4 | 271 | (0, 2) |
| 1021 | 1 | 6 | 991 | (2, 4) |
| 4093 | 1 | 22 | 4049 | (4, 1121) |
| 16381 | 2 | 16 | 16567 | (0, 4) |
| 65521 | 1 | 35 | 65761 | (28357, 64561) |
| 1048573 | 1 | 14 | 1048193 | (992029, 718933) |

## Native verification

From the repository root:

```sh
cargo test --release --locked --example classical_ecdlp_archive
cargo run --release --locked --example classical_ecdlp_archive -- check
cargo run --release --locked --example classical_ecdlp_archive -- summary
cargo run --release --locked --example classical_ecdlp_archive -- solve
(cd research/classical_ecdlp_20261028 && sha256sum -c HISTORICAL.sha256)
```

These are development correctness/replay commands, not timed experiments.
`check` validates the complete 60-case population, recomputes all six curve
orders, and checks all 120 recorded candidate/rho answers by scalar
multiplication. For each saved witness it rebuilds the evaluation matrix,
checks ranks, K*M=0, labels, systematic form, signature kernels and column maps,
then verifies the zero determinant, elliptic point sum and scalar equation.
It also checks 84 distinct-x S3/sign triples on the small F17 fixture and
enumerates the two finite candidate populations shown in the report.
Negative tests corrupt a kernel, a label and a minor-column map.

`solve` invokes the existing repository W12 engine on one public trial-zero
target per curve and independently checks the returned scalar. It uses that
engine's RNG and exceptional/shortcut paths, so it is a native correctness
regression, not a reproduction of the historical random trajectory or timings.
`summary` recomputes medians and ranges from the historical raw JSON without
remeasuring anything. [Validation receipts](validation/) record the checks
actually completed for this import.

The original [Magma receipt](historical/guess_determine/witness-magma.json)
has `success: true`; the accompanying
[self-contained checker](historical/guess_determine/verify_witnesses.magma)
is retained. Printed PASS strings alone are not treated as success.

## Probability scope and performance limits

The historical probability diagnostic and the native finite enumeration
distinguish candidate occurrences from distinct candidate sets: for h=6,
d=2 the counts are 420 and 262; for d=3 they are 1680 and 662. These two
finite checks do not establish a general probability law or asymptotic cost.
The existing W12 engine already reports the distinct-versus-multiplicity
distinction; this archive claims no new counting theorem. No new general
mathematical result or Lean proof is claimed.

Historical probability samples are separate scalar calibrations, not EC
timings. The old wall times are Python reference measurements against a simple
three-part Floyd rho. Their host, CPU affinity, isolation, A/A calibration and
strongest matched rho evidence are missing. They do not satisfy the current
validation gate, and they establish no native performance result or speedup.
Preserving those raw values does not admit them as benchmark rows.

No `VALIDATION.json`, leaderboard row, benchmark-index row or progress-metric
series is added. No new timed experiment was run for this import. A performance
comparison remains open and would require the repository's native protocol,
strongest matched rho reference and required experiment environment.

The canonical index-calculus scoreboard, IC progress timeline, IC leaderboard
and performance-gains figures were checked for relevant APS/W12 entries.
This import changes no admitted measurement, incumbent or graph coverage, so
they remain unchanged. The visual below compares finite candidate counts,
not runtime; the `benchmarks/` performance-publication rule is not triggered
by a new performance run here.

## Provenance and reports

- [HISTORICAL.sha256](HISTORICAL.sha256) covers the 19 original files. Import
  copies were compared byte for byte against the original bundles.
- [APS historical README](historical/aps/README.txt) and
  [guess-and-determine historical README](historical/guess_determine/README.txt)
  retain original instructions and limitations. Their Python reproduction
  commands are provenance, not the active repository workflow.
- [Evidence report PDF](reports/report.pdf), [editable report source](reports/report.typ),
  [candidate-count graph](reports/candidate-counts.svg),
  [classical route diagram](reports/classical-routes.svg) and
  [rendering-only source](reports/render_visuals.py) accompany this import.
- The original result bundle is [results.json](historical/guess_determine/results.json).
  The APS comparisons and unsuccessful capped attempts remain beside their
  original scripts; nothing is removed to improve the reported success rate.
- [Task-only session record](validation/SESSION.md) records development work and
  open checks. Automatic approval review rejected upload of the shared daily
  log because of unrelated infrastructure details; that shared file is excluded.

Flow for this import: 19 files planned and copied; 60 saved instances replayed;
six complete witnesses replayed; six native fixture solves requested; zero new
performance cases planned or attempted. Receipts distinguish completed checks
from these requested checks. No historical case is excluded.

Conductor checks were attempted before edits. The client could not reach its
local control plane (`localhost:8080`, connection refused), so no task lease
was obtained. Work was kept on an isolated branch in this study directory,
the new replay example and the append-only daily session log; this service
availability gap is recorded rather than represented as a successful check.
