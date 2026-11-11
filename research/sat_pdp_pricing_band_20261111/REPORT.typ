#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Native-XOR SAT oracle: pricing its conflicts removes the sub-rho reading

_Derived re-pricing of committed data (ecbench session `ECBS1hb55a1bdabf18`, n = 17 Koblitz, 40 runs per arm). Every number is an unverified assertion [pcc:unverified]._

#figure(image("band.svg", width: 100%), caption: [Mean cold S of IC native-XOR SAT over matched strong rho against the GAE charged per CDCL conflict. Rho's unpriced native operations are held at 0 GAE (solid) and 1 GAE (dashed). The orange span is the session's own wall-time price of a conflict.])

#table(columns: 5, align: right,
  [*w (GAE/op)*], [*SAT S*], [*rho S*], [*SAT / rho*], [*Frob. MITM / rho*],
  [0], [3.41], [4.00], [0.854], [4.27],
  [0.1], [30.3], [4.13], [7.34], [4.13],
  [1], [272], [5.34], [51.0], [3.20])

- The order of SAT and rho flips inside the AGENTS.md §5a band, so §5a does not rank the two arms. SAT is below rho only at w = 0.
- *Break-even:* SAT equals rho at 0.0022–0.0072 GAE per conflict. The two values correspond to rho's native operations at 0 and 1 GAE. There are 2,755,966 conflicts, about 68,900 per target.
- *Wall-implied price (descriptive, same host):*
  - 15.1 µs per conflict, against 158–241 ns per GAE, gives 63–96 GAE per conflict;
  - at that price SAT is 3,100–4,800 × rho.
- *Reading:* the sub-rho row was an accounting gap, and the lead closes at toy scale under its own registered criterion. Unit propagations and XOR elimination are not counted, so a per-conflict price is only a floor on solver work.
