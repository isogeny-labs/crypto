# Archive-backed IC and frozen comparison protocol

This change connects the native factor-base factory to `ic bench`, audits
relations before elimination, and freezes the finite comparison in
[archive-comparison-v1.json](archive-comparison-v1.json). The protocol is a
preregistration. Acceptance tests establish import and accounting correctness;
no candidate has been selected by the comparison.

| Requested deliverable | Implementation and acceptance |
|---|---|
| Archive-backed IC adapter | `archive::{PrimeArchiveBase,BinaryArchiveBase}`; exact identity, signed point order, hash and recipe replay tests |
| Useful-rank accounting | `useful_rank::{AuditedMatrix,replay}`; verified equations, rank increments, dependencies, duplicates, replay and tampering tests |
| Frozen comparison | Versioned JSON with fixed grid, discovery/confirmation seeds, caps and selection gate |
| S3-backed reuse | Existing factory fetch into a verified local cache; IC workers need no AWS credentials |
| Review and merge | PR CI, current-head review and guarded merge; validation recorded on the PR |

## Import contract

The factory now exposes the same verifier used by its CLI as a library. The
adapter verifies the manifest address, every shard's hash and length, the
normalized recipe and regenerated ordered point set. It checks the exact field,
modulus, model coefficients, asserted group order, prime subgroup order,
cofactor and generator against the native instance. Native subgroup checks
run again during import. The manifest retains the factory's validation status:
an asserted group order does not become independently proved by importing it.

`--archive-store` and `--archive-manifest` construct the exact native instance
from the manifest. They cannot be combined with generated-instance flags, a
sweep file or `--auto`: automatic planning could replace the selected archive.
Choose the oracle and relation matrix explicitly. To compare a native control on that same instance, supply its
`--factor-base` explicitly and enable `--audit-rank`.

The adapter preserves signed-point order. `--archive-symmetry` selects `none`,
`negation` (the default), `frobenius`, `cubic` or `quartic`; the equivalent
factor-base parameter is `archive:symmetry=...`. Every canonicalisation returns
`C = [mu]P`; import verifies that equation on every point and assigns the
coefficient `mu^-1 mod r`. A false or noninvertible coefficient is rejected.
Binary subspace metadata is available only for a single unshifted space;
shifted/union/orbit archives remain usable by table oracles without inventing
an algebraic encoding. Prime fields must be below `2^62`, binary extension
degrees at most 62, and subgroup orders must fit `u64`. Wider archives remain
valid factory artifacts but receive `unsupported_archive_width` here.

```sh
# Fetch by immutable digest on the trusted controller. PREFIX must be the
# prefix recorded in the archive's publication receipt.
factor-base fetch /tmp/fb-cache MANIFEST_SHA256 setec-astronomy-artifacts PREFIX
ic bench --archive-store /tmp/fb-cache --archive-manifest MANIFEST_SHA256 \
  --oracle subtract --targets random --linalg incremental-gauss-full-rank-checked \
  --max-trials 100000 --seed 1001 --rho-runs 0 --json
```

Use the repository's normal `ic` output flags; the report contains the archive
manifest in its pipeline spec. A live fetch is required only when the archive
is absent locally. It never changes the archive or its historical receipts.

## Useful rows and replay

For each base point mapped as `P_i -> (column j, coefficient c_i)`, audit setup
constructs the column point `C_j = [h / c_i]P_i` in the order-r subgroup and
checks that every member agrees. It checks subgroup membership of columns,
target and generator. Each matrix row is admitted only after verifying
`sum(row[j] C_j) + row[target] Q = [rhs]G`. A false equation is rejected before
it reaches elimination and invalidates the run's verification result.

For a native cofactor-projected base, a zero coefficient is accepted only after
checking `[h]P = O`. `cofactor_kernel_points` records these known-zero
contributions; they create no columns and no independent relations. A zero
coefficient on any nonzero projection is rejected. The optional field defaults
to zero when replaying older v1 receipts.

`independent_rows` counts verified rows increasing the coefficient-matrix rank,
including the target column. `base_rank` is the rank of the projection onto base
columns. That projection does not certify reusable factor-base logarithms.
`dependent_rows` counts verified rows adding no rank. Duplicates are normalized
full equations, including the right-hand side, so scalar multiples count once.
These quantities are separate from decomposition hits and final recovery.

The report retains row coefficients, right-hand side, group verification,
duplicate status and before/after ranks. It also binds the subgroup, cofactor,
generator key and target key. `useful_rank::replay` requires the exact archive
and workload context and recomputes the report from fresh elimination; changing
a row, counter, rank or context fails replay. Reports are not signatures or
independent-host certificates.

