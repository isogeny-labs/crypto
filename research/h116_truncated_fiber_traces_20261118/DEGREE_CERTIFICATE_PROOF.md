# H116 paired-degree certificate proof

Lean now isolates the exact geometric bridge from the already-proved ring
factorization

```text
(V-1)(pi-1) = [p+1-t].
```

`degree_eq_norm_of_paired_factor` proves, for an abstract multiplicative degree,
that `left*right=scalar`, `deg(scalar)=N^2`, and
`deg(left)=deg(right)` imply `deg(left)=N`.
`verschiebung_sub_one_degree_of_dual_certificate` composes this lemma with the
existing norm factor and concludes `deg(V-1)=N`.

For an actual elliptic curve the remaining inputs have precise meanings:

| Certificate premise | Required elliptic theorem | State in pinned Mathlib |
|---|---|---|
| `degree (f*g)=degree f*degree g` | multiplicativity of isogeny degree | unavailable |
| `degree [N]=N^2` | degree of scalar multiplication | unavailable |
| `degree(V-1)=degree(pi-1)` | `(V-1)^vee=pi-1` and equal degree of duals | unavailable |

The pinned Mathlib audit found affine/projective elliptic points and
division-polynomial degree results, but no elliptic-isogeny type, dual isogeny,
or morphism-degree API.  Consequently these are explicit caller obligations,
not hidden axioms.  `degree_product_without_duality_control` checks the logical
boundary: at `N=6`, unequal degrees 4 and 9 still multiply to `N^2=36`, so the
dual/equal-degree premise cannot be dropped.

The current Lean source SHA-256 is
`fe907f1887b4a963dcb1dafbcf3f93eac8037355c656189eacc5a227dcc2d3ca`.
The pinned build succeeds, and the repository auditor reports 128 theorems,
zero failures and zero trusted native decisions.  This proves the implication
from the geometric degree certificates; it does not construct those elliptic
certificates and is not a subexponential ECDLP proof. [pcc:unverified]
