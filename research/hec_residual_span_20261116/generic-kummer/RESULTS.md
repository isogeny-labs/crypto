# Generic Kummer primitive result

The original-base-field arithmetic correctness gate passes; this implementation fails the tenfold cost gate. This is a scoped local experiment, not an independently certified result. [pcc:unverified]

The once-only execution at e3e877638c070f073d1de86fe25d142db1a6925a exited 0. All 3,168 checked steps matched native projective addition and signed-root outputs, including identity, theta-divisor and doubled-point controls. There were 1,556 signed recoveries and 52 uses of non-first charts. Both repetitions of every primitive and setup vector agree. The Rust summary validates 9,504 primitive records, 36 setup records and the REDC counter identity. Ordinary-chain cost ratios exclude the prospectively separate edge controls.

| p | source multiple | native REDC | full candidate REDC | kernel REDC | native / full |
|---|---:|---:|---:|---:|---:|
|101|1|29400|50830|21158|0.578399|
|101|17|63470|64340|25870|0.986478|
|251|1|29486|55108|21398|0.535058|
|251|17|64498|78568|26002|0.820919|
|601|1|60424|109924|13288|0.549689|
|601|17|93588|137694|25576|0.679681|

These sums use ordinary chains in repetition 0. The full candidate includes differential addition, splitting/root decoding, charged signed recovery on smooth points and signed point extraction. The native arm uses add_place for degree-one sources. The kernel is a nested prefix of the full candidate and must not be added again. Setup is retained separately in SETUP.tsv, not hidden in a cold-solve claim.

Full candidate REDC exceeds native in every group. Some inversion counts fall, especially for source multiple 17, so the REDC ratio alone is not a wall-time model. Nevertheless, even the optimistic native-full / candidate-kernel REDC comparison is only 1.378–4.547. That deliberately mismatched bound gives the rest of the candidate primitive zero cost; it is not a matched addition speedup. Conversely, making the kernel free while preserving this recovery gives at most 1.650. These are conditional bounds for this circuit and stream, not impossibility results for Kummer methods.

VECTOR-BOUNDS.tsv also reports max_j(native_j/candidate_j) across the 16 recorded counters: 1.741–3.306, with no positive native component having a zero candidate denominator. Therefore any common nonnegative weighting of these recorded components, with positive total denominator, has native/candidate ratio below 3.306 for each group. Counts are nested and omit some CPU, memory and integer work; this is not a bound on wall time or unrecorded operations, and it supplies no arbitrary field/scalar exchange rate.

Do not build a full solve around this unchanged circuit: the primitive supplies no tenfold cost case. A different factorization or signed recovery can be screened under a new frozen protocol. Generic Fp arithmetic remains eligible; split fast-theta arithmetic still cannot be silently substituted on these models. No priority, external-fastest, matched-rho or complete-solve claim follows. The genus-2/3/4 mathematical objective remains open.

Reproduction and custody: PROTOCOL.md, SOURCES.md, generator/source/tensor/native/input/binary hashes precede execution; RUN.txt retains its original commit. CONTROL.tsv, SETUP.tsv, stderr, EXIT.txt and OUTPUT.sha256 retain the run. summarize.rs derives SUMMARY.tsv and VECTOR-BOUNDS.tsv without executing the producer again. Git is the durable custody mechanism under the user's S3-outage direction. No cloud resource was started.
