#set page(paper: "a4", margin: 16mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 8.8pt)
#set par(justify: false, leading: 0.55em)
#image("routes.svg", width: 100%)
#pagebreak()
#for chunk in read("REPORT.md").split("\n\n") {
  if chunk.starts-with("# ") {
    heading(level: 1, chunk.slice(2))
  } else if chunk.starts-with("## ") {
    heading(level: 2, chunk.slice(3))
  } else if chunk.starts-with("    ") {
    raw(chunk.replace("    ", ""), block: true)
  } else if chunk.starts-with("|") {
    let rows = chunk.split("\n").filter(row => row != "" and not row.starts-with("|---"))
    let cells = rows.map(row => row.trim("|").split("|").map(cell => text(cell.trim()))).flatten()
    block(breakable: false, table(columns: (1fr, 1fr, 1.4fr), inset: 4pt, ..cells))
  } else {
    par(text(chunk))
  }
}
