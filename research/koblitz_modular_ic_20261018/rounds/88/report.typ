#set page(paper: "a4", margin: 20mm)
#set text(size: 10pt)
= R88: rational inverse of the diamond source map

The accepted R85/R86 forward map has an explicit inverse. In the
transformed coordinates `p,q,t`, let `k=p+q+p²` and `G=pk²+p+k²`.
Then `z=1/k`, `V=1/k+1/p`, and
`T=G[t+(k+1)²]/(pk⁴)` on the registered open.

#image("inverse88.svg", width: 100%)

The frozen Singular run checked both composition directions, one
target quadratic ideal lift and three source quadratic ideal lifts.
Both corrupted lambda returns failed. The target base return has
the exact product `qResidual=k E_target`; Lean proves this product,
its consequence under the target equation, and the affine lambda
cancellation. The source-side ideal lifts remain native certificates.

With R87's degree-16 equation preservation, this fixes the rational
field map and inverse. The next construction is the regularity test:
transport all 32 accepted RR PID generators, clear their poles, and
produce full module lifts in both directions.
