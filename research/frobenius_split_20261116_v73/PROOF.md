# Frobenius-split squarefree five-source reduction

Let E/K be an ordinary binary curve y^2+xy=x^3+A x^2+B, B nonzero, T=(0,sqrt(B)), and Q in E(K) outside {O,T}. Every finite point with x in K lifts to K2 and satisfies pi(P)=P or pi(P)=-P. Non-two-torsion lifts have a unique rational/anti-rational type. Their intersection is E(K)[2]={O,T}.

Any finite original S3 chain can be lifted coherently by choosing signs at each intermediate, including torsion, to points P1,...,P5 with sum Q, and conversely such a point chain with finite ground-field intermediate x coordinates satisfies every original equation. Orders, repeated sources, torsion and finite-auxiliary restrictions remain mandatory.

Split the source sum into rational A0 and anti-rational B0, assigning T sources to A0. Frobenius applied to A0+B0=Q gives A0-B0=Q, hence 2B0=O. Therefore B0 is O or T and A0 is Q or Q+T. This is an exact branch classification, not a rationality heuristic.

Assume (G1) x(Q),x(Q+T) are outside source domain D; (G2) neither Q nor Q+T has a two-source decomposition with source x in D; (G3) no distinct nonzero x,y in D satisfy xy=sqrt(B). If five source x values are distinct, every lift is rational. Indeed anti-source count one contradicts non-torsion; count two forces equal x (sum O) or xy=sqrt(B) (sum T); count three leaves a forbidden rational pair; count four leaves a forbidden rational singleton; count five forces Q two-torsion. Zero anti sources is the remaining case. Repeated-source branches are retained separately, never silently deleted.

For the actual B=1 polynomial-basis prefix of dimension18 in GF(2^83), products have degree<=34<83, so xy=1 implies x=y=1. Thus G3 holds for distinct sources without enumeration. G1/G2 are a falsifiable complete <=2^18 quadratic-root scan for the actual Q and Q+T. Native computation checks the field polynomial and point membership assumptions separately in retained provenance; the header itself does not prove irreducibility.

This reduction could route the squarefree hard branch into exact rational group arithmetic while moving nonrational branches to repeated/lower-arity problems. It does not solve the remaining squarefree five-sum search or establish novelty against the full literature. Frobenius eigenspaces and quadratic twists are classical; the guarded complete-search architecture is a research hypothesis for this program.
