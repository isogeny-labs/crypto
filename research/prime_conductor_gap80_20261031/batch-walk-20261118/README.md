# Bounded projective walks for the complete evaluator

18 November 2026 · PR #4703 · native development controls.

The supplied-case evaluator now accepts `--projective-batch CAP` together with
its existing explicit-window option. Baby and giant walks can use Jacobian
mixed additions and convert their x-coordinates in bounded batches. The
default remains affine; the coverage gate, quotient and point-map formulas,
subgroup inputs and direct-PARI cap are unchanged. This implements an optional
coordinate backend; elapsed-time advantage is unmeasured.

## Coordinate and resource contract

For a finite Jacobian point `(X,Y,Z)`, the affine coordinates are
`(X/Z^2,Y/Z^3)`. Mixed addition uses the established EFD `madd-2004-hmv`
formulas; doubling uses the general-a `dbl-2007-bl` formulas. Infinity, doubling,
inverse pairs and doubling of a point with y=0 have explicit branches before
the generic mixed-addition formulas. The field characteristic must exceed 3.
The [source reference](sources.json) records the general EFD page and its hash.
No new formula or algebraic identity is asserted.

The walker holds at most `min(CAP, block length)` points' X coordinates,
Z coordinates and prefix products in three GMP-integer vectors. It inverts
the accumulated nonzero Z product and traverses the prefixes backwards to
normalize the batch, then emits x-coordinates in their original order. It
refuses a walk containing infinity. The evaluator still emits no final
quotient or images before complete coverage and destination checks.

`CAP` must be between 1 and 65536. This bounds those three temporary vectors;
it is not a byte-memory cap for the evaluator. GMP limbs, scalar starts,
root vectors, polynomial products, jets, log norms and serialization remain
separate storage/work. The direct leftover loop stays affine. No peak RSS or
time has been measured. A small batch or a one-point block can add normalization
work without eliminating an affine recurrence inversion.

The earlier [window audit](../window-audit-20261118/README.md) remains pinned to
its pre-batch source, retained in [baseline/complete_pipeline.cpp](baseline/complete_pipeline.cpp).
Its old manifests are not rewritten or presented as hashes of this new source.
The complete 80-bit window remains an independent construction bottleneck.

## Development checks and counter boundary

The frozen qualifier compares baseline, new affine-default and optional
projective outputs for six schedules in the supplied fields 6666679, 71 and
11, with projective batch caps 1, 7, 64 and 4096. It includes explicit windows,
partial batches, kernel inputs, infinity inputs and repeated point records.
Direct PARI checks both quotient coefficients and every image. Arithmetic
controls separately enumerate three small groups and compare mixed additions
and short walks with PARI, including exceptional inputs.

The recorded `walk_backend` counters cover projective walk additions,
normalized finite walk points, batch inverse calls and largest batch length.
They exclude setup/kernel-validation inversions, the leftover loop, point-map
arithmetic, polynomial/log-norm work and PARI. They are not total field-operation
counts. The graph compares batch caps within this backend, not elapsed times
or a baseline-versus-candidate speedup. Each cell is one development replay;
deterministic operation counts have no timing uncertainty interval.

Raw checks and their completion status are recorded under [results](results/).
The separate pre-call manifest was created before invoking the independently
copied frozen qualifier; it is not replaced by later source or reporting work.
The frozen qualifier is a producer's check, not an independent certificate.
No benchmark, gap record or execution verification is promoted from these
controls. [pcc:unverified]

## Build and replay

Use the native build command in the earlier
[evaluator report](../evaluator-20261118/README.md#build-and-replay). Keep
`projective_walk.hpp` beside `complete_pipeline.cpp`; it is a build input.
The worker Docker context must copy and hash this header as well before a
new image can bind this source. The older qualified image covers its historical
source only, not this version.

```text
complete_pipeline --evaluate MAX_WINDOW BLOCK [--pari-reference]
                  [--baby-window B] [--projective-batch CAP]
complete_pipeline --projective-controls
```

The `--projective-controls` command is a finite development check over small
public curves. It is not a parameter search or an ECDLP timing driver. Replaying
the qualifier should use a new output directory rather than overwrite the
archived results; its rejection-record sentinel prevents a second complete
invocation from appending into the same directory.

## Original requirements remain separate

| Requested outcome | Status of this change |
|---|---|
| Minimum 80-bit prime endomorphism-conductor gap crossed by an ordinary-curve isogeny | Still needs the full target quotient and images |
| Several fields and dozens of point inputs | Small development controls only; repeated records and exceptional inputs included |
| Repeated matched scalar and ECDLP timings on both endpoints | No new measurements |
| Index-calculus comparison | Not implemented by this coordinate change |
| Source-bound cloud worker and durable results | New image publication/qualification and cloud admission remain separate |

Primary formula source:
https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html.
KB wrapper unavailable (exit 127); source scope is the pinned implementation,
EFD formulas and direct PARI controls, with no record or novelty claim.
The existing frozen conductor/measurement graphs are unchanged; this study's
new graph is confined to its own normalization counters.
