#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
#set heading(numbering: none)
= Full recovery: unit elimination has no demonstrated E2E gain
17 November 2026 · fixed weak Fp6 validation case · producer evidence

The complete existing graph pipeline has a median worker-process wall of
*20.71 s*, versus *21.10 s* with five-variable Nagao elimination. The paired
geometric ratio is *0.99832*, with95% bootstrap interval *0.96725–1.02717*:
this cohort does not demonstrate an end-to-end improvement. The earlier27.7%
incomplete component subtotal is not an E2E result. [pcc:unverified]

#image("wall.svg", width: 100%)

#table(columns: (2.8fr, 1fr, 1fr), inset: 5pt,
 [*Complete native method*], [*Median s*], [*Minimum s*],
 [Graph IC, six Nagao variables], [20.71449], [20.60457],
 [Same graph IC, five variables], [21.10177], [20.84097],
 [Registered rho, plain / batch16], [111.40204], [111.02453],
)

Five measured calls per method;3warmup and6A/A controls retained separately.
All24recoveries and independent checks complete, all24local isolation records
are uncontended, and all three A/A max/min ratios satisfy the frozen1.10limit.
No outlier, failed arm or source change is excluded. Six worker tests and
55independent qualifier tests pass; scoped Clippy passes. [pcc:unverified]

== The full interval
Every new process parses and validates the public challenge, builds its
thread pool and cold transfer/model/factor base, collects and checks relations,
filters the graph, computes the kernel, attempts descent, determines the
affine target, checks the scalar on the elliptic curve and serializes output.
Counting instrumentation and in-process calibration are included. The owner
generates the challenge outside the solve; compilation, isolation preflight
and the separate independent audit are also harness work. No reusable attack
state or answer is supplied to a worker.

The copied qualifier replays original source/norm/Cantor relations, graph
witnesses, homogeneous rank/kernel, descent/affine equations, unreduced target
recovery and the elliptic endpoint. Both IC methods have identical collection,
kernel, accepted descents and target, exact repeated work signatures and zero
foreign hooks. Only descent arithmetic is allowed to differ. [pcc:unverified]

This is local operational timing of the restricted weak Fp6 family, p1009,
seed389, worker401. It is a known-answer fixed case; its bootstrap reflects
process-timing variation, not curve/target/walk sampling. The native rho is
the best previously observed registered configuration on this case; global
parameter optimality and fastest-external-reference status are not asserted.
Primitive prices remain incomplete and no general rho-parity tier is claimed.

PCC-20261117-6432aa is unverified. Frozen source, input, commands, manifests
and complete statistics are linked in README.md. No new paid allocation.

#pagebreak()
= Individual measured recoveries and provenance

#table(columns: (0.7fr, 1fr, 1fr, 1fr), inset: 5pt,
 [*Round*], [*Six vars, s*], [*Five vars, s*], [*Rho, s*],
 [0], [20.714492], [20.840968], [111.402041],
 [1], [22.183273], [20.900736], [111.122133],
 [2], [20.665350], [21.511353], [113.755720],
 [3], [20.604573], [21.101767], [111.024526],
 [4], [21.753544], [21.346093], [118.026806],
)

IC order reverses on odd rounds and rho's position rotates. Primary values
are paired by round. Warmups and controls do not enter these medians.
Paired-round bootstrap uses10000draws with the frozen deterministic seed.

#table(columns: (1.5fr, 1fr, 1.4fr), inset: 5pt,
 [*Paired wall ratio*], [*Geometric mean*], [*95% bootstrap interval*],
 [Five vars / six vars], [0.998320], [0.967251–1.027166],
 [Five vars / registered rho], [0.187009], [0.183716–0.189282],
 [Six vars / registered rho], [0.187324], [0.183247–0.193696],
)

The median ratio five/six is1.018696. It differs from the paired statistic
because every matched round is retained, including the slower baseline in
round1. This is not evidence of an E2E gain from the new finder.
[pcc:unverified]

== Frozen implementation and input
Scientific source: `7802b7f864594d757f88991f7d70b77c38715ebe`.
Public input: `c1b9bd4e87535974d554ab7f9a383ebb79b4ad56`;
SHA256 `48584910dd72ce9c30dba0063ba7f96cf7854c55d48fadf6e58f2dca7fba3852`.
Library: fcda1e22cb0c5424ca3244f783e69be8b99792f4 plus the explicit
solver-callback overlay. Native Rust1.100.0; one Rayon worker, CPU0,
AMD EPYC9V74, Linux6.18.57. VM host scheduling/frequency remain outside the
local isolation tool's reach. The original checker files are byte-exact.
No current-main global library build is claimed.

== Evidence and custody
Twenty-four fresh processes,24valid recoveries, zero automatic retries;
degree24/staircase64,8192residual/32accepted-descent caps,300-second worker
and7200-second outer limits. All statistics are computed by the frozen native
Rust analyzer; Python renders precomputed values only.

All109raw members are retained in the19071290-byte local archive, SHA256
`e1c5951e4091642e33996dd51c22de0cc310b00b14d9c529f2a2cdd9c1f71f4f`.
The custody manifest lists every original report, isolation/stdout/stderr record
and owner fixture. No authenticated AWS identity is present in this runtime;
no artifact-store upload is asserted. Compact results and source are public.

The report establishes this fixed-case measurement at producer standing.
Independent signed admission, unknown-log/growing-family evaluation and a
globally strongest external reference remain separate requirements.
