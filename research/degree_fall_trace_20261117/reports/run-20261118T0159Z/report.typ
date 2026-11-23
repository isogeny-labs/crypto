#set page(paper: "a4", flipped: true, margin: (x: 15mm, y: 11mm))
#set text(size: 9.5pt)

= Candidate-base effects across a toy relation flow
#text(fill: luma(90))[2026-11-18 · Run 20261118T0159Z · Singular 4.4.1 · F₁₀₁]

#image("base_flow.svg", width: 100%)

Each base has five field elements and 125 ordered triples. The flow tests $x+y+z=t$, then $y(x+z)=s$. The spread base reaches 60 distinct full target pairs across the grid. Both compact bases reach 35. At the shared target, the spread and consecutive bases yield five triples with observed degree 3. The centered base yields seven triples with observed degree 4. [pcc:unverified]

The pins use the common solution $(0,1,2)$. Only the mixed relation contributes direct symbolic coupling. The Gröbner staircase counts were checked against exhaustive enumeration at each populated stage. The original variable-coloring fixture was also replayed byte for byte. See #raw("RUN.md") and #raw("base_flow.csv") for the protocol and measurements.
