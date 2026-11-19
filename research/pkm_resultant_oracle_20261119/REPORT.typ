#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#set par(justify: true)

= A quasi-linear algebraic decomposition oracle at `m = 2`

EXP-PKM-RESULTANT-001 · 2026-11-19 · protocol `PROTOCOL.md` (frozen in `b8c8408de2` before any row, no amendments)
· oracle `resultant_decompose_m2` (`src/cryptanalysis/pkm_smooth_tower.rs`) · instrument
`examples/pkm_algebraic_oracle.rs --membership resultant` · rows `runs/` · analysis `analyze.py` → `results.json` ·
figure `make_figures.py`. Counted F#sub[p] multiplications. Nothing here concerns cryptographic sizes or Pollard rho
[pcc:unverified].

EXP-PKM-DENSE-DEC-001 left one question: the F4 decomposition oracle loses to enumeration over `V` by a factor that
grows like `S` — is that a property of algebraic decomposition, or of F4? This study replaces F4 by a structured
oracle. `g(X) = ∏_(v∈V) S#sub[3](v, X, x#sub[R])` (the resultant in `Y` of `S#sub[3](Y, X, x#sub[R])` and
`∏(Y − v)`, up to a constant) is built by a balanced product and evaluated on all of `V` by a remainder tree;
`x#sub[1] ∈ V` starts a relation iff `g(x#sub[1]) = 0`. Each hit is finished exactly as enumeration does.
The polynomial arithmetic is the repository's counted toolkit (`isogeny_eval::poly`), Karatsuba or three-prime NTT,
whichever counts cheaper per product.

== Result

The gap to enumeration is no longer polynomial: it grows by 8.4× from `S = 2^8` to `2^16`, against the
F4 oracles' `S` (dense) and `S^2.5` (tower). The oracle is 30–120× cheaper than F4 with dense membership and
250× (`S = 16`) to 4.6·10⁵× (`S = 1024`) cheaper than the tower. It still costs more than enumeration from `S = 32` on, 56× at `S = 2^16`.

#table(columns: 4, align: left,
  [], [prediction (frozen)], [verdict], [measured (planted targets, median)],
  [Validity], [relations equal enumeration's; shared targets identical to EXP-PKM-DENSE-DEC-001], [*valid*],
    [150 rows agree; 90 targets shared with the committed rows, all identical in target, relation count and
     enumeration cost],
  [R1], [resultant ÷ F4 dense `< 0.01` at `S ≥ 256`], [*falsified* (held at `S = 4096` only)],
    [0.033, 0.025, 0.019 (`S = 256 … 1024`), 0.013 (2048), 0.0086 (4096)],
  [R2], [resultant ÷ enumeration `> 1` at every `S`], [*falsified* at one cell],
    [`P1`, `S = 16`: 0.86 (cheaper than enumeration); then 1.43 (`S = 32`) rising to 55.7 (`S = 2^16`)],
  [R3], [growth exponent in `[1.05, 1.45]` (`P3`, `S = 2^8 … 2^16`)], [*held*], [`S^1.38`],
  [R4], [ratio to enumeration grows `< 16`× from `2^8` to `2^16`], [*held*], [8.4×],
  [R5], [random ÷ coset `V` in `(0.8, 1.25)`], [*held*], [1.000 at every `S`],
)

#figure(image("figures/cost.svg", width: 100%),
  caption: [Left: counted cost of finding every relation of a planted target. Right: the oracle's cost over
  enumeration's; the dashed curve is `log² S` scaled to the last point.])

== Reading

- *The polynomial gap was F4's.* Writing the same decomposition question as one resultant and a multipoint
  evaluation removes it: the oracle grows like `S^1.38` here (about `S log² S`), so its gap to enumeration is
  polylogarithmic. At `S = 2^13`, where F4 dense was censored at ≥ 8.1·10⁹ multiplications per target, it needs
  5.4·10⁷.
- *Enumeration still wins.* Enumeration is linear, about 200 multiplications per element of `V`. A quasi-linear
  algebraic method pays at least a `log² S` factor on top of a larger constant, and at `S ≤ 16` the two cross: that is
  the one cell that falsifies R2, an effect of constants rather than a trend.
- *Where the cost is.* The remainder-tree evaluation is 70–74% of the work for `S ≥ 128`, the product of quadratics
  about 20%, the product tree of `Z = ∏(X − v)` 6–7%, the finishing stage under 1%. The tree of `Z` and its Newton
  reciprocals depend only on `V`, so in relation collection (one `V`, many targets) they could be paid once; this
  study charges them per target, as the protocol says. Amortising them would shrink the constant, not the `log² S`.
- *Smooth or random `V`.* Irrelevant again (R5): the cost depends only on `|V|`.
- *R1's threshold was too optimistic below `S = 4096`.* The oracle is 30–120× cheaper than F4 dense, not 100×
  everywhere; the advantage grows with `S`, as both exponents predicted.

== Limits

`m = 2` only: at `m = 3` the same construction needs a degree-`S²` product and is not expected to help. Degree-2
Kummer towers, two primes, 4 planted and 2 random targets per cell. One target per cell was also run as a smoke check
before the full run (`P3`, `S = 256`, after the protocol was frozen); its rows equal the full run's. F4 dense
references are the committed EXP-PKM-DENSE-DEC-001 medians on the same targets. NTT word multiplications are counted
at one `M` (the toolkit's stated conversion); inversions at `2⌈log₂ p⌉` (the enumeration's convention).

== Reproduce

`cargo build --release --example pkm_algebraic_oracle`, then `run.sh` (under a minute), `python3 analyze.py`,
`python3 make_figures.py`. The binary's SHA-256 is in `runs/binary.sha256`. The library test
`resultant_oracle_finds_exactly_the_enumerated_relations` checks exact agreement with enumeration on three primes.
