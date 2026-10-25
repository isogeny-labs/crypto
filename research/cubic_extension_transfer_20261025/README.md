# Cubic-extension transfer: frozen Sage evidence

This PR preserves the completed study requested by the owner. It imports the
existing SageMath 10.6 source snapshot, recorded results, and compact exact
certificates. It adds no native execution path or new research measurement.
The historical Python files are retained as provenance under AGENTS.md's
"Preserve frozen inputs, historical measurements, source snapshots, and
certificates as evidence" rule, rather than presented as a native replacement.

## Read the evidence

- [Result report](historical/extension-field-experiments/research_lead/FOUR_TRACK_RESULTS.md)
  and [methods](historical/extension-field-experiments/research_lead/FOUR_TRACK_METHOD.md).
- [All twelve obstructed orders](historical/extension-field-experiments/research_lead/results/forced_two_step_audit.json)
  and [explicit divisor witnesses](historical/extension-field-experiments/research_lead/results/explicit_forced_two_step_witnesses.json).
- [Two-step transfer and recovered source scalar](historical/extension-field-experiments/research_lead/results/forced_source_pipeline.json).
- [Same-endomorphism-order comparison](historical/extension-field-experiments/research_lead/results/endomorphism_audit.json)
  and [complete finite CM screen](historical/extension-field-experiments/research_lead/results/cm_coverage.json).
- [Saved validation](historical/extension-field-experiments/research_lead/results/four_track_validation.json)
  and [source hashes](historical/extension-field-experiments/research_lead/results/four_track_summary.json).
- [Import report](reports/IMPORT.md), [PDF](reports/IMPORT.pdf), and
  [publication manifest](PUBLICATION.json).

## Exact scope

The source family is ordinary, non-subfield elliptic curves over F_(p^3),
with prime full group order r, p = 1 mod 3, and r = 4 mod 9. In the recorded
p=157 example, every one of twelve transverse candidates has Jacobian order
3871629 instead of source prime order 3868159. Two nonprepared sources each
have a unique rational degree-three isogeny into that failing prepared model.
The retained two-step pipeline recovers scalar 271828 and verifies it on the
original source curve. This refutes a guarantee of one total degree-three
step within this construction. The guarantee of one extra step after
preparation remains open.

The p=109 comparison exhibits different coverage at geometric endomorphism
discriminant -21195. Its finite CM audit has twelve Frobenius orbits, one
with zero coverage; a degree-five isogeny repairs that orbit. These are
specific recorded examples, not universal coverage or novelty claims.

The retained logs contain ten complete prototype transfers and seven
targeted exceptional-input checks. Their "independent validation" means the
saved source answers, exact line intersections, graph substitutions and
linear systems were checked by the retained verifier. It is not an
independent-host or independent-agent native replay.

## Admission and publication status

This is a historical-evidence import. The timing observations were collected
with Sage, some under concurrent load. They are not isolated native
measurements, calibrated operation counts, a strongest-folded-rho comparison,
or leaderboard-admitted evidence. No performance promotion, fastest-tool
claim, new attack exponent, or cryptographic-size recovery is made.

Five original files exceed the artifact-store thresholds and await unchanged
S3 upload and readback. Their exact hashes and sizes are in PUBLICATION.json
and the snapshot's .artifacts.json. No remote upload is claimed. The complete
original 164-file bundle remains preserved with SHA-256
`6c6e079c8ebb64b52f03039bad743dc0948d0f1953b9f528b76e684959405aba`.
Until hydration, historical links or scripts needing those five files cannot
be fully replayed from this checkout. The compact order and transfer
certificates linked above are included.

After upload, hydrate with the repository artifact tool, then verify every
file against PUBLICATION.json before replay. Do not execute new research
comparisons with this historical Python path: a native implementation and
the repository's matched validation protocol are still required.

Conductor, Cargo/Rust, and Lean/Lake are not available in this task
environment. A later read-only CLI check with permitted proxy access verified
the GitHub login; the earlier authentication warning was caused by blocked
network access. No repository-wide native test run is claimed. Existing Sage execution records are
preserved; repository-wide release tests and formal proof checks are not
reported as passed. See [the admission problem](../../problems/cubic-extension-transfer-admission.md)
for the exact remaining obligations. The repository scoreboard, benchmark
index, registry and leaderboard remain unchanged: these diagnostics have not
met their admission gates. The browsing-only copy under benchmarks has no
admitted row.

## Requirement status

| Requirement | Status | Evidence or gap |
| --- | --- | --- |
| Preserve the requested code and results | Partial publication | 159 original files included byte-for-byte; five need S3 upload |
| Certified total-step counterexample | Recorded exact certificate | Twelve target orders and explicit divisor witnesses retained |
| Complete toy transfer | Recorded complete | Ten source answers; one automatic two-step example |
| Universal extra-step bound | Open | Bounded searches do not decide it |
| Universal exceptional-chart coverage | Partial | Generic tangent chart and bounded retries remain |
| Native matched performance result | Not established | Port, isolation, strongest rho, native pricing and replay missing |
| Lean formalization | Blocked / not verified | No Lean/Lake toolchain; central structural bridges remain unformalized |
| Repository release tests | Blocked locally | No Cargo/Rust or authenticated CLI checkout; CI pending |
