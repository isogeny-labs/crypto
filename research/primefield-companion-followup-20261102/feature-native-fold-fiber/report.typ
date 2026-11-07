#set page(paper: "a4", margin: 15mm)
#set text(size: 9.5pt)
= Retained signs for single-image recovery
T-1807 • 2026-11-07 • PCC-20261107-d60455 • unsigned producer evidence

The adapter now retains the individual signs computed during a folded query. When one image z=(k+c)^m becomes known, it lifts to its degree-m scalar fiber and replays the retained signs. This avoids querying a second folded chart. Source batching itself is unchanged.

#image("flow.svg", width: 100%)

== Exact transition
For z nonzero, start with (X+c)^m−z; for z=0 use X+c so that the root has multiplicity one. Replay each saved sign using the existing polynomial gcd update. The resulting roots are exactly the fiber members consistent with every original observation. Replaying the individual signs implies their folded products, so prior constraints are preserved. A singleton is accepted only after the public point check.

For m=2, c=0 and r congruent to3 modulo4, the initial validation sign distinguishes k from −k. Thus once that image is known, no further source query is needed for orientation. Higher powers may require the retained orbit signs or additional queries. Source and transcript caps return unresolved before additional work.

== Final software checks
Validation-02 passes four selected methods,157 imported methods filtered.897 reference-derived fibers across six primes and divisors m from2 through8 give5,382 exact replay-prefix root-set comparisons. All897 end as singletons after the specified six-orbit reference histories;23 have zero images.

All46 public cases recover: the original frozen20 on E/F509, y²=x³+2x+10, P=(3,68), r479,m2; plus all13 targets on E/F7, y²=x³+3, P=(1,2), r13 with both m3 and m6.28 finish from retained signs with zero extra source queries;18 terminate on identity or source zero.462 source calls are recorded for this cohort. It differs from the preceding33-case cohort; no timing or source-count ratio is claimed.

Twelve conditional degree-six fiber cases exercise the residual helper: nine ordinary-sign exits, three zero-source exits and twelve empty-schedule unresolved controls. These cases supply reference-derived images and do not test image acquisition. False final candidates, noncanonical points, invalid images/signs/offsets and transcript caps have controls.31 source/input hashes and generation hashes match.

== Conditional residual guarantee
For distinct k,t, exactly (r−3)/2 shifts a satisfy χ(k+a)=χ(t+a). The fractional-linear ratio (t+a)/(k+a) maps the allowed shifts onto the nonzero field elements except1; equality selects the nontrivial squares.1,420 exact finite pair counts agree with this formula.

For n residual candidates and q fresh independent uniform offsets, the false-survivor probability is at most (n−1)((r−3)/(2r))^q, with n at most m. This derived bound applies only after lifting. The implementation accepts a caller-provided offset schedule; its deterministic tests are not a randomized experiment or evidence of fast image separation.

== Costs, proof map and next work
Replay uses no source calls but pays L degree-m root updates for L retained records. Transcript storage, supplied extra offsets, field inversions and temporary image/fiber overlap are charged. The image and history are dropped before extra queries. The original m source evaluations per folded update remain; source calibration and validation remain charged. No cloud work or timing panel ran; the original4,563 cohort is unchanged.

Lean4.34.1 checks FoldFiber.replay_exact for arbitrary predicate intersections, with only standard propext. Finite-field interpretation and the probability bound remain written proofs. Initial passing source snapshots and final receipts are retained. Sources: local Ward, power-fold and root notes; DLMF27.9; Cheon–Kim2016, doi:10.1112/S1461157015000303. KB unavailable.

Next: exploit retained signs earlier or accelerate the source orbit product, while treating restricted image separation as its own unresolved construction.
