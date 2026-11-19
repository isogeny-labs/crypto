# One-root and target-aware Semaev root search

## Requirement and status

| Requirement | Status | Evidence |
| --- | --- | --- |
| Return one exact root without all-root enumeration | Implemented and verified `[pcc:unverified]` | `one-root`; 900 randomized certificates across nine field/curve fixtures |
| Find whether a root is in an x-coordinate target set | Implemented and verified `[pcc:unverified]` | `target-root`; 900 randomized hit and 900 randomized miss certificates |
| Avoid coefficient materialization | Source-inspected implementation property `[pcc:unverified]` | Both paths consume lifted input points directly and never call `poly_from_roots` |
| Characterize hit/miss and target-size crossover | Provisional measurement complete | Two retained benchmark series; final serial integration benchmark remains open |

## Semantics

For input points `P_1,...,P_(m-1)`, `P_1` is fixed positive. A mask bit selects
the sign of each subsequent point. Therefore

```text
S = P_1 + sum_i epsilon_i P_i
```

certifies one root `x(S)` whenever `S` is finite. If `S` is the identity, the
certificate is the projective root at infinity and the specialized affine
polynomial has a degree drop. Returning infinity preserves exact semantics.

When only one root is needed, selecting a mask and computing `S` takes `m-2`
point additions, versus producing `d=2^(m-2)` signed sums. This is an algorithmic
change in requested output, not a faster all-roots implementation `[pcc:unverified]`.

## Target-set algorithm

For a target x-coordinate `q_x`, lift one `Q=(q_x,q_y)`. A root matches the
target exactly when some fixed-sign decomposition satisfies

```text
L + R = Q  or  L + R = -Q.
```

Split the sign variables into a left and right group. Enumerate both groups in
Gray order, hash full right-hand curve points, and query `Q-L` and `-Q-L` for
each target and left sum. A matched hash entry reconstructs the complete sign
mask; recomputing that signed sum is the returned certificate.

The split is chosen by minimizing the simple work model

```text
2^right_bits + 2 * target_count * 2^left_bits.
```

All `Q-L` additions for a given target/sign use one batch inversion. The first
prototype used one affine inversion per probe; its slower results are retained
as unsuccessful evidence rather than overwritten.

## Correctness evidence

`make test` exercises all three campaign fields and all three curve models. The
target extension adds:

- 900 randomly selected sign masks checked against the complete signed-sum root
  multiset;
- 900 target hits recovered with a recomputable sign-mask certificate; and
- 900 curve-point target misses checked against complete enumeration.
- nine explicit edge fixtures covering a projective infinity root beside a finite
  root and a repeated finite root.

The benchmark performs an additional per-case untimed equality check against
complete MITM enumeration before timing. It validates the hit/miss classification
and every returned mask.

## Provisional benchmark evidence

The command below ran while six optimization worktrees shared five CPUs, so these
timings are retained as provisional and must not be promoted to final matched
claims:

```bash
./build/semaev_lab benchmark-target \
  --prime 1000003 --a 2 --b 3 --seed 6000285944033488934 \
  --min-arity 3 --max-arity 11 --bench-cases 50 --bench-repeats 10 \
  --output results/benchmark_target_batched_p1000003_provisional.csv
```

At degree 512, 50 validated cases and 10 repetitions measured:

| Targets | Hit/miss | One root | All-root membership | Target search | All / target |
| ---: | --- | ---: | ---: | ---: | ---: |
| 1 | miss | 1.132 us | 38.651 us | 12.363 us | 3.126x |
| 1 | hit | 1.137 us | 39.001 us | 12.345 us | 3.159x |
| 4 | miss | 1.129 us | 39.314 us | 20.684 us | 1.901x |
| 4 | hit | 1.131 us | 38.846 us | 19.581 us | 1.984x |
| 16 | miss | 1.180 us | 39.784 us | 39.834 us | 0.999x |
| 16 | hit | 1.268 us | 39.067 us | 35.777 us | 1.092x |

A smaller high-degree diagnostic used 20 cases and 5 repetitions:

```bash
./build/semaev_lab benchmark-target \
  --prime 1000003 --a 2 --b 3 --seed 6000285944033488934 \
  --min-arity 12 --max-arity 14 --bench-cases 20 --bench-repeats 5 \
  --output results/benchmark_target_batched_high_p1000003_provisional.csv
```

At degree 4096, one-target target search measured 9.156x (hit) and 10.098x
(miss) versus all-root membership. Four targets measured 4.307x to 4.918x.
The one-root path measured 1.472-1.475 us versus 319.734-326.443 us for all-root
membership on the one-target rows, a stage ratio of 217.186x-221.306x. These are
shared-CPU branch measurements, not end-to-end or final serial results.

## Accounting and limitations

- Timed paths begin with affine input points and already-lifted target points.
  Input square roots, sampling, and target construction are excluded.
- All-root membership includes root generation, sorting/deduplication, and binary
  searches. Target search includes half-table generation and hashing.
- A target hit may terminate early; hit timing depends on target and table order.
- Target search returns the first certificate, not every matching mask or root
  multiplicity.
- For large target sets the target algorithm performs more work than listing all
  roots. The benchmark explicitly retains these negative rows.
- The implementation is single-threaded and uses the existing generic affine
  field/group arithmetic. Fixed-field and projective optimizations are separate
  tracks and their gains cannot be multiplied without an integrated measurement.
- No end-to-end relation-generation or ECDLP timing was measured.

## Retained evidence

- `results/benchmark_target_p1000003_provisional.csv`: initial serial-probe design
- `results/benchmark_target_batched_p1000003_provisional.csv`: batched probes,
  degrees 2-512
- `results/benchmark_target_batched_high_p1000003_provisional.csv`: batched probes,
  degrees 1024-4096
- corresponding exact console logs under `results/raw/`
- `results/SHA256SUMS.target-search`: integrity manifest for all six evidence files
