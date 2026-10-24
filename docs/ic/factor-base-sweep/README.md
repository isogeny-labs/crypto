# One-command native factor-base sweeper

See the [validation receipt](VALIDATION.md) and [four-page report](report.pdf)
for tested scope and current upstream integration blockers. Native validation
uses Rust 1.98.0.

Install the standalone native executable once from this checkout (Rust is
needed for this source-install step, not to launch an installed executable):

```sh
cargo install --locked --path . --bin factor-base-sweep
```

Put Cargo's installation directory (`$HOME/.cargo/bin` by default) on `PATH`.
Then launch from any directory, without Cargo or a source checkout:

```sh
factor-base-sweep run --out runs/my-bases --preset both
```

The release workflow also builds, smoke-tests and includes `factor-base-sweep`
in the Linux, Apple-silicon and Windows Rust archives. That download option
becomes available after this PR lands and a release succeeds; older archives
do not contain it. Extract the matching archive, verify its published checksum,
and put `factor-base-sweep` (or `factor-base-sweep.exe`) in a directory on
`PATH`. The executable starts its own native workers; no Cargo launcher or
language interpreter is used at runtime. Linux builds require compatible glibc.
Run `factor-base-sweep --help` to see its commands.

The preset uses **small native functional fixtures**: Koblitz `a=1,n=7`
and the repository's 16-bit prime roster. It tests geometric progressions,
scaled coordinates, symmetric exponents, bounded multiplicative-order
generators, Frobenius orbits (two strides), mixed product bases, random bases,
three trace-zero constructions, subfields, prime abscissae, Kummer towers and
isogeny-derived abscissae. A missing subfield is recorded as inapplicable.
Construction failures remain visible. A shape name alone is never evidence
that successful decomposition is cheap.

Expand the grid and control the work from that same command:

```sh
factor-base-sweep run --out runs/grid \
  --preset both --seed 42 --seed-count 4 --dimensions 3,4 \
  --train 32 --holdout 16 --max-degree 8 --candidate-seconds 30 \
  --max-abscissae 64 --max-matrix-cells 2000000 --memory-mib 2048 --threads 1
```

Seeds expand constructor recipes; all candidates on a curve get the same
seeded training targets and a separate matched holdout set. At most 1,024
candidate configurations are accepted. Different recipes can produce the same
point set: compare the curve identity and `factor_base.points_sha256` in the
native inventory when counting unique point sets. FB1 also binds family and
parameters; a different basis parameterization can have a different FB1 even
with the same points. Its algebraic system is still a distinct measurement
configuration. The coordinate basis in particular does not change with its seed.

For other existing native curve constructors or exact bases, first write a
plan, edit its explicit candidate list and budgets, then run it:

```sh
factor-base-sweep plan --out plan.json --preset binary
factor-base-sweep run --plan plan.json --out runs/custom
```

Each candidate declares `curve` (the native ecbench `CurveSpec`), a `label`,
`seed`, and either `shape` with `dimension`, or a `factor_base` constructor
string. An explicit binary recipe can name every basis word, for example
`binary-subspace:dimension=3,basis=0x1;0x2;0x4`. JSON plans retain the semicolons
literally. Invalid, dependent or out-of-field basis words are rejected.
Supplied public-target fixtures are refused: this search generates matched
public known-answer probes and does not solve a user-supplied discrete log.

## What the live lines mean

`Info:` lines are flushed as stages, candidates and trials complete. F4 emits
begin/matrix/complete events with variable count, degree, matrix dimensions,
productive degree, reduction multiplications and elapsed milliseconds. A
heartbeat reports the active stage even during construction or a long solve.
`--json-progress` replaces the human stderr lines with the same structured
events; stdout contains the final machine-readable run summary.

Relation-collection lines show:

- verified decompositions, refutations and exhausted probes separately;
- cumulative and last-eight-probe verified decompositions per second;
- independent training rank, columns and independent relations per second;
- provisional remaining-probe and rank-completion ETAs;
- provisional rate ratios to the first completed baseline and the previous
  completed candidate **on the same curve**, after at least four probes.

Rates cover the whole probe phase: finite-inventory correctness census,
system construction, F4, point-witness checks and incremental logging. They
exclude initial curve/base construction. A higher raw hit rate can still give
less independent progress. ETAs assume constant observed throughput and rank
yield; rank can stall. No ETA covers the complete discrete-log algorithm.
Unknown quantities print `unknown`, rather than zero seconds. Uncertainty
intervals and an accepted performance comparison are not implemented.

