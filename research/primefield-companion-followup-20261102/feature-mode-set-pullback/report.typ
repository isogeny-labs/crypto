#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Recovering from a bounded set of images
T-1813 · 2026-11-07 · native producer evidence

Weighted modes can now transition to scalar recovery before the image becomes unique. A capped polynomial pullback preserves every candidate fiber; replaying already collected signs refines it without new source queries.

For a monic divisor $F(Z)$ of $Z(Z^d-1)$, let $epsilon=1$ when $F(0)=0$, and zero otherwise. Then
$ L(X)=F(X^m)/X^((m-1)epsilon) $ is squarefree and has degree $N=m deg(F)-(m-1)epsilon$. Its roots are exactly the scalar preimages of the retained images. The zero correction is essential. Check N against the cap before allocating.

#image("flow.svg", width: 100%)

*Public results.* All63 inherited fixtures recover, including the17 F509/r479/m2 cases unresolved by the preceding four-radius mode-only adapter. The new path records511 source calls,180 retained-sign replays and115 extra queries. Every candidate passes public-point verification. Different control flows and charged replay work prevent interpreting source counts as a timing comparison.

*Finite checks.* 712 image subsets;356 containing zero;2,756 replay prefixes;689 cap rejections. Exact root sets and degrees agree with an independent enumeration reference. All63 public fixtures also exercise the one-call stopping control. Reference enumeration is confined to checks.

*Bounds.* Four prescribed radii; scalar degree cap64; at most17 prescribed extra offsets; source-call cap128. Image/compiler memory and source setup are separately charged. Replay uses modular character powers and gcds at the shrinking lifted degree; no scalar or image root list is built at runtime.

*Evidence and scope.* validation-01, modes.rs, pullback.rs and generation.json preserve source, hashes, raw outputs and warnings. The unchanged prior adapter still reports its17 unresolved cases. Prime-field source restrictions and the original4,563-case cohort remain unchanged. No elapsed-time study, full formal proof or independent signed verdict is supplied.

*Next:* reduce repeated modular-power work during retained-sign replay, while preserving the exact set semantics and fixed cost accounting. Source batching and domain extension remain separate construction tasks.

Sources: local weighted-mode, retained-fiber and half-GCD components; FLINT nmod_poly conventions (flintlib.org/doc/nmod_poly/index.html). KB unavailable; fallback recorded in DERIVATION.md. Polynomial inflation and gcd are standard operations.
