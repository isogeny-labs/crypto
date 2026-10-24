#set page(width: 178mm, height: auto, margin: 4mm)
#set text(font: "New Computer Modern", size: 11pt)
#let box(color, body) = block(width: 100%, inset: 3mm, fill: color.lighten(90%), stroke: color, radius: 2pt, body)
#box(rgb("b14e45"))[
  *Linux gate failed: 4,473 passed / 15 failed / 119 ignored* \
  Nine missing-fixture failures; six solver failures. No solver card launched.
]
#align(center)[↓ restore exact fixtures; rerun the entire suite with one test thread]
#box(rgb("ad8526"))[
  *Queued: complete native gates and bounded construction* \
  Same source and test limits; 4 GiB / zero swap / CPU 31 construction.
]
#align(center)[↓ only after native, proof and construction gates pass]
#box(rgb("356282"))[
  *Preexecution admission: independently check, commit, push and match* \
  Actual source, executable and input hashes; shared lock released while publishing.
]
#align(center)[↓ reacquire benchmark lock; fail closed on any invalid control]
#box(rgb("328b7e"))[
  *Pending frozen card: 22 controls → 22 replays → 18 natural queries* \
  Retain all failures and caps; collect receipts; independent audit still required.
]
#v(2mm)
Source: validation-v5, frozen PROTOCOL.md and admission scripts. Status at
2026-10-24 07:24 UTC. This diagram contains no timing comparison.
