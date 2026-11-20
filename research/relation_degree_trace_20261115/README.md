# Which redundant relations lower the polynomial-solving degree?

Native Rust research instrumentation for exact prime-field polynomial systems.
The active path has no Python dependency. It imports custom equations, builds
certified redundant consequences, and traces individual additions, removals,
and greedy selection on the same affine ideal.

The current [compact-certificate follow-up](COMPACT_REPORT.md) integrates the
real Rust F4 codebase and freezes a [three-arm 15x target](COMPACT_SPEED_PROTOCOL.md).
The existing six-variable case retains 442 local cofactor terms instead of
20852 expanded final-basis terms, with the same complete basis on both fields.
This is a storage/correctness observation; the new runtime comparison has not
run `[pcc:unverified]`.

The earlier standalone result is in [FINAL_REPORT.md](FINAL_REPORT.md). Five of six
predeclared fresh four-variable cases pass the per-case tenfold rule, with
23.4–23.9× native checked-wall median ratios. One failed A/A control is excluded.
These are local producer component observations, pending signed independent
validation; mature CAS diagnostics are retained separately `[pcc:unverified]`.

[LEAD_STATUS.md](LEAD_STATUS.md) records the next bounded work: a direct-quadratic
projection with explicit certificates, a selectable strong native baseline,
and a separately frozen [successor protocol](QPROJECTION_PROTOCOL.md).
Development checks are under `quadratic-development/`. No successor timing
has run: the required S3 checkpoint upload currently has no transport.

The stronger [quotient-algebra prototype](QUOTIENT_PROTOCOL.md) is now
implemented and passes exact development checks. It constructs coordinate
maps and the complete basis without a reference-basis oracle. Its
[future timing protocol](QUOTIENT_SPEED_PROTOCOL.md) uses the best existing
native route as baseline. No quotient speed ratio has been measured; see
`quotient-development/` for the checked scope and limits `[pcc:unverified]`.

The algorithms now live in the shared `crates/certified-relations` Rust crate.
This study reexports that implementation, and the main codebase exposes it
through `crypto_lib::cryptanalysis::f4_fp::certified`. The default F4 engine
keeps its existing behavior. `LEADS.md` is the exact frozen early claim subject;
use LEAD_STATUS.md for current notes. See INTEGRATION_HYPOTHESIS.md for the
codebase integration hypothesis and qualification scope.

[INTEGRATION_REPORT.md](INTEGRATION_REPORT.md) records the tested native API,
the retained proof-budget failures and the fixed-cap breadth-first result.

## Run

From this directory, with a current stable Rust toolchain:

```sh
cargo test --release --locked
cargo clippy --all-targets --locked -- -D warnings
cargo run --release --locked -- --out /tmp/relation-degree-new-run
cargo run --release --locked -- \
  --input fixtures/system-2147483647.json --out /tmp/relation-degree-custom
```

Use a fresh output directory; the CLI refuses to overwrite existing runs.
Defaults: the two fixed fixtures, degree cap 8, eight candidate consequences.
`--max-degree` and `--candidate-limit` override these values (maximums 16, 32).
The coefficient moduli are **2147483647** and **9007199254740881**, a prime
below 2^53. The integer 2^53 is not a prime modulus. Imported systems require
1–16 variables and a prime modulus at most i64::MAX; arithmetic uses exact
u64 residues and u128 products. Primality uses deterministic u64 Miller–Rabin.

Outputs: `trace.json` (full degree/candidate profiles and membership
certificates), `degrees.csv`, `relation-effects.csv`, and `relation-effects.svg`.
The SVG displays measured Hilbert degrees only; untraced degrees are blank.
No plot zero is inferred from an exhausted degree or column budget.

Input JSON uses `prime`, `variables`, and `polynomials`, with each polynomial
an array of `{ "exponents": [2,0], "coefficient": 1 }` terms. Signed integer
coefficients are reduced modulo p; repeated terms are combined. There is no
expression evaluation. See the fixtures for complete examples. Candidate
relations in the default tracer come from the baseline scout. The new
`cheap` module separately recognizes paired f3 inputs and constructs the
useful cubic directly, with exact certificates and no scout. See FOLLOWUP.md.
An external geometric candidate/certificate importer remains a follow-up.

## Exact definitions and completion checks

