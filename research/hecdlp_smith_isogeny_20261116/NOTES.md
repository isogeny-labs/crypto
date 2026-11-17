# Smith isogeny as a DLP transfer (SMOKE notes, not a verdict)

Study dir `research/hecdlp_smith_isogeny_20261116/`. Module
`src/cryptanalysis/smith_isogeny.rs`. All statements are `[pcc:unverified]`
assertions of this session; toy primes, no cost claim, no leaderboard row.

## Finding that changes the brief

`research/genus_ladder_20261114/RESULTS.md` section 3 says the explicit
isogeny was never built. It has been since study `smith_canonical_20261115`:
`smith_trigonal.rs` (Recillas' curve X, correspondence R),
`smith_canonical.rs` (`CanonicalMap` psi: X -> C as a linear map of the
Veronese point; `Transfer::transfer_divisor` = Psi on Mumford classes of J_H;
`CountedTransfer`), with Example 1 (F_37) and a 70-curve sweep in
`research/smith_canonical_20261115/RESULTS.md`, including a planted DLP solved
by BSGS on C. I therefore did not rebuild the isogeny. I read those modules
and built what was missing: the link to the solver instrument.

## What smith_isogeny.rs adds

* `SmithIso` (curve, HJac, Transfer, usable points, `#J_H` from N1..N3).
* `check_homomorphism`: Psi(D1+D2) = Psi(D1)+Psi(D2) on random classes,
  `[#J_H] Psi(D) = 0`, Psi(D)=0 iff rational 2-torsion D lies in S.
* `planted`: G of prime order r = largest prime factor of `#J_H` (odd, so Psi
  is injective on <G>), T = [t]G computed on H, G and T transferred
  independently, packaged by `quartic_ladder::make_instance_from_classes`
  (which checks T = [t]G and [r]G = 0 on C), `t_secret` cleared to 0.
* `certify`: a solver's t' is accepted only if [t']G = T on J_H (Cantor).
* `find_admitted`: smoke search for Smith-positive curves with `#J = h r`.

## Tests

`ex1_order_and_homomorphism`, `ex1_planted_dlp_all_solvers` (Example 1,
#J = 55666 = 26 * 2141; Diem Full, Diem 2LP and negation rho each recover t;
certified on H; wrong t' and None rejected), `smoke_transfer` (ignored).
Results: see the bottom section.

## Sources and unverified details

Smith, J. Cryptology 22 (2009) / arXiv:0806.2995 (construction, Example 1);
Diem, "On the discrete logarithm problem in class groups of curves" (Diem IC
on plane quartics, via `quartic_ladder`). Not verified here: Smith's statement
that ker Psi has order 8 is only checked through the rational 2-torsion and
order ratios in the earlier study; Psi is only evaluated on classes reachable
as sums of rational points over U (brute-force, toy p); `deg F = 8` has no
HJac; no research-kb or conductor check was done (task constraints: no git,
no shared-state tools).

## Open issues / plan for the rest

1. Counted end-to-end cost: price `CountedTransfer` plus Diem on C against
   hyperelliptic IC/rho under the 0 / 0.1 / 1 GAE band (AGENTS.md 5a) at
   p >= 1031 (no brute force). Needs the near-prime-census lead from
   RESULTS.md section 3 (only 1/28 admitted instances Smith-positive).
2. Preregistered protocol before any cost claim.

## Smoke results (release build, 2026-11-16; SMOKE, one seed per prime, not a verdict)

Example 1 (p = 37, #J = 55666 = 26 * 2141): homomorphism 12/12 additivity,
12/12 `[#J]Psi(D) = 0`, 1/1 rational 2-torsion vs S, 12/12 nonzero images.
Planted DLP (r = 2141): Diem Full recovers t (70 Pic ops), negation rho
recovers t; both certified on J_H. Diem 2LP did NOT recover at q = 37 (base of
19 of ~42 points exhausts); the unit test does not assert 2LP there.

`smoke_transfer` (ignored test): p = 67 (#J = 310328 = 8 * 38791), 101
(1124412 = 12 * 93701), 131 (2090224 = 16 * 130639): homomorphism checks pass,
Diem 2LP and rho both recover t and certify on J_H. Group-op counts
(2LP 121/131/171 Pic ops; rho 634/571/452) are on C and are not a cost
comparison: transfer cost, native-op pricing (5a) and rho matching are absent.
