# Natural M31 relation stream: balanced-circuit and solver stopping events

## Result

This study replays a fixed, source-ordered prefix of an existing M31 relation
capture and measures graph closure separately from exact linear-algebra events.
For the exact-two-large-prime panel, the first ordinary graph cycle, first
balanced circuit, first large-prime dependency, first nonzero target projection,
and first increase in projected solver rank all occur at accepted edge 97
(source position 457). The circuit has a nonzero target coefficient
`744325019 mod 2147446831`. [pcc:unverified]

With declared large-prime pool size `K = 168`, this is one uncensored empirical
observation

```text
rho(168) = 97 accepted exact-two-large-prime edges.
```

It is not an estimate of an asymptotic exponent, a concentration theorem, or an
ECDLP speedup. One stream at one `K` cannot distinguish linear, square-root, or
another scaling law. [pcc:unverified]

## Frozen input and provenance

The input is
[`input/m31-first-512.jsonl`](input/m31-first-512.jsonl), a byte-for-byte copy of
the committed first-512 admitted prefix from the retained M31 capture:

- producer commit: `577d6c2c1c4a527604d965e6fb62c44df7902f36`
- producer ten-query blob: `7fd27b9888058399d8814cef373d373243e6a1e5`
- admitted-prefix blob: `f1b02205745a74d1771578a68f0f1f435f4a8823`
- input SHA-256: `cc7aee3ebe57ce951e36f3a2c264857f8f1ed52aa4d76eb5c1fc587542945bd3`
- historical seed: `0x7033315f6d340101`
- scalar field: `F_2147446831`
- curve: `y^2 = x^3 + 847514846*x + 2087444724` over `F_2147483647`
- `G = (625037715, 2100627251)` and `Q = (976979610, 1228979761)`

The original producer searched fixed public `aG+bQ` queries and retained all
group-replayed relations before the fixed ten-query terminal boundary. The
512-row prefix ends at source position 540 because rows with more than two large
terms were excluded by the pre-existing admission rule. Collection did not stop
on a cycle, dependency, rank change, or target-recovery event. The prefix is
therefore uncensored with respect to the stopping events measured here, though it
does not contain the producer's complete reject/probe trace. [pcc:unverified]

## Exact event ledger

### Exact-two-large-prime panel

There are 113 exact-two-large-prime rows in the prefix. They touch 125 distinct
large columns and have final large-block rank 111, hence left nullity 2. Both
dependencies have nonzero target projection. [pcc:unverified]

| event | accepted edge | source position | exact outcome |
|---|---:|---:|---|
| first ordinary graph cycle | 97 | 457 | closes an existing component |
| first balanced simple circuit | 97 | 457 | gain-consistent closure |
| first exact LP dependency | 97 | 457 | LP rank remains 96 |
| first nonzero target projection | 97 | 457 | target coefficient `744325019` |
| first projected-rank increase | 97 | 457 | projected small payload is independent |
| first exact target uniqueness | not observed | censored at 512 admitted rows | target is not unique |

The first circuit witness uses source row IDs and coefficients

```text
- row 12
+ row 38
- row 40
+ row 456
```

where minus means coefficient `2147446830 = -1 mod 2147446831`. It cancels the
large-prime block exactly, leaves 9 nonzero small columns, and has projected RHS
`1846896350`. [pcc:unverified]

### All-admitted operational panel

The all-admitted panel includes zero-, one-, and two-large-prime rows. Its first
solver-visible dependency occurs at admitted row 3/source position 3, before any
graph circuit, because that row has zero large-prime terms. At the 512-row censor
the panel has large-block rank 157, left nullity 355, projected rank 355, and 348
target-projecting dependencies, but still no exact target uniqueness.
[pcc:unverified]

This separates three claims that must not be conflated:

1. a graph circuit exists;
2. an LP dependency has nonzero target projection;
3. the accumulated solver system uniquely determines the target.

The first two coincide at edge 97 in the exact-two-large-prime panel. The third
does not occur in this prefix. In the operational panel, a zero-LP full relation
beats graph closure to the first solver-visible row. [pcc:unverified]

## Method and independent checks

[`analyze.py`](analyze.py) performs source-order streaming elimination over the
prime scalar field. It maintains:

- gain-aware disjoint sets for ordinary and balanced graph closure;
- exact sparse large-block elimination while carrying small columns, RHS, and
  source-row weights;
- incremental projected, target-deleted, and augmented ranks; and
- the first transition for each declared event.

[`verify.py`](verify.py) does not reuse the analyzer's elimination direction. It
recomputes direct ranks with greatest-column pivots, checks every reported
transition on the `n-1` and `n` prefixes, replays source witnesses, and finds graph
closures independently by breadth-first path gains. The analyzer also contains
planted odd-triangle, balanced-square, and projected-but-nonunique controls.

Reproduce from the repository root:

```sh
python3 research/ic_natural_cycle_stream_20261126/analyze.py --self-test
python3 research/ic_natural_cycle_stream_20261126/analyze.py \
  --input research/ic_natural_cycle_stream_20261126/input/m31-first-512.jsonl \
  --output research/ic_natural_cycle_stream_20261126/results.json
python3 research/ic_natural_cycle_stream_20261126/verify.py \
  --input research/ic_natural_cycle_stream_20261126/input/m31-first-512.jsonl \
  --results research/ic_natural_cycle_stream_20261126/results.json
```

The machine-readable output is [`results.json`](results.json). These are unsigned
repository measurements pending independent scientific review. [pcc:unverified]

## Consequence for the graph mechanism

The observation shows that the balanced-circuit mechanism can produce a real,
nonzero target-projecting dependency in a natural retained stream. It does not
yet show an index-calculus advantage: `97 > K/2 = 84`, the target is not uniquely
determined at the censor, complete collection cost is outside this replay, and no
second `K` exists from which to infer scaling. The next decisive experiment is a
preregistered multi-`K`, multi-seed terminal capture that records every accepted
and rejected candidate, continues independently of success, and reports the
distribution of all three stopping events. [pcc:unverified]
