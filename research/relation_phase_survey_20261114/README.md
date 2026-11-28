# Relation-phase levers: survey and next test (2026-11-14)

Follow-up to the closed cover-image factor-base line (rounds 1–7, `research/cover_image_fb_*`). The question:
which relation-phase lever with measured gains has the most room left? This note only reads existing records;
every number is quoted from them and is [pcc:unverified] unless a claim id is given. Knowledge base: the
`research-kb` CLI is not installed in this container, so sources were read directly (paths below).

## What the records say

| lever | where | result | status |
|---|---|---|---|
| extra-relations retune (64 → 2) + rank-deficiency retry | `research/ic_e222_m221_20261106/RESULTS_extra_relations.md`, `RESULTS_default_flip.md` | trials 0.123–0.148× at fb 8; with fb 32 and the pair-sum table, ~700× vs the old *untuned* CLI defaults | confirm-panel validated; the big factor is mostly removing baseline waste |
| pair-sum table, restarted walk, cofactor projection (h > 1) | `research/ic_e222_m221_20261106/CAMPAIGN_SUMMARY.md` | table 0.26× solver over the best hoisted sweep; cofactor 0.14–0.26× trials (h = 4, 8) | table validated; cofactor not independently checked |
| stride-renew relation targets | PR #2826 (open) | −16 % at 20 bits, −2 to −3 % at 28, null at 24 and 32 | validated |
| `prime_fast` multi-target: table collector, lock-step batching, bucketed pair table | `CAMPAIGN_SUMMARY.md` §A2 | 1.5–2.8× faster; 0.55–0.61×; 0.47–0.68× (same-binary A/B) [pcc:unverified] | validator reports listed there |
| multi-target IC vs rho, counted (v5) | `research/mt_fieldops_v5_20261112/RESULTS.md` (PCC-20261112-22e5d2) | unranked in all 27 cells: dearer at h = 0 (1.06–1.16×), ~tie at h = 0.1, cheaper at h = 1 (0.71–0.80×); flat across 2^24–2^32 | validated as a measurement |
| multi-target IC vs rho, wall time | `research/mt_timed_20261112/RESULTS.md` (PCC-20261112-ec2f3f), `research/mt_timed_runpod_20261112/RESULTS.md` (PCC-20261112-dc58b3) | faster at 2^24–2^28 (0.70–0.99×) [pcc:unverified], slower at 2^32 (1.01–1.29×); a fast hasher narrows but does not reverse it | validated as a measurement |
| multi-target IC vs BSGS | `research/mt_bsgs_gap_20261111/README.md` | 1.11–1.6× everywhere; the gap is structural (random-log table: n/(2K) trials vs BSGS's tiled n/(4Mx)); offset choice recovers only 4–6 % | closed as a negative result |

## Where the room is

1. **Single target**: the IC is 15–528× rho (`RESULTS_prime_fast_vs_rho.md`), growing with size. Nothing above
   changes the exponent; the gains are constants on a losing slope.
2. **Multi-target vs BSGS**: closed (structural).
3. **Multi-target vs rho**: the only place the IC is near or below rho. In *counted* operations the ratio is flat
   with group size, but in *wall time* it climbs from 0.70× (2^24) to 1.18× (2^32). `mt_timed_20261112`
   attributes the climb to hash-table lookups that leave cache as the IC's table grows (0.04–0.15 GAE per lookup
   at 2^10–2^16 entries, 0.27–0.85 at 2^22–2^24). A cheaper hash function was tried; **the table layout and the access
   pattern were not.** The target-phase table in `examples/mt_fieldops.rs` is a std `HashMap<u64, (u32, u16, i8)>`
   probed once per trial, one target at a time.

So the lever with measurable room is the multi-target IC's target-phase memory behaviour at large groups. It is
an implementation-level change to the IC baseline, which the success criterion would count once measured; it
cannot change the BSGS verdict.

## Next test (preregistration draft: [`PROTOCOL.md`](PROTOCOL.md))

Same-binary A/B on the multi-target IC arm of `mt_fieldops`: the current std-HashMap target phase against a
flat, compact table (32-bit x fingerprint + 32-bit slot index, open addressing) probed in lock-step batches
across targets with software prefetch. The decisive cells are 2^32 at T = 4096 and 16384, where the IC currently
loses to rho by 1.11–1.29×.

## Not claimed

No measurement was made for this note. It changes no leaderboard or `benchmarks/` row.
