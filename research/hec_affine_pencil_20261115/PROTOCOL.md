# Challenge-conditioned affine pencil: equivalence gate

2026-11-15. Previous turn was progress: real point-sieve gate1f24a8daac,
65236admissions/38393pure checks,17recovered and1rank cap. Read its SESSION
and newest dated log before choosing this step. No previous process is live.
T-6086checked/claimed this directory before edits. Shared resume untouched.

## Literature review and immediate collision
KB ready lexical revision9141e386254340a9bdba3036f2bb706a, refresh
2026-11-14T21:44:48Zpredates work. Queries: Sarkar/sieve/batch, HEC/sieve/cost,
HEC/two/Riemann/sieve, Nagao/interpolation, HEC/two-parameter. Nagao literature
notes are abstract-only; this is not a completed full-text Nagao novelty audit.
Primary point-sieve Section2.2/2.3read again:
https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf.
Primary interpolation addition/reduction paper read, introduction and Section3.2:
Bernatska--Kopeliovich, https://arxiv.org/html/1912.13277 (2020).
The 2011/306ePrint group-law PDF returned403; the accessible primary paper and
current jac.rs provide the relevant addition/reduction framework.

The proposed two-parameter construction is algebraically ordinary addition
of two small places followed by reduction. This collision is derived below;
the gate checks it before any speed experiment or novel-mechanism assertion.

## Derivation before code
Given degree-g reduced D=[u,v], choose distinct small places P_i=(x_i,y_i),
P_j=(x_j,y_j) with u(x_i)u(x_j)!=0. Let z_i=(v(x_i)-y_i)/u(x_i), similarlyz_j.
The affine w=lambda*x+mu interpolating z_i,z_j gives t=v-u*w with t(x_i)=y_i,
t(x_j)=y_j and t=vmodu. Put U=u*(x-x_i)*(x-x_j). Then U divides t²-f.
The residual R=(t²-f)/U, made monic, has degreeg if lambda!=0, and degreeg-1
if lambda=0. The latter case follows because f has degree2g+1but t has
degree<=g. R cannot vanish identically for squarefree odd-degreef.

Its oriented divisor E=[R,t modR] is reduced since degR<=g, and

    E = -(D + P_i + P_j)

as divisor classes. This follows both from the principal divisor of y-t and
from the usual Mumford reduction U -> (f-t²)/U, v -> -t: the sign on R's
leading scalar disappears when making it monic. Current Curve.add_place twice
computes the same class. R need not split; when it does, factoring E gives
exactly the conventional shifted-walk relation. The quadratic for g2 is the
usual reduced genus2degree, not a new smoothness probability theorem.

There may be arithmetic/precomputation savings in a batched CRT implementation,
and reusing D across pairs changes correlations. Neither implies a new source
of independent relations, new asymptotics or10xcomplete speed. Price them
against optimized add_place/walk if pursued as engineering.

## Frozen finite gate
Native Rust only, existing rlib. g2/3/4,p11/13/101,c1/2where squarefree,
16seededdivisors each. D=sum of grandom FBplaces; skip and count lowdegreeD.
Use first min(8,FBsize)canonical places, both signs excepty0, all unordered
distinct-abscissa pairs, skip/count u-roots. Check exact divisibility, Rdegree
and degreedecrease when lambda0, R,tvalidity, and exact reduced Div equality
E=neg(add_place(add_place(D,P_i),P_j)). Compare native oracle split decision/
roots against exhaustive root division. For split cases check full principal
identity with all multiplicities and orientations. Count all trials, degree
drops, repeated/ramified roots and split cases; no timing or confidence claim.
Fail any mismatch. Preserve terminal output/hash/source and producer replay.
No change to closed records/defaults/benchmark standings. Class relabelling/
engineering collision screen; not a mathematical breakthrough candidate.

## Session
### What was done
Reviewed prior streams and sources; derived the exact existing group-law
equivalence before implementing an instrument.
### Decisions made
Do not build a new scientific campaign around a rephrased two-place walk.
Check the equivalence and retain it as a pruning result, then return to the
joint point-sieve stopping-time cost case or a distinct mathematical lead.
### Open threads
Run native equivalence gate and publish the derivation with actual coverage.
### Errors
2011/306PDF403; accessible primary source used without claiming inaccessible
paper was read. Initial attractive genus2quadratic observation is explained
by ordinary genus2reduction; no novelty or speed claim was made.
