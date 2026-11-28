# Short scalar returns to the M61 root base

The native [scalar-return example](native/examples/scalar_returns.rs) takes all
2,447 canonical lifts with x-coordinate in `μ_4950` on
`y²=x³−3x+40` over `p=2^61−1`. For every base point `P` and every
`m=2,…,128`, it computes `[m]P` by successive group addition and looks up
its x-coordinate in the same base. Both signs of a base column are accepted
by the x lookup. The exhaustive run performs **310,769** group additions,
encounters no infinity, and finds **zero** root-base returns. Its frozen
output is [scalar_returns.out](scalar_returns.out).

Run from the repository root:

```sh
cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example scalar_returns
```

A return `[m]P=±P_j` would give a directly checked two-column relation,
with coefficient `m` on one column. The uniform group model predicts only
`310769·(2·2447)/(7r)≈6.60×10⁻¹⁰` returns in this scan, so zero is the
ordinary prediction. This exact selected-instance result rules out short
scalar returns for `m≤128`; it is not evidence for a general lower bound.

For `m=2`, the Lattès x-map is the degree-four rational function

`x([2]P)=(x⁴+6x²−320x+9)/(4(x³−3x+40))`.

Intersecting its image with `μ_4950` can equivalently be expressed as the
common-root test of `x^4950−1` and the numerator of
`x([2]P)^4950−1`. Such an algebraic test still needs actual witnesses and
independent relation rank. A bounded collection of scalar maps, even if it
had a few hits, would not supply the `Θ(B)` independent seven-term rows and
two target descents needed by the proposed `Õ(B²)` pipeline.
