# C95: sum-scan refutation and measured cross-target reuse

Three immutable experiments passed with one core and4GiB per process. The sum-scan performance hypothesis was refuted as a default on the larger fields; its exact mathematics and all results remain preserved. A changed cross-target reuse hypothesis then produced cold component gains on every requested NIST prime. This is not the full rootfinding breakthrough or a genuine F4/F5/F6 comparison. The50x end-to-end goal and unchanged gate remain active/unpassed.

## Exact interval source sums

The proposed fixed-target prime-field scan uses S=x+y,T=xy and the exact identity S3=(T-uS-2u^2-A)^2-4f(u)(S+u). Under2M<p andM^2<p it enumerates2M+1sums, at most2products each, then exact integer-discriminant support recovers every original ordered source pair. The strong comparator scans x only and solves the original quadratic for y, batching all nonzero denominators. Both are O(M). Comparing only to an O(M^2) pair scan would exaggerate this proposal.

Independent literal evaluation passed336prime/domain/target cases,195complete roots,2,046separate identity checks,58cases withf(u)=0, and4no-wrap rejection controls. Characteristic3,zeros,repeats,torsion and singleton domains are included. The cold all-five-NIST panel usedM31,targets0/1/a frozen repeated-source target,3repetitions alternating methods:90complete child runs with identical original root sets. Source medians were0.0311–0.0753s;sum medians0.0328–0.1246s. On P521 the sum/source ratios were1.67,1.79,1.66; the P384zero case was1.76. The sum variant does not merit default integration. Full per-target medians and all streams remain in raw evidence. These subsecond conditional cases cannot satisfy the hard gate.

## Changed mechanism: reuse source roots across targets

Index-calculus relation collection revisits one source domain for multiple targets. Cache the square class and normalized root of f(x) once per source/target. The original quadratic discriminant16f(x)f(u) then needs multiplication, not a fresh square root for every pair. Both direct and cached arms share field/nonsquare setup and ONE batch of denominator inversions for the entire target panel. Cache construction,all16targets,original verification and complete serialized output are charged in every cold child. No pre-existing cache is supplied.

Ten independent tiny all-target controls matched133complete pairs. All five NIST fields then usedM63,T16:0,1,the previously frozen diagonal target and13source-independent SHA512targets. All30cold children (2arms,3reps,5fields) completed with identical target-indexed original root sets. Each batch has one supported repeated-source pair, all other target sets empty; retain this low-output calibration scope rather than imply a positive dense workload. The portable replay independently enumerates all original source pairs for every actual target, establishing complete original-root equality without relying on the cached/discriminant solver.

| Field | Direct cold median | Cached cold median | Scoped component gain |
| --- | ---: | ---: | ---: |
| P192 |0.18156s|0.05282s|3.44x|
| P224 |0.73568s|0.14136s|5.20x|
| P256 |0.27323s|0.07221s|3.78x|
| P384 |0.65279s|0.14671s|4.45x|
| P521 |1.37983s|0.23139s|5.96x|

The actual square-root-call tally is1,022direct versus78cached, with1,024source/target pairs. The two diagonal source-target coincidences use exact linear fibers, not quadratic square roots. Both arms record one denominator inversion. Their counters omit the one common setup inverse ofdelta; it is still included in cold time. No zero denominator or nonsquare/twist branch is dropped.

## Memory, boundaries and decision

Conic/source cold child wait4 RSS was12,800–12,928KiB. Cross-target cold children were12,416KiB. Outer lab process peaks were100,864KiB (oracle),99,840KiB (sum comparison),111,748KiB (batch controller); these are process high-water/inherited accounting, not the summed tree or a method-isolated allocation. Streams and observers are saved before assertions. All limits are per-process inherited address space. Host isolation is unqualified; three repetitions do not supply an A/A isolation record.

Review scope: the batch API assumes canonical source intervalM<p; frozen cases satisfy it, but its guard currently checks onlyM<=1023. Add an explicitM<p rejection in a fresh immutable variant before exposing it as a generic API. Both methods still use generic Tonelli-Shanks; the next strong comparison must share optimized p=3mod4 square roots and precomputed P224constants. These straightforward arithmetic improvements may shrink the present ratio. No novelty clearance is claimed: conic coordinates,discriminant caching and batch inversion are established. The new evidence is scoped exact implementation, a retained refutation and cold measurements across all five primes.

Next: cached chord/tangent recovery that avoids repeated unused quadratic coefficient construction; equal optimized root contexts on all arms; target-count/source-size crossover; independently verified binary sparse-block whole-chain propagation. The full339-variableGF83solve,odd-extension/binary/Koblitzhardpanel,genuine fullmatchingF4/F5/F6,20preregistered hardinstances,qualifiedisolation/A/A and50x end-to-end remain open.

Replay: PYTHONDONTWRITEBYTECODE=1 python research/autolab_cycle_20261116_v95/REPLAY.py
