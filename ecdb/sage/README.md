# Sage curve generators for ecdb

`generate.sage` writes curves as JSONL (`ecdb.generated/v1`).
`ecdb import generated` re-verifies each record and loads it.
It was written at the owner's request (2026-10-31: "run the sage scripts to
fill the database up with curves"). Sage does the computing: SEA point
counting, CM, class polynomials, genus-2 Frobenius and Igusa–Clebsch
invariants. The importer in Rust is the gate that decides what enters the
database.

| Set | Units | What it writes |
|:--|:--|:--|
| `census-fp` | `p0005` … `p1021` | One curve per F_p-isomorphism class over every prime field with 5 ≤ p < 2^10. The model is the least (a u⁴, b u⁶) over u ∈ F_p*. Each record has #E, a sample point, the twist order, and Frobenius/CM data (t² − 4p = f² D_K, h(D_K)). |
| `census-f2m` | `m01` … `m10` | Every ordinary curve y² + xy = x³ + ax² + b over GF(2^m), m ≤ 10. a ∈ {0, the least trace-1 element}, b ≠ 0, over the modulus that ICV1 prescribes. |
| `crypto-prime` | `rand-032` … `rand-256`, `cm-032` … `cm-256` | Random prime-order curves (SEA with early abort) at 11 sizes and their quadratic twists. CM curves for the nine class-number-one discriminants, with all their twists and cofactor/subgroup data. |
| `pairing` | `<family>-<bits>` | BN, BLS12, BLS24, KSS16 and KSS18 curves at 160–512 bits from the family polynomials, three per size, with r, the cofactor, a generator and the embedding degree. |
| `isogenies` | `census-p0005` … `census-p1021`, `from-<set>.<unit>` | Every F_p-rational ℓ-isogeny (ℓ = 2, 3, 5) of every F_p census curve, which gives the complete ℓ-isogeny graphs. Also the ℓ-neighbours of the crypto and pairing curves, each written as a curve record followed by its edge. Edges are plain Vélu: `isogenies_prime_degree` can compose with an isomorphism, so the isogeny is rebuilt from its kernel. The target is the census representative, reached by (a w², b w³). |
| `genus2` | `b05` … `b32` | Random genus-2 curves y² = f(x), f a monic squarefree quintic, over primes of 5–32 bits. Each has L(T), #J, #C and the Igusa–Clebsch invariants. |

The ICV1 identity is never taken from Sage: the importer rebuilds the model
JSON and slug. It then checks every claim it can afford before anything is
inserted, and refuses a record that fails:

- the model is nonsingular and #E lies in the Hasse interval;
- the sample point is on the curve and [N]P = O;
- the generator is on the curve, [r]G = O and r·h = N, and r passes
  Miller–Rabin (deterministic below 3.3·10²⁴);
- the twist order is 2q + 2 − N, and t² − 4q = f² D_K;
- the embedding degree is the least k with r | q^k − 1;
- for genus 2, L(T) has the shape 1 + a₁T + a₂T² + p a₁T³ + p²T⁴ within
  the Weil bounds, #J = L(1) and #C = p + 1 + a₁. #C is recounted point by
  point for p < 2^14.

Statuses: values Sage computes deterministically (orders, L-polynomials,
class numbers) are `proved`. A subgroup order above 3.3·10²⁴ that passes
Miller–Rabin is `probable`. CM data is null when t² − 4q did not factor
within the trial-division limit.

## Running

```sh
export TMPDIR=/dev/shm/tmp   # Sage writes preparsed files to TMPDIR
sage ecdb/sage/generate.sage census-fp out=/path/out           # one set
sage ecdb/sage/generate.sage crypto-prime out=/path/out workers=2 worker=0
sage ecdb/sage/generate.sage pairing out=/path/out units=bn-256,bls12-384
```

Sage's own option parser consumes `--flags`, so arguments are `key=value`.
Each unit writes `<set>/<unit>.jsonl` and then `<unit>.done`, so a rerun skips
finished units. Each unit seeds Sage's RNG from its name, so a rerun writes
the same curves.

Load into a local database:

```sh
ecdb import --repo-root .. generated /path/out
```

Load into the deployed one, chunked through the admin Lambda:

```sh
ecdb/deploy/load-generated.sh /path/out
```

The 2026-10-31 run used SageMath 10.9 from conda-forge, with three workers
in a 4-CPU cloud container. That is a computation, not a measurement: no
timing here is a benchmark.
