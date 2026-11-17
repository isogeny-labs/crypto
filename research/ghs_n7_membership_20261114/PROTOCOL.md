# Frozen n=7 generalized GHS membership check

14 November 2026, 21:36 PST. Conductor T-5944. This is a finite algebra and
membership check for a **published** genus-14 locus, not an ECDLP performance
run or an assertion that the full attack beats rho. Claims remain
`[pcc:unverified]` until the repository's independent verdict gate accepts
their proof records.

## Literature and hypothesis

Menezes and Teske, [*Cryptographic Implications of Hess' Generalized GHS
Attack*, §5.2](https://eprint.iacr.org/2004/235.pdf), define `W1` over
`K=F_(q^7)`, `k=F_q`, `q=2^ell`, using either irreducible cubic
`p=T^3+T+1` or `p=T^3+T^2+1`. They state that they did not have a membership
test below `q^2.5` operations for this case. Hess's
[*GHS Attack Revisited*](https://www.iacr.org/archive/eurocrypt2003/26560374/26560374.pdf)
gives the general Artin–Schreier transfer; the later
[Karabina–Menezes–Pomerance–Shparlinski analysis](https://math.dartmouth.edu/~carlp/WeilDescent-Product.pdf)
changes asymptotic attack comparisons. Repository prior work includes
[`research/teske_variants_field_types_20261018/REPORT.md`](../teske_variants_field_types_20261018/REPORT.md)
and [`research/ghs_binary_cubic_isogeny_20261026/REPORT.md`](../ghs_binary_cubic_isogeny_20261026/REPORT.md).

Hypothesis H1: enumerating one representative of every `k*`-line in
`V=ker p(sigma)` decides `W1` exactly in at most `q^2+q+1` quotient tests.
Hypothesis H2: every `z=gamma1*gamma2` from this locus has relative trace
zero, allowing a cheap rejection test before H1. These are proposed
mathematical results, separate from any attack-cost claim.

## Frozen inputs, controls, and stop conditions

- Inputs: `ell ∈ {1,2}`, `q=2^ell`, `K=GF(2^(7ell))`; choose the first monic
  irreducible binary polynomial of degree `7ell` in ascending integer-bit
  order with constant term one. Test both cubics above. The native source is
  `src/main.rs`; no Python research arithmetic is used.
- Direct control: enumerate all nonzero `gamma1 ∈ V`, all nonzero `w ∈ V`,
  and `c ∈ k*`; collect `z=gamma1*(w+c)`.
- Projective arm: enumerate one representative of each `k*` orbit in `V*`;
  reject `z` when `Tr_(K/k)(z) != 0`; otherwise test whether
  `gamma2=z/gamma1` has exact Frobenius order `p(T)(T+1)`.
- Positive/negative checks: cardinalities `|V|=q^3`, `|P(V)|=q^2+q+1`,
  direct/projective set equality for each `p`, and relative trace zero for
  every direct member. Record the union across the two cubics and the full
  trace-zero population. Any assertion failure falsifies H1 or H2 for that
  finite field and stops promotion.
- Record exact source curve witnesses as `y^2+xy=x^3+a*x^2+b` with
  `Tr_(K/F2)(a)=1`, field modulus, coefficient masks, and independently
  enumerated group order. These are membership examples only; no subgroup
  transfer or logarithm cost is inferred.
- Native command: `cargo test --manifest-path
  research/ghs_n7_membership_20261114/Cargo.toml --release`, then
  `cargo run --manifest-path research/ghs_n7_membership_20261114/Cargo.toml
  --release`. Retain stdout, binary hash, source revision, host, and run time
  in a write-once run directory. A second implementation or verifier is
  required before promoting finite counts.
- Resource bound: local CPU, expected under one minute at these two field
  sizes; stop after a ten-minute wall limit or any assertion failure. There
  is no rho arm because this protocol tests membership, not an attack.

## Decision boundary

If H1/H2 pass, publish the exact bound and finite observations as unverified
research evidence, with a proof map and independent prior-work review. The
known `W1` locus does not become a new curve family. A vulnerability claim
requires an evaluated nonzero subgroup map, target decomposition and linear
algebra, complete cost, and a matched same-target rho arm.
