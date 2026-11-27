# Route 2 — how to reproduce

Python 3 only, no third-party packages. Runtime ≈ 13 s for the whole suite.

```sh
cd /Volumes/SSD990/llm/tmp/opencode/subexp_routes/route2

python3 route2_join.py all            # every stage, writes route2_results.json
python3 route2_join.py geom           # §4  V-level vs E-level structure
python3 route2_join.py join           # §2  hits/pass, ops per relation, totals
python3 route2_join.py structured     # §3  the three structured joins, costed
python3 route2_join.py caps           # §1  |V| reachable per tower, NIST caps

python3 route2_join.py all --json out.json     # choose the store path
```

Stages are independent and each rewrites the JSON store with only its own keys
(`all` writes all of them). `run_all.log` is the recorded output of `all`.

## What each stage measures

| stage | quantity |
|:--|:--|
| `geom` | `|V|`, `|F|`, `\|V+V\|` and `\|V·V\|` as x-value sets, 3-term-AP z-score, and at the point level `\|F+F\|` and additive energy `Σ_c r(c)²` against 3 matched controls (same size, same negation symmetry) |
| `join` | hits per target pass (`|F|²/N` predicted), ops per relation (`2N/|F|` predicted), total probes to collect `|V|` relations (`≈2p`), plus a direct probe-until-found simulation |
| `structured` | (a) Kummer membership `x^(2^t)=g^(2^t)` vs a dict hit; (b) Vélu tower membership `L(x)=c` (with an exactness check against V) vs a dict hit; (c) sumset table `½|F|²` group ops + one lookup per target, against the enumerative pass, and the fitted exponent of the best total |
| `caps` | `v₂(#E')` histogram over random aux curves, the largest `|V|=2^t` actually built, the ~2^t search cost, and the 2-adic cap table for P-192/224/256/384/521 and secp256k1 |

## Instances

| p | v₂(p−1) | target curve | #E |
|---:|---:|:--|---:|
| 1009 | 4 | first prime-order curve from seed `0xC0FFEE^p` | 1009 |
| 12289 | 12 | same rule | 12421 |
| 40961 | 13 | same rule | 41023 |

Targets in the join stages are uniform points of E (each curve has prime order,
so E = ⟨G⟩ and every target is reachable).

## m = 3 addendum

```sh
python3 route2_m3.py        # all stages; writes route2_m3_results.json
                            #   and route2_m3_relations.jsonl
python3 route2_m3.py --json out.json --jsonl out.jsonl
```

| stage | quantity |
|:--|:--|
| A `m3_join` | the 3-sum join at \|F\| ≈ p^(1/3): \|F+F\|, \|F+F+F\|, relations/target (exact census), ops/relation, totals for j1 (enumerative) and j2 (pair table), 3-sum energy vs a matched control, and a sampled check of the hit count |
| B `rank` | relations needed for a full-rank matrix, m = 2 vs m = 3 (40 trials per size); the m = 2 incidence-matrix rank formula and its ±1 kernel, 200 trials |
| C `m3_terms` | both cost terms at B = p^(1/3): pair-table join, Route-1 `2^T` per-system, measured-F4 per-system, random-triple join; ratios to rho and implied exponents |
| D `m3_strategies` | the three joins as closed forms over \|F\| = 4…64 at p = 1009/4099/12289, their minima, fitted exponents, and a model projection to N = 10¹² |
| E `m2_m3` | m = 2 vs m = 3 pair-table totals at the same base |

Recorded output: `run_m3.log`. Note that `route2_m3.py` imports `route2_join.py`,
so both files must sit in the same directory.
