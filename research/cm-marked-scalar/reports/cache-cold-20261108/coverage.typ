#set page(width:18cm,height:5.8cm,margin:4mm)
#set text(font:"Libertinus Serif",size:9pt)
*Software qualification coverage; these are not performance measurements*
#v(4pt)
#let rows=(("Fixed-mu point/schedule cases",6300),("Retained-point label checks",71570),("Complete cold paths, clock off",625))
#for (label,value) in rows [
 #grid(columns:(6.6cm,1fr,1.2cm),gutter:3pt,
  [#label],
  rect(width:value/75000*8cm,height:3mm,fill:rgb("406c9c"),stroke:none),
  [#value])
 #v(6pt)
]
#text(size:8pt)[Units: individual checks in different declared components; no timing, speed ratio, statistical confidence or cross-component efficiency comparison.]
