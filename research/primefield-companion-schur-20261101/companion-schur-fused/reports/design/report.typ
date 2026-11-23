#set page(paper:"a4",margin:18mm)
#set text(size:10.5pt)
= Fused Schur adapter and certificate transport
2026-11-01 · Derived integration; syntax-checked source; zero integration trials

The adapter inherits the existing Schur construction and replaces only its
full-source multiplication with the fused operator. Leading-block solves,
wrap guards, correction signs and obstruction propagation remain inherited.
The historical capped Schur source is unchanged. The separate full fused
kernel validation is still running; integration execution has not begun.

#image("pipeline.svg",width:100%)

*Exact identities.* Let A have blocks E,B,C,D, with E invertible and
S=D−C E⁻¹ B. For b=[b1;b2], use h=b2−C E⁻¹ b1. The adapters give
x=[E⁻¹(b1−Bz);z] and w=[−E⁻ᵗ Cᵗv;v]. Independent reference checks verify
Ax−b=[0;Sz−h], wᵗA=[0;vᵗS] and wᵗb=vᵗh, including arbitrary residuals.
Thus vᵗS=0 and vᵗh=1 transport a normalized exclusion witness.
Feature-annihilation uses the actual feature matrix in a separate check.

*Derived generator cost.* For source dimension N=n+m and width s, the
inherited generator schedule uses q=2s+6 full-source calls: s+3 in each
direction. There are s+2 leading solves in each direction. With the fused
kernel, source plus correction field products/additions/inversions are
m, q+m(2s+5), q. Source coefficient reductions are qN; native products
q(4s+8); packing bytes q(3s+1)Nd. These are derived counts, not measurements.
Reference leading-oracle work remains separate: 2(s+2)n² dense terms and
2(s+2)n coefficient reductions. Data-dependent native bit volumes must be
summed from actual events. This adapter supplies no new leading solver.

*Full validation design.* Preserve all321 original contexts in both arms:
642 rows, including the96 full-invertible/leading-singular controls. Add
zero/ones/seeded primal and dual residual checks for every valid leading
block. For every singular S, construct a public normalized test from a
nonzero left-null vector: choose its first nonzero coordinate j and set
h_j=v_j⁻¹. Keep modified leading coordinates, RHS, pairing and count controls.
Leading obstructions must propagate through all three lift adapters with
their exact failure schedules; they do not determine full-matrix rank.

*Files and next step.* adapter.py, certificates.py and INTEGRATION.md.
Both Python sources passed syntax checks only. After the active kernel panel
reaches terminal verification, finish and freeze the full integration checker,
analyzer, source/input locks and expanded seed schedule before trials.
The42 completed historical Schur rows must never replace the full642-row plan.
