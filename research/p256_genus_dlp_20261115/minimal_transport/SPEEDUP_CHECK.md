# Test for a net cryptanalytic speedup

This is an acceptance addendum to the existing protocol, dated 2026-11-15.
It defines the pending comparison; it contains no new performance result and
does not modify the frozen candidate, qualifier, reports or numerical caps.

## Present evidence and implementation boundary

The integer coefficient endpoints and earlier folded-walk correctness evidence
are described in `REPORT.md` and `../folded_walk/REPORT.md`, with their precise
proof and execution scopes. They do not supply a complete-cost comparison.
The new native candidate has zero executed qualification cases; its CLI exposes
`qualify`, not an admitted performance session. [pcc:unverified]

The inherited solver explicitly counts setup scalar-multiplication events
without charging their internal work. Its group-addition counts cannot compare
elliptic and Jacobian arithmetic. Before ranking arms, integrate the applicable
adapters and complete counters into native ecbench, freeze the runnable spec,
and preserve its source/build/pre-call identities. Provider credentials alone
do not complete that implementation work. [pcc:unverified]

## Cost model and acceptance criterion

For one target, define complete costs in a single common unit:

\[
T_E=F_E+N_E C_E,\qquad T_J=F_J+N_J C_J.
\]

Here F includes setup, transport, recovery and source verification. N counts
online transitions, and C is their average complete cost, including addition,
representative selection, coefficient updates, hashing, distinguished-point
handling and cycle/escape work. Charge every operation exactly once. Record
memory and failures alongside costs. A Jacobian addition is not one elliptic
addition in this accounting.

The ideal uniform-walk model compares generic signed orbits of size six with
source negation orbits of size two: N_J/N_E is approximately 1/sqrt(3). Under
that assumption alone, T_J<T_E requires

\[
C_J < \sqrt3 C_E + \frac{\sqrt3(F_E-F_J)}{N_E}.
\]

With equal fixed costs, the per-step allowance is sqrt(3), approximately 1.73.
This is an orbit-count allowance, not a measured speedup or a universal bound
on possible implementations. Extra transport/setup cost reduces that allowance.
Nonuniform walks and fruitless cycles can invalidate the ideal step ratio;
use observed complete totals for the experimental decision. [pcc:unverified]

Primary source: Galbraith, *Mathematics of Public Key Cryptography*, theorem
14.4.3 and sections 14.4--14.4.2, including its heuristic and cycle exclusions.
The reviewed book PDF SHA256 is
`ca75ee62b96daa0d46faaa2d4dc28205683455ad3a2a48586b9b39299c25022d`.
This reuses the known equivalence-class analysis; no novelty is asserted.

## Required comparison

- Use the same subgroup, generator, target and recovered scalar in every arm.
  Keep the target scalar out of solver inputs; verify every answer on the
  source curve with a separate replay path.
- Include the strongest applicable documented source negation solver, a source
  A/A copy, a destination generic control, and the destination signed-rotation
  candidate. Freeze all applicable optimized source variants before seeing
  results. The existing affine correctness adapter is insufficient as the sole
  performance reference.
- Charge the complete cold one-target interval. If jump construction is moved
  to the source followed by mapping, include those costs and declare that
  variant in advance. Report amortized setup only as a separately defined
  workload. Preserve failed, capped and restarted runs.
- Use native ecbench with at least eight independent targets, one warmup and
  five interleaved measured rounds. Use at least four sizes for a scaling claim.
  Emit common field/bit-operation costs across E and J; incomplete group-event
  counters cannot establish a cross-group ranking. Apply the same required
  unpriced-operation sensitivity band to every arm.
- Replay every deterministic case and inspect the paired complete-cost ratio
  and its uncertainty. Admit a cost advantage only if the candidate/reference
  ratio is below one with its interval excluding one and the ranking survives
  the required pricing band. Admit wall time only with L2 isolation and a
  passing A/A noise control; a RunPod allocation alone does not establish either.

Complete toy DLP recovery supports conclusions for those fixtures. Actual
P-256 phase measurements plus a random-walk projection remain a modeled
estimate, explicitly labeled with assumptions. The bounded actual-P-256
transition trace is not a full-order DLP solve. Shorter map coefficients do not
reduce the subgroup order or establish a lower DLP exponent. [pcc:unverified]

## Execution dependencies and next action

On 2026-11-15 the documented read-only RunPod API route returned HTTP 401.
The mandatory checkpoint gate also failed because no upload token or configured
AWS profile was available. No pod was allocated and no new candidate case or
timed session was launched. Sanitized status is in
`checks/speedup-readiness-20261115.json`. [pcc:unverified]

Restore authenticated RunPod access and checkpoint upload through environment
secrets or an already configured host; never put credential values in research
artifacts. Then pass the checkpoint gate and run the existing immutable native
qualifier within the original envelope. Only after qualification and the
benchmark integration above should the frozen full-cost comparison execute.
Stop on a wrong scalar, replay discrepancy, failed admission control, missing
cost category or exceeded cap. A null or slower result is a valid outcome.

Sources checked (KB): no KB MCP tool is loaded; `scripts/research-kb status`
and the mechanism search return 127 because the catalog executable is absent.
Fallback sources are the primary book sections above, the existing frozen
reports/protocols, the actual candidate/solver source, and
`.agents/skills/ecbench-measure/SKILL.md`. No absence or novelty conclusion
is drawn from unavailable KB access.
