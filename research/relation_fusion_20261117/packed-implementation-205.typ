#set page(paper: "a4", margin: 1.5cm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.45em, spacing: 0.8em)
#heading(level: 1, "Complete packed constructor: correctness205")
The complete packed polynomial/provenance constructor is implemented. Across the six fixed seed7 LAST-star inputs (p2147483647/p9007199254740881, n3/4/6), all36 sparse/DAG and kernel103/231/487 combinations emit byte-identical complete proofs and equal metadata to the legacy constructor. This is correctness evidence; no affected200 timings exist. [pcc:unverified]
#parbreak()
#grid(columns: (1fr, 1fr), gutter: 8pt,
  rect(fill: rgb("#e9f0f6"), inset: 9pt)[*Legacy control*\ Original Vec keys\ Shared production algorithms\ Existing complete v1/v2 proof],
  rect(fill: rgb("#e3f1ec"), inset: 9pt)[*Packed candidate*\ Private u128 keys\ Same production algorithms\ Equal complete v1/v2 proof],
)
#parbreak()
Packed keys contain16 seven-bit exponent lanes plus separate cached degree and dimension. Guarded products require matching dimensions and total degree≤64 before lane addition. Intermediate polynomials and provenance stay packed through projection, coordinate propagation, change of order, internal verification and proof completion. Conversion occurs at input decoding and complete wire export. Exact coefficient arithmetic remains unchanged. Unsupported shapes and typed range refusals return errors; unrelated panics propagate.

All original legacy test modules are byte-preserved. Shared production includes avoid a second copied mathematical algorithm. Separate checker arithmetic does not construct the answer. Original-ideal membership, input containment, reduced monicity and every strict-standard/checked-product S-pair obligation remain.
#table(columns: (1.2fr, 0.6fr, 2fr), inset: 5pt,
  [*Check*], [*Result*], [*Meaning*],
  [Producer release195], [68pass], [65original +3packed arithmetic/range tests],
  [Counter-enabled204], [69pass], [Includes original conditional counter test],
  [Separate checker197], [14pass], [Original full-proof tests and mutation canaries],
  [Fixed matrix196], [36cases], [72complete replays;126rejected mutations; exact old/new proof bytes and metadata],
  [Actual-root198/199], [36 +36], [New +old backends,2 +2test functions;7public mutation classes and refusals],
  [Saved-root audit203], [36proofs], [Binds native/control bases and replay stats;2hash-consistent sabotage views rejected],
)
All above tests have0failures/ignored. Actual-root198/199 are explicit opt0 release correctness builds; their durations are not performance data. Artifact audit203 does not independently re-execute native operation counts. Full ordinary root unit qualification remains incomplete after interrupted165 and its46observed failures; all failures remain in185/187. [pcc:unverified]

Evidence: PACKED_QUALIFICATION_197.json, ROOT_PACKED_OUTCOME_198.json, ROOT_PACKED_SOURCE_PINS_198.json, ROOT_PACKED_AUDIT_203.json and PACKED_CHECKS_204.json. Source, raw failed check193, successful logs, complete matrices and small frozen tools are retained. No fresh seed selection or weakened gate.

*Next experiment:* prospective1260-call protocol200,12strong controls (frozen and rebuilt),6candidate ablations, five repeats and A/A for every control. Primary packed-dag487 must exceed15 on allsix simultaneous lower bounds, with complete proofs and no monitor-flagged contention. Publish source/protocol/tool/input pins before data. No affected200 measurement has run; original15x goal remains active. [pcc:unverified]

Sources: hypothesis182 primary maintained kernels, separate checker and full profiles179. KB/conductor unavailable as recorded192; no novelty claim. No F4 solve, intrinsic regularity or observed solving degree is inferred. LocalCPU0/one-thread/zero-cloud and checkpoint/S3 waiver persist. All new evidence is unsigned producer data.