At matrix degree D, include inputs of degree at most D and their monomial
multiples of degree at most D. Reduce with grevlex-compatible sparse row
elimination. Recycle every rank-increasing residual as a new source, and
repeat multiplication and elimination until that degree closes. Rank alone,
variable pinning, and sparsity do not imply completion.

`solving_degree` means the first completed degree at which this affine closure
contains the reduced grevlex Gröbner basis of the original ideal. A separate
textbook Buchberger path computes that reference, with original-input
membership certificates. Its inputs reduce to zero, and every S-polynomial
reduces to zero. The development tests additionally compare its complete basis
against the frozen historical SymPy basis. The reference never injects rows
into the closure computation. These are separate algorithms written by the
same producer, not independent signed validation.

The trace exposes submitted rows and their nonzeros, ambient columns, rank,
new pivot degrees, elementary row reductions, degree falls, closure rounds,
and reference-basis containment. A fall means a nonzero submitted row reduces
to a nonzero polynomial of smaller degree. It is an observed row event, not
a certified first- or last-fall invariant.

The trace's matrix threshold is distinct from both maximum final basis degree
and expanded certificate degree. Each candidate carries an exact identity
`g = sum h_i*f_i`; record both the matrix degree at which it was found and
`max_i(deg(h_i)+deg(f_i))` for the stored certificate. Multiplication after a
fall can make the latter exceed the matrix threshold.

For regularity, form J from the highest homogeneous parts of the supplied
inputs and compute H(S/J,D) by homogeneous Macaulay ranks. `top_form_d_reg` is
the first observed D with H=0: the Bardet–Faugère–Salvy vanishing definition,
when its finiteness hypothesis holds. `null` means no vanishing was witnessed
within the budget, not a numeric lower bound or a proof of nonexistence.
It is **not** Castelnuovo–Mumford regularity of the affine ideal or its
saturated homogenization. Adding a redundant equation preserves the affine
ideal but can change this generator-dependent top ideal and the solver work.

The original degree-tracing CLI produces no solver wall-time speed ratio. Its reference construction timer
is diagnostic only. The closure reports deterministic row counts and degrees;
they include provenance work conceptually but are not complete field-operation
or pipeline cost accounting. In the initial scout mode, discovery is the whole baseline closure; its
rows and peak matrix degree remain visible beside downstream selection.
The follow-up direct constructor avoids that scout. Its separate full-basis
timing campaign is reported in FINAL_REPORT.md; degree alone does not predict
that campaign's cost.

## Fixed elliptic-curve fixtures and observed development checks

The fixtures are two equations f3(x1,x2,c_plus), f3(x1,x2,c_minus), where

```
f3(x,y,z) = (x-y)^2*z^2
            - 2*((x+y)*(x*y+a)+2*b)*z
            + (x*y-a)^2 - 4*b*(x+y)
```

on y^2=x^3+2x+3, with G=(3,6), R=2G, S=3G, c_plus=x(R+S),
c_minus=x(R-S). The fixtures are unchanged from the historical prototype.
Native regression tests check this formula's coefficient expansion and the
recorded witness as well as the reference bases and provenance identities.
This has no factor-base constraint, P/Q coefficient tracking, or x^p-x field
equations. It measures polynomial geometry, not the cost of recovering a
logarithm or of isolating all F_p solutions. The coefficient field does not
make a two-variable fixture a cryptographic-size decomposition system.

The following are **producer development observations**, with no signed
independent verdict `[pcc:unverified]`:

| Supplied consequences | Closure threshold | Top-form d_reg |
|---|---:|---:|
| None | 5 | not witnessed |
| Cubic c0 | 5 | not witnessed |
| Cubic c1 | 4 | 6 |
| c1 and c0 | 4 | 4 |

The same values were observed on both fixed primes. Removing c1 from the full
candidate set raises the threshold to 5; removing any other one leaves it at
4. Both cubics have degree 3; their effects differ. c1 was discovered at matrix
degree 5, with stored expanded certificate degree 6. The discovery-inclusive
peak matrix degree stays 5. These are fixture-specific observations, not a
claim of reduced end-to-end work `[pcc:unverified]`.

The raw baseline top ideal is <x1^2*x2^2>, so its positive-degree quotient
Hilbert function stabilizes at 4. The table's missing finite vanishing index
must not be read as infinite Castelnuovo–Mumford regularity.

