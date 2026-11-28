# Ratio fibers of the M61 root x-base

This tests a possible way to organize pair sums using the multiplicative
structure of `H=μ_4950`. Let `z=636328216386749903` be the verified
generator and let `v_i∈{0,1}` say whether `x=z^i` lifts to a rational
point on `y²=x³−3x+40`. The native census gives `B=Σv_i=2447`.
For each ratio `t=z^s`, define

`C_s=Σ_(i mod 4950) v_i v_(i+s)`.

This counts ordered lift-valid pairs `(x,tx)`. `C_0=B`,
`C_s=C_(4950−s)`, and `Σ_s C_s=B²`. In particular the exact mean over
nonidentity ratios is `B(B−1)/4949=1209.408365…`.

The [dependency-free Rust probe](native/examples/ratio_fibers.rs) checks
those identities and scans all 4,949 nonidentity ratios. It finds
**minimum 1,157, maximum 1,257**, with RMS deviation 17.098 from the
fixed mean. The densest fiber has only 3.94% more pairs than the mean.
The same run checks that the defining cubics of every nonidentity ratio
have no common roots, supporting the [genus-four cover calculation](ratio_cover.md).
The `t=−1` fiber contains 1,232 ordered pairs, or 616 unordered pairs;
its extra involution and quadratic-in-`x²` pair query are analyzed in that
cover note.
Its [frozen output](ratio_fibers.out) compares 128 deterministic shuffles
of the same 2,447 valid positions: their mean maximum is 1,271.54, with
range 1,262–1,304; every shuffled maximum exceeds the actual maximum.
Their mean minimum is 1,148.42, and 125 of 128 minima are at most the
actual minimum. The root mask's RMS is lower than 117 of 128 shuffled
controls. This is a finite mask diagnostic, not a significance claim for
an instance family.

Run from the repository root:

```sh
cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example ratio_fibers
```

A ratio-fiber norm could describe pair-sum factors by setting
`x_j=t x_i`; the summation polynomial `F₃(x_i,t x_i,U)` is quartic in
`x_i` and quadratic in `U`. This is a compact algebraic presentation,
closely related to the cyclic-norm folding already discussed in the
repository's prime-field program §4.6. The measured fiber sizes provide
no high-density ratio class: all retain `Θ(B)` pair work, and about `B`
ratio classes remain. More importantly, `C_s` concerns lift validity
only. It says nothing about locating target-dependent elliptic sums or
extracting signed witnesses. The desired `Õ(B²)` relation locator is
still absent.
