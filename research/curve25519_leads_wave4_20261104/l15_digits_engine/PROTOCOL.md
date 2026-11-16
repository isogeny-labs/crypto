# Protocol: L15, engine confirmation of the digit-set frames (frozen 2026-11-04)

Frozen in git before any data. Parent: L14 (`../l14_digit_set_frames`,
validated pass_with_errata): a greedy digit set covers 522 points on 22
columns at t = 5 and 1,268 on 43 at t = 6 at k = 20, against 248 (24
columns, L = 815271) and 570 (48 columns, L = 889850) for the best of 2,000
translation frames. Engine support: `Membership::Digits` and the f4cost
form `digits@LO:c1;…;ct` (commit `b847203692`; two-power digits reproduce
the published `bits` cells 11/42 and 24/198 exactly).

## Question

Does the digit set keep the F4 system of `bits` (same solving degree,
terms and per-target cost) while delivering its census coverage, so that
the F4 work per relation falls by the coverage ratio? And where does it
land against the isogeny winner?

## Cells (k = 20, 100 targets, target seed 2026111500 + t, cap 3600 s, m = 2, full)

| cell | form | t | G |
|:--|:--|--:|--:|
| ctl_bits_lo0_t5_g3 | `bits` | 5 | 3 |
| trans_t5_g3 | `bits@815271` | 5 | 3 |
| trans_t5_g4 | `bits@815271` | 5 | 4 |
| digits_t5_g3 | `digits@113971:289589;761384;273167;963514;602728` | 5 | 3 |
| digits_t5_g4 | same | 5 | 4 |
| trans_t6_g5 | `bits@889850` | 6 | 5 |
| digits_t6_g5 | `digits@51134:293482;1015139;374525;914144;700453;505371` | 6 | 5 |

Binary: `/home/user/ctarget-l15/release/curve25519_family` (hash recorded
at launch; built from `b847203692`).

## Predictions (decided from the JSON outputs)

- **D1 (coverage = census).** `columns` / `covered_points`: digits t = 5
  22 / 522; digits t = 6 43 / 1,268; trans t = 5 24 / 248; trans t = 6
  48 / 570. Exact equality in all four, else failed.
- **D2 (same system).** At each (t, G) pair (5,3), (5,4), (6,5):
  `solving_degree_max` and `poly_terms_per_specialisation` equal between
  digits and the translation frame, `undecided` = 0 and `capped` = false
  in both, and 0.95 ≤ mpt(digits)/mpt(trans) ≤ 1.05. All three pairs, else
  failed at the named pair.
- **D3 (per-relation gain).** F4 mul-adds per relation (mpt ÷
  `coverage_exact`) digits ÷ trans ≤ 0.6 at (5,3) and at (6,5). Points
  248/522 = 0.475 and 570/1,268 = 0.450.
- **D4 (does not overtake the isogeny winner).** Modelled F4 work per
  solve E = columns × mpt / covered (r cancels) for digits at (5,4)
  against the T12 winner's 22 × 52,816 / 210 = 5,533 is ≥ 2 (the digit
  set stays at least 2× behind). Reported as a ratio either way.
- **D5 (reproduction).** ctl_bits_lo0_t5_g3 reproduces the published
  3,036,911.76 mul-adds per target within 1e-6 relative, columns 11,
  covered 42.

## Costs and scope

- The greedy search cost for these digit sets is L14's: 62,000 Legendre
  symbols and 31,034 [r]P labels at t = 5; 126,000 and 62,902 at t = 6
  (rep 0). Per-curve precomputation, reported charged and amortised in the
  same way as the frame searches.
- Engine-only, counted F4 mul-adds; no timing; no end-to-end claim. The
  bits-form arm has no ecbench factor base for digit sets yet
  (`prime-interval` covers intervals only), so no S is reported.
- Rho remains ahead; L10 fixes the exponent. Failed predictions stay
  failed.
