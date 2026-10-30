#set page(width: 174mm, height: auto, margin: 5mm)
#set text(font: "New Computer Modern", size: 10pt)
#let box(title, content) = block(width: 100%, inset: 7pt, radius: 3pt,
  fill: rgb("edf5f4"), stroke: 0.6pt + rgb("2b8c7c"))[
  *#title* #linebreak() #content
]
#box[Raw construction receipt][Successful exit · 4 GiB · zero swap · one CPU · pinned image/command]
#align(center)[↓ native policy check]
#box[Before launch: freeze the host card][Require passed native/proof gates and matching construction executable digest]
#align(center)[↓ committed file hashes bind the consumed card]
#box[After collection: audit the archive][Repeat the raw resource-policy checks; reject altered caps and failed containers]
#v(2mm)
Source: `n83_d18_shared_card_audit.rs`, revision `5e2dc2583`.
This diagram describes validation. It supplies no solver timing or natural-query result.
