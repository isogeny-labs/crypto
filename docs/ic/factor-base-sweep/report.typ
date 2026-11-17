#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)
#let v = json("validation-summary.json")
= Unified native factor-base sweeper
Engineering implementation · 2026-10-17–18

One launch command enumerates structured bases, constructs exact native point
inventories and polynomial systems, runs degree-bounded F4, verifies actual
point decompositions, and preserves resumable numbered attempts. Human stderr
lines follow the repository's CADO-style stage conventions; durable JSONL has
the same events.

== Real-time feedback
The display separates verified decompositions, refutations, exhausted probes
and independent training rank. It reports cumulative and rolling verified
rates, independent relations per second, provisional phase/rank ETAs, and
provisional ratios to matched prefixes of the previous and first completed
baseline candidates on the same curve. Positive controls and holdout probes
never enter training rank. Unknown estimates remain unknown.

Rates include finite-inventory truth census, system construction, solving,
witness checks and incremental logging. Initial base/curve construction is
separate. Shared-host telemetry is not isolated timing evidence. No total-cost
winner or end-to-end speedup is admitted; native pricing remains incomplete.

== Implemented scope and limits
Default functional fixtures cover small binary and prime fields. Seed and
dimension grids are available without editing JSON. Binary recipes retain
every basis word; exact ordered signed inventories retain FB1, FBM1, an order
Merkle commitment and artifact hashes. Verification independently reconstructs
inventories and replays point witnesses and relation rows; F4 is not rerun.

The solver path uses two summands. Prime systems use direct S3 and finite-set
membership with coefficient modulus at most one million. Binary shape
construction is bounded to field degree 31 and dimension 8. Wide 83/130-bit
curves, general nonlinear binary inventories, other summand counts,
algebraic-closure certificates and certified degree of regularity are not
implemented. Exhaustion does not establish nonexistence. Successful solving
degree is an observed procedural diagnostic, not certified regularity.

== Actual validation
#table(columns: (1fr, 2.6fr), inset: 6pt,
  [Native library], [#v.library],
  [CLI unit / integration], [#v.cli],
  [Affected native checks], [#v.native],
  [Historical trace replay], [#v.traces],
  [CI-scope helper], [9 tests passed after minimal base integration repair.],
  [Unavailable], [Conductor absent; pinned workflow HTTP 404. Neither is a pass.],
)
Source and artifact binding: #text(size: 8pt)[#v.source].

#pagebreak()
== Execution and evidence boundaries
#image("architecture.svg", width: 100%, height: 208mm, fit: "contain")

This diagram describes an implementation and functional checks, not a
cryptanalytic performance result. No historical CPython generation or S3
maintenance operation ran. Frozen historical evidence, registries, aliases,
stores and the draft, unrun, zero-funded planning card remain unchanged.

Sources: #link("README.md")[launch, plan, resume, verify and limitations];
#link("IMPLEMENTATION.md")[requirements]; #link("VALIDATION.md")[validation receipt].
Editable visual source: #link("architecture.dot")[architecture.dot].

#pagebreak()
== Retained functional fixtures
#image("functional-witnesses.svg", width: 100%, height: 220mm, fit: "contain")

The native run completed 15 candidate configurations; one requested binary
subfield was inapplicable. The completed cases retained 105 trials. Native
verification checked all 16 sealed attempts and 29 actual point witnesses.
Positive controls are separate from the plotted training/holdout counts.
No regularity, best-shape or speed conclusion follows from these small samples.

Source: #link("functional-fixture-observations.json")[recorded native counts];
#link("render-functional-plot.py")[rendering source].

#pagebreak()
== Integration blockers and preservation
The full library command failed three unchanged trait-query cases because
the committed traits JSON lacks the required #raw("levels") field, plus one
unchanged Jacobian classification expectation (#raw("Boundary") versus
#raw("FormulaMarginal") for c2pnb304w1). The exact traits literal is present in
the retained unit binary, with all 731 library inputs matched to source.

Historical native trace replay is blocked by the registration guard: all 48
model rows in the frozen historical source are missing from main's current
2,375-row registry, while their alias targets remain. Two of four old trace
unit tests therefore fail; no complete 384-record replay is claimed here.

The naming check reports 4,042 problems on unchanged inputs. Relevant ancestor
pin files were checked; none was missing. Canonical maintained Clippy flags
the unchanged research driver at line 206 (#raw("drop_non_drop")) and the
unchanged #raw("n83_precomputed_rho") example at line 249
(#raw("manual_is_multiple_of")). No assertion, resource limit or coverage
condition was relaxed. Conductor remains absent, and its pinned workflow was
HTTP 404; those checks are unavailable, not passed.

An environment restart interrupted earlier work. Partial and failed logs are
retained alongside complete retries; unknown exits are not promoted to passes.
Before the feature could build, a separate prerequisite commit repaired merged
native declaration/import/dispatch defects, preserving both quartic families.
All new library regression cases pass.

This feature leaves main's frozen research, backlog, registry, aliases and store
unchanged. That preserves scope; it does not claim to restore already-missing
upstream registry rows. Historical CPython generation was not rerun. No new
point logarithms, native performance claim, live S3 verification, paid host or
research campaign is reported. The planning card remains draft, unrun,
zero-funded and unleased.

Canonical scoreboard, leaderboard, timeline, performance-gain and figure paths
were reviewed and left unchanged: there is no accepted new performance or
mathematical result to place on them. Future measured comparisons retain the
symmetric native-pricing bands and matched fastest external-reference gate.

Full details and source hashes: #link("VALIDATION.md")[VALIDATION.md].
Raw commands, failures, retries and input hashes:
#text(size: 8pt)[#v.records].
All 567 raw member hashes pass after extraction; deterministic repacking is
byte-identical. Native verification of the extracted run also checks all 16
sealed attempts and 29 actual point witnesses. These integrity checks leave
the recorded failures unchanged. See #link("validation/README.md")[bundle receipts].
