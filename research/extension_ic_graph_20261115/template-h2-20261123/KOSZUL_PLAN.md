# H2 phase 2: bounded principal-syzygy template

Distinct successor to the completed phase-1 generic-provenance probe. Local,
Rust, proposed; no performance result. Same immutable saved input hash and
exposed cases 0/1; the phase-1 command and its guard stay consumed.

## Method and exact guard

For a fixed degree bound D enumerate each original generator f_j multiplied
by monomials m with deg(m)+deg(f_j)<=D. Omit m*f_j when the leading monomial
of an earlier f_i divides m. The principal syzygy f_i e_j - f_j e_i expresses
that row using rows from earlier generators and smaller multipliers for j.
Grevlex is degree-first, so every replacement stays within the same total
degree bound. Repeated substitution terminates in this position/monomial
order. This prunes principal syzygies only, not the full syzygy module.
This algebraic argument is producer reasoning, not a signed or Lean proof.
[pcc:unverified]

Freeze input leading monomials; reject specializations changing them.
The online exact membership, input-containment and S-pair checker from phase
1 remains mandatory. A recorded leading ideal is only a policy guard; no
training coefficient or reference basis polynomial supplies an online row.
All online rows come from the new original input's retained multiples.

The existing saved unrestricted F4 basis supplies the development leading
ideal. This is an offline, oracle-assisted construction control. The previous
F4 call's cost is not measured afresh here, so no cold performance ratio may
be inferred. A later matched comparison must construct/price that basis.

## Qualification and once-only probe

Keep the original nine controls. Add native controls that compare pruned
and unpruned bounded row spaces on small ordinary/zero/constant examples,
check principal-row counts and exact accepted ideals, and refuse a changed
input leading monomial. Invalid input and resource caps remain explicit.
Publish qualified source/binary hashes before the saved-case call.

Exactly one distinct probe, in order D=5 then D=6, will construct from case
0 and apply each bounded template to case 0 then case 1. Retain each refusal
and all attempted arithmetic; do not tune/prune after observing case 1.
Do not retry a degree or relaunch this protocol on failure. Save phase-start
events before expensive work. A constructor refusal skips that degree's
applications and moves to the next prespecified degree.

Limits: one thread, local, no paid allocation; 8 GiB virtual-memory limit;
180-second outer wall bound and five-second TERM grace. Compiled template
caps are 1536 rows, 1024 columns and 4,000,000 coefficient/provenance cells.
At most 100,000 enumerated multiples and total degree at most 16. Both degree
attempts and exact checker work are inside the same outer bound.

Record all construction/refusal status, enumerated/kept rows, column count,
rank, online elimination products/inversions/updates, exact checker work and
mutual containment versus saved F4. Compare no incomplete counter against a
full runtime. Complete output is a viability result, not an IC speed claim.
Any timeout preserves the prefix and leaves unexecuted cases explicit.

Sources checked: primary Larsson–Åström–Oskarsson 2017 §§2–3 (generator
representations, syzygies, prime-field template construction), the existing
phase-1 exact checker and completed refusal record. The principal syzygy
identity is the restricted row-pruning mechanism; neither generic training
support nor an assumed quotient dimension certifies completeness. KB and
Conductor remain unavailable as recorded in the prior source review.

Git/source/hash custody is local and explicitly authorized; S3 checkpoint,
large artifact publication and isolation admission remain unavailable. Keep
oversized witnesses in LOCAL custody with exact manifests, without splitting
to avoid artifact limits. The production default remains unchanged.
