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

The adapter preserves signed-point order and folds negation into columns with
coefficients `+1` and `-1 mod r`. It does not infer a Frobenius or GLV coefficient
map. Binary subspace metadata is available only for a single unshifted space;
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
