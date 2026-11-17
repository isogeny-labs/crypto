#set page(width: 18cm, height: 5.4cm, margin: 4mm)
#set text(font: "Libertinus Serif", size: 9pt)
*Finite search for the same scalar label and selected eigenline*
#v(4pt)
#grid(columns: (1fr, 7mm, 1fr, 7mm, 1fr), align: horizon,
 box(inset: 6pt, stroke: 0.6pt, radius: 3pt)[
  Bound the sum\
  $s = mu + j n$\
  $d s^2 <= 4 n N_0$
 ], [#align(center)[$arrow.r$]],
 box(inset: 6pt, stroke: 0.6pt, radius: 3pt)[
  Preserve orientation\
  $b = b_0 + ell z$\
  Select nearest two $z$
 ], [#align(center)[$arrow.r$]],
 box(inset: 6pt, stroke: 0.6pt, radius: 3pt)[
  Verify every candidate\
  $a = s-b$\
  Norm, label, kernel
 ])
#v(5pt)
Derived algorithm, not an executed minimum search. Enumeration includes every sum below the initial norm bound. Evaluation cost is a separate obligation.
