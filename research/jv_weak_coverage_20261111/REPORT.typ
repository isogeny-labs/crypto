#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= How often a curve over F#sub[p⁶] lands in a Joux–Vitse weak class
_Protocol frozen before the runs, two disclosed amendments. Every number is an unverified assertion [pcc:unverified]._
#figure(image("coverage.svg", width: 100%), caption: [Admitted shares, census truth at small p, and the reach of the implemented walk.])
- *Admission:* 22–24.5 % of uniform curves pass the proved necessary condition at every p from 37 to 1823. Among curves with full 2-torsion it is 59–63 %. This is the upper bound on coverage.
- *Census:* at small p, 98.8–99.6 % of admitted classes hold a weak curve. At p = 1009, three random admitted classes were reached: the first large-field witnesses.
- *Implemented walk:* it reaches about 40 % of admitted classes at p = 37–53, 3 of 60 at p = 1009, and 0 of 60 at p = 1511 and 1823. Its odd jumps need rational ℓ-torsion points.
- *Operational coverage:* about 10 % of curves at toy sizes, about 1 % at p = 1009.
- *Cost:* successful random-start walks cost 0.008–0.14 × rho at p = 1009.
- *Consequence:* the walk charge holds only for starts the walk can reach. A full class-group walk using kernel polynomials of degree (ℓ−1)/2 is the next construction step.
