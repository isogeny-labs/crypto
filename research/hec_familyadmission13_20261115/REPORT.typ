#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Subgroup admission for the portability family
Genus 3 · 15 November 2026
#image("flow.svg",width:100%)

The Fermatquartic family used for destination-section controls has an elliptic-product Jacobian over the tested fields. It cannot supply representative ell~q³subgroups for a generic genus-three performance comparison. Its section and cancellation checks remain valid.

== Explicit quotient structure
On x⁴+y⁴=1, sign involutions give E0:s²=1−t⁴twice, via (y,x²)and(x,y²), and E1:s²=t⁴+1via(x/y,1/y²). Pullbacks of dt/s are −x dx/y³,y dx/y³,dx/y³. These are independent canonical differential directions, yielding an Fp-isogeny E0²×E1→J. The geometric implication is a readable argument, not a full formalization or a consequence of pointcounts alone.

== Once-only frozen audit
Source5e5e52cabbd6, checkpoint20261116T005753Z. Elliptic Legendre counts include correct infinity points; independent curve enumeration checks the trace relation. All applicable quotient equations/differential substitutions pass. No DLP or timing run occurred.
#table(columns:5,inset:5pt,[p],[Counted E0],[Counted E1],[Derived J order],[Largest prime],
 [97],[80],[80],[512000],[5],
 [257],[256],[256],[16777216],[2],
 [509],[520],[500],[135200000],[13],
 [1009],[1040],[1040],[1124864000],[13])

Derived Jacobian orders depend on the explicit isogeny argument. The rational point trace check alone does not independently certify the full L-polynomial. All statuses0/stderr empty; no scientific rerun or dropped field.

== Distinguish the actual source
The executed transfer source at127has order2057786=2·1028893. Any proper rational factor has dimension≤2and order bounded by (sqrt127+1)⁴, strictly below13⁴=28561, too small for the established prime divisor1028893. Thus the Fermatobstruction does not apply to that actual source. This is not a proof of absolute simplicity after field extension.

== Formal scope and decision
PrimeBound.lean formalizes only that a prime divisor of a²bwith positive a,b≤Bcannot exceed B. It does not prove geometry or pointcounts. The primary context is Fité–LorenzoGarcía–Sutherland2018, doi:10.1007/s40687-018-0162-0; our explicit quotients fix rational field-of-definition issues for the actual controlfamily.

Keep this family as section controls only. Future scaling tests need certified large-prime source subgroups, explicit transfer, matched native baselines and charged construction. No generic tenfold claim follows from these controls. Genus two/four and the mathematical objective remain open; the earlier failed coldcost comparison remains unchanged.
