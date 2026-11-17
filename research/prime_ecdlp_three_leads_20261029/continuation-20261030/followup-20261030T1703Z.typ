#set page(paper: "a4", margin: (x: 15mm, y: 14mm))
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.6em)
#set heading(numbering: none)
= Symmetric PKM: scoped validation and accounting follow-up
2026-10-30 · Collection at 17:03UTC · Later read-only source review

*Full question remains open:* can symmetric PKM yield useful relations cheaply enough to improve a complete cold single-target discrete logarithm? G1–G3/G6 are partial, G4/G5 unrun. This follow-up collects software checks and identifies concrete accounting fixes. No scientific panel or priced result is supplied.

== Fresh completed checks
Source: faade165a46fd51e5a0903ba8e4df85a134c68db; all 17 bound source/fixture objects remain clean and equal through metadata 2c5066171.

#table(columns: (2.5fr, 1fr, 3fr), inset: 6pt,
  [*Check*], [*Observed*], [*Evidence/scope*],
  [PKM oracle example], [15 passed], [Includes all 729 F3 scalar-space controls and integrated recovery/kernel checks.],
  [Legacy symmetric/Hecke examples], [4+4 passed], [Regression checks. Across all 3 suites: 23 passed, 0 failed,0 ignored.],
  [Scoped clippy / rustfmt], [Exit0], [Strict warnings; no allow/skip/waiver.],
  [CLI build / checkpoint refusal], [Expected exits], [Build0; refusal1 AND actual checkpoint-refusal message. No scientific execution.],
  [Backlog build / validation], [Exit0], [Existing blocked/unfunded card, unchanged historical pin.],
  [Library build], [Exit0], [Serial execution declares 5646 tests; separate 40-minute execution cap.],
)

The full library started 16:47:54UTC and was active at 17:03; no terminal summary or full-suite pass is claimed. A later one-time telemetry sample found the unchanged JV end-to-end test active, 11 process threads, VmRSS 692476 KiB and lifetime VmHWM 1071324 KiB. The main thread was waiting on a futex; the process reported 221% lifetime CPU utilization. These observations do not diagnose a deadlock or nontermination. Final counts, cap and log hash/size must be collected after termination. Large raw logs remain out of Git and are NOT_UPLOADED without approved transport.

Final proof validation is unchanged: all five supporting contracts build; fresh project audit passes 18 declarations/zero trusted exceptions, and official module kernel replay passes against imports, not --fresh. Existing failed/16/13-declaration receipts stay separate. No old receipt is overwritten or used for changed source.

== Concurrent remote merge
PR2949 is OPEN at 0cc5e0d23a37a502dd363e41d866ec4991d3afb3. Reviewed its three intervening merges against both parents. All 179 prior study blobs are unchanged and every parent log paragraph is preserved. Eight bound source/dependency paths changed, including Cargo, library source and the Lean gate. Current pinned-source results do not validate this newer integration. No bound source is edited or newer base merged while the existing run is active. Local work remains unpushed; auto-merge remains paused.

Evidence: validation/scoped-20261030T1703Z/receipt.json contains exact commands/phase outcomes, 17 source objects, captured-byte hashes and six binary hashes. validation/remote-0cc5-review-snapshot.json records both-parent review and changed objects. G2_ACCOUNTING_AUDIT.md maps exact emission/closure obligations. The initial report and frozen receipts remain unchanged.

#pagebreak()
#image("accounting-coverage.svg", width: 100%)

== What the audit changes about the next implementation
Constructor/reference/scalar inversions already execute through counted multiplication helpers. A future ledger must choose expanded leaf arithmetic or a calibrated whole-call cost, avoiding charging both for one execution. Temporary reference validation counts are currently discarded. Query-generation GroupOps field counts exist in memory but are skipped by serialization. These are specific emitter fixes, not an established cost advantage.

F4 row-reduction counts, partial native logical counts and root multiplication counts do not cover every material class. Unknown classes cannot be priced as zero. Coordinate-field p and scalar-field n arithmetic need distinct identities/calibrations; the tiny fixture uses p=17, n=19. Failed/capped processes need a durable attempted/terminal journal. Independent replay remains outside producer clocks with incomplete count coverage. Process-lifetime RSS does not supply an isolated query peak.

The next implementation must retain and emit these classes, define nonoverlapping ownership, and test conservation/refusal on deterministic native paths. Then freeze the full target stream/protocol and matched external/rho controls. Apply 0, 0.1, 1 GAE per residual native operation symmetrically, retaining unknowns and censored arms. This audit neither implements the full-panel entrypoint nor changes the existing goal thresholds.

== Sources, scope and blockers
PKM2016 Algorithm 1/§3.2 includes relation dependence and linear algebra: people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf. Repository AGENTS §5a requires emitted counts and symmetric pricing. docs/ic/measurement/README.md §3 distinguishes GAE/RPS, plain counts and time. Actual reviewed code: pkm_oracle_program.rs, algebra.rs, reference.rs, pipeline.rs, f4_fp.rs and ic_boundary.rs. The diagram is an explanatory audit map, not a performance plot; no canonical numeric graph or curve edge changes.

Checkpoint/artifact/lab identities and source-bound native runtime/resume qualification remain unavailable. No panel, funding, lease, IAM, quota or pool change occurred. Comparison admission remains false. Complete pricing, external/scaling measurements and Rust/polynomial/cost proof bridges remain open. Software validation or a PR merge does not complete G1–G6.
