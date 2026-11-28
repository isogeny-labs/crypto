#set page(paper:"a4",margin:17mm,numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
= Koblitz cover transfers — R36
*Exact class-operation observations; no IC performance claim*

Two accepted frozen native runs implement exact subspace division with all
original generators in the 770-dimensional closed-point algebra. They use
the actual 196-dimensional divisor space inside V of dimension324.

#image("round36.svg",width:100%)

quotient36-01 verifies the zero control, a 68-dimensional flip using a section
that vanishes on9Q, and exact recovery of the original196-space by flipping
back with the same section. The section is not inverted in the local algebra.
An actual nonmember of the multiplication target is found and rejected.

group36-01 tests self equality (witness dimension1), source against zero (0),
constructs its negative, checks the negative against zero (0), and verifies
that Addflip(source,negative) is equivalent to zero (1). Thus this one actual
source class has direct native class-arithmetic nonprincipality evidence.

All12subspace divisions retain residual maps, kernel coordinates, reconstructed
candidates and rank-nullity checks. All construction products and final
memberships against every original generator are checked with two different
arithmetic implementations. No uncertified deflation or random IGS is used.

The geometric interpretation uses the retained proper/local/pole and complete
Riemann–Roch bridge. Native arrays and these group algorithms are not formally
verified in Lean or independently replayed externally. R35 coefficient-lift
Lean lemmas are separate. Resource limits remain4GiBvirtual/1200s per native
invocation; no timing comparison, IC relation rank or scalar recovery.

Next: the actual signed combination in certified deck order and its intended
F2 modular-factor / nonzero order11 action. Full quotient saturation, proper
polarized embedding, theta and arbitrary/larger collector supports remain.

Reference: Khuri-Makdisi math/0409209v3, algorithms3.10,3.14,3.16 and remark3.17.
See REPORT.md, prospective protocols and retained exact receipt/certificates.
