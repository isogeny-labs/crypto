# Closeout: PKM / structured factor bases for prime-field ECDLP index calculus

Lead: PKM / structured (Kummer-coset, smooth-isogeny) factor bases for prime-field ECDLP index calculus.

Status: **closed (negative)**, 2026-11-19. Five studies, all measured in counted field or group operations.

Mirror: no lead atlas is checked out in this repository. This note should also be mirrored into
`knowledge/leads/ecdlp/` of crypto-autoresearcher, where the canonical atlas lives.

Every result below is quoted from the study files and is [pcc:unverified]: this note does not re-run or re-prove
anything. Nothing here is at cryptographic size, and no matched rho was measured, so none of it is a leaderboard row.

## The idea

Take the factor base `V` to be a multiplicative coset of abscissae (`g·mu_S`, a Kummer tower of degree-2 maps) or the
kernel of a smooth isogeny, so that "x is in V" is a low-degree system and a Groebner or F4 oracle can find
decompositions faster than enumerating `V` (the hypothesis) [pcc:unverified].

## What was tried

- EXP-PKM-ODD-001 (`research/pkm_odd_tower_f4_20261114/`, `results.json`, `REPORT.typ`): odd, mixed and isogeny
  towers under F4, against enumeration. 262 rows agree with exhaustive search. Degree 3 and 5 towers change the
  constant (r = 0.24 to 0.30 at `S` = 27 to 243), not the slope (F4 work grows as about `S^3.1` to `S^3.6`). Isogeny
  form is 0.76 to 1.47 times the Kummer form. The algebraic relation oracle costs 2e2 to 1.2e7 times enumeration, and
  5e7 times at `m` = 3, `S` = 32 [pcc:unverified].
- EXP-PKM-IDEAS-001 (`research/pkm_ideas_20261115/`, `results.json`, `REPORT.typ`): is any smooth structure cheaper
  than a random `V`? Dense membership `prod(x - v)` makes F4 cheaper than the tower (dense over tower 0.052 at
  `S` = 16 down to 9.8e-4 at `S` = 256, `m` = 2), and a random `V` costs the same as the coset. Trace replay saves
  only 0.38 to 0.91 of fresh work, missing the 0.5 threshold in 5 of 11 cells [pcc:unverified].
- EXP-PKM-DENSE-DEC-001 (`research/pkm_dense_decompose_20261115/`, `results.json`, `results_large.json`,
  `REPORT.typ`): finding decompositions, not just refuting. Dense beats tower (0.050 to 0.00099 at `m` = 2), yet
  dense over enumeration is 11 to 181 at `S` = 16 to 256, then 203 to 3.2e3 at `S` = 256 to 4096. Dense growth is
  `S^2.0` at `m` = 2 and `S^4.9` at `m` = 3, against enumeration's `S^1.03` and `S^2`. Cells at `S` = 8192 and above
  hit the 1,800 s budget [pcc:unverified].
- EXP-PKM-RESULTANT-001 (`research/pkm_resultant_oracle_20261119/`, `results.json`, `results_amortised.json`,
  `REPORT.typ`): a quasi-linear oracle at `m` = 2 (resultant plus remainder tree). It grows as `S^1.38`, about
  `S log^2 S`, and is 30 to 120 times cheaper than dense F4. It still costs more than enumeration from `S` = 32: 55.7
  times at `S` = 2^16, or 52.9 times with the `V`-only work amortised over targets [pcc:unverified].
- EXP-PKM-IC-001 (`research/pkm_ic_factor_base_20261119/`, `results.json`, `REPORT.typ`): Kummer, random and
  small-`x` factor bases inside the repository's actual `m` = 2 index calculus on three 20, 24 and 28-bit curves. All
  108 solves are correct. Kummer over small-`x` charged operations are 0.99 to 1.08. The whole solve costs 114 to
  131 (20-bit), 184 to 282 (24-bit) and 665 to 1,308 (28-bit) times the theoretical rho cost `sqrt(pi n / 2)`
  [pcc:unverified].

## Why it failed

- The structure of `V` bought nothing that any oracle could use: coset, random and small-`x` bases cost the same in
  every dense oracle (ratio 1.00 to 1.04, one `m` = 3 cell excepted) and in the whole solve (within 8%)
  [pcc:unverified].
- The tower formulation spends variables to save degree, so it is the most expensive form; once membership is dense
  the smooth description confers no edge over a random set.
- The only superlinear-to-quasi-linear step (resultant) removed F4's polynomial gap but not the `log^2 S` factor or
  the constant, so enumeration of `V` stayed cheaper at every measured size.
- A multiplicative coset has no additive structure that the group law sees, so the sums `+-F_i +- F_j` spread over
  the group like those of a random set [pcc:unverified].

## The cost argument

Derivation, [pcc:unverified]; not a measurement. With a table or collision decomposition, prime-field index
calculus costs about `p/S + S^2` group operations for `m` = 2, 3 and 4. The three shapes are: a pair table; a table
plus a scan over `V`; two tables meeting in the middle. The `p/S` term is the trials, one scalar multiplication per
decomposition attempt. The `S^2` term is building the table of sums. The total is minimised near `S = p^(1/3)` at
about `p^(2/3)`, against rho's `sqrt(p)`. The exponent 2/3 exceeds 1/2, so the gap grows with `p`; EXP-PKM-IC-001
sees the same trend at 20 to 28 bits.

So beating rho needs a decomposition oracle whose per-target cost is sublinear in `S`. Every algebraic oracle
measured here was superlinear in `S`: tower `S^3.5`, dense `S^2.0` at `m` = 2, resultant `S^1.38`, against
enumeration's `S^1.0` [pcc:unverified].

## What would reopen it

- A measured decomposition oracle with per-target cost `o(S)` at `m >= 2`, charged in the same counted unit as the
  enumeration it replaces, with the `V`-only setup priced.
- A structure that changes the distribution of the sums `+-F_i +- F_j` (additive structure the group law can use),
  so a trial succeeds with probability above `S^2/p`.
- A change of group (hyperelliptic or other) in which the same factor base is charged in full and the exponent
  differs; this is a different lead.

A constant-factor gain from F4 trace replay or odd-degree towers does not reopen it: those moved constants and left
the slope and the comparison with enumeration unchanged.

## Sources

- `research/pkm_odd_tower_f4_20261114/` (`PROTOCOL.md`, `results.json`, `REPORT.typ`)
- `research/pkm_ideas_20261115/` (`PROTOCOL.md`, `results.json`, `REPORT.typ`)
- `research/pkm_dense_decompose_20261115/` (`results.json`, `results_large.json`, `REPORT.typ`)
- `research/pkm_resultant_oracle_20261119/` (`results.json`, `results_amortised.json`, `REPORT.typ`)
- `research/pkm_ic_factor_base_20261119/` (`results.json`, `REPORT.typ`)
- Sibling-repository lead atlas (not checked out): crypto-autoresearcher `knowledge/leads/ecdlp/`.
- Knowledge base (`research-kb`) was not queried for this note.
