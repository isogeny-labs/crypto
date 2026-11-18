# Native N83 K4096 terminal phase diagnostic

The frozen native x86-64, five-summand, signed-Frobenius K4096 diagnostic
completed under its declared caps. The model entered SAT and reached exactly
1,000 conflicts, then returned `UNKNOWN_solver_cap`. One public relation
query was attempted; it produced zero SAT models and zero relation rows, so
the observed matrix rank was zero. The query was independently replayed as
`[a]G+[b]Q` against public fixture 0. This single capped trial does not
estimate natural relation yield, select a factor base, or measure complete
index-calculus runtime. `selected_best_total_runtime` remains null.

The executed [protocol](PROTOCOL.md), [input-path amendment](AMENDMENT-1.md),
and [panel-staging amendment](AMENDMENT-2.md) are retained. The successful
runner job was `20261019-020823-n83-k4096-phases-final-e51bdf`, on pinned
commit `8856f2eb36e1aa132af2c0c20c9d422789ab0774`, native `c7i.xlarge`,
one CPU Docker limit, 4 GiB hard memory, zero swap, and no container network.
The worker's exact S3 archive SHA-256 is
`ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad`.
The runner finished normally and the instance terminated. The complete raw
runner output is under
`s3://ephemeral-runs-104186811093-us-east-1/jobs/20261019-020823-n83-k4096-phases-final-e51bdf/`;
the compact [native receipts](native/) and a SHA-256 manifest are committed
here. The factor-base archive's long-lived original and setec S3 custody is
recorded in the [frozen protocol](PROTOCOL.md).

| Measured phase, relative to decomposition entry | Elapsed |
| --- | ---: |
| 339,968-x domain ready | 91.270 ms |
| 4,096-representative Frobenius closure ready | 336.925 ms |
| 1,773,110 domain clauses counted | 400.720 ms |
| 679,936-point index ready | 629.181 ms |
| core encoding built | 644.387 ms |
| 131,958-variable, 2,021,197-clause, 4,980-XOR model installed | 780.945 ms |
| SAT entered, 1,000 conflicts available | 780.977 ms |
| SAT returned unknown at exactly 1,000 conflicts | 138,237.881 ms |

Thus the measured solver interval from `solver_enter` to `solver_unknown`
was 137.457 seconds. Importing the retained base took 7.624 seconds;
the worker's pre-summary wall was 148.223 seconds, with 751,669,248 bytes
of peak cgroup memory. The separate runner `rusage.txt` covers checkout,
compilation, Docker setup, and the worker; it is not a one-CPU solver timing.
The host job elapsed from its recorded start to done was about 10 minutes
55 seconds. The worker cap was 600 seconds and did not fire; the SAT
conflict cap did.

The [completed-run audit](native/completed-run-audit.json) verifies the
pushed source snapshot byte-for-byte, the staged archive and every panel and
fixture SHA-256, the declared cgroup configuration, all 12 flushed phase
markers, the terminal conflicts, summary counters, and the independent
public-group query replay. A second local replay of the query and phase
stream passed. Four altered inputs—query x-coordinate, installed variable
count, terminal conflict count, and archive byte—were rejected
([controls](native/mutation-controls.json)). The cloud phase audit was rerun
locally, and the existing Python attempt replay also passed. The positive
small-field trace and required Rust/Python test receipts are in
[validation](VALIDATION.md).

The local Bash 3.2 launch initially stopped before creating an instance.
The first paid instance then failed before compilation because its role
could not read the factor base outside `jobs/*`; the next paid instance
read the object but failed before compilation because the staging list
omitted a required panel sidecar. Their [status and logs](launch/) remain
separate from the successful trial. Neither failure supplies a relation
or SAT outcome.

The next factor-base comparison needs matched source and resource conditions,
more than one public natural query per candidate, useful independent rank,
column-log and individual-log phases, and a matched rho reference. The
current K4096 receipt settles where the short native query spent its time:
SAT search, after a completed model build. It does not settle which base
minimizes fully charged runtime.
