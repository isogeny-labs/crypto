# Certified rank stop: construction and review

The fixed two-LP implementation collects the whole finite Bspace before
descent. In recorded29this supplied1253original verified relations and
1122complete affine rows, while the counted replay30determines the same
target from471of those saved rows. That retrospective prefix does not
establish an early collection gain: graph pivots/peeling/cycles may change
with the input prefix, and all checkpoint work must be paid. [pcc:unverified]

The new optional collector uses the existing verified homogeneous graph
projection, not a relation-count proxy. At complete1024Bbatch boundaries,
after at least small_base-1accepted sources, it rechecks each original
Jacobian equation, filters actual omitted Fpabscissae, replays every
combination certificate, and computes rank over the prime scalar modulus.
An exact rank>=small_base-1triggers collection stop. Later checks require
collection_rank_stride additional sources; None preserves the fixed
collector and its existing serialized option fields. [pcc:unverified]

For a nonzero factor-base logarithm vector, homogeneous relations cannot
have full rank. Rank small_base-1then leaves one scale undetermined, but
that readiness condition does not itself determine the target. Two descent
equations can remain dependent. The implementation retains the existing
bounded descent stream, protected target/RHS, affine certificate replay,
modular target solve and actual elliptic verification. No original-solve
fallback is added. A native independent-solver control demonstrates a
ready homogeneous rank with two dependent descents and an unknown target,
then an extra independent descent determines it. [pcc:unverified]

All graph verification, projection and counted rank arithmetic are included
in the LAphase, with every checkpoint certificate/sourceprefix/Bcount/rank/
legacy product count/primitive delta retained. New checks consume no RNG.
The qualifier requires exact original-relation prefix equality with the
fixed reference and equality of their common accepted-descent prefix. It
replays checkpoint combinations, independently computes modular ranks,
checks the first successful threshold/stride/batch criteria and counter
containment, and applies the unchanged final affine target checks.
[pcc:unverified]

Meaningful source controls cover dependent duplicate rows versus actual
rank; malformed homogeneous RHS/out-of-range columns; checkpoint RHS,
combination and target-column tampering; forged rank/readiness; insufficient
phase counters; and the dependent-descent counterexample. Existing29summary
reanalysis is byte-exact after the shared qualifier extraction. No claim
of independent signed verification follows from these producer controls.
[pcc:unverified]

Sources checked(KB): exact and negative incremental-LP queries unavailable,
validation/rank-stop-kb-search{1,2}.log. Primary Joux–Vitse§3.3/§6 explains
the adapted split-quadratic variation and the practical tradeoff of extra
relations against structured elimination. The change is an implementation
choice on this bounded degree9weak family, with no novelty or transferred
asymptotic claim. Read the original source at
https://www-fourier.ujf-grenoble.fr/~viva/research/articles/CoverDecomp.pdf.
[pcc:unverified]

Fresh31validation is prospective in Amendment26/rank-stop-plan.json.
Native source checks pass:26sieve tests with2existing ignored, public-input
1/1, production example build, library/example warnings-denied Clippy,
17rank-stop and14oldqualifier checks/build/Clippy, and byte-exact29summary
replay. Two root-source hashes remained unchanged throughout testing. The
fresh31scientific call has not run. Complete costs, comparative eligibility,
fastest-external and rho assertions remain outside this protocol; counts
for native containers/scans/fixed-factor internals, PRNG internals/other
sampling, hashing/serialization remain incomplete. [pcc:unverified]
