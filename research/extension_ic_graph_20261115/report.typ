#set page(paper: "a4", margin: 19mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Cubic-extension IC: graph elimination integration
15 November 2026 · Implementation note · No benchmark observations

== Construction
The native adapter connects the existing cubic-extension algebraic relation
finders to the repository's symbolic large-variable forest. Both the pivot
baseline and forest candidate use the same target, factor base, oracle and seed.
The comparison API protects the target logarithm and a constant coordinate.
These statements describe the implementation; independent sign-off is pending.

#figure(image("integration.svg", width: 100%), caption: [Integration boundaries.
This is a construction diagram; no measured gain is drawn.])

== A separate boundary correction
The historical collector places the target logarithm at column `small`, while
its pivot eliminator treats columns at or above `small` as auxiliary. The new
comparison path protects `small + 1` columns for both backends. The historical
entry point remains available for frozen replay. A gain from the corrected
boundary must not be attributed to the graph replacement. [pcc:unverified]

For a relation `sum(c_i * x_i) = rhs`, the adapter sends
`sum(c_i * x_i) - rhs * ONE = 0` to the existing forest. Its protected symbolic
variables are the ordinary base logarithms, the target, and `ONE`. Returned
constraints are converted back to inhomogeneous rows. Constructor primality
and graph-merge multiplication counts are retained. [pcc:unverified]

== Planned comparison and controls
Amendment 1 adds a Rust symmetric-coordinate S3 pair oracle and a hybrid triple
variant. With s = x1+x2 and t = x1*x2, the two nonconstant extension components
are linear in t. Their eliminant has degree at most three. GCD compatibility
with the constant component precedes root extraction; vanishing denominators
and dependent equations retain explicit recovery or counted scan paths.
The filtered release run reports 30 passing tests for this continuation.
These are producer-run correctness checks. [pcc:unverified]

The native driver uses the existing Weil-restricted S3 pair oracle,
symmetrised S4 solver, and hybrid scan/S3 solver. It interleaves two identical
pivot controls with the graph candidate, reversing arm order on odd rounds.
Both symmetric variants are included as additional oracle blocks. Every IC arm
rebuilds its factor base and starts from the same seed. Actual cubic r-adding
rho A/A arms use the same instances. A retained warmup precedes measured
rounds. Raw JSONL is create-new; failures and scalar replays are retained.

The frozen exploratory cohort has p = 7, 13, 19, 31 and seeds 3, 7, 11, 23,
with five rounds. A separate p = 43, seeds 101 and 103 cohort is held out.
The resource ceiling is USD 5, four CPUs, 8 GiB, a 900-second deadline and no
automatic retries. Submission is pending authenticated checkpoint transport,
published source and a tested EKS command image/admission. These are planned
inputs and limits, not observations.

== Accounting and evidence limits
The existing multiplication-equivalent model is reported only as a diagnostic.
Scalar additions/inversions and native collection work outside the graph's
meter remain incompletely priced. Raw clocks need EKS/L2/A/A admission. No
complete native-cost ranking, external-tool comparison or speedup is claimed.
No leaderboard measurement changes are supported by this packet.

== Local development checks
The earlier integration's filtered release-library run reports 24 passing Gaudry/cubic-graph
tests, including the target-boundary regression, actual-oracle prefix/scalar
comparisons and input-fingerprint equality. The touched example compiled in its
Cargo test target before that control; it defines no example-local tests.
That head's example also passed its release type check. The earlier full default-feature library
run reported failures outside the touched module and was interrupted after
more than twelve minutes of test execution; it is not a passing whole-library
result. A missing archived fixture was reproduced, and one device case passed
after restoring tracked emulator assets. Other failures remain unclassified.
These are producer-run checks, not independent signed verdicts.
[pcc:unverified]

The symmetric-oracle continuation passes 30 filtered release tests, including
exhaustive tiny-group target checks,
all constructed signed pairs, a non-binomial cubic fixture, hybrid-triple
comparison, and explicit dependent/vanishing-denominator regressions. Its
first test compile found archived include fixtures absent from this sparse
checkout. Exact tracked fixtures were restored before the passing run.
The continuation's release example type check also passed locally.
The earlier 24-test result remains evidence for the earlier source only.
[pcc:unverified]

Raw command output is retained under `validation/`. The checkpoint gate reports
no configured AWS profile or checkpoint upload token. No benchmark launcher
was invoked. [pcc:unverified]

Source baseline: `9e95f83262997336c1785ced4170b1edf8bd0d21`.
Editable protocol: `protocol.json`. Native driver:
`examples/gaudry_cubic_graph_compare.rs`. Adapter:
`src/cryptanalysis/gaudry_cubic_graph.rs`.

== Sources and reuse
- `src/cryptanalysis/gaudry_cubic.rs`: existing algebraic oracles and pivot
  elimination; target-column convention.
- `research/pencil_large_prime_20261029/src/lib.rs` and `src/meter.rs`:
  existing symbolic forest and semantic event accounting.
- `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md`, sections
  11.7 and 11.11: prior fill and merge-depth experiments.
- Gaudry, Theriault, Thome (2005), sections 2.3 and 3.1,
  #link("https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf")[primary paper]:
  full relations and graph cycles differ. Its higher-genus complexity theorem
  is not a theorem about this integration.
- KB unavailable: no MCP connection loaded and the repository wrapper had
  no executable. Repository sources and the primary paper were used instead.
