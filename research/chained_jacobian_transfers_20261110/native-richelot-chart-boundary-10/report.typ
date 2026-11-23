#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 10.5pt)
#set par(justify: true)
#set heading(numbering: none)
#align(center)[
  #text(size: 19pt, weight: "bold")[Richelot chart boundary]
  #linebreak()
  Prospective Rust correctness unit 10 · 16 November 2026
]

#block(fill: rgb("fff2dc"), inset: 8pt, radius: 3pt)[
  *No execution result.* Preregistration 809f0c freezes inputs and acceptance conditions before implementation. Claims are unsigned. [pcc:unverified]
]

== The concrete boundary
CT49 and CT50 give explicit F101 Richelot codomains. The current native constructor requires a monic source product and a square target leading coefficient, and exposes an unfinished infinity-fibre case. A change of curve model is one component needed to investigate those boundaries. It does not supply a full divisor correspondence. [pcc:unverified]

#align(center)[#image("chart.svg", width: 100%)]

== Proposed coordinate contract
Choose the first base-field a and the smallest positive b satisfying b² = H(a) ≠ 0. Put u = 1/(x−a), v = y·u³/b and F(u) = u⁶·H(a+1/u)/b². Its intended leading coefficient is one. The points (a,±b) become the two signed new infinities; old rational infinities become points at u=0. Nonrational infinity points must not be fabricated. This is a proposed source-level contract, with the shared-library Lean assembly still open. [pcc:unverified]

== Frozen development checks
The fixture file contains all four CT49 source sextics and all eight forward targets. The planned Rust unit compares independently evaluated homogeneous sextics on all 102 projective F101 arguments; checks rational point and infinity equations, both inverse roundtrips and image-set equality; and compares an independent quadratic-character count. The eight target counts of 120 are prior-report checks, not measurements made here. Malformed, singular, degree-drop, normalization, coefficient and lost-infinity controls remain mandatory. [pcc:unverified]

== What the unit cannot establish
Changing the sign of b gives another legitimate curve isomorphism. Equations and bijectivity therefore cannot establish the Richelot dual sign. Full balanced divisor maps, nonrational support, composition and dual [2], actual subgroup preservation, compiler refinement and solving costs remain separate obligations. No full 12,317-class Jacobian enumeration, original LP population, benchmark or paid run is authorized by this protocol. [pcc:unverified]

#text(size: 8.5pt)[
Primary records: Consumer 7f8208a5, complete CT49/CT50 reports and nonsplit_map.rs; literal coefficients and full protocol accompany this report. KB and Conductor are unavailable in the retained environment; no novelty or prior-art absence is inferred. Standard-library Rust, one compiler worker and a 120-second development-unit cap are planned. [pcc:unverified]
]
