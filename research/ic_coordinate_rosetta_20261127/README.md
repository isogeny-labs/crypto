# S59 index-calculus coordinate Rosetta comparison

This study compares coordinate choices without conflating three different
questions:

1. **Intrinsic index-calculus coordinates.** Nine coordinate/predicate arms
   receive the same planted relations on the public 59-bit prime
   `p = 288230445959117953` and the NIST-shaped curve
   `y^2 = x^3 - 3x + 11`. The primary measurements are relation yield,
   factor-base density, invariance checks, exclusions, and counted coordinate
   work.
2. **Point-storage and arithmetic charts.** The repository's pinned prime-field
   EFD catalogue supplies ten models, 33 projective/quotient charts, and 471
   formulas. Every chart is round-tripped over S59. Applicable additions and
   doublings are checked against the independent affine law and their
   straight-line field-operation counts are retained.
3. **Transfers to another group.** Plane quartics and hyperelliptic Jacobians
   are separate DLP groups. They enter a comparison only with an explicit map,
   subgroup-preservation certificate, recovery path, and complete transfer
   accounting. The supplied PDFs do not provide an S59 transfer fixture, so
   these rows are classified rather than benchmarked as coordinate changes.

The experiment is intentionally a replication/engineering study. The existing
lead atlas records that genuine degree-two coordinates tie at the exponent
level and that model changes can alter constants. This work makes those
constants and applicability boundaries reproducible in Rust on S59; it does
not claim a sub-square-root prime-field ECDLP algorithm.

## Counted outcome

The frozen counting pass evaluated 2,097,152 planted triples per arm. All hard
controls passed: 18,874,368 planted zero-sum checks succeeded, no deliberately
wrong relation was accepted, affine `x`, its A/A repeat, and normalized
Jacobian `x` agreed exactly, and raw projective `X` failed the registered scale
invariance control. None of the seven registered admission-rate comparisons
against `x` survived Holm correction. The useful differences are therefore
algebraic class, invariance, and exact extraction cost—not demonstrated
relation-yield superiority.

The chart pass emitted 43 rows that passed the registered checks: 33 maintained
EFD catalogue charts and 10 synthetic affine conversion/oracle baselines. See
[REPORT.md](REPORT.md), the printable [REPORT.pdf](REPORT.pdf), and the JSON
artifacts under `artifacts/`.

## Follow-on lead

[LEAD-CKM19-NATIVE-PARITY.md](LEAD-CKM19-NATIVE-PARITY.md) opens a separate
`open_next_step` experiment from Cheng--Kodera--Miyaji's 2019 observation that
the fixed-target native twisted-Edwards system has unified total-degree parity.
It freezes the comparison to native Edwards `y`, Montgomery `x`, and
short-Weierstrass `x`; direct and symmetric `m=3,4` systems; an
identical-system ordering control; complete relation/rank/recovery accounting;
and matched `rho.negation` over three increasing rungs. Missing terms or an
isolated solver win do not pass its greater-than-10% complete-cost gate. No
lead experiment has been run.

## Instrument

`src/bin/ic_coordinate_rosetta.rs` is the measurement instrument. It emits
versioned JSON and has no Python/Sage/Magma compute path. The deterministic
summary, bootstrap, multiplicity correction, and report tables are also
implemented in Rust by `src/bin/ic_coordinate_rosetta_report.rs`.

```text
cargo run --release --bin ic_coordinate_rosetta -- inventory
cargo run --release --bin ic_coordinate_rosetta -- \
  chart-all --workers 4 --iterations 20000
cargo run --release --bin ic_coordinate_rosetta -- \
  relations-all --workers 4 --samples 262144 --seed 5995792941424594770
```

Parallel batches are correctness/counting throughput runs; their wall fields
are explicitly inadmissible. Admitted wall measurements use one worker per
isolated EKS execution, interleaved arms, one warm-up and five measured rounds.

The canonical exporter later exposed that `explicit_list` seeds are cycled
globally across arm/instance/replicate tuples, whereas amendment A1 and the
instrument use the same explicit trace at a given instance for every arm. The
raw counted rows retain the intended matched traces and their seed values, but
no canonical full export or L2 timing claim is made from protocol v2. A future
timing protocol must correct that administrative mapping before submission.

## Frozen boundary

- Public/known-answer fixtures only. No deployed key or imported target.
- Rust is the implementation language for all new compute and analysis.
- The S59 primary curve has a planted point `(1,3)` and planted relations
  `P + Q + R = O`. The instrument makes no unsupported curve-order claim.
- `x`, its A/A repeat, normalized Jacobian `x`, and Möbius(`x`) are the
  decisive degree-two family. Raw projective `X` is a negative control because
  it changes with scale. The other rational functions are descriptive unless
  their algebraic degree/model preconditions match.
- Formula counts exclude model/chart conversion and validation; timed chart
  rows include them.
- Local wall time is never a result. L2 isolation and the A/A interval are
  required for timing.

See [COORDINATE_MATRIX.md](COORDINATE_MATRIX.md) for the complete inventory and
`protocol.json` for the preregistered comparisons, seeds, stopping rules and
decision vocabulary.

## Sources checked

- The five supplied Rosetta PDFs, pinned by `source_manifest.json`.
- `docs/efd/{README.md,prime-field.md,ic-representations.md}` and the pinned EFD
  source URLs/hashes in `src/prime_ecc/efd/catalogue.json`.
- `crypto-autoresearcher` lead
  `LEAD-EXO-curve-model-representation` and its cited
  `research/ecgfp5_representation_audit_20261102/README.md`.
- FGHR 2014 and FHJRV 2014 as identified in that audit for Edwards/Jacobi and
  two-torsion-aware summation representations.
- Cheng, Kodera and Miyaji, “Differences among Summation Polynomials over
  Various Forms of Elliptic Curves,” IEICE Transactions on Fundamentals
  E102-A(9), 1061--1071 (2019), DOI 10.1587/transfun.E102.A.1061; official
  metadata plus user-supplied screenshots of the substantive pages.

The shared research-KB MCP and CLI wrapper were unavailable in this runtime.
The lead-atlas/repository read is a bounded fallback; no priority or absence
claim is inferred.
