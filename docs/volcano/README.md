# `crypto volcano`: isogeny-volcano tools

Native (Rust) tools for the ℓ-isogeny volcanoes of ordinary curves, built into
the `crypto` binary. Source: `src/cryptanalysis/volcano/`, CLI
`src/cli_volcano.rs`.

```bash
cargo build --release --bin crypto --bin ecbench
./target/release/crypto volcano --help
```

| command | what it does | typical cost |
|---|---|---|
| `volcano profile` | Frobenius discriminant `t² − 4q = D_K f²`, conductor `f` and its factors, volcano depth per prime, class numbers, `E(F_q)` structure at every level, for every crater conductor | milliseconds |
| `volcano level1` | `Φ_ℓ(X, j₀)` over `ℤ` for a class-number-one crater, by CRT | seconds to hours, `≈ ℓ³` |
| `volcano koblitz-step` | the level-1 neighbours of a Koblitz curve over `F_(2^n)`, as j-invariants in the field, grouped into Frobenius orbits, with a verification | `level1` plus seconds |
| `volcano ecbench-spec` | writes ecbench specs for the Koblitz crater and a sample of verified level-1 neighbours, so the layers can be measured in ecbench's one unit | `level1` plus seconds |

## The picture

For an ordinary curve over `F_q` with trace `t`, write `t² − 4q = D_K f²` with
`D_K` fundamental. For a prime ℓ the rational ℓ-isogeny graph of the curve is a
**volcano** of depth `v_ℓ(f)`. Level `i` holds the curves whose endomorphism
ring has conductor `c·ℓ^i`, and its size is the class number of that order.
A non-floor curve has ℓ + 1 rational isogenies; a floor curve has one. When
`ℓ | f` the Frobenius acts as a scalar on `E[ℓ]`, so **all** ℓ + 1 kernels are
rational. Every curve in the class has the same `#E = q + 1 − t`, and
`E(F_q) ≅ ℤ/n₁ × ℤ/(N/n₁)` with `n₁ = gcd(A, f/d)` at conductor `d`
(Lenstra; Kohel).

## `profile`

```bash
crypto volcano profile --ecc2k130 --ell 263
crypto volcano profile --koblitz-n 83 --ell 6473
crypto volcano profile --q 1000003 --trace 1234          # a prime-field curve by (p, trace)
```

Prints `D_K`, `h(O_K)`, `f = ∏ ℓ^e`, `#E` with its factorisation, and for each
prime dividing `f`: depth, Kronecker symbol `(D_K/ℓ)`, the number of
horizontal and descending isogenies at the crater and the size `h(O_ℓ)` of
level 1. With `--ell L` it prints one level table per crater conductor; the
Koblitz curve (End = `O_K`) is the volcano with crater conductor 1.
If `f` cannot be fully factored the table is withheld and `f` is a lower bound.

## `level1`

```bash
crypto volcano level1 --d -7 --ell 263 --mod2           # -7: the crater of K_0
crypto volcano level1 --d -7 --ell 1009 --threads 4 --out phi_1009.txt
```

`D` is one of `-3 -4 -7 -8 -11 -19 -43 -67 -163` (class number one). The output
is `Φ_ℓ(X, j₀)`, monic of degree ℓ + 1; `--out` writes the integer coefficients,
`X⁰` first. For a split ℓ it equals `(X − j₀)^{1 + (D/ℓ)} · H_{Dℓ²}(X)`; for an
inert ℓ it is the Hilbert class polynomial `H_{Dℓ²}`.

**Method.** For a word-size prime `q = N(1 + ℓγ)` the CM curve with
`j = j₀` over `F_q` has Frobenius `≡ 1 (mod ℓ)`, so all of `E[ℓ]` is rational.
Vélu's formulas on each of the ℓ + 1 cyclic subgroups, advanced in lockstep with
batched inversions, give the ℓ + 1 neighbours; their product is
`Φ_ℓ(X, j₀) mod q`. Garner's algorithm reconstructs the integers and stops when
the symmetric lift reproduces every coefficient modulo a fresh batch of primes.