## Provenance, prior work and accuracy corrections

- `historical/` preserves the original Python source, test source and output
  bytes. It is not an active execution path or independent validation.
- `development/` contains the native development smoke output and test/lint
  receipts. These are producer checks, not a new timed research campaign.
- `PROTOCOL.md` freezes the development acceptance checks and resource caps.
- Interpretive record: `PCC-20261115-5b7981`; measurement record:
  `PCC-20261115-e6d18a`. Numerical diagnostics remain
  `[pcc:unverified]` pending independent PCC validation.

The native path corrects prototype generality/reporting issues: imported
variable names do not enter a symbolic parser; zero Hilbert values are not
invented beyond a truncated trace; row/source budget interruptions retain an
incomplete degree and its attempted rows; a capped solve is never assigned a
finite threshold; prior output cannot be silently overwritten.

Sources checked (KB): unavailable (MCP not loaded and repository wrapper exits
127: no executable). Fallback sources read before the native approach:

- [Degree reporting correction](../degree_reporting_20260922/README.md): rank
  proxies and pinning are not basis completion.
- [Guarded degree invariants](../guarded_degree_invariants_20261030/REPORT.md):
  filtered closure, top-form finiteness, and the CM-regularity distinction.
- [Existing forecaster](../../src/cryptanalysis/dreg_forecast.rs): predictions
  must remain distinct from exact traced values.
- Caminata–Gorla, [arXiv:2112.05579v2](https://arxiv.org/html/2112.05579v2),
  Definitions 1.1, 1.4, 1.5 and the degree-compatible closure equality:
  definitions used here, no first-fall heuristic or novelty claim.

## Limits and delivery scope

| Requirement | Status/evidence |
|---|---|
| Trace matrix degrees, ranks and falls | implemented; development tests |
| Measure effects of individual relations | single-addition, all-addition and leave-one-out runs |
| Track top-form regularity | exact truncated Hilbert ranks; explicit null/cap status |
| Requested fields | fixed fixtures at 2^31-1 and the stated 53-bit prime |
| Native path and certificate checks | Rust library/CLI; no runtime Python |
| Independent signed verdict | pending; producer observations remain unverified |
| Direct discovery of the cubic for recognized f3 pairs | implemented without a scout; timed follow-up in FINAL_REPORT.md |
| Hook into factor-base/P,Q collection | follow-up; not supplied by this fixture harness |

The stand-alone crate avoids changing the repository's production library,
existing result records or dependency graph. Its own release suite and lint
checks are in scope; the full root-library suite was not run from this API-
staged partial source tree. No leaderboard, benchmark, progress KPI, ecdb or
priced dashboard entry is promoted: this is a documented instrumentation
exclusion, with no frozen IC/rho cost, reference, curve registry entry, or
validation status changed.

The independent reference computation has pair/basis caps but no wall-time or
term-expansion cap; arbitrary imported systems may still be expensive before
tracing. Closure limits are 5,000 columns, 2,000 sources and 200,000 submitted
rows per run; interruption reports no completed threshold. Greedy selection
is bounded, lexicographic, and does not certify a globally best subset.

## Direct-construction speed experiment

[FOLLOWUP.md](FOLLOWUP.md) derives the short pair syzygy and records native
development checks. [SPEED_PROTOCOL.md](SPEED_PROTOCOL.md) freezes a tenfold
target, matched accounting, held-out shared-point fixtures, A/A controls and
failure rules. Successor protocols, all losses and fresh confirmation are
linked in [FINAL_REPORT.md](FINAL_REPORT.md) `[pcc:unverified]`.

```sh
cargo build --release --locked --bin compare --bin campaign
target/release/compare --prepare /tmp/relation-speed-new-fixtures
```

The `compare` worker computes a full Gröbner basis from scratch; it does not
use the tracer's completion oracle as a free solver. `campaign` preserves
all attempts, pins one CPU, enforces worker time/address-space limits, and
checks exact returned-basis equality. Execution used the offered local L0
backend after the user's instruction to continue. No cloud run was launched.
Supported Singular and msolve basis replays are complete on the fresh panel;
their diagnostic clocks and checking obligations differ from native charged
wall intervals. No fastest-solver claim is made.

[Current native Rust handoff](RUST_INTEGRATION_RESUME.md) records the integrated
API qualification and the remaining matched-runtime gate.
