# Item 15: additional toy factorial diagnostic

Prospectively frozen before `ecbench plan` or any trial. This is an
additional diagnostic, not a replacement for the original complete-cost
experiment. The exact frozen input is `factorial-spec.json` and its SHA-256
is retained in `runs/factorial-spec.sha256`.

The panel has p=13,17,29,37, constructor seed1, eight targets each with
target_seed20261018, one operations-only L0 round. Arms are neither,
2LP-only, D4-only, both, current eager D4, exact D4 A/A, and rho.negation.
LP exponent0.95 is identical on both LP arms and inherited from the prior
held-apart training; there is no tuning on this new panel. The two base
types are different transported factor bases on the exact same curve,
generator and target. The harness's workload/round seed is shared by all
arms. No wall-clock claim is permitted.

Historical algebraic charge constants are stated in the spec. They do not
measure a newly executed full multivariate oracle. The harness counts the
exact-enumeration stand-in oracle as unpriced, and the report must keep all
of these IC rows diagnostic lower bounds. Enumerate all unpriced counters
and price them at0,0.1,1GAE consistently on every arm; no ranking may be
promoted when counters are missing. Setup-excluded comparisons also owe a
matched preprocessing-rho reference; this panel will not promote one.

Run `ecbench plan`, inspect registered curve identities, then
`ecbench run --wait --allow-busy --cpus none --spec factorial-spec.json
--out runs/factorial-session` from the repository root with full paths.
The session directory must not exist. Audit with `--replay-all` into a
receipt beside the sealed session. Preserve failures rather than edit
records. The A/A records must match in deterministic operation counts and
recovered logarithms. Export the native table and all six IC/rho pairwise
comparison diagnostics; keep per-curve values beside pooled values.

The historical p101..1301 stacked experiment and round2 p101..1601 study
are distinct prior evidence. Toy finite-field correctness does not measure
a new asymptotic exponent or a deployed-curve attack. A successful audit
completes this narrower factorial diagnostic, with the original item's
fully priced oracle requirement remaining open.
