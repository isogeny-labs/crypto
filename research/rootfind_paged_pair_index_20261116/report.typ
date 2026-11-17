#set page("a4", margin: 1.55cm)
#set text(font: "Libertinus Serif", size: 9.5pt)
#set par(leading: 0.55em)
#set heading(numbering: none)

#align(center)[
  #text(size: 17pt, weight: "bold")[Sparse paged binary pair index]
  #linebreak()
  #text(size: 10pt)[Source-bound correctness and storage screen · 2026-11-16]
]

#block(fill: rgb("eaf2fa"), inset: 9pt, radius: 4pt)[
  *Scope.* Four standalone tests and exact requested-backing arithmetic.
  No collector integration, complete-call timing, process RSS or performance
  conclusion. Protocol commit `d80f3e97f1`; prototype commit `3007d15058`.
]

== Proposed data path

The current binary torsion-harvest collector keeps the first witness under
each key `x+1`. The opt-in flat array first builds a hash table and then
initializes `2^n+1` eight-byte slots. The proposed bounded layout inserts
directly into 16-word pages selected by a four-byte directory. An occupied
slot returns the same first witness to the collision caller. Production
group checks and rank updates are outside this prototype.

#figure(image("diagram.svg", width: 100%), caption: [Proposed direct insertion;
the diagram is a design, not a measured execution.])

== Fixed checks

The dependency-free Rust test executable compiled with rustc 1.93.1 on
macOS arm64. Four tests passed: a deterministic 16,384-pair sequence matched
a hash map's first witnesses and collision order; all K13 keys were probed;
boundary values, missing keys and refusal paths were checked. These are
data-structure checks, not curve-point or target-answer checks. A separate
shell and jq arithmetic cross-check matched the Python derivation on all
three pinned collision-census JSON inputs. Both checks came from this
producer.

#table(
  columns: (auto, auto, auto, auto, auto),
  inset: 4pt,
  stroke: 0.45pt + rgb("94a3b8"),
  table.header[*Cell*][*Keys*][*Flat bytes*][*Paged bytes*][*Flat / paged*],
  [K13], [261], [65,544], [39,172], [1.673],
  [K19], [4,800], [4,194,312], [758,404], [5.530],
  [K23], [18,842], [67,108,872], [4,534,916], [14.798],
)

#text(size: 8pt)[Cells are Koblitz E0 over GF(2^13), GF(2^19), GF(2^23),
with subgroup orders 2,003; 130,873; 2,095,853 and cofactor four.
Full curve IDs, base hashes and census-input hashes are in storage.json.]

#pagebreak()

== Exact backing arithmetic

For degree `n` and `a` factor-base abscissae, the flat layout requests
`8(2^n+1)` bytes. The prototype requests
`4(floor(2^n/16)+1) + 128 min(floor(2^n/16)+1, a^2+1)` bytes.
The first term is the directory; the second preallocates bounded pages.
At K23 the flat layout initializes 8,388,609 words for 18,842 first-witness
keys, or 445.21 initialized words per key. The requests do not depend on
how those keys distribute among pages.

#figure(image("requested_backing.svg", width: 100%), caption: [Derived fixed
backing requests from three source-bound census cells. Units are MiB; the
vertical axis is logarithmic. No process-memory or time values are plotted.])

== Interpretation and next check

The paged prototype requests fewer fixed backing bytes on these cells and
avoids an intermediate hash table by construction. It also adds a directory
read and page-allocation bookkeeping. Vec headers, allocator overhead,
collision rows, curve arithmetic, rank, verification, RSS and wall time are
unmeasured. The directory itself grows with `2^n/16`, so this bounded design
does not transfer to the N83 root-index workload.

After the reserved collector paths are released, integrate the layout behind
a distinct method parameter and check actual K13/K19/K23 first witnesses,
collision streams, signed group re-add and target answers. Only a separately
frozen same-target complete-call panel can decide the speed question. The
existing F4/F5/F6 panel remains a separate admission and replay gate.

#text(size: 8pt, fill: rgb("475569"))[
Provenance: REPORT.md, PROTOCOL.md, storage.json, test.stdout and
verify-storage.stdout in this study directory. Current binary source:
src/cryptanalysis/ic_boundary.rs at 7a3c3f2153. Earlier primary record:
papers/ecdlp-index-calculus-program/sections/rootfinding.tex;
KB source:e9db8cc2be204429e8fb1daa12a3324c. No canonical performance
graph changes follow from this screen.
]
