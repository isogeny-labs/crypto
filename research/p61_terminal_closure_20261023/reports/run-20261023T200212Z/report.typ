#set page(paper: "us-letter", margin: (x: 0.7in, y: 0.65in), numbering: "1 / 1")
#set text(font: "Helvetica Neue", size: 9pt, fill: rgb("172033"))
#set par(justify: true, leading: 0.55em)
#show heading.where(level: 1): it => block(above: 0.8em, below: 0.45em)[
  #set text(size: 15pt, weight: "bold", fill: rgb("0f4c5c"))
  #it.body
]
#show heading.where(level: 2): it => block(above: 0.7em, below: 0.3em)[
  #set text(size: 11pt, weight: "bold", fill: rgb("0f4c5c"))
  #it.body
]
#let chip(body, color: rgb("0f766e")) = box(
  inset: (x: 7pt, y: 4pt),
  radius: 4pt,
  fill: color.lighten(82%),
  stroke: color,
  text(size: 8pt, weight: "bold", fill: color, body),
)
#let hash(body) = text(font: "Menlo", size: 7pt, body)

#align(center)[
  #text(size: 21pt, weight: "bold", fill: rgb("0f4c5c"))[p61 registered-root terminal closure]
  #v(4pt)
  #text(size: 11pt, fill: rgb("475569"))[Reproducible run report · 23 October 2026]
  #v(8pt)
  #chip[PASS] #h(8pt) #chip(color: rgb("2563eb"))[358,094,304 vertices accounted]
]

#v(10pt)
#grid(
  columns: (1fr, 1fr), gutter: 10pt,
  block(fill: rgb("f1f5f9"), inset: 9pt, radius: 5pt)[
    *Curve* `p61-i0025` \
    *Field* $p = 2^61 - 1$ \
    *Model* $y^2 = x^3 + 1055x + 16361$
  ],
  block(fill: rgb("f1f5f9"), inset: 9pt, radius: 5pt)[
    *Source* #hash[3b66e5cae94ab709…] \
    *Run* #hash[run-20261023T200212Z] \
    *Control plane* v31, 89 pinned bindings
  ],
)

= Outcome

The final row range $[19,890,496, 19,894,128)$ generated and replayed
successfully. Across 18 lanes it contributes *65,376 vertices* and *18 closure
certificates*. The immutable prefix plus this segment exactly matches the
unconditionally certified class number of *358,094,304*.

Four layers support the result: successor generation, full native replay,
digest-bound admission, and a separate Python 3.13 audit that streamed every
record and checked dense coordinate/index occupancy, parent edges, and all
closure lanes.

#image("terminal-closure-flow.svg", width: 100%)

= Exact accounting

#table(
  columns: (1.25fr, 0.75fr, 2.2fr),
  inset: 5pt,
  stroke: rgb("cbd5e1"),
  fill: (x, y) => if y == 0 { rgb("e2e8f0") } else { white },
  [*Artifact*], [*Bytes*], [*SHA-256*],
  [Terminal segment], [8,306,946], [#hash[1b576b189b7354bb9c025f17a14409cb…]],
  [Checkpoint], [7,311], [#hash[ef9c6857a84a497bc99a4de534f2cf54…]],
  [Generation receipt], [9,783], [#hash[eec5774ebd9ea7b0da959324bf3a6a52…]],
  [Replay receipt], [1,220], [#hash[94ec6052a4e6461f518bcee6637561c89…]],
  [Admission manifest], [1,943], [#hash[f3c03ef4965a4980e403d2e187ed4b45…]],
  [Structural audit], [2,521], [#hash[a165b95e5c26c9cad590a427fbc8b556…]],
)

The segment expands to 36,207,183 bytes and contains 65,396 JSONL records:
one header, 65,376 nodes, 18 closures, and one summary. The recomputed record
chain is #hash[c8a0499eea65009e87beba9b24d348b4…].

= Root cause and correction

The degree-13 action returned to the seed j-invariant. The earlier terminal
check nevertheless rejected lane `(0,0)` because it compared a generic
canonical coefficient pair with the separately registered root pair. The two
models differ as coefficients but are isomorphic over $F_p$.

The successor selects and checks the target j-invariant, constructs the Elkies
isogeny, and links the Velu codomain directly to the model stored at `x=0`.
Checkpoint validation now permits a source transition only when the complete
configured predecessor checkpoint digest matches.

= Timing and controls

#table(
  columns: (1.5fr, 0.7fr, 2fr), inset: 5pt, stroke: rgb("cbd5e1"),
  fill: (x, y) => if y == 0 { rgb("e2e8f0") } else { white },
  [*Stage*], [*Elapsed*], [*Control*],
  [Generation], [35.134 s], [8 configured threads; audit seed 0],
  [Native replay], [31.059 s], [audit seed 7; two points],
  [Structural follower], [5.986 s], [all 65,396 records],
)

Dispatch occurred at one-minute load 10.6665 under the frozen limit 12.0, with
6.75 TB free under a 1.50 TB reserve. These are correctness-run timings, not a
parallel-scaling benchmark.

#pagebreak()

= Compatibility finding

The ordinary v1 follower correctly failed closed because it requires distinct
producer/verifier binary digests and zero closure records. A terminal-specific
v2 follower now binds its last passing ordinary receipt and checkpoint to the
terminal structural audit. Managed control-plane v31 reports both the terminal
generation gate and terminal audit follower as complete. The source-head
serialized release-library suite passed 4,081 tests with zero failures and 114
ignored in 554.47 seconds. The rebased current-`main` code head then passed six
focused tests and the complete serialized release-library suite: 4,479 passed,
zero failed, 119 ignored in 592.14 seconds.

= Evidence interpretation

#table(
  columns: (1fr, 1.25fr, 1.45fr), inset: 5pt, stroke: rgb("cbd5e1"),
  fill: (x, y) => if y == 0 { rgb("e2e8f0") } else { white },
  [*Layer*], [*What passed*], [*What it establishes*],
  [Generation], [65,376 nodes; 18 closures], [The corrected terminal path constructs every lane],
  [Native replay], [Full segment reconstruction], [Segment, checkpoint, kernels, links, and chain agree],
  [Admission], [All identities and digests bound], [The predecessor-to-successor transition is immutable],
  [Structural audit], [Every JSONL record streamed], [Dense occupancy, index map, parent edges, and closures agree],
)

= Scope boundary

This run resolves the registered-root census obstruction. It does not close
global uniqueness, complete adjacency, endpoint resolution, reciprocal
multiplicity, attack-label, held-out-split, null-control, or M7 confirmation
gates. At the report snapshot, adjacency accounted for 37,199,232 of
2,506,660,128 directed edges. Every remaining censored or incomplete state must
stay explicit in the training corpus.

= Next transition

#table(
  columns: (1.55fr, 0.65fr, 1.5fr), inset: 5pt, stroke: rgb("cbd5e1"),
  fill: (x, y) => if y == 0 { rgb("e2e8f0") } else { white },
  [*Gate*], [*State*], [*Required evidence*],
  [Census accounting], [#chip[COMPLETE]], [358,094,304 admitted and terminal-audited],
  [Global uniqueness], [Active], [External sort/index over every j-invariant],
  [Selected adjacency], [Active], [2,506,660,128 directed edges],
  [Endpoint and reciprocity], [Waiting], [Dense targets and exact multiplicities],
  [M7 confirmation], [Waiting], [Masks, held-out splits, null controls, frozen evaluation],
)
