# Native factor-base structure screening

Status: implementation and correctness validation; **no new research panel run**.
The requested goal of finding a factor base with lower verified decomposition
cost remains open. This addition makes the next comparison concrete without
calling a shape score a regularity certificate or a speed result.

## The lead selected after checking prior work

The lead atlas identifies `LEAD-ICE-linearization-excess-fb-prior` as
unexamined on this repository's descent cells. A historical sibling study
reports an empirical two-summand relation between solving degree and
`e = n - dim(V) - dim(V^(2))`. That study is not a native measurement of
this CLI. Its fitted thresholds are not imported as predictions.

The new direct native command computes the exact product-space dimensions
`dim(V), dim(V^(2)), ..., dim(V^(k))` from explicit polynomial-basis columns:

```sh
factor-base-sweep profile --input profile.json --out structure.json
```

An editable [input example](profile-example.json) is included. It specifies a
field, not a curve, target, subgroup or factor-base point inventory. The
profile can describe fields through degree 571; this does **not** extend the
sweeper's bounded solver to degree 83, degree 131, or more summands.

The command validates the modulus with Rabin's irreducibility criterion,
rejects zero/dependent/out-of-field columns, emits canonical bases for every
product space, and assigns a basis-invariant `FSP1h...` identifier to the
field/subspace. This identifier is **not** an FB1 point-base identity.
It writes a new output file and refuses to overwrite an existing one.
Progress is printed line by line. It requires no Cargo process at runtime.

`linearization_excess_m2` is an exact arithmetic expression, not a proven
solver-degree formula. `sum_product_dimensions` is a containment-space size,
not proof that all corresponding symmetric coordinates are independent or
that a symmetric solution splits into the original summands. The product
multiplication counter excludes modulus validation, squaring, elimination,
allocation and all point/solver work. It cannot price or rank whole methods.
Predicted solving degree, certified regularity and total-cost winner stay null.

## What prior work rules out or leaves open

- Merely preferring cofactor-compatible pairs does not supply a new scalable
  route: the current atlas records the cofactor-filter obstruction on the
  ECC2K-130 family. Cofactor classes still matter for correctness and matched
  subgroup support. They are a control, not a new gain claim here.
- Geometric progressions minimizing product dimension are known mathematics,
  not a discovery of this search. Bachoc–Serra–Zémor Theorem 3 applies to a
  prime-degree finite-field extension, dimensions at least two, and product
  dimension at most `n-2`. Do not extend its uniqueness conclusion past that
  range or to arbitrary composite-degree extensions.
- The previous native asymmetric study remains the reference for actual
  successful bounded solver work; its conditioned controls do not establish
  random-target yield or a complete speedup. Its sealed records are unchanged.
- Conservation of total decomposition multiplicity over **all** group
  targets does not imply equal coverage, subgroup-target yield or rank.
  Matching subspace dimension alone does not match usable projected columns.

The [literature record](LITERATURE.md) identifies source versions and which
claims were actually read. No novelty or fastest-tool claim is made.

## Next discriminating comparison

The [unrun protocol draft](PROTOCOL_DRAFT.md) pairs top-excess, median-excess
and random controls, requires actual subgroup-qualified inventories, and
keeps selection targets separate from fresh holdout. Its endpoint is
complete charged successful decomposition and useful independent relations,
not rejection rate. The m=2 fit cannot be relabelled as an m=3/4 degree rule.

The existing m=2 CLI can support a first native transfer comparison after
the execution gate is resolved. An m=3/4 experiment also needs its native
adapter and matched exported systems before a runnable protocol is frozen.
None of the previous draft cards is promoted or dispatched.

## Execution blocker and original goal

Current [AGENTS.md](../../AGENTS.md#resume-checkpoints-in-s3-before-every-run)
requires an S3 checkpoint before every experiment and says Codex must run
`scripts/checkpoint.sh gate` and must not launch if it fails. The user's
existing restriction forbids S3 operations. The local gate returned 2
before any upload because `.resume/RESUME.md` was absent. No upload, credential
inspection, research run or old-panel rerun was attempted. This is an
authorization/policy conflict, not a mathematical obstruction or evidence
that AWS access is unavailable. Checkpoint authorization or an explicit
owner-approved alternative is required before the new experiment can run.

The implementation/tests can be reviewed independently of that execution
decision. The proposed comparison and the broader factor-base goal remain
pending; no result is silently substituted for them.

## Visual and validation

- [Method diagram](method.svg), [PDF report](report.pdf), [editable source](report.typ).
- [Validation receipt](VALIDATION.md); failed setup commands remain recorded.
- The scoreboard, leaderboard, browser and old benchmark figures are unchanged:
  no new measured solver cost, yield, rank, IC/rho ratio, registered curve,
  accepted performance result or previous evidence correction is introduced.

Requirement status: native structural preflight implemented; correctness
validation recorded separately; matched successful solver/rank comparison
blocked and unrun; useful factor-base improvement not established.
