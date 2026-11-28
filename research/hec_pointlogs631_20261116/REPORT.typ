#set page(paper:"a4",margin:24mm)
#set text(size:11pt)
= F631 sparse destination relations
Genus 3 · 15 November 2026

#table(columns:(1fr,1fr),[Quantity],[Observed],[Rational points / small base],[597 / 128],[Pairs / distinct lines],[8128 / 7124],[Split relations],[2858],[Original stored coefficients],[22464],[Eliminated point columns],[469],[Sparse core rows / columns],[2388 / 127],[Core entries],[35090],[Reconstructed point values],[597],[Original residual checks],[2858 pass])

== Construction
Independent point enumeration → canonical reference-line ordering → pair lines and rational intersections → sparse original relations → large-prime elimination → scalar Lanczos → reconstructed point values.

The first four points are the discovered split canonical divisor. Their actual line is computed and checked. The old x-zero assumption is removed. Factor-base size128 is frozen before execution.

Lanczos succeeds on attempt zero with exact core residuals. Every original relation also has zero residual after reconstruction. Split relations comprise34withzero,416withone and2408withtwo large primes.

== Custody and limits
Source7547375ea2f7 and checkpoint20261116T024522Z precede the single execution. Exit zero; stderr empty. Initial compilation failure is retained. No old point-log table is supplied.

A relation kernel alone does not prove Jacobian logs or one-dimensionality. Geometric reconstruction, conic descent and final source/destination scalar verification remain. No timing, tenfold gain or genus-two/four mechanism is claimed.
