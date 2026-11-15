# Final bounded product-criterion comparison

Preserve all prior runs. Common baseline and candidate now use an equivalent
final checker that validates every original membership certificate and every
input/S-pair normal form, while omitting unused proof propagation in the
polynomial-only tests. The legacy checker is retained and compared in tests.
This strengthens both arms before their matched measurement; old timings are
not spliced into this comparison.

Candidate engine: initial interreduction plus the standard product criterion
for relatively prime leading monomials. Final verification does not use that
criterion: every pair is explicitly checked. Two ablations: raw inputs, then
certified shared-vertex projections. Neither changes the output ideal.

Same 24 exploratory cases, both original primes, 21 paired samples, seven A/A
controls, 10000 paired bootstrap draws, fixed seed and per-case lower endpoint
>10 criterion. Same one-thread local L0 host/affinity, 30-second worker and
20-minute campaign limits, full charged interval, 2-GiB memory cap, complete
basis and failure rules. No controls run after a failed pair. No incomplete
baseline becomes a numeric win. Interleave sizes/seeds/fields as before.

This is the final bounded engine attempt in this task before reporting the
actual result, including an unmet tenfold target if that is the observation.
Fresh confirmation is required for any exploratory win. External supported
CAS comparisons remain required; no fastest or ECDLP claim follows from a
native component ratio. All observations remain producer-only and
`[pcc:unverified]` until independently signed validation.
