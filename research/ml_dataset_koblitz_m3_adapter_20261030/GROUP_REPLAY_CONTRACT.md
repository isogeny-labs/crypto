# Group replay envelope fields

Artifact identity remains `koblitz-m3-group-replay.json`, role
`restricted-koblitz-m3-group-replay/v1`, encoding `json`.

The strict outer object has schema
`crypto.ml-dataset.koblitz-m3-group-replay/v1`, `signed` (the existing strict
`crypto.ml-dataset.koblitz-m3-signed-prefix/v1` object),
`reported_witness_check_additions` (integer 0 through 3), and
`reported_total_additions` (integer u64). Unknown fields at either level are
rejected. The whole encoded artifact is capped at 128 KiB.

The verifier authenticates original binding, public/factor-base/input digests
and addition limit through the nested signed replay, independently recomputes
its arithmetic, and requires the total to equal enumeration additions plus
witness-check additions without exceeding the original allowance. A saved
witness must match the first candidate and requires three reported reference
calls. With no candidate, reference calls must be zero. Before three reference
calls finish, the sole candidate must remain the final unacknowledged event;
enumeration cannot advance past it. Partial counts zero, one and two are retained
as partial reports, not rounded up or discarded.

These checks certify consistency of reported counters with the retained
mathematical prefix. They do not verify actual execution counts, native matrix
derivations, outer completion, or durable witness publication. The separate
capture matcher, artifact registry, receipt identity check and outer outcome
policy remain required. Native matrix and result fields are not defined here.

Controls must use the actual signed producer at the first-candidate addition
boundary plus zero through three reference calls. Preserve fixed inputs and the
original allowance in each control; corrupt schema and counter values by
reserializing the JSON and require rejection independently of file hashes.
