#set page(paper: "a4", margin: 17mm)
#set text(font: "Libertinus Serif", size: 10pt)
#set par(justify: false)
#set heading(numbering: none)
#set document(title: "Higher-pole rational relations: algebra and anchor rank")
= Higher-pole rational relations
*Algebra and anchor-rank screen · 15 November 2026*

An established rational interpolation construction passes the algebra gate
and permits smaller anchor spaces on the three tested curves. The cost of
generating enough useful relations rises in the genus-3 and genus-4 cases.
This is a finite correctness/count study; elapsed-time performance is unmeasured.

#image("RATIONAL-RANK.svg", width: 100%)

*Reading the result.* At genus 2, $p=1009$, d2 reaches rank 7 with eight
anchors after 705 visited functions. The original d0 family exhausts all
224 functions at that anchor size at rank zero. It reaches rank 11 with
twelve anchors after 714 visits. At genus 3, reducing anchors from 24 to 14
with d2 changes visited functions from 6049 to 10611. At genus 4, reducing
anchors from 28 to 16 changes visits from 2372 to 6924. These are deterministic
counts on one selected curve per genus; no population or timing ratio follows.

*What was retained.* All 36 arms remain: 23 reached rank, 12 exhausted the
tuple family, and one reached its 1024-tuple cap. The 32768-class cap was
never reached. Full Gaussian ranks of the admitted matrix and its off-anchor
restriction independently reproduce all 36 anchor ranks. Original per-arm
counts are unchanged. Source: #raw("RATIONAL-RANK-VERIFIED.jsonl").

*Reproduce:* #raw("sh rational_gate.sh /tmp/hec-rational-replay"). See
#raw("RATIONAL-RESULTS.md") for commands, source hashes, caps and full limits.
Protocol freezes: #raw("10ff4d7727"), #raw("c6903c64ea"), #raw("ced6386c0e").

#pagebreak()
= Construction and the remaining cost condition
For genus $g$, choose $d >= 1$, and $M=g+2d$ signed anchors. Write
$A(x)=product_i (x-x_i)$ and $w_i=y_i/(A'(x_i))$. The weighted subset formula
constructs $a,b$ with $deg a <= g+d$, $deg b <= d-1$, and
$a(x_i)=sigma_i y_i b(x_i)$. Thus $A$ divides $a^2-f b^2$, and the residual
has degree at most $g$. Every nonzero primitive pair whose residual splits
gives a principal relation after orienting roots by $y=a/b$.

*Algebra evidence.* Direct subset evaluation agrees with Walsh evaluation on
9408 signed curve cases, with an independent Gaussian interpolation reference.
All 1108 primitive split cases pass the complete Jacobian identity. The panel
retains 694 common-factor rejections and 81 residual degree drops. Separate
controls cover 2401 exhaustive arbitrary interpolation assignments, zero pairs,
higher nullity, branch anchors and duplicate inputs. All five new algebra
tests and all 17 tests in the existing CI target pass locally.

*Identity receipts.* Nine #raw("identity.certificate/v1") records cover
the nodewise polynomial specialization for $g=2,3,4$ and $d=1,2,3$, after
absorbing signs into arbitrary weights and clearing the denominators. Each is
producer-issued at 32 derived finite-field points and rechecked at 16 further
points. They are evaluated certificates, with no formal-proof pointer or
independent authorship. Example g2,d2: #raw("IDC1h0a44102490e13efe").
All records and statements: #raw("RATIONAL-IDENTITIES.json").

*Dimension constraint.* For pole bound $n>2g-2$, Riemann--Roch gives
$dim L(n infinity)=n-g+1$. Prescribing $n-2$ distinct anchor zeros therefore
requires at least $g-2$ dependencies among those evaluation equations for a
nonzero function when $g>=3$. Increasing pole degree alone does not remove
that gap. This is a dimension derivation, not an independence or probability
claim. For a squarefree full-degree residual, the present two-off-anchor
filter still needs one anchor residual at g3 and two at g4. Repeated roots,
coefficient cancellation and lower degree are retained exceptions.

*Next performance gate.* Charge cold subset construction, all constructed
Walsh outputs including unused tails, normalization, root orientation,
deduplication, rank/recombination, setup and target handling on identical
single-target inputs. The smaller-anchor observation cannot be multiplied by
speedups from earlier experiments. Existing held-out timing graphs remain
unchanged because this study adds no runtime observation to their workload.

*Primary sources.* D'Andrea, Krick, Szanto, Theorems 2.2 and 3.1:
#link("https://arxiv.org/html/1211.6895v3")[arXiv:1211.6895]. Official Magma
#link("https://magma.maths.usyd.edu.au/magma/handbook/text/1473")[Riemann--Roch documentation].
The algebra is established prior art; novelty of this application is unassessed.
KB revision and source receipt are retained in #raw("RATIONAL-PROTOCOL.md").
