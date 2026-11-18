#set page(paper: "us-letter", margin: 0.65in)
#set text(font: "DejaVu Sans", size: 9.5pt)
#set heading(numbering: none)

= Branch-helper-cast renewal machine review

#text(fill: rgb("475569"))[Attempt 1 · 2026-11-10T23:59:01Z to 2026-11-11T00:02:14Z · execution commit `f3c1b8cc7663ca7d15374ae5e9a16c86839c9d18`]

== Outcome

The bounded attempt ended terminally after *192.1372683380032 worker-wall seconds*. Pre- and post-custody passed. `BranchRootMultiplicity` and `BranchCoefficientMultiplicity` compiled. `IndependentControls` failed with ``simp` made no progress` at lines 22 and 28, so the thirteen-name readback and expected `False` rejection were not attempted.

The machine-review outcome is *needs revision*. This is a partial machine result, not complete five-call validation.

#figure(
  image("outcome-flow.svg", width: 100%),
  caption: [Recorded outcomes and custody state. Grey stages have no execution result.],
)

== Exact machine evidence

- Root module: exit 0; `.olean` SHA-256 `e999a7c8e322b2c2fd4a0cb0182a04fed188458b41e1effa72df20cc373c786a`.
- Coefficient module: exit 0; `.olean` SHA-256 `e770427600c120992331b2ecd77764643d3fb09211f097d960ef604a79e8fa84`.
- Printed declarations for both successful modules listed only `propext`, `Classical.choice`, and `Quot.sound`; no `sorryAx` appeared.
- Controls: exit 1; no `.olean`; stdout SHA-256 `83d553eb35a7030e8074a0fae2460b33683aa4b7b801837bca163ada4a8fdbeb`.
- Readback and expected-failure sources were present and frozen but were not invoked.

== Custody, resources, and terminal state

Both custody passes checked 16,736 dependency files totaling 3,111,892,318 bytes. Their dependency, retained-import, and runtime inventories are byte-identical; all 26 retained source/olean bindings match. The attempt used the declared 1 CPU, 8 GiB memory, zero swap, 128-task limits, no retry, and remained inside the 600-second attempt envelope and absolute deadline. Root charges the full 600 reserved seconds.

All three recorded phase owners are terminal. The failed compiler systemd unit retains a failed record but has no control group; its manager and compiler child PIDs are absent.

== Claim boundary and next action

The two successful calls establish acceptance by the pinned Lean compiler and declared dependency trust boundary. There was no independent binary kernel rechecker, and this review did not independently rederive theorem semantics. The failed controls and unattempted modules block complete machine validation and any semantic promotion.

Preserve this terminal attempt. A controls correction is a new source revision requiring publication, affected-source review, and separate authorization; this attempt must not be resumed or described as if its unattempted controls passed.

_Authoritative structured record: `review.json`; artifact hashes: `evidence-bindings.json`; packet manifest: `manifest.json`._