**Measured cost** (4 threads, `D = −7`):

| ℓ | primes | largest coefficient | time |
|--:|--:|--:|--:|
| 263 | 288 | 17,345 bits | 0.5 s |
| 503 | 592 | 35,995 bits | 2.3 s |
| 1009 | 1,296 | 78,252 bits | 18.8 s |
| 2003 | 2,752 | 167,281 bits | 153 s |

Time scales as `ℓ³` (primes `∝ ℓ`, work per prime `∝ ℓ²`). Extrapolating to
ℓ = 6473 gives about 85 minutes on 4 threads; see
`research/volcano_koblitz_n83_20261018/` for the measured run.

## `koblitz-step`

```bash
crypto volcano koblitz-step --n 131 --ell 263          # ECC2K-130: 0.7 s
crypto volcano koblitz-step --n 41 --ell 409 --verify 32 --out neighbours_n41.txt
```

Steps: (1) `Φ_ℓ(X, −3375)` by CRT; (2) reduce mod 2 and factor over `F₂` with
bit-packed arithmetic; (3) one root per Frobenius orbit in `F_(2^n)` and its
conjugates; (4) for a sample of neighbours build `y² + xy = x³ + a₂x² + 1/j`
and test `[N]P = O`, `N = 2^n + 1 − t_n`, with a control of random j-invariants
(which must essentially never pass). The command exits nonzero unless every
count matches theory (2 horizontal at `j = 1` when ℓ splits, `h(O_ℓ)` distinct
neighbours, none left over). `n` must be odd.

Checks run on every invocation. For ECC2K-130 (`--n 131 --ell 263`) it finds the
two horizontal kernels and 262 neighbours in two orbits of 131.

## `ecbench-spec`

```bash
crypto volcano ecbench-spec --n 41 --ell 409 --neighbors 12 --targets 8 --out specs/n41
ecbench run --wait --spec specs/n41/crater.json    --out sessions/n41-crater
ecbench run --wait --spec specs/n41/neighbors.json --out sessions/n41-neighbors
ecbench compare --dir sessions/n41-crater --a rho-neg --b-dir sessions/n41-neighbors --b rho-neg
```

Writes `crater.json` (ecbench's `koblitz` curve), `neighbors.json`
(`binary_explicit` curves, one per verified neighbour, generator of the
prime-order subgroup included) and `curves.json` (j-invariant, twist, order,
generator of every neighbour). `n ≤ 62` and a cofactor below `2^24`. The
neighbour curves are not in `docs/curves/registry.json`; register them before
citing their slugs in prose (AGENTS.md §11).

## Verification

* Unit tests: `cargo test --release --lib volcano::` (Montgomery arithmetic,
  Frobenius norm forms, F₂ factoring, class numbers, binary curve order tests).
* `Φ_3(X, j₀)` equals the repository's tabulated `Φ_3`; `Φ_ℓ(X, j₀) mod p`
  equals the `q`-expansion `Φ_ℓ mod p` of `isogeny_walk::modpoly` for ℓ up to 19,
  craters `D = −3 −4 −7 −8 −163`, three primes up to 62 bits.
* An external oracle, run once and recorded in the study directory: PARI's
  `polmodular` agreed on all 34 `(D, ℓ)` pairs compared.

## Limits

* Class-number-one craters only: the neighbours of the *Koblitz curve* and of
  the other class-number-one CM curves. The neighbours of an arbitrary vertex
  (the second step down) are not computed here; PARI's `polmodular` does not
  provide them over `F_(2^n)` either.
* `ecbench-spec` needs `n ≤ 62`. For `n = 83` the neighbours are computed and
  verified, but ecbench has no wide generic binary curve, and rho at that size
  (`≈ 2^41` operations) is not run.
* The degree-ℓ isogeny between a neighbour and the crater is **not** computed:
  the tools find the j-invariants and the group order, not the maps. Without
  the map nothing here says a neighbour is easier or harder against an
  attacker who can walk.
