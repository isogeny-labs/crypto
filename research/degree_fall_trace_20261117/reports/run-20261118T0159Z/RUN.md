# Candidate-base effects across a toy relation flow

Run 20261118T0159Z uses Singular 4.4.1 on Linux x86-64. The protocol starts
from the same single-variable pin and finite-difference definitions as the
[original coloring run](../run-20261118T0135Z/). The sequential toy flow has
membership in a five-element set, a sum relation (x+y+z=t), and a mixed
relation (y(x+z)=s), all over F₁₀₁.

The matched candidate bases are centered ({-2,-1,0,1,2}), consecutive
({0,1,2,3,4}), and spread ({0,1,2,17,43}). Count ordered triples
across the entire (101 	imes 101) target grid and record every nonempty
bin. Empty bins are exactly the complement. The shared target ((3,2))
comes from ((0,1,2)), which belongs to every base. The experiment stops
after three complete 125-tuple sweeps and a Gröbner staircase verification
of each nonempty bin. The basis count must equal exhaustive enumeration at
every stage. This is a deterministic fixture with exact counts and no
runtime comparisons.
