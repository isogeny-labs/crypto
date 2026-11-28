#set page(
  paper: "a4",
  margin: (left: 17mm, right: 17mm, top: 16mm, bottom: 22mm),
  header: [#text(size: 8pt, fill: rgb("5b6f7b"))[ISOGENY LABS / RR5 B677 CIRCUIT QUALIFICATION]],
  footer: context [#h(1fr)#text(size: 8pt, fill: rgb("5b6f7b"))[#counter(page).display("1")]],
)
#set text(font: "DejaVu Sans", size: 9pt, fill: rgb("172d3b"))
#set par(justify: false, leading: 0.55em, spacing: 0.7em)
#set heading(numbering: none)
#show heading.where(level: 1): set text(size: 19pt, weight: "bold", fill: rgb("165e7c"))
#show heading.where(level: 2): set text(size: 12pt, weight: "bold", fill: rgb("165e7c"))
#show heading.where(level: 3): set text(size: 10pt, weight: "bold")
#show raw: set text(font: "DejaVu Sans Mono", size: 7.5pt)
#show link: set text(fill: rgb("165e7c"))
#set table(inset: 4pt, stroke: 0.35pt + rgb("c7d3da"))
#set image(width: 100%)