The receipt/duplicate prefix has an 8 MiB payload budget and audit supports at
most 4096 columns. When the prefix fills, rank and group-verification counters
continue, `transcript_complete` becomes false, and `duplicate_checks_omitted`
counts the unchecked tail. `duplicate_rows` then describes a checked prefix,
not an exact whole-run duplicate total. Replay refuses truncated transcripts.
Use an incremental Gaussian solver; deferred sparse solvers are refused for
this audit rather than assigned misleading per-row ranks.

## Cost boundaries

Import reports bytes read, local archive replay time and index time, plus an
explicit missing original-generation/transfer counter. Native subgroup checks,
column-map checks and per-row group verification contribute counted group work.
The base-projection elimination contributes counted row operations. Setup wall
time is added once; per-row audit time is already inside linear algebra.
Hashing, factory recipe replay, indexing and normalization lack calibrated
operation prices. Audited reports expose `cost_qualification`, retain partial
counted cost, and leave `s_over_rho` null. These numbers cannot select a complete
cost winner. Cold accounting must later add recipe search, original generation,
validation, PUT/GET, bytes, transfer and all unsuccessful attempts. Online timing
and setup amortization must be separately specified.

## Frozen comparison rules

The JSON freezes curve constructors against a source revision, recipe grid,
resource limits, arity, targets, discovery/confirmation split, and selection
rule. Before the first discovery query, materialize and register exact curve
identities, freeze all full CurveSpecs, candidate manifests and public workload
points, and seal their hashes with the protocol hash. An unavailable constructor
or empty base remains a failed stratum; no substitute is chosen after inspection.

Compare only exact retained-column matches within the same curve and oracle.
Unmatched sizes remain descriptive. Require group-valid rows and complete
transcripts; preserve exhaustion, timeouts, memory failures and unsupported
regularity as censored or unavailable, never as zero cost or wins. Retain at most
two discovery finalists by the frozen Pareto/tie rule and retain the control.
Freeze finalist hashes before opening confirmation results. Any recipe change
requires a new protocol version and fresh held-out panel.

The regularity measurement is the factory's exact homogeneous leading-form
metric within its cap. It is distinct from affine solving degree and runtime;
table-oracle rank results do not measure regularity. Prime-field algebraic
regularity and unsupported encodings stay unavailable. The confirmation gate
in JSON is a screening rule, not a statistical significance claim. An IC/rho
speedup requires the repository's separate `ecbench` one-public-target protocol,
complete matched resource accounting and independent replay certificates.

## Curve families and extension points

Field, curve model, symmetry, base recipe and relation strategy are separate
choices. A family label never enables a map by itself.

| Family | Archive import and symmetry | Additional requirements |
|---|---|---|
| Prime-field short Weierstrass | `none`, `negation` | Exact p, a, b, order, subgroup, cofactor and generator |
| Prime-field j=0 | Also `cubic` | Rational cubic automorphism and verified subgroup eigenvalue |
| Prime-field j=1728 | Also `quartic` | Rational quartic automorphism and verified subgroup eigenvalue |
| Generic binary, prime or composite extension degree | `none`, `negation` | Exact irreducible polynomial and coefficients; prime extension degree does not imply extra symmetry |
| Binary Koblitz K0/K1 | Also `frobenius` | Coefficients over F2, exact point-count recurrence, eigenvalue verified on the imported generator, and archive closure under signed Frobenius |
| Anomalous or other special-order prime curves | Generic prime import when the native subgroup contract holds | A special order does not select a special-purpose solver |
| Odd-characteristic extensions, GLS, other subfield models, Edwards/Montgomery, Jacobians | Additional backend/codec required | Versioned exact model and field encoding, counted group operations, verified maps and native scalar-width support |

Koblitz certification uses the manifest's field representation and generator.
The current specialised plugins additionally require a full signed action of
order `2n`: a shorter Frobenius period or a Frobenius power equal to negation
stays on the generic binary path, with no assumed `2n` reduction.
It never replaces them with the repository's catalog representative. A
Frobenius archive must already contain every signed orbit member; import
rejects incomplete orbits instead of adding points to a hashed archive.
The prime cubic/quartic column fold can group the retained members of an orbit
without enlarging the point set; this does not claim that the set is closed or
that a specialised orbit table can use it.

