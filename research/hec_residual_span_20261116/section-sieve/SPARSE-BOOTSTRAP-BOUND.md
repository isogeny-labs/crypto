# Exact first-admission bound for fixed sparse point pools

Finite counting argument, not a general IC lower bound or novelty claim.
It applies directly to the recent sparse one-linear-residual producer.

## Declared family

Let G=J(F_q) have size N; Dfixed and Tuniform in G\{0}. For each of Kfixed
restart choices let initial signed rational point pool S_j, size M_j, be
independent of T. Anchor offsets aD belong to a fixed set of at most Avalues
per attempt. An algorithm cannot derive its first new label or target
constraint until a principal relation appears with anchor T+aD, gknown
points from S_j and one residual rational point Q. This is the degreeg+1
section family leaving a linear factor after gknown roots. Require distinct
known abscissas, as current bucket code does; ignoring this restriction only
enlarges the bound. Initial labels may contain unknown symbolic variables.

## Bound without independence assumptions

For any prescribed unordered gtuple P1,...,Pg and rational residual Q,
the group equation determines at most one target:

    T=-aD-sum_i[Pi-infinity]-[Q-infinity].

There are at most #C(F_q) choices for Q, even granting infinity. Hence
the number of nonzero targets permitting a first admission is at most

    A * sum_j binom(M_j,g) * #C(F_q).

Union bound over tuples/offsets/attempts gives

    Pr(first admission) <= min(1,
       A*sum_j binom(M_j,g)*#C(F_q)/(N-1)).

Multiplicity of the residual root, overlapping target sets, singular skips,
the requirement that the relation actually lies in this section family, and
fixed global signs can only reduce the count. No random-root, Poisson or
independent-incidence model is used. Target-dependent restart choice after
stall does not escape the bound when every restart pool belongs to the same
fixed finite list; take union over the whole list prospectively.

If there is no first admission, no pool growth is possible. The bound thus
also bounds success for a solver requiring such an admission. It does not
cover independent target-identifying setup, multiunknown row production,
target-dependent initial pool selection, more anchor offsets, or other
section families. Charge any mechanism escaping these hypotheses.

## Current genus-two specialization

Each attempt starts with two nonramified five-orbits and both signs:20points
at10distinct abscissas. Valid pairs number at most binom(20,2)-10=180,
excluding opposite signs at a shared x. A=16,K<=3, so

    Pr(success) <= min(1,8640*#C(F_q)/(N-1)).

The initial seed-discovery rule and source core are target independent.
Neither branch learning nor consensus acceptance can create a row before
the first usable bucket. This bound is valid for strict and consensus modes.
It is often vacuous at current small fields; successful small panels do not
establish scalable first-admission probability.

## Scaling implication and research decision

For fixed genus, Weil bounds give #C(F_q)=Theta(q),N=Theta(q^g). With fixed
A,K and uniformly bounded pool M, upper probability is O(M^g/q^(g-1)),
which tends to zero. For nonvanishing success within this family one needs
A*sum_j binom(M_j,g)=Omega(q^(g-1)). With fixed A,K this requires
M=Omega(q^(1-1/g)):sqrt(q),q^(2/3),q^(3/4)forgenera2,3,4.

This is a necessary condition, not sufficient coverage, a cost exponent or
a universal impossibility. Increasing formal seeds increases label/system
dimension; increasing anchors increases production work. A next proposal
must price that scaling explicitly instead of extending the constant-seed
campaign. Multivariable symbolic inactivation or component-aware new supports
remain possibilities, but their rank/cost advantage is not established here.

Background: standard principal-divisor/Jacobian relation in Stoll's
Arithmetic of Hyperelliptic Curves,section4; Weil counts in Milne's
Abelian Varieties,finite-field chapters:
https://www.mathe2.uni-bayreuth.de/stoll/teaching/ArithHypKurven-SS2019/Skript-ArithHypCurves-pub-screen.pdf
https://www.jmilne.org/math/CourseNotes/AV.pdf
The displayed finite counting inequality is the elementary deduction here,
not an attack bound or measured result attributed to those references.
