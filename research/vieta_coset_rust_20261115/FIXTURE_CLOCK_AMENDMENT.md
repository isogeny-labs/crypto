# Fixture-clock repair, a new source epoch

Filed 2026-11-15 after the original main attempt, before repaired measurements.
Original raw files, qualifier, manifests and protocols remain unchanged.

The first main attempt observed all 960 primary two-term arm calls and 1140
secondary-panel arm calls. Twelve planned three-term calls were never reached:
the P-224/m128 configuration stopped after retained round 6 when its untimed
fixture generator reused a context whose 120-second clock included previous
benchmarking. This is a harness defect, separate from the twelve genuine Vieta
stage timeouts at P-224/m8192.

The repair creates a fresh fixture context at each three-term round. It changes
no target-generation rule, seed, arm order, relation algorithm, field operation
counter, stage limit or measured interval. A new immutable qualifier and protocol
identify this source epoch. The repaired P-224/m128 configuration repeats all
two warmups and ten retained calls for all arms; it does not fill only missing
rows. Earlier incomplete-epoch results remain in the raw data and flow, separately
labeled and excluded from the repaired comparison. No pooling across epochs.

One repaired configuration is allowed, with the same 120-second per-stage limit
and a 600-second process limit. No retry or timeout enlargement for the original
P-224/m8192 algorithm failures is authorized by this amendment. Their original
bounded observations remain the only observations at that configuration.

The repaired source is tested, copied, hashed and published before calls. The
same unmodified isolated runner reserves CPU4, with default thresholds and
below-L2 clock standing. User-authorized Git-only checkpoint custody continues.
