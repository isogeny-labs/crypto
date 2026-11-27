# Exact height of the PKM norm-one Dickson fibre

Frozen 2026-11-25 before a qualifying `p+1` witness or timing row. This
refines the construction decisions in
`research/pkm_nonenum_decomp_20261125/WIDE_PM1_DECISIONS_20261125.md`.
It is a mathematical construction and source audit, not a solver benchmark.
The `p-1` Kummer map and the auxiliary-isogeny map are separate families.

## Full, unramified fibre

Let `p` be odd, `p+1=2^v q` with `q` odd, and
`U={z in F_(p^2)^*: z^(p+1)=1}`. The trace `Tr(z)=z+z^(-1)` belongs to
`F_p`. The degree-two Dickson map `D_2(x)=x^2-2` satisfies
`D_2(Tr(z))=Tr(z^2)`. Hence `D_(2^t)(x)=c_top` has a compact chain of
`t` quadratic equations even when its degree is `B=2^t`.

If `q>1`, choose `h in U` of odd order greater than one and set
`c_top=Tr(h)`. For every `0<=t<=v`, the equation `z^(2^t)=h` has exactly
`2^t` solutions in `U`: the power map is invertible on the odd-order
subgroup, and its kernel has size `2^t`. Their traces are distinct. Indeed,
equal traces imply `z_2=z_1` or `z_2=z_1^(-1)`; the latter would imply
`h=h^(-1)`, contrary to the odd order of `h`. Thus the Dickson polynomial
has exactly `2^t` distinct `F_p` roots, and every quadratic layer splits
without ramification. One can obtain `h` deterministically by scanning
norm-one elements `g`, taking `h=g^(2^v)`, and accepting the first `h!=1`;
the scan order, nonsquare defining `F_(p^2)`, and resulting `c_top` must be
frozen before timing. This proof establishes the fibre size, not how many
abscissae lift to the elliptic curve or how fast F4 extracts witnesses.

If `q=1`, the simple `c_top=0` fibre is unramified for `t<=v-2`:
`Tr(z^(2^t))=0` asks for a fourth-root trace at the top, so the torus
must contain elements of order `2^(t+2)`. At `t>v-2` this zero-top
construction fails. For any `q`, the same `c_top=0` choice requires
`t+2<=v`. The current native `dickson_abscissae` explicitly rejects a
ramified or nonsplit layer and checks that it produced `2^t` distinct
roots; it does not select the nonzero `c_top` needed at full height.

Two named fields have explicit full-height top-value routes. The NIST
P-192 prime is `p=2^192-2^64-1`, so
`p+1=2^64(2^128-1)` and `3 | (2^128-1)`. An order-three element `h` of
`U` satisfies `h+h^(-1)=-1`; therefore `c_top=p-1` gives exactly `2^64`
distinct abscissae through 64 quadratic layers. This fixes the high-
and low-degree maps mathematically but does not make the current `u64`
solver or full-rank pipeline feasible. The s59-plus prime is
`p=2^58+2^18-1`, so `p+1=2^18(2^40+1)` and
`257 | (2^40+1)` because `2^8=-1 (mod 257)`. For a norm-one element
`g` with `g^((p+1)/257) != 1`, choose
`h=g^((p+1)/257)` of order 257 and `c_top=Tr(h)`. A deterministic
`F_(p^2)` representation and scan for the first such `g` still need to
be frozen and replayed before an s59-plus solver run.

## Named-field decisions for the existing three-summand plan

The prime valuations and density proxies are from `WIDE_M3_GATE.md`.
`t_zero` is the maximum height supported by the zero-top fibre alone;
`t_full` is the mathematical maximum with a noncritical top fibre. The
height used by a resource-gated design can be smaller than either maximum.

| Field | `v=v2(p+1)` | Odd part `q` | `t_zero` | `t_full` | Construction and cost decision |
|:--|--:|:--|--:|--:|:--|
| s59 minus | 1 | `>1` | none | 1 | A noncritical height-1 fibre exists, but the three-summand density screen rejects this side. |
| s59 plus | 18 | `2^40+1`, divisible by 257 | 16 | 18 | Select an order-257 norm-one `h` and `c_top=Tr(h)` as above. The current `u64`, `t<30` representation can express height 18; numeric selection, split/lift verification, F4, rank, memory and cold cost remain open. |
| s59 registry | 1 | `>1` | none | 1 | Height 1 is constructible with noncritical top; density rejects this side. |
| NIST P-192 | 64 | `2^128-1`, divisible by 3 | 62 | 64 | Use `c_top=p-1` from an order-three norm-one element. The exact degree-`2^64` fibre exists mathematically, but current field/tower widths and full-rank resource scale block implementation. |
| NIST P-224 | 1 | `>1` | none | 1 | A height-1 fibre exists; density rejects this side. The distinct `p-1` height-76 row remains in the main gate. |
| NIST P-256 | 96 | `>1` | 94 | 96 | The main gate's height 87 can use `c_top=0`; its required field width and full-rank scale exceed the bounded implementation. |
| NIST P-384 | 32 | `>1` | 30 | 32 | Height 32 needs nonzero `c_top`, but the density proxy already rejects this side. |
| NIST P-521 | 521 | `1` | 519 | 519 | The main gate's height 175 can use `c_top=0`; field width and full-rank scale exceed the bounded implementation. |

An unshifted trace of the entire order-`2^t` subgroup has only
`2^(t-1)+1` distinct traces because inversion pairs elements. The
noncritical **fibre** above is a different set: it has exactly `2^t`
traces. Use its measured lifted-point count, not either algebraic root
count alone, in the relation-density and full-rank cost ledger. A `p+1`
row is not admitted to timing until a source-pinned `c_top`, split-root
certificate, curve lifts, and witness-producing solver pass the same
cold-cost controls as the `p-1` candidate.
