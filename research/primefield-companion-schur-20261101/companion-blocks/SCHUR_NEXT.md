# Derived next step: Schur generators with two correction columns

Derived 2026-11-01. An unexecuted prototype is now in ../companion-schur/schur.py;
only syntax has been checked, with no Schur numerical trials in this round.
This is a direct block derivation for the project's companion conventions.

Let A=[[E,B],[C,D]] with E n-by-n invertible and trailing dimension m.
Assume Z_(n+m,a) A - A Z_(n+m,b)=GK^t. Let S=D-C E^-1 B,
U=[-C E^-1,I_m], V=[-E^-1 B;I_m]. Then

    U A=[0,S],  A V=[0;S],
    U (L A-A R) V=(L22-C E^-1 L12)S-S(R22-R21 E^-1 B).

With u_j first and v_j last coordinate vectors, the companion crossing
blocks are L12=a u_n v_m^t and R21=u_m v_n^t; L22=R22=N_m,
the nilpotent subdiagonal shift. Put Gbar=UG and Kbar=V^t K. Therefore

    N_m S-S N_m
      = Gbar Kbar^t
        + a C E^-1 u_n (S^t v_m)^t
        - (S u_m)(B^t E^-t v_n)^t.

For arbitrary distinct new wraps c,d, add their wrap corrections and group:

    Z_(m,c) S-S Z_(m,d) = GS KS^t,

    GS = [ Gbar | a C E^-1 u_n + c u_m | -S u_m ],
    KS = [ Kbar | S^t v_m | B^t E^-t v_n + d v_m ].

The width is s+2, not s+4. This statement concerns the constructed width;
actual minimal displacement rank can be lower. No generator compression is
needed to establish the displayed identity. It holds whether S is singular
or nonsingular; only E's invertibility was used.

## Operator implementation path

A trailing Schur application requires two full-source applications and one
E solve: apply A to [0;x] to obtain Bx,Dx; solve E z=Bx; apply A to [z;0]
and return Dx-Cz. The transpose analog uses A^t twice and E^-t once.
No dense B,C,D or S need be materialized by these operator interfaces.

Columnwise construction of Gbar,Kbar uses s forward and s transpose E
solves and2s full-source operator applications. The two endpoint vectors
C E^-1u_n and B^t E^-t v_n add one solve/application of each direction.
Acquiring S u_m and S^t v_m by the uncached Schur paths adds four source
applications and two E solves. Thus this explicit uncached schedule uses
2s+6 full A/A^t applications and s+2 solves in each E direction, plus
scalar updates and O(ms) generator storage. These are nested call counts,
not constant-time oracles or a fast solve bound. Batch multiplication,
reuse and rank-profile handling must be evaluated explicitly.

A singular E does not imply singular A: [[0,1],[1,0]] is an invertible
counterexample with zero leading scalar block. The implementation must
return a block-pivot obstruction or use an authorized charged pivot policy;
it cannot label A singular on that condition. This example belongs in the
next fixed controls, alongside singular and zero Schur complements.

## Literature and next gate

Bostan et al. (2017), Section6.2, also derives low-width Schur generators
for its different displacement conventions and uses generic rank profile.
The formula here is our convention-specific specialization; the paper's
randomized success guarantee is not imported for the small base field.
https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf

Next freeze and validate the unexecuted Schur prototype using the
checked arbitrary-dimension interface and explicit reference E oracles.
Freeze exact dense Schur identities, both operator directions, oracle and
boundary accounting, corrupted correction columns, zero/singular Schur
cases, and the invertible-A/singular-E counterexample before any trials.
Only then replace the reference E oracles with a recursive backend.
