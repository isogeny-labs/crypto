# Prospective four-corner differential state construction
Scope: different transition family from original generic G-step; uses torsion step T with2T=0, keeps non2-torsion signed G as recovery anchor. Preserve prior two-call proposal and all failure/helper packets. This packet is a bounded software construction check, not collector performance.

Freeze group assumptions: valid abelian Jacobian, rational nonzero T with2T=0, valid signed G not in2-torsion, Q in r-torsion and rG=0. Let N=2r<=2^63 and canonical labels a,b<N, initial(0,1). R=aT+bQ. Store grid=[kappa(R),kappa(R+T),kappa(R+G),kappa(R+G+T)] and fixed quotient points kappa(0),kappa(T),kappa(G),kappa(G+T). Initialization performs signed additions and projections and records their counters; no target-dependent oracle is presumed free.

Formulas fixed before code:
A=xDBL(R), implemented by repaired xADD(kR,kR,k0).
B=xADD(kR,kR+T,kT)=kappa(2R+T), since difference is ±T.
C=xADD(kR,kR+G,kG)=kappa(2R+G), since difference is ±G.
D=xADD(kR,kR+G+T,kG+T)=kappa(2R+G+T).
Compute c=branch(kR) in{0,1}. For c0 return[A,B,C,D]. For c1 return[B,A,D,C], since2T=0. Thus output entry2 is kappa(R'+G), not a separately projected expected-divisor neighbour. Update a'=(2a+c)modN,b'=2bmodN. rQ=rG=0 and2T=0 justify label reduction;2a+c<=2N-1<=2^64-1. Recovery receives G, output entries0/2 and fixed kG only. Identity and2-torsion R remain explicit recovery domains; assert actual fixture results, retain failures.

Fixed workload: the84 previously derived valid T/Q/G fixtures, three consecutive grid transitions each, no search or retry to select successes. Function grid_step must accept only quotient arrays, field/constants/action tables, no expected signed points or labels. The test harness computes signed reference from labels only after update, compares all four projected corners, then checks exact signed recovery using propagated corner2. Negative dependency control: no expected point passed to grid/recovery production helper; retain source interface and hashes for review. All four repaired operations and initialization/recovery are counted separately in native counters. Verification counters recorded separately; no complete cost or timing claim.
One120s offline locked build/run with5s kill grace in new private directory. No cloud jobs, scientific solves, publication or original-source mutation. Preserve any failed checks. Candidate does not purport to implement the original R->2R+cG family when T is not2-torsion.
