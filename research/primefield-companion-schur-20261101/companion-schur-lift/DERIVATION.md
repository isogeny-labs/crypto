# Schur RHS reduction and primal/dual transport

Derived adapters, syntax checked only; no numerical trial in this round.
Let A=[[E,B],[C,D]], E invertible, S=D-CE^-1B, b=[b1;b2].

The reduced RHS is h=b2-CE^-1b1. If Sz=h, then
x=[E^-1(b1-Bz);z] solves Ax=b. For arbitrary z, the exact residual is
Ax-b=[0;Sz-h], so the adapter is checkable without assuming the solve succeeded.

For arbitrary trailing vector v, put w=[-E^-t C^t v;v]. Then

    w^t A=[0,v^t S],       w^t b=v^t h.

Thus v^tS=0,v^th=1 gives the normalized full-system exclusion witness
w^tA=0,w^tb=1. This is the concrete certificate path needed by the existing
public feature compiler. Any separate feature-annihilation requirement
remains a separate check; Gram-null transport alone does not establish it.

Each adapter uses one full-source matrix application and one leading-block
solve, both tracked by the Schur object's existing counters. Additional
scalar work is m subtractions for RHS reduction and n for either lift,
where n=dim(E),m=dim(S). Oracle work is not treated as free. The dual
adapter uses the transposed source and transposed leading solve.

`lift.py` implements these three adapters using the Schur prototype. A
LeadingBlockObstruction propagates unchanged. No hidden scalar labels,
point tables or source-field extension are inputs. Next freeze checks of
both exact residual identities, normalized left-null witnesses, altered
RHS/sign controls, and complete nested costs, after the running Schur
construction panel is reconciled. No adapter experiment has run yet.
