# Exact recovery of two-factor binary summation-polynomial blocks

Research draft; conditional algorithm and development evidence. Novelty and full end-to-end superiority are not established.

## Abstract

We implement exact recovery of all original triples in two adjacent binary Semaev constraints with fixed endpoints and prefix-domain sources. Rebracketing reduces the generic block to at most two symmetric source-pair fibers. Explicit two-torsion and projective-infinity branches preserve finite-field completeness, including nonrational point lifts and repeated sources. Exhaustive small-field controls and repeated GF(2^83) conditional benchmarks validate the implementation. A newly frozen 16-bit source block completes in median0.035394s versus21.654271s for an exact one-source scan, a scoped611.81xcomponent ratio. This is not a whole-chain solve or a genuineF4/F5/F6 comparison.

## Problem and algorithm

Let K=GF(2^n), B!=0 and E:Y^2+XY=X^3+A X^2+B. Define S3(r,s,t)=(rs)^2+(rt)^2+(st)^2+rst+B. Given finite endpoints a,b and D=span_F2(1,alpha,...,alpha^(d-1)), return all ordered (x,y,z) in D×D×K satisfying S3(a,x,z)=S3(z,y,b)=0.

1. Include the z=0 bridge x=sqrt(B)/a,y=sqrt(B)/b when endpoints are nonzero and both sources belong toD.
2. For every finite w inK satisfying S3(a,b,w)=0, solve S3(x,y,w)=0 exactly inD². Use s=x+y,t=xy: t²+wt+w²s²+B=0 is binary-linear in d+min(2d-1,n) coefficient bits. Enumerate the entire bounded affine coefficient fiber; enforce product support by solving X²+sX=t inD and output both source orders.
3. If a=b, include every x=y inD, subject to an explicit diagonal work/output cap.
4. For every candidate pair from steps2/3, solve the ORIGINAL first constraint for all finite z inK and verify the original second constraint. Deduplicate triples.

Any capped fiber demotes the entire block to unresolved with exact countnull. Membership and serialized lower-bound triples may still be retained. No resultant-only witness is reported as an original intermediate coordinate.

## Completeness theorem

For admitted fields/domains and completed fibers, the algorithm returns exactly the original triple set. Soundness follows from original-equation verification and the explicit zero identity. For completeness, z=0 is step1. Otherwise the shared intermediate point is not two-torsion. Frobenius sends any lift of a finite K-abscissa to itself or its negative; a zero-sum triple with a non-two-torsion shared point forces all its non-two-torsion lifts to have the same Frobenius sign. Applying this to both original triples makes the recombined endpoint sum have finite K-abscissa w, unless it is infinity. The finite case is covered by step2 and exact original restoration. Infinity forces a=b and x=y, covered by step3. The proof includes a negative Frobenius sign, so twist/nonrational-lift polynomial roots remain. The unique nonzero binary two-torsion point has abscissa0 and is handled separately. See the implementation proof for all exceptional cases and caps.

## Cost and limitations

Write m=d+min(2d-1,n). A finite-w coefficient matrix has nrows,mcolumns. If its affine fiber has nullityh, there are2^hcoefficient states; each supplies at most two source roots before original restoration. Work includes exact field construction, matrix formation/elimination, all coefficient states, constrained quadratic recovery, original reconstruction and output. For n83,d18,m=53. High-nullity fibers are explicitly bounded; no generic rank assumption defines correctness. Equal endpoints retain a possible2^d diagonal scan and may remain unresolved.

The competing exact source scan needs2^d source iterations, full-field z recovery and constrained y recovery. Rebracketing avoids that scan on completed generic distinct-endpoint blocks. Discovering endpoints in the ORIGINAL five-source chain remains a global search. Naive integration still has O(|D|^3) outer work, approximately2^54states atd18. This draft supplies neither a global polynomial bound nor the requested50xend-to-end release.

## Evidence and prior work

The frozen implementation matches672independent exhaustive endpoint cases overGF4/8/16,B1/B2,834roots. Twelve roots require the zero bridge and166require infinity. Eighteen cold processes atd8/12/16 complete with matching original triples; source/rebracket medians are0.110885/0.034900s,1.418071/0.036211s,and21.654271/0.035394s. A first d16rebracketing run0.069850s is retained; isolation/A/A is unqualified. Cold peakprocessRSS is12,800–12,928KiB. Two actual d18regular blocks complete with two triples each; a large equal-endpoint control remains unresolved.

Symmetric variables, group-law rebracketing and resultant decompositions are prior art. In particular, [Galbraith and Gebregiyorgis, Summation polynomial algorithms for elliptic curves in characteristic two](https://www.math.auckland.ac.nz/~sgal018/shishay.pdf) studies variable changes and recursive decompositions. This draft claims a scoped implementation and reproducible evidence, not novelty clearance. Optimized full matching F4/F5/F6,20preregistered hard whole inputs,all requested fields,qualified isolation,independent global completeness and the full performance gate remain required.

## Reproducibility and next step

Frozen source/input hashes, proof, rawstreams, observers, original roots and budgets accompany REPORT.md and the81-member raw archive. Run `PYTHONDONTWRITEBYTECODE=1 python research/autolab_cycle_20261116_v96/REPLAY.py`. A fresh whole-cell direction is derived in NEXT_AFFINE_DUAL_CELLS.md: a rejected separator dual defines a verified affine cell of at least2^30separators at n83,d18. Its rank/overlap/cost and coverage must be measured before claiming any global pruning benefit.
