#set page(paper: "us-letter", margin: 0.61in)
#set text(font: "Helvetica Neue", size: 9pt)
#set par(justify: true)
#set heading(numbering: none)

= Inline-singleton binary pair index
_Bounded correctness and backing record · 2026-11-16_

This report records observations for a standalone first-witness index. It
does not determine complete collector time. A four-byte directory entry
holds the first key's slot and packed pair witness. A second distinct slot
promotes the page to a 16-word spill. Equal keys keep the first witness.
The construction is limited to binary degrees at most 23 and pair indices
below 512.

#image("diagram.svg", width: 92%)

== Exact parity

The source committed at `c3ad96f431` passed four dependency-free unit tests.
The unchanged K13/K19/K23 production pair streams passed nine harness tests:
every insertion matched a hash map and the previous paged index, all
duplicate group equations held, and the complete collision vectors matched
the production torsion builder. The earlier passing curve run produced the
same three structured parity records. These are same-producer checks.

#table(
  columns: (0.7fr, 0.9fr, 0.9fr, 0.8fr, 1.1fr),
  inset: 5pt,
  stroke: 0.5pt + rgb("c5d1d8"),
  [Cell], [Inserts], [First keys], [Collisions], [Singleton / spill],
  [K13], [272], [261], [11], [161 / 43],
  [K19], [4,830], [4,800], [30], [4,159 / 314],
  [K23], [18,906], [18,842], [64], [18,158 / 340],
)

#pagebreak()

== Requested backing

#image("requested_backing.svg", width: 100%)

The values are derived from four bytes per directory word plus 64 bytes per
reserved spill page. They exclude allocator overhead, process RSS, relation
collection, rank, and verification. Cargo/test resource logs and the test
runner's reported time do not measure a complete collector call.

== Source, failed admissions, and next gate

Protocol: `PROTOCOL.md`. Final machine record: `RESULTS.json`. Full raw
outputs and UTC/resource logs are retained. The first Cargo launch refused
the copied lockfile's old package name before compilation; its output was
preserved. The one-line correction preceded both passing curve runs, and an
additive 16-slot control preceded the final run.

The repository release library suite compiled but is incomplete and
failing: 2,796 tests reported passing and five ecbench band/compare tests
reported failures before a recorded interrupt. One isolated failure
reported a missing artifact-managed `records.jsonl` fixture. Its path is
reserved by another active task; the remaining failures were not
individually diagnosed. This suite result is separate from the passing
focused parity checks.

The next implementation step is guarded production integration and
same-target answer parity after the reserved source paths are released.
An isolated complete-call F4/F5/F6 panel remains separate. No canonical
performance graph changed because this record has no timing edge.
