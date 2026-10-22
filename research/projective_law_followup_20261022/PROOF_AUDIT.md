# Weighted-law proof audit, 2026-10-22

This audit addresses the exact case split in `src/bin/projective_law_gate.rs` on
the 42 recorded Scholten models. It adds a finite, deterministic audit; it
does **not** certify universal completeness. The two already measured
1,260-target collector arms and their cost ratios are unchanged.

## Input and execution

`projective-proof-audit` reads the 42 distinct prime/model receipts in
`../five_leads_e2e_20261019/correctness/genus2-model-gate-v1.jsonl`. Each
receipt verifies the affine coordinate map to the monic quintic without an
`x^4` term on 100 transferred divisors and 100 additions. For each mapped
model, the new Rust audit constructs a 2,048-step divisor pool and draws
deterministic 2+2 and 1+2 pairs at scales `(1,1,1,1)` and `(2,3,5,7)`.
It compares every returned weighted result to the independent Cantor
`Jacobian::add`. The seed and branch counts for each model are in
`results/proof-audit-20261022.jsonl`.

From the repository root:

```sh
cargo build --release --offline --locked --manifest-path research/projective_law_followup_20261022/Cargo.toml --bin projective-proof-audit
research/projective_law_followup_20261022/target/release/projective-proof-audit > research/projective_law_followup_20261022/results/proof-audit-20261022.jsonl
```

The audit checked 1,020,000 sampled 2+2 pairs and 204,000 sampled 1+2
pairs across all 42 mapped models, with zero mismatches or unresolved
`Option::None` returns. The 2+2 branch counts were 1,018,216 common
degree-two, 594 degree-one drop, 617 shared cancellation, 562 shared
positive, and 11 same-`u` mixed. The 1+2 branch counts were 203,752
common, 124 cancellation, 121 coincident double plus other, and 3 tripling.
These counts are finite observations, not probabilities or a proof.

## Case predicates that can be discharged algebraically

Assume a smooth quintic `y²=f(x)` over a field of odd characteristic,
valid reduced Mumford inputs, and nonzero projective weights. The following
facts concern branch *selection* only; they do not establish the output
formula identities.

1. In `add12_weighted_complete`, `k=0` means the point's `x` is a root
   of the quadratic `u`. Both its `y` and the `v` value at that root
   square to `f(x)`, so they are equal or opposite. Thus
   `coincident_y_mismatch` is unreachable for valid inputs.
2. In `add22_weighted`, `bb=0` and `dq=0` imply
   `bb=-dr²=0`, hence `dr=0`; the same-`u` branch has already run.
   Thus `shared_zero_q_delta` is unreachable for valid inputs.
3. In the same-`u` branch, equal `v` slopes and unequal constants
   imply `v₂-v₁=c≠0`. Because both `vᵢ²=f mod u`,
   `c(v₁+v₂)=0 mod u`. The sum has degree at most one, so it vanishes
   identically: `v₂=-v₁`. The inverse branch has already run. Thus
   `same_u_mixed_zero_slope_delta` is unreachable for valid inputs.
4. In the 2+2 shared-support branch, the recovered points share an
   `x` coordinate on the same curve. Their `y` values are equal or
   opposite after accounting for weights; `shared_y_mismatch` is
   unreachable for valid inputs.
5. For a non-Weierstrass point `P`, the degree-two representation of
   `2P` has `u=(x-x_P)²`. Therefore a second point `R` with
   `x_R≠x_P` cannot hit a secondary coincident support in the
   `2P+R` path. If `x_R=x_P`, the remaining possibilities are the
   inverse or tripling branches. Smoothness excludes a valid reduced
   `u=(x-x_P)²`, `v(x_P)=0` at a Weierstrass point because it would
   force `f'(x_P)=0`.

The [published complete law](https://doi.org/10.1007/s13389-024-00351-7)
is for the mapped equation family. The implementation corrected the
homogeneity of one `W` power in the doubling expression; the retained
`p=739` regression demonstrates that the literal transcribed term gives
a different result. The paper's completeness statement cannot by itself
certify the modified implementation.

## Remaining proof obligation

### Exact generic 1+2 identity, added after the finite audit

The new native `projective-symbolic-12` checker proves seven equalities as
canonical polynomials over
`Z[x,y,q,r,s,t,a0,a1,a2,a3]`. The output is
`results/symbolic-1plus2.json`. This is an exact coefficient proof for
the **affine-input, K ≠ 0** branch, not another finite-field sample.

Write `u(X)=X²+qX+r`, `v(X)=sX+t`, `F(X)=X⁵+a3X³+a2X²+a1X+a0`,
and `F-v²=uH+C`, where `deg C<2`. A valid Mumford input has `C=0`.
For a point `P=(x,y)`, put `K=u(x)`, `A=y-v(x)`,
`B=sK+Aq`, and `D=tK+Ar`. The branch condition is `K≠0`.
Let `h1` be the coefficient of `X` in `H`; exact division gives
`h1=a3+q²-r`. Define

```text
Q = K²(x-q)-A²
R = K² h1 - A(B+sK) + xQ
S = AQ-BK²
T = ARK²-DK⁴
U = K²X²+QX+R
N = Kv+Au
G = K²H-2KAv-A²u.
```

The checker verifies, without reducing modulo a prime or substituting
random values,

```text
K²F-N² = uG + K²C
G = (X-x)U + G(x)
G(x) = K(F(x)-y²-C(x))
K²(K²N-AU)+K²SX+T = 0.
```

Hence, if `C=0` and `y²=F(x)`, then with `L=N/K`,
`F-L²=u(X-x)(U/K²)`, and
`L≡-[(S/K³)X+(T/K⁵)] mod U/K²`. These are exactly the generic
affine-input coordinates returned by `add12_weighted_generic` after
normalization: its `q3=Q`, `r3=RK²`, `s3=S`, `t3=T`, and
`z3=K,w3=1`. The standard divisor-of-`y-L` argument identifies this
reduced Mumford pair with `P+D₂`. This proves the branch's output
identity for valid affine inputs and `K≠0`. The native checker proves
the stated polynomial equalities; the displayed divisor argument is
written mathematics, not a Lean theorem.

Proof map: the exact coefficient identities are discharged by
`src/bin/projective_symbolic_12.rs` and its committed result. The
divisor-class bridge and the weighted-coordinate generalization have no
Lean declaration yet. A core-only Lean 4.34.1 probe found neither
`Field` nor `Polynomial` in `Std`; completing a faithful Lean version
needs a pinned algebra library or local definitions and a verified
divisor-class representation. No weakened conditional Lean theorem is
claimed as the full group-law proof.

### Open cases

A machine-checkable symbolic reduction is still needed for every output
branch other than the affine-input generic 1+2 case: show its nonzero
weighted coordinates normalize to the Cantor sum, including degree drops
and shared-support positive composition. The finite audit and five
branch-selection lemmas above do not supply these remaining identities.
In particular, no exhaustive all-divisor
pair enumeration was run on any of the 42 curves. No counterexample to
the *implemented* weighted law was found in the stated finite audit;
absence of a counterexample is not a universal proof.
