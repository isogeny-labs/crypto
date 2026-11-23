# Index-calculus arity sweep, m = 2..9

Two sweeps over factor-base points per relation (`m`, the arity / summand
count), run 2026-10-27:

- **Prime field, measured.** `crypto_autoresearcher.index_calculus sweep` on
  generated prime-order curves at 12, 14, 16, 18 and 32 bits, both the
  exhaustive and the MITM decomposition engine.
- **Binary and Koblitz, analytic.** `ca-ic budget --max-arity 9` in the
  cryptanalysis suite, which charges relation collection and sparse linear
  algebra against rho and asks whether a descended system can be formed inside
  that budget. It has no prime arm.

The generated prime factor bases are **not** in this repository. They are at
`/Volumes/data/ic-arity-sweep-20261027/` (111 files, 54,937 bytes), with
SHA-256 in `prime/factor-bases.manifest.json`. See **Factor bases** below.

## Prime field, measured

Commands (also in `REPRODUCE.sh`):

```sh
python3 -m crypto_autoresearcher.index_calculus sweep \
    --bits 12 14 16 18 --m 2 3 4 5 6 7 8 9 --fb small_x \
    --curves 3 --rho-curves 3 --reps 500 --engine enumerate --out prime/sweep.jsonl
python3 -m crypto_autoresearcher.index_calculus sweep \
    --bits 12 14 16 18 --m 2 3 4 5 6 7 8 9 --fb small_x \
    --curves 3 --rho-curves 3 --reps 500 --engine mitm --out prime/sweep-mitm.jsonl
```

234 rows, 0 failed; every logarithm re-checked as `kP = Q`. The cost unit is
the repository's own: IC S<sub>3</sub> root solves (MITM table included) against
rho walk additions. These are different units, as the tool's report says.

Best arity by that median (3 curves per size):

| bits | exhaustive | MITM |
|---|---|---|
| 12 | m = 2, 10.6x rho | m = 5, 2.8x |
| 14 | m = 2, 16.2x | m = 3, 3.3x |
| 16 | m = 2, 20.4x | m = 3, 5.8x |
| 18 | m = 2, 60.5x | m = 3, 4.3x |

Fitted exponent in N (rho = 0.51):

| m | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|---|---|---|---|---|---|---|---|---|
| exhaustive | 0.86 | 0.95 | 1.07 | 1.11 | 1.07 | 1.06 | 1.10 | 1.23 |
| MITM | 0.86 | 0.63 | 0.85 | 0.74 | 0.83 | 0.65 | 0.85 | 0.60 |

The odd arities have the low exponents; the even ones sit at 0.83–0.85 because
the default MITM table split `(m+1)//2` leaves one more level on the search
side. The m = 5 short-window fit (0.74) is distorted by table overhead; the
repository's longer 12–32 bit fit is 0.60 (`index_calculus/README.md:345`).

32 bits, m = 5..9 only (m = 2..4 were capped), three curves:

| m | \|F\| | S<sub>3</sub> / rho walk | median seconds |
|---|---|---|---|
| 5 | 86 | **11.5x** | 1.2 |
| 6 | 50 | 61.7x | 123 |
| 7 | 34 | 14.9x | 40 |
| 8 | 26 | 60.6x | 176 |
| 9 | 22 | 22.2x | 60 |

So the absolute winner moves from m = 3 (14..18 bits) to m = 5 (32 bits);
m = 7 is second at 32 bits, m = 9 third. No arity in 2..9 crosses rho, and the
best measured gap still grows: the repository's previous arity sweep put m = 5
at about 15x rho at 32 bits (`index_calculus/README.md:348`), which this run
reproduces.

Caveats: toy sizes, isolation L0, one base (`small_x`), and the default MITM
table arity. m = 6 and m = 8 are dominated by that default, not necessarily by
the arity itself.

## Binary and Koblitz, analytic

Command (cryptanalysis `0fa835a2a13e`, `ca-ic` 0.1.0):

```sh
ca-ic budget --standard --max-arity 9 --json --out budget/standard/koblitz.json
ca-ic budget --standard --max-arity 9 --no-frobenius --json \
    --out budget/standard/binary-generic.json
```

"Viable" is strict: the linear algebra fits inside rho's budget **and** the
descended system can be formed inside it
(`suite/src/cryptanalysis/index_calculus_budget.rs:124`).

| n | viable m in 2..9 | best m | log2 budget at best (binary / Koblitz) |
|---|---|---|---|
| 113–283 | none | — | — |
| 409 | 4, 5 | 4 | 78.39 / 78.39 |
| 571 | 4, 5, 6 | 5 | 163.75 / 171.00 |

The viable window is the same for generic binary (`--no-frobenius`) and for
Koblitz; crediting the Frobenius orbit quotient changes the slack, not the
window. m = 9 is never formable at any tested degree: the ANF deficit grows
faster than the budget. On the toy ladders (Koblitz 11..41, binary 15..27) no
cell is viable; the first positive budget appears only at n = 31, m = 5 and
n = 39/41, m = 8, and the system still blocks there.

This is a verdict under a cost model, not an attack or a lower bound on ECDLP.

## Factor bases

- Root: `/Volumes/data/ic-arity-sweep-20261027/`
- `factor-bases/n{bits}-c{curve}-m{m}.json`: one file per sweep cell
  (24 each at 12, 14, 16, 18 bits; 15 at 32 bits).
- `manifest.json`: SHA-256, byte count, curve fields and |F| for all 111.
- Format: curve (`p`, `a`, `b`, `N`), generator, arity, requested and actual
  |F|, and `points` as affine `[x, y]`. The base is `small_x`: the first |F|
  x-coordinates that lift, one point per +/- pair. Its `seed` is recorded for
  provenance but `small_x` does not use it, so the files are deterministic.
- The analytic budget sweep builds no factor base; there is nothing to save
  for it.

`scripts/dump_factor_bases.py` rebuilds and verifies every file from the row
files: it regenerates the curve instance, checks `p, a, b, N` and |F| against
the recorded row, and writes the manifest. `scripts/run_curve.py` fills in one
curve index when `ic sweep --curves N` would otherwise repeat curves 0..N-2.

## Provenance

- Artifact base: `crypto` `origin/main` `2b9f3f72f6`.
- Prime: `crypto-autoresearcher` `480187a0d1`, version 0.3.0, Python 3.12.8.
  The checkout had unrelated staged changes in `.github/` and `analysis/`;
  `src/crypto_autoresearcher/index_calculus/` was at that commit.
- Budget: `cryptanalysis` `origin/HEAD` `0fa835a2a13e`, `ca-ic` binary
  blake3 `c0fef707972df911ba6134ad2fcb67f175a84ce9232eb9d3955b2ee9ebba18b3`.
- The string `m` is the number of signed factor-base points per relation in
  both sweeps.
