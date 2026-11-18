# P1 instrument: design decisions (2026-11-17)

All numbers here are `[pcc:unverified]`.

## Why not a fresh crate

`genus_ladder` (commit `fffa1b1243`) is the lab's newest verified instrument for
genus 2-5. P1 patches that file instead of re-implementing it, so the genus 2-5
path stays count-identical. Its fixed sizes were the only hard genus limit. The
patch is `instrument.patch`, a diff against `fffa1b1243` under `src/`:

| Change | Effect at g <= 5 |
|---|---|
| `MAX_GENUS` 5 -> 16, `CAP` 24 -> 4g+8, `REL_FB` 5 -> 16 | none (array capacity only) |
| `count_points_ext(k)` for any k: enumerates F_{p^k} with a squares bitmap and forward differences | unused (g <= 5 keeps K = 2) |
| `jacobian_order` at g >= 6: K enumerated point counts from a cost model; an ambiguous window retries with K+1, up to K = g (exact L(1)) | unused |
| Arm `D` (`D_deg2_full`): every degree-1 and degree-2 place in the base | new arm |
| Arm `E` (`E_deg2_lp`): degree-1 2LP base (same `nb` as arm `c`); degree-1 places outside it and every degree-2 place are large primes, at most two per relation | new arm |
| CLI: letters `D`, `E`; `GL_VERIFY_RELATIONS` env re-checks each relation by group arithmetic; `GL_KNOWN` forces K | none unless set |

## The group-order problem, and the choice made

Full DLP verification needs `#J = h r`. The lab code computed it from N_1, N_2
and baby-step giant-step over the Weil window, which is infeasible from genus 6
upward. **Choice:** compute N_k by enumerating F_{p^k} for k up to K, then run
baby-step giant-step on the remaining window. Every admitted order is still
checked on 32 random divisors, and every IC and rho solve is certified by
`[t]G = T` on a planted log. Cost-only measurement (LA modulo a random prime)
was rejected for P1: no certificate.

The price is that #J must fit u64 (`(sqrt p + 1)^{2g} < 2^63`) and the field
must be small enough to enumerate. Upper limits on p: about 421 at g=6, 131 at
g=7, 53 at g=8, 29 at g=9, 17 at g=10 and 7 at g=12.

## What this implies, stated before the data

At those p, `g` is not small against `p`. A degree-1 base has about p/2 columns.
A degree-g divisor splits into distinct degree-1 places with probability about
`C(n, g) 2^g / p^g`, where `n ~ p/2`. This is below the textbook `1/g!` (a
falling-factorial penalty), and it is zero once `g > n`. So in the verifiable
regime, degree-1 index calculus is at its worst. That is exactly where
degree-2 places should help most.

Two consequences:

1. The arm `c` baseline at g >= 6 is the lab's own algorithm, but on these
   small p it is a weak baseline. Ratios D/c and E/c measure the speed-up over
   the lab's method in this regime. They do not measure the speed-up at
   cryptographic p, where the falling-factorial penalty vanishes. They should
   shrink as p grows at fixed g. Measuring that trend is a P1 deliverable.
2. The honest headline per cell is each arm's distance to the matched
   negation-map rho on the same instance, not the ratio to `c`.

## Gates (piloted before freezing)

- (a) Count identity at genus 4: the P0 cell reproduced field for field by the
  patched binary (`gates/p0_identity_g16b.jsonl`).
- (b) Relation verification: `GL_VERIFY_RELATIONS=1` at g=3, p=101, arms A, c,
  D, E, two curves and two seeds. Result: 0 bad relations, all certified
  (`gates/verify_g3_p101_ACDE.jsonl`).
- (c) Order cross-check: K=2 and K=3 give identical #J on 4 genus-3 curves at
  p=211. At g=5, p=23, K=4 and K=5 agree.
- (d) Every reported solve is certified (`certified: true`).
