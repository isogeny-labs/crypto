# Separate Rosenhain setup gate

Frozen on 2026-10-22 at 05:28 UTC before execution. The earlier
[signed-invariant protocol](PROTOCOL.md) and its evidence remain immutable.

Test the coordinate-model setup prerequisite on the same 21 independently
factored quintics. For each `f(x)=c5*prod(x-rj)`, try all 20 ordered pairs
`(r0,r1)` of distinct finite roots as the `0,1` normalization. Put
`delta=r1-r0` and `t=(x-r0)/delta`. Choose the first pair in sorted-root
order for which `c5*delta^5` is a square in `Fp` and record its square root.
Set the other three transformed roots to `(rj-r0)/delta`; independently
reconstruct the degree-five monic Rosenhain polynomial from
`0,1,lambda,mu,nu` and compare every coefficient with the transformed
original polynomial. Retain every per-model row and failure. Report whether
an `Fp` Rosenhain map exists in this normalization for every model.

This is a model-setup diagnostic, separate from the completed target gate.
It does not establish theta constants, Kummer projection, xADD/xDBL,
signed point recovery, or any collector cost. Publish raw rows, a graph, and
a PDF with an independent run record. Candidate/rho costs remain null.
