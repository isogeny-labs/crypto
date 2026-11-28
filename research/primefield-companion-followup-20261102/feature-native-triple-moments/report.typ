#set page(paper: "a4", margin: 18mm)
#set text(size: 10.5pt)
= Bounded moments through a third sign
T-1795 • 2026-11-07 • PCC-20261107-95192f • unsigned producer evidence

The new native module generates a requested moment prefix for three nonzero quadratic-character signs from two central coefficients. For consecutive offsets and scalar primes r=1 modulo4, a bounded CM construction supplies both seeds without constructing a field-sized polynomial.

#image("flow.svg", width: 100%)

== Derived recurrence
Let h=(r−1)/2 and B_j be coefficient h−j of ((X+1)(X+tau))^h. The weighted cubic character sum is −B_j for 0≤j≤h−1. Differentiating the quadratic power gives:

#align(center)[(h+j+1) B_(j+1) = tau(h−j+1) B_(j−1) − j(1+tau) B_j.]

Pair recurrences and explicit corrections at 0, −1 and −tau complete each sign-class moment. The canonical zeroth moment is the exact candidate count. General supplied seeds are assumed correct; this API does not authenticate them.

== Effective special case
For tau=2, reflection gives B1=−B0 when r=1 modulo4. Cornacchia and a bounded quadratic-nonresidue search produce r=a²+b², with a odd, b even and a+b=1 modulo4. The classical CM trace is 2a, giving seeds (2a,−2a). Primality remains a caller precondition. Exhausted search returns Limit; r=3 modulo4 is rejected by this constructor.

== Native evidence
Validation-02 passes three methods, with no filtered methods and no warnings. All offsets tau and eight sign classes across eight small primes give 14,184 moment equalities against direct sums. Twenty CM primes add direct point-count/seed checks and 1,352 moment equalities. Altered certificates, malformed large certificate coordinates, changed supplied seeds, limits and domains have controls.

At scalar prime 998244353, eight 33-moment prefixes satisfy the partition identity. The certificate has a=−3943, b=31348. This large-prime check is internal consistency, not independent enumeration or curve recovery. Validation-01 and its passing source snapshot remain intact; validation-02 hardens certificate bounds.

== Cost and next task
The requested prefix costs O(d+log r) field operations and O(d) words; seed synthesis adds bounded nonresidue exponentiations and Euclidean work. Full root conversion still needs all m moments and O(m²) Newton work, with m generally of order r/8. This component does not integrate public-point recovery. The original 4,563-case cohort is unchanged.

Next obtain the weighted seed for r=3 modulo4 and extend moments beyond three signs. The frozen r=479 fixture remains in that unresolved automatic-seed branch. No timing comparison or cloud launch was performed.

== Sources and custody
Kowalski, Elliptic curves, rank in families and random matrices, Example1.10, supplies the CM normalization. DERIVATION.md gives its link, proof details, prior repository work and the KB access gap. Validation receipts pin native source and command logs; publication.json pins the complete component. No signed independent verdict is claimed.
