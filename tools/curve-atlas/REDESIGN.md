# Diagram redesign, 2026-11-15

The user rejected the first exported diagram's visual quality. The overlapping
Jacobian arrows and small labels made the relationships hard to read. Replace
the shared all-to-one layout with a light, panelled atlas: one row per isogeny
and one row per cover, with the same source E repeated and explicitly labelled
as repeated. Each cover row contains C -> E and the two separate E/J(C) arrows.
No mathematical nodes, maps or search bounds are changed.

Source context: current Curve Atlas renderer/exporter and the earlier browser
export checks. This is a layout task, not a mathematical or performance claim.
PR4245 remains open with a pending Copilot conflict-resolution request; source
changes here do not resolve or choose sides in its conflicted shared log/ledger.
Retain the earlier pinned logs unchanged and record new checks separately.

## Layout check

The first redesigned browser suites exit 0 for both bundled and modular HTTP
pages. Opened the exported light-background PNG and mobile screenshot. The PNG
now has separate isogeny and cover panels with readable local arrows. These are
producer observations, not signed independent execution standing.
[pcc:unverified]

Final polish removes redundant +0 and exponent-1 notation in displayed cover
formulas. Add interaction/coverage checks to ensure each original JSON edge is
still represented and node/edge selection works with repeated source labels.
Keep mobile horizontal scrolling explicit rather than scaling every panel into
unreadably small type. Existing pinned logs and conflicted shared metadata stay
unchanged while Copilot's requested merge resolution is pending.

## Final producer checks

`redesign-validation.log` retains the final browser output and exit status.
Both HTTP variants exercise PNG/SVG downloads, full-graph framing, tall graphs,
mobile controls and encoder failure, with additional checks for edge-set
preservation against the JSON export and node/edge inspection. No mathematical
engine or prior pinned validation log changed. These are unsigned producer
observations, not independent execution standing. [pcc:unverified]
