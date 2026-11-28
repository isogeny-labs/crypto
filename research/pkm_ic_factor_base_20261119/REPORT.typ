#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#set par(justify: true)

= A Kummer-coset factor base in the prime-field index calculus

EXP-PKM-IC-001 · 2026-11-19 · protocol `PROTOCOL.md` (frozen in `dd36ca1ce7` with the curve fixtures, before any row;
no amendments) · instrument `ec_index_calculus::build_factor_base_from_xs`,
`ec_index_calculus_dlp_staged_seeded_with_factor_base`, driver `examples/pkm_ic_factor_base.rs` · rows `runs/` ·
analysis `analyze.py` → `results.json` · figure `make_figures.py`. Nothing here concerns cryptographic sizes
[pcc:unverified].

The PKM studies found that a Kummer-coset factor base (abscissae `g·μ_S`, the degree-2 tower) costs the same as a
random one in every decomposition oracle measured. This study puts the same factor base into the repository's actual
prime-field index calculus — the `m = 2` pair-sum table, relation collection, linear algebra and verification — on
three prime-order `a = −3` curves whose `p − 1` carries `2^14` or more, and compares it with the default small-`x`
factor base and a random one of exactly the same size `M`, on the same targets with the same relation sampler.

== Result

*The shape of the factor base does not matter, and the solve is far from rho.* All 108 solves return the correct
logarithm. In every cell the Kummer and random factor bases cost within 8% of the small-`x` default, in charged
group operations and in trials per relation. The whole solve costs 114× to 1,300× the expected rho cost at these
sizes.

#table(columns: 4, align: left,
  [], [prediction (frozen)], [verdict], [measured (median of 6 targets)],
  [K0], [every solve correct], [*held*], [108 / 108],
  [K1], [charged group ops ÷ small-`x` in `(0.8, 1.25)`, Kummer and random], [*held*],
    [Kummer 0.99–1.08, random 0.95–1.01],
  [K2], [trials per relation, Kummer ÷ small-`x`, in `(0.8, 1.25)`], [*held*], [0.98–1.09],
  [K3], [charged group ops ÷ `√(πn/2)` `> 1`], [*held*],
    [114–131 (20-bit), 184–282 (24-bit), 665–1,308 (28-bit)],
)

#figure(image("figures/cost.svg", width: 100%),
  caption: [Left: charged group operations per whole solve. Right: the same divided by the expected rho cost.])

== Reading

- *Why the shape cannot matter here.* A trial succeeds when the random target's abscissa is one of the `≈ M²`
  abscissae of `±F_i ± F_j`. For any factor base of `M` points those sums are spread over the group in the same way
  unless the abscissae have additive structure; a multiplicative coset (`g·μ_S`) has none the addition law can see.
  The decomposition oracles of the earlier studies saw the same thing from the algebraic side.
- *Distance to rho.* The charged cost is the table (`≈ M²` additions) plus `≈ n/(2M²)` trials per relation at a
  scalar-multiplication each; at the best `M ≈ n^(1/3)` that is `≈ n^(2/3)` against rho's `n^(1/2)`, so the ratio
  grows with `n`. The cells here use `M` below that optimum at 24 and 28 bits (the protocol capped `M` for the linear
  algebra, which turned out to be cheap), so these ratios are upper ends; the trend, not the constant, is the point.
- *Where the PKM line stands.* Across EXP-PKM-ODD, IDEAS, DENSE-DEC, RESULTANT and this study, the Kummer/isogeny
  structure of `V` gave no measured advantage in any oracle or in the whole solve, and at `m = 2` the cheapest relation
  oracle is the plain table [pcc:unverified].

== Limits

`m = 2` only; three curves (20, 24, 28 bits), two factor-base sizes each, 6 targets per cell. Charged cost is the
harness's group-operation count; the elimination's modular work is counted by the harness as `*_uncharged` and left
out, as are square roots in factor-base construction. The rho reference is the theoretical `√(πn/2)`, not a measured
matched rho, so K3 is a distance, not a validated comparison, and nothing here goes to the leaderboard.

== Reproduce

`cargo build --release --example pkm_ic_factor_base`, then `run.sh` (about a minute), `python3 analyze.py`,
`python3 make_figures.py`. `find_curves.py` regenerates `curves.json`; the driver re-checks `[n]G = O`.
