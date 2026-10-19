# Fresh-target worker confirmation

The completed r5 physical-host panel measured all 384 planned pairs, but
one of 256 prespecified A/A controls exceeded its per-pair limit. Its
qualified speedup is unknown. This plan uses a new public target set and a
prospectively frozen aggregate A/A criterion. The new criterion applies
only to a new run; it does not reclassify r5.

## Frozen target construction

The 32-byte seed is
`d8bea711f2893203fc0bde0963f109f698070cd7e644890be1e4b558a7d818c5`.
It is SHA-256 of the byte string
`rho-worker-tier0-confirmation-v1|` followed by the 32 decoded bytes of
the earlier target-manifest SHA-256
`f46ad43788b7f9241927904541cbeb03ea70a96ca95aa84d527a9b22ba8d12b8`.
The existing public fixture generator maps this seed to 32 secp256k1 Q
points and 128 `(a,b)` pairs per Q. The new
[fixture manifest](confirmation-fixtures/manifest.json) is SHA-256
`546fe0b842a2f559395c9ac5d74ced12b96ce366e41322387eb0baec5dcb3adc`.
The [independent affine verifier](confirmation-fixtures/verification.json)
checked all 32 Q points and all 4,096 expected `aG+bQ` outputs.
No target was selected using a timing result.

## Frozen comparison to prepare

Use the existing exact secp256k1 job and walk policy: 128 walkers, 12
distinguished-point bits, an 8,192-step cap, check-ins every 64 walkers,
serial width 1 and batch width 32. Run one serial/batch comparison, one
serial/serial control, and one batch/batch control for each of the 32 new
Q points. Repeat the 96-case sequence four times with alternating arm
order: 768 arms, 384 same-point pairs, 128 comparisons, and 256 A/A
controls. Charge Q-dependent setup and walking through
`total_charged_ns`; retain output replay outside that timed interval.
Use a single serialized queue on a dedicated physical Linux host with
exclusive core and SMT sibling, NUMA memory, IRQ routing, fixed frequency,
and the same per-arm noise and memory receipts as r5. Freeze source,
binary, plan, manifest, host allocation, and all limits before timing.
Run a separate one-pair calibration outside the panel.

The predeclared confirmation gate requires:

1. All 768 arms and 384 pairs complete with exact point and distinguished
   point replay, stable source hashes, and valid strict-runner host
   preflight and postflight. Every arm satisfies the r5 per-source IRQ,
   pressure, involuntary-switch, fault, steal, throttle, frequency, and
   NUMA bounds. Any invalid arm or pair makes the aggregate unknown.
2. For the 256 A/A controls, no individual ratio lies outside
   `[0.80, 1.25]`; at most two ratios lie outside `[0.90, 1.11]`.
   For serial/serial and batch/batch separately, the median log ratio
   exponentiated lies in `[0.98, 1.02]`. For each Q and each A/A mode,
   the median log ratio over its four repetitions exponentiated lies
   in `[0.95, 1.05]`. Preserve and report every outlier. These aggregate
   controls allow rare timing excursions while bounding systematic
   drift; they were specified after r5 and must be tested on these
   fresh targets.
3. For every comparison pair, peak batch RSS is at most 1.10 times
   serial RSS and at most 64 MiB higher. A missing RSS value fails
   the gate.
4. Compute each Q's median of four log(serial charged time / batch
   charged time) values. Bootstrap the 32 Q clusters with 10,000 draws
   and a fixed seed, then exponentiate. Enable default secp256k1 batch
   routing only if the 95% lower bound exceeds 1.05 and all preceding
   gates pass. Keep serial routing for other curves and as an explicit
   override.

The new source bundle, confirmation runner/auditor implementation and
its bootstrap seed, allocation receipt, binary hash, and concrete
dispatch manifest are not yet frozen. Their hashes must be committed
before a timed confirmation begins. AWS quota for another dedicated
host remains pending; do not dispatch on the busy existing research host.
