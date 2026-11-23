# Linearized two-summand root finding vs the Semaev pair loop (m = 3, binary curves)

**Date** 2026-11-17. **Status** measured on the repository's one-word field `Gf2` (`n ≤ 62`); every
linearized witness re-added in the group and checked against `S₄ = 0`; the rational verdict checked against
an independent group-law triple enumeration at `l ≤ 8`. **Scope** the decomposition subroutine of
index calculus at a fixed factor base, not an end-to-end run.

**Code** `src/cryptanalysis/semaev_linear.rs` (solver + tests), `examples/semaev_linear_bench.rs` (this table).
Ported from `isogeny-labs/lfd-summation-systems` PR #28; that repository also has the standalone three-word
kernel used at `n = 131` (`rust/ic_rootfind`, above both `Gf2` and `Gf2_128`).

## The two methods

Deciding whether a target `R` splits as `P₁ + P₂ + P₃` over the low-order subspace
`V = ⟨1, z, …, z^{l−1}⟩` of `F_{2^n}` (`n = 3l`), Koblitz `b = 1`:

- **Pair loop** — `semaev_decomp::decompose` / `SubspaceOracle::decompose`: loop over the `2^{2l}/2`
  pairs `(x₁, x₂)` in `V`, build the quartic `S₄` in the third variable and keep its roots in `V` via
  `gcd(q, L_V mod q)`. `O(2^{2l}·l)` field operations.
- **Linearized two-summand** (this work) — `semaev_linear::LinearDecomposer::decompose3`: `P₁ + P₂ = T`
  is a line through `−T`; `x₁+x₂` and `x₁x₂` are `F₂`-affine in the slope, so the two-summand problem is
  one `F₂`-linear system in `l+1` unknowns plus one quadratic. `m = 3` fixes one summand over the `2^l`
  abscissas of `V` and solves for the other two. `O(2^l·l²)`.

## The distinction, measured

The pair loop decides the **abscissa** question `S₄ = 0`, which also admits solutions whose points lie in
the quadratic twist / extension (not rational relations). The linearized solver decides the **rational**
question directly (points over `F_{2^n}` summing to `R`), so it never returns a twist-only hit and needs no
separate lift check. Below, `absc%` is the pair loop's acceptance rate and `rat%` the linearized solver's;
their small gap is exactly the twist-only abscissa solutions. The linearized rational verdict equals an
independent group-law triple enumeration on every tested target (`l ≤ 8`).

## Cost unit (worst-case target, Koblitz a = 1, b = 1)

`pairs(loop)` = pairs the pair loop processes to refute; `scanned(lin)` = fixed summands the linearized
solver scans to refute; `unit ×` = their ratio.

| l | n | \|V\| | pairs(loop) | scanned(lin) | unit × | absc% | rat% |
|---|---|---|---|---|---|---|---|
| 5 | 15 | 32 | 528 | 47 | 11.2× | 42.8 | 42.5 |
| 6 | 18 | 64 | 2080 | 77 | 27.0× | 22.2 | 22.2 |
| 7 | 21 | 128 | 8256 | 117 | 70.6× | 12.8 | 12.8 |
| 8 | 24 | 256 | 32896 | 255 | 129.0× | 13.5 | 13.5 |
| 9 | 27 | 512 | 131328 | 505 | 260.1× | 13.0 | 13.0 |
| 10 | 30 | 1024 | 524800 | 927 | 566.1× | 14.2 | 14.2 |
| 11 | 33 | 2048 | 2098176 | 2013 | 1042.3× | 15.2 | 15.2 |

## Wall time per target (400 random rational-point targets, single thread, Koblitz a = 1)

| l | n | pair loop | linearized | speed-up |
|---|---|---|---|---|
| 5 | 15 | 63.9 µs | 16.6 µs | 3.9× |
| 6 | 18 | 316 µs | 39.6 µs | 8.0× |
| 7 | 21 | 1.48 ms | 77.5 µs | 19.1× |
| 8 | 24 | 6.39 ms | 200 µs | 31.9× |
| 9 | 27 | 28.0 ms | 423 µs | 66.1× |
| 10 | 30 | 120 ms | 904 µs | 132.7× |
| 11 | 33 | 509 ms | 2.00 ms | 254.7× |

The speed-up doubles with each `l`, as `O(2^{2l})` vs `O(2^l)` predicts. `a = 0` matches
(`run-a0-l10.txt`). The linearized solver here uses readable bit-iterated linear maps, not the byte-table /
shared-inversion kernel of the standalone crate, so its constant factor is loose; the scaling is the point.

## Reproduce

```bash
cargo run --release --example semaev_linear_bench            # Koblitz a = 1, to l = 11
cargo run --release --example semaev_linear_bench 10 0       # Koblitz a = 0, to l = 10
cargo test  --release --lib semaev_linear                    # solver tests incl. independent brute
```

## Limits and honest framing

- `Gf2` is `n ≤ 62`; the low-order subspace gives `l = n/3 ≤ 20`. `n = 131` (ECC2K-130) is above `Gf2`
  and `Gf2_128`; measured separately in the standalone three-word crate.
- This is the decomposition subroutine at a fixed factor base. The number of fixed summands that must be
  scanned, the relation count and the linear-algebra stage are unchanged; no end-to-end or sub-rho claim is
  made here.
- The pair loop's twist-only acceptances are not relations; a real relation search built on it lift-checks
  every hit, which the linearized solver avoids by construction.
