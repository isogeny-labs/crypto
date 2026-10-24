# First actual finite-count proof check

Run `uniform-lean-v2` begins 2026-10-24 01:44:51 UTC after recovery of the
interrupted empty v1 launcher. The gate compiles the actual finite-count
theorem source, not an assumed statement or an unchecked native result.

![Verification boundary](../../figures/joint-mechanism.svg)

The compiler exposes an unresolved goal in the inductive counting proof:
`1+ks.length=ks.length+1`. The build fails and its downstream declarations
show `sorryAx`; they are not accepted as proofs. The failed source text,
log and source hash remain in `runs/uniform-lean-v2/`.

The repair supplies addition commutativity explicitly. The next v3 build
is separate. Every residual label being less than n remains a stated
hypothesis; no domain-size, image-injectivity or uniform historical-sample
assumption is silently added. This is one-worker, 8 GiB, bounded proof
verification, with no performance result or quantitative plot.