These are live operational observations on a shared host. They are **not**
isolated timing evidence or an accepted speedup. There is no automatic
total-cost winner: construction, root extraction and verification are not yet
fully priced under the symmetric native-operation bands. A future fastest
claim also needs the repository's matched external-reference gate.

## Algebraic scope and correctness

This backend uses **two summands (`m=2`)**. Binary systems use the existing
S3 descent on the exact linear subspace with Boolean field equations. Prime
systems use direct S3 plus exact finite-set membership polynomials; this is
not a low-degree tower/incidence formulation. Prime root extraction requires
`p <= 1,000,000`; binary shape construction requires field degree at most 31
and dimension at most 8. Extension fields, wide 83/130-bit targets, arbitrary
nonlinear binary inventories and `m>2` are not supported by this solver path.
Unsupported requests fail explicitly; no smaller substitute is run.

Each candidate must first solve a known decomposable positive control **in
the target prime subgroup**, checked by multiplying its point by the recorded
prime order. A base whose finite two-point sums never reach that subgroup is
retained as an unqualified failure. The finite search is counted in setup
point operations and remains under the candidate deadline; it is not free.
Completed records declare `positive_control_scope: target-prime-subgroup`.
The earlier source snapshot used any finite curve-point sum for its control;
its sealed records keep that historical scope. Upgrading changes executable
bytes, so resume requires the original binary as documented below. Random
train/holdout probes record independently checked finite-inventory truth.
Every accepted F4 root satisfies the original system; every reported
decomposition must pass actual point addition using the recorded inventory.
Only training witnesses enter the cofactor-projected relation matrix over its
prime subgroup modulus. Holdout/control rows do not affect training rank.
This validates relations, not a recovered discrete log.

`observed_solving_degree` is the maximum input degree and productive F4 step
degree over that solve's substitution tree. Refutations, successful
decompositions and exhausted solves are distinct. The best successful degree
uses training probes only. These are bounded procedural diagnostics, **not
certified degree of regularity**. No algebraic-closure certificate is produced.
Negative verdicts can use finite-field substitution/enumeration as well as F4;
their observed negative solving degree is not a certified algebraic refutation
degree. `certified_refutation_degree` stays null too.
`--max-degree` limits F4 critical-pair degree; it does not reduce the degree of
the original membership equations, which is retained in the observed diagnostic.
`certified_degree_of_regularity`, `total_cost_winner` and
`full_pipeline_speedup` remain null.

## Records, resume and verification

```sh
factor-base-sweep run --out runs/my-bases --resume
factor-base-sweep verify --run runs/my-bases
```

Resume requires the **same executable bytes and frozen plan**. It verifies
the recorded `F4_*` solver environment too; changing those overrides refuses
resume. Ratios to a candidate retained from an earlier resume segment are
marked explicitly, since host and load conditions can have changed. It verifies
sealed attempt hashes and skips finished attempts, including recorded failures
and caps. It retains an interrupted attempt and starts a new numbered attempt;
it never overwrites a completed inventory or transcript. To change budgets,
seeds or executable, start a new output directory. Concurrent writers are
refused. A torn final event-log line is preserved and requires explicit
recovery; it is never silently truncated. Exit zero means orchestration
finished; inspect each candidate's outcome in `summary-NNNN.json`.

The output retains `manifest.json`, durable `events.jsonl`, write-once
summaries, and each candidate's numbered attempts. Completed attempts include:

- exact ordered signed point inventory and native FB1 identity (`base.json`);
- recipe, explicit basis words, FBM1 measurement identity and an order Merkle
  root bound to the number of points (`derivation.json`);
- signed point-to-column/coefficient lookup (`lookup.json`);
- every exact system, target, verdict, degree/resource counters and verified
  witness, plus training rows and rank gains;
- captured worker stderr and SHA256/lengths of every attempt artifact.

These hashes detect changed files; they are not an authenticated digital
signature or a regularity certificate. `verify` additionally reconstructs the
native inventory, recipe, FBM1 and order root, checks systems, replays point
witnesses and the projected relation matrix. It does not rerun F4 or certify
the solver's negative verdict independently beyond the finite point census.

Each candidate has a supervisor deadline covering construction and solving.
Linux workers also have an address-space limit and die if their supervisor
exits. Other platforms report address-space enforcement unavailable. The
matrix threshold is enforced by killing the worker on an observed over-budget
F4 matrix event; it is asynchronous, not an allocation-time memory bound.
Timeouts, caps and unqualified positive controls never count as refutations.

The historical #1898/#1975 evidence, store objects and #1963 draft experiment
card are untouched. This CLI introduces no S3 operation or research campaign.
