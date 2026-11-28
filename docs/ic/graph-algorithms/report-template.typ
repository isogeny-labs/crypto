// Pandoc template; report.typ is the generated, directly compilable source.
$definitions.typst()$

#set document(title: "ECDLP graph algorithms and asymptotic costs")
#set page(paper: "a4", margin: (x: 18mm, y: 18mm), numbering: "1", number-align: center)
#set text(font: "DejaVu Sans", size: 9pt, fill: rgb("16283d"))
#set par(justify: false, leading: 0.68em)
#set heading(numbering: none)
#set math.equation(numbering: none)
#show math.equation: set text(font: "DejaVu Math TeX Gyre")
#show link: set text(fill: rgb("007c83"))
#show heading.where(level: 1): set text(size: 20pt, weight: "bold")
#show heading.where(level: 2): set text(size: 14pt, weight: "bold", fill: rgb("007c83"))
#show heading.where(level: 3): set text(size: 10.5pt, weight: "bold")
#show table: set text(size: 8pt)
#set table(stroke: (x: none, y: 0.3pt + rgb("d5e0e5")), fill: (x, y) => if y == 0 {rgb("16283d")} else if calc.odd(y) {rgb("f0f5f7")} else {white})
#show table.cell.where(y: 0): set text(fill: white, weight: "bold")
#show raw: set text(font: "DejaVu Sans Mono", size: 7.8pt)
#show figure.where(kind: image): set block(breakable: false)

$body$
