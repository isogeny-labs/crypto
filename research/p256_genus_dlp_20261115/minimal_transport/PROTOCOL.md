# Minimal transport in the existing genus-3 eigenline

## Preserved goal and scope

Continue the user's search for a transfer yielding a net P-256 DLP speedup.
Use RunPod for the requested execution; do not replace it with local timing.
PR4258 is merged. Its code, reports, proof records and outputs remain immutable.
This new round checks a transport optimization and its precise degree boundary;
it does not discharge the pending full cold-cost comparison.

Start from main6f1bd2d9c2c7426f3c8d64b12b73bfd30bcc24c5. Read current AGENTS,
latest journal, ecdlp-surfaces/higher-genus, endomorphism rules and relevant
lesson briefs. KB status127 and no connector. RunPod's documented proxy route
still returns401. No pod allocated. No checkpoint token, AWS identity/CLI,
Lean toolchain or shared Lean checkout was initially present.

## Hypotheses and literature before implementation

Write phi_(a,b)=a alpha0+b alpha1 in the existing elliptic-map basis.
The prior adjoint matrix has diagonal2, off-diagonal-1, and S acts by
T(a,b)=(-b,a-b). The finite eigenline condition is b+lambda*a=0 modulo n.
Investigate its Eisenstein norm q(a,b)=a^2-a*b+b^2, preservation under T,
divisibility by n, positivity and minimum. A shortened vector must preserve
the exact finite subgroup and admit source-side answer checking.

Distinguish the degree of the pulled-back principal polarization from the
degree of an isogeny between elliptic curves and from map evaluation cost.
Search GLV/Eisenstein literature before a novelty statement. Primary references:
Smith arXiv1310.5250 sections1--5; Wang--Ouyang--Li--Hu eprint2021/090 section3
(Cornacchia excerpt; direct PDF retrieval may fail); prior Galbraith14.4.
No novelty claim is intended.

## Work and admission boundaries

1. Write the general algebra and faithful proof map. Apply lean-instance-gate
   before Lean edits, use shared crypto-lean/Mathlib laws rather than copies,
   and preserve exact scope when a geometric bridge is not formalized.
2. Implement a separate native Rust coefficient/map candidate and negative
   controls. Compile-only checks are not numerical qualification.
3. Before any executable research panel, fill .resume/RESUME.md and pass
   scripts/checkpoint.sh gate. Do not bypass failed upload access.
4. Freeze source/build and distinct pre-call manifest. Copy the qualifier to
   an immutable run directory and hash it before calls; never edit a live copy.
5. Planned numerical panel: prime subgroups109,823,4111 and actual P-256 order;
   norm/eigenvector/recovery congruences, six-unit invariance, malformed inputs,
   source-map equality, and unchanged public DLP scalar/replay. Actual P-256
   remains a bounded transition/map test, not full-order DLP recovery.
6. RunPod: one CPU pod, at most4vCPUs/30GB, allocation<=30min and total<=USD1,
   authenticated rate check and allocation-specific deadline/deletion guard.
   Preserve source/binary/host identities and all failed or interrupted runs.

No local timing or operation ranking is admitted. Short coefficients alone
do not certify faster map execution or a faster destination solve. Net benefit
still requires the matched source/A-A/generic/folded experiment, complete setup,
mapping, online cycles, recovery and source verification, and independent replay.

### Proof-artifact clarification before supplemental serialization

Once the general polynomial identities pass the Lean kernel gate, their
required identity records may be serialized/replayed with the unchanged
existing identity checker as proof-artifact verification. This separate tool
imports no candidate gcd/map code and accepts no P-256 coefficient inputs.
It neither qualifies the new numerical candidate nor supplies any timing.
The checkpoint requirement still applies to every new native qualification
and cloud experiment; these remain unrun until upload/provider access works.

## Stop and success conditions

Reject any wrong congruence, invalid divisor, changed scalar, unpriced advantage
or incomplete source binding. Do not replace failed fixtures or raise caps
after observations. Numerical calls have120s caps and remain pending if the
checkpoint/provider access fails. Build/proof work has a30CPU-minute initial
budget. Publication is a new regular PR with report, diagram, inspected PDF,
proof/identity records and unsigned standing stated explicitly where necessary.
