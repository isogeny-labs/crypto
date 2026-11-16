#set page(width: 18cm, height: 9.5cm, margin: 4mm)
#set text(font: "Libertinus Serif", size: 9pt)
#let data=json("plot-data.json").curves
*Exact D+A minima for the original fixed scalar*
#linebreak()
Blue: direct. Grey: binary split control. Orange: relation residual split.
#v(4pt)
#for row in data [
 #let values=(row.direct_score,row.binary_score,row.relation_score)
 #let colors=(rgb("406c9c"),rgb("92989e"),rgb("d08239"))
 #grid(columns:(1.4cm,1fr,1cm),gutter:4pt,
 [#row.curve],
 stack(spacing:1pt,..values.enumerate().map(((i,value))=>rect(width:value/650*12cm,height:2mm,fill:colors.at(i),stroke:none))),
 stack(spacing:1pt,..values.map(value=>box(height:2mm)[#text(size:8pt)[#value]])))
 #v(7pt)
]
#text(size:7.5pt)[One complete frozen42,480-row census. All raw schedules and weights2..4 retained. D+A counts explicit group API calls; no clock or population inference.]
