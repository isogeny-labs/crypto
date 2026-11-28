# Fixed-curve affine scaling: exact applicability restriction

All six frozen fixtures have centered gap-gcd K=1 and exactly one admissible
x-scaling. The two y-signs are identity and hyperelliptic negation. This rules
out a direct affine-scaling orbit as a new same-curve relation multiplier on
these fixtures. It does not rule out more general transformations or methods.

The exact criterion and proof are in PROTOCOL.md and REPORT.pdf. For monic
odd degree d with p odd and p not dividing d, center f to F(X)=f(X+s),
s=-f[d-1]/d. Leading coefficients require b²=a^d for a map x→ax+t,y→by.
Centering forces t=s(1-a). Each F_i!=0 gives a^(d-i)=1, so a^K=1.
The multiplicative square subgroup is cyclic of order (p-1)/2, hence the
number of compatible a is gcd(K,(p-1)/2). Nonzero a^d has exactly two square
roots when a is square. Conversely the coefficient identities reconstruct
f(ax+t)=b²f(x), so the criterion is sufficient as well as necessary in this
map class. Squarefreeness is relevant to curve interpretation, not this
polynomial identity. Fixed fixtures were validated by their original producer.

Source f76761dacff5 and checkpoint20261115T121650Z-f76761dacff5 precede finite
execution. 40 fixed polynomial controls and six existing family-0 fixtures
pass, enumerating every a!=0 and comparing original-coordinate substitution
independently with the gcd criterion. Both genus2fixtures, both genus3fixtures,
and both genus4fixtures have nonzero centered coefficients at gaps2and3,
which already force gcd1. Positive controls include p17,d9,X^9+X with eight
admissible x-scalings; the checker does not simply reject every map.
No seed/fixture/cap was selected or changed after results.

This is a known algebraic boundary and count gate, not mathematical novelty
or a performance measurement. Negation is already folded by the factor base.
The finite-field pinpointing source does not by itself provide a compatible
second-side construction on a fixed curve. The required successor must give
such a construction, retain same-target information, and price all work
against the compact incumbent before running a new cold campaign.

## Reproduce

```sh
TC=/Users/adamburan/.rustup/toolchains/stable-aarch64-apple-darwin/bin
$TC/rustc --edition 2021 -O research/hec_scaling_gate_20261115/check.rs \
  --extern hec_native_ic=/Users/adamburan/pq-work/hec-followup-target/release/deps/libhec_native_ic-282bfd176edc4a03.rlib \
  -L dependency=/Users/adamburan/pq-work/hec-followup-target/release/deps \
  -o /tmp/hec-scaling-check
/tmp/hec-scaling-check research/hec_point_sieve_20261115/fixtures
```

Library build via crates/hec-native-ic/Cargo.toml recreates the rlib; resolve
its emitted compiler-artifact path if the hash changes. CHECKS.txt preserves
all rows and terminal counts. No scientific Python, cloud workers, time
ratio, benchmark admission or closed-record mutation. PDF/diagram visually
inspected. This standalone check is local evidence, outside existing native CI.

Source access gaps and KB receipts are in PROTOCOL.md. The primary Joux PDF
was initially extractable; later archive/eprint fetches returned403. Original
paper domain is multiplicative finite-field DLP, not a demonstrated curve
algorithm. The restriction above is independently derived and checked.

Errors: first Typst render treated a literal underscore as markup and failed
with an unclosed delimiter. Replaced it with coefficient(f,d-1), rendered and
visually inspected the one-page report. Scientific code/execution unchanged.
