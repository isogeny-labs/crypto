#set page(paper: "a4", margin: 17mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false, leading: 0.65em)
#image("routes.svg", width: 100%)
#pagebreak()
#for block in read("REPORT.md").split("\n\n") {
  if block.starts-with("# ") {
    heading(level: 1, block.slice(2))
  } else if block.starts-with("## ") {
    heading(level: 2, block.slice(3))
  } else if block.starts-with("    ") {
    raw(block.replace("    ", ""), block: true)
  } else {
    par(text(block))
  }
}
