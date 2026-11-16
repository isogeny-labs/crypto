#set page(paper: "a4", margin: 20mm)
#set text(font: "New Computer Modern", size: 10pt)
#set heading(numbering: "1.")

= Pair-sum collision census

*Run:* run-20261116T0711Z  \
*Source commit:* `880d8717cce6b69ac6281192a6456a1c5c8758b3`  \
*Host:* macOS 27.2, ARM64, local shared process  \
*Protocol:* `research/rootfind_pair_collision_census_20261116/PROTOCOL.md`

== Exact observations

The scalar pair table's first-witness entry and insertion counts matched the
instrumented enumeration. Every repeated sum abscissa yielded a checked
four-point group identity. The deliberately perturbed control failed in all
three cells. Each cell completed one exact pass with exit status 0.

#table(
  columns: (auto, auto, auto, auto, auto),
  inset: 5pt,
  table.header([*Cell*], [*Base points*], [*Columns*], [*Repeated keys*], [*Rank*]),
  [K13/d5], [33], [17], [27], [5],
  [K19/d7], [139], [70], [99], [11],
  [K23/d8], [275], [138], [201], [23],
)

#figure(
  image("collision-rank.svg", width: 100%),
  caption: [Recorded repeated keys and independent homogeneous rows; one exact pass per fixed cell, run-20261116T0711Z.],
)

The exact curve IDs are
`ICV1:f2m-13-752c6463:181:8012:0x1:-7:unk:r:515ee5696294`,
`ICV1:f2m-19-87f45376:797:523492:0x1:-7:unk:r:b6cf2467887c`, and
`ICV1:f2m-23-c07a6b8c:5197:8383412:0x1:-7:unk:r:69e76b739aa7`.
The respective subgroup orders are 2,003, 130,873 and 2,095,853; each
cofactor is 4. Their standard-subspace dimensions are 5, 7 and 8.

== Evidence and next step

Raw witness rows, summary counters, identity controls and base digests are in
`k13.json`, `k19.json` and `k23.json` beside this PDF. The recorded command,
binary hash, environment, pre-run dirty state, stderr, exits and source commit
are in the same directory. Peak resident sets were 7,045,120, 8,765,440 and
13,074,432 bytes, within the protocol's 512 MiB ceiling.

These rows constrain base logarithms after cofactor projection. They contain
no target coefficient. Positive rank triggers implementation of a charged
harvest path and a matched target-relation count test. A separately compiled
dense Gauss-Jordan replay of all saved rows returned the same ranks (5, 11,
23); its three outputs and source are beside this PDF. This is a second
algorithm in the same session, not an independent reviewer. No speed
comparison was performed.

Sources: the frozen protocol; `papers/rootfinding-decomposition/`
`rootfinding_decomposition.tex`; `src/cryptanalysis/ic_boundary.rs`;
`src/cryptanalysis/koblitz_index_calculus.rs`.