`CountedGroup`, `FactorBaseBuilder`, `DecompositionOracle`, `SystemSolver` and
`RelationSolver` remain the stage interfaces. A new column symmetry implements
`RhoClasses` and calls the public `archive::import_with_symmetry`, which checks
its coefficients independently. New models must also bind the native arithmetic
to the full manifest identity and decode points exactly. The v1 archive codec
supports the two listed Weierstrass models; other models need a new explicit
codec/schema, not a misleading family string. The current u64 scalar/context
and rank-receipt contracts must be widened together for wider native backends.

## Relation-finding options and compatibility

`ic bench --list --json` lists the stage plugins and the `archive_techniques`
capability catalogue. The options below compose only when their stated gates
hold; an option is not a promise of better yield or lower solving degree.

| Technique | Available path | Accounting and gate |
|---|---|---|
| Negation / Frobenius / cubic / quartic column fold | `--archive-symmetry` | Same immutable points; exact coefficients and resulting column count |
| Direct subtraction and pair-table MITM | `subtract`, `mitm:m=3` | Any supported native group; table setup charged |
| Frobenius-folded pair tables | `mitm-frobenius-counted:m=2` or `m=3` | Certified Koblitz archive, `symmetry=frobenius`, complete signed orbits |
| Single / double large prime | `--large-primes 1` or `2`, `--lp-domain-factor` | The m=3 Frobenius collector; bounded disjoint residual domain, exact residual elimination and verified completed rows |
| Walk targets | `--targets walk` | Tracked target coefficients and restart costs |
| Batched prime tables and algebraic/root-finding solvers | Registered oracle and solver parameters | Curve/domain compatibility checked by the selected plugin; solver budget exhaustion retained |
| Coordinate, random-linear, geometric, trace-zero, shifted/union and orbit bases | Factory recipe plus immutable archive | Field/recipe validation; algebraic encodings are separate from table compatibility |
| Rational 2-torsion symmetrisation | Native `koblitz-symmetrised` base with `symmetrised` oracle and matching divisor | Explicit alternate base on the imported exact instance, with `--audit-rank`; its coordinate frame and cofactor equations are required |
| CM maps, GLS/subfield combinations, isogeny/cover transfers, other residual collectors | Extension interfaces | Require a verified subgroup action or transfer and complete relation/accounting integration; not automatically enabled |

For example, after fetching a closed Koblitz orbit archive:

```sh
ic bench --archive-store /tmp/fb-cache --archive-manifest MANIFEST_SHA256 \
  --archive-symmetry frobenius --oracle mitm-frobenius-counted:m=3 \
  --large-primes 2 --lp-domain-factor 3 --targets random \
  --linalg incremental-gauss-full-rank-checked --rho-runs 0 --json
```

Here a "large prime" is a residual subgroup point outside the factor base.
Partials and residual pivots are reported separately. Only equations whose
residuals have been eliminated, and whose complete group equation verifies,
reach useful-rank accounting. A single-large-prime control uses `1`; the direct
control uses `0`. These flags are rejected on oracles that do not implement
them. Generic prime-field and generic binary residual collectors are not yet
registered; their algebraic feasibility is separate from implementation.

Translation by a torsion point is not a scalar endomorphism on the target
subgroup. The torsion-aware path uses its own frame, exact base and cofactor
projection. Raw torsion points are not smuggled into the subgroup-only factory
archive or counted as independent logarithms. More general torsion-coset or
multi-map constructions require explicit domain/projection data and replay.

The Frobenius construction follows the applicability conditions studied in
[Galbraith, Granger, Merz and Petit, On Index Calculus Algorithms for Subfield Curves](https://pure-oai.bham.ac.uk/ws/files/104974339/On_Index_Calculus_Algorithms_for_Subfield_Curves.pdf).
Double-large-prime graph methods have a separate literature, including
[Diem, On the discrete logarithm problem in class groups of curves](https://www.ams.org/journals/mcom/2011-80-273/S0025-5718-2010-02281-1/home.html);
the native collector's applicability and benefit must be established on its
actual elliptic-curve workload rather than inherited from a different family.

The v1 JSON and its hash remain unchanged and retain their negation-only grid.
[archive-technique-comparison-v2.json](archive-technique-comparison-v2.json)
separately freezes the symmetry, folded-table and large-prime ablations, plus a
descriptive native torsion-coordinate control. It adds j=0, j=1728 and a
composite-degree generic binary stratum, binds the inherited v1 grids/holdouts
by hash, and requires full identities and runner hashes to be sealed before
discovery. Symmetry comparisons hold the archive point set fixed and report
the changed required rank; raw rank counts with different column counts are
not a common yield denominator. No discovery or confirmation run is part of
these implementation acceptance tests.
