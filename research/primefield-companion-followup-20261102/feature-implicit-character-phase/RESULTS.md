# Implicit phase and summary-only degree bound

The retained first-orbit pair (f0,fplus*fminus) is invariant under rotation
and reflection of its binary sign word. Its number of values is at most the
binary bracelet count B_m. With L=(r-1)/(2m) and epsilon fixed, at least one
summary requires V-annihilator degree ceil((L-[epsilon=1])/B_m). The derivation
also gives the W-domain and fixed-multiple-radius bounds. This concerns a
polynomial based solely on that retained summary and covering all compatible
candidates; it is not a bound on using the public point or other information.
PCC-20261108-448fc5 is unsigned producer evidence. [pcc:unverified]

Validation03 passes8184 binary words,180208 dihedral-invariance equalities,
10 bracelet-formula comparisons and36 actual observation-fiber controls.
B3=4 and B5=8. For r281,m5,epsilon1, the tested first orbit has27 V values,
14 W values and5 summaries, with maximum support7 in V and4 in W.
For epsilon=-1 it has28 V values,14 W values and8 summaries, with maximum
support5 in both coordinates. These are exact small-field fixture counts.
[pcc:unverified]

The supplied-constraint kernel computes gcd(sf(q),D_L-2epsilon) by quotient
Dickson doubling. It passes3120 dense-remainder comparisons,2112 squarefree
intersection comparisons and6 large-exponent controls. In the latter, r is
998244353, q has degree24 and comes from public constant reciprocal traces;
L reaches998244352. The largest tracked recurrence vector has47 coefficients,
and at most66 quotient products were used in these cases. These counts exclude
engine scratch space and are not peak-memory or timing measurements. No
public-target q construction or ECDLP recovery is claimed. [pcc:unverified]

Validation01 failed to compile because the standalone engine lacked the
separate GCD extension. Validation02 passed its kernel controls, then panicked
because the observation fixture's prime list had no field admitting m7.
Added the existing hash-bound gcd extension and prime29 respectively; snapshots,
partial outputs and errors are preserved. Fresh validation03 build/check pass
within90/60-second bounds. Two copied source files and six compiler dependencies
are hash-bound. This standalone component changes no recovery adapter, so
inherited end-to-end recovery was not rerun. [pcc:unverified]

Next investigate constraints using multiple radii, oriented modes or direct
public-point relations, with construction cost and candidate completeness
accounted for. The summary-only lower bound rules out a uniformly small
first-summary annihilator at fixed m; the quotient kernel does not supply
missing information. Independent signed review remains pending.
