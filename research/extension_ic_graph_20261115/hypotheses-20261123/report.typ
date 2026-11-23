#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9.2pt)
#set par(leading:0.45em)
#set heading(numbering:none)
= Four hypotheses for extension-field graph IC
23 November 2026 · Proposed mechanisms; no new solver or timing experiment

The search targets cheaper algebraic relation finding and useful projected equations in the existing genus-3 cover over Fp². It does not propose a general prime-field attack. All candidate gains are unmeasured; derivations and producer checks remain *[pcc:unverified]*.

#image("hypotheses.svg",width:100%,height:66mm,fit:"contain")

== 1. Choose a sparser multiplication operator
The linear stage currently fixes the last coordinate. Try a bounded set of other coordinates in the same accepted quotient basis. A cheaper separating operator reduces every Krylov product; unit-column counts alone do not establish that its remaining columns are sparse.

*Test:* select on a development cohort, freeze the policy, then compare exact roots and total linear-stage work on disjoint inputs. Include matrix construction, projections, BM, read-out, nonseparating attempts and fallback. Reject if the complete stage's cost does not fall.

*Rust:* an opt-in action-variable argument in `zero_dim_la::fast_inner`, with a coherent read-functional map. The old NF-pattern replay is a different, previously closed lead.

== 2. Compile a syzygy-pruned Nagao template
Use the repeated family's polynomial provenance and syzygies to construct the multiplication operator from a smaller fixed elimination template, instead of running the full F4 sequence per residual. Optimize the basis and operator together with H1.

*Test:* compare template generation, fill, coefficient evaluation, elimination, extraction and all guarded fallbacks against the existing six-coordinate trace on disjoint inputs. Reject if it is no cheaper or completeness cannot be qualified. Returned-root checks and a 64-monomial stop do not certify completeness.

*Rust:* a separate solver callback through `jv_nagao_unit::decompose_using`. Existing group replay, field code and materializer remain the reference interfaces.

== 3. Switch to affine descent before nullity one
Stop homogeneous collection with certified nullity k greater than one, retain a complete kernel basis K, and carry the k nuisance coordinates into the protected-target affine system. The extra descents may cost less than the collection tail.

*Test:* compare saved collection work with added kernel, descent and recovery costs at a bounded checkpoint set such as k=1,2,4,8. Freeze the policy before disjoint evaluation. In `C t + d x = a`, unique target recovery requires `rank([C|d]) = rank(C)+1`; nuisance coordinates need not all be recovered. The current k=1 case still generally needs two independent rows.

*Rust:* an opt-in generalization of the kernel quotient and counted affine system. A single kernel vector is insufficient when k is greater than one.

#pagebreak()
= 4. Sieve a family for one fixed residual
Use the existing monic degree-seven cover H and a validated reduced residual R=(u,v), with u monic of degree three. Write W=(H−v²)/u and use

```
C = x² + c1 x + c0
L = l2 x² + l1 x + l0
A = u L − C v
S = C² W + 2 C v L − u L²
```

The norm identity `C²H−A² = uS` is qualified separately by the native helper and certificate `IDC1h3a70da000d31ad8a`. Eleven native tests and scoped Clippy pass; this is producer evidence, not a signed verdict. Five free Fp² coefficients yield ten Fp coordinates; requiring the eight lower coefficients of the monic degree-eight S to lie in Fp supplies eight quadrics. Expected dimension two is only a naive count. It establishes neither independence of the constraints nor a rational parametrization or a cheap sieve. [pcc:unverified]

The functions lie in `L(11 infinity − R)`. Reuse setup for many functions of this one residual, rather than solving a new square system for each walked residual. The quotient has eight points instead of six, lowering full-split probability in the random-polynomial model.

*Break-even hurdle:* full-only splitting changes from 1/720 to 1/40320. Per admissible coefficient solution, candidate generation must therefore be more than 56 times cheaper before setup. With independent uniform quadratic-anchor coverage f, the heuristic acceptance models are

```
q6(f) = (1 + 15f + 45f² + 15f³) / 720
q8(f) = (1 + 28f + 210f² + 420f³ + 105f⁴) / 40320
```

Require `g8/g6 < q8/q6`, including setup and failed fibers in g8. Neither random-polynomial distribution nor independent anchor coverage is established for this family. Repeated outputs can also be dependent. Reject if no useful fiber description meets this cost/yield condition. Do not assume the original constant-C quadratic sieve applies to this larger family.

*Rust:* a separate coefficient-form builder, then a qualified component control using the existing exact arithmetic, signed class encoding and witness replay. Only the norm-identity helper is implemented by this search.

== Evidence that constrains the proposals
The November 19 L1 paired study already reports 96 fresh-target panels for anchored quadratic acceptance, with an inconclusive promotion interval. It is prior unsigned evidence, not a new hypothesis. The L5 audit places Krylov and BM at 87.60% of its linear-stage multiply-adds; NF reductions contribute no multiplications there. This motivates changing the operator rather than repeating the closed NF-replay lead. No prior timing or whole-solver gain is imported. [pcc:unverified]

== Sources and retrieval limits
Joux–Vitse, EUROCRYPT 2012, sections 3.1–3.3: norm quotients and the distinction between fixed families and per-residual systems. Faugère–Mou, Sparse FGLM, arXiv:1304.1238, sections 2–3: sparse multiplication operators and shape-position qualifications. Larsson–Åström–Oskarsson, CVPR 2017, sections 2–3: prime-field elimination templates and syzygies. Larsson et al., arXiv:1803.04360, sections 1–2: quotient-basis selection. Exact links, source pins and prior study paths are in README.md.

KB tools/wrapper and Conductor are unavailable; the sibling atlas could not be decoded. This is a bounded search, with no novelty or repository-wide absence claim. Graph implementation ownership #4905/#4929 is preserved. This report provides no new runtime or solver measurement; canonical scoreboards stay unchanged.

Full hypotheses, falsifiers and native receipts: #link("https://github.com/isogeny-labs/crypto/pull/4986")[PR #4986], `research/extension_ic_graph_20261115/hypotheses-20261123/`.
