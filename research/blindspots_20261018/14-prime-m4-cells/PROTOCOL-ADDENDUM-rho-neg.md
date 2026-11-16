# Item 14 protocol, addendum B: a negation-map matched rho (`rho-neg`)

Written 2026-10-22, before any measured `rho-neg` run. It adds one reference arm
and its continuity controls. It changes no existing cell, arm, counter, rule or
threshold of `PROTOCOL.md`, and no recorded row.

## Why

An independent validator found that the matched rho of this item is not the
strongest rho available on these curves, so the validation gate
(`docs/VALIDATION_GATE.md`, `scripts/validation_gate.py::check_rho_reference`)
refuses it (`strongest_available` is false). The finding is in
`benchmarks/20261018-blindspots-prime-m4-cells/rho-reference.json` and
`validator-report.json`. The recorded `rho` (`rho_run` in
`research/blindspots_20261018/instruments/blindspots_14_prime_m4.rs`) is a plain
16-jump r-adding walk `P -> P + T_j` with the jump index taken from a hash of
`x(P)`. `P` and `-P` diverge after one step, so the walk does not use the
negation map. The x-keyed distinguished-point table only catches `+-` matches
between stored points. Every cell has `a, b != 0` (so `j != 0, 1728` and
`Aut(E) = {+-1}`) and `E(F_p)` has trivial Frobenius. Negation is therefore the
strongest symmetry available, and the matching reference is Pollard rho with the
negation map, expected near `sqrt(pi n / 4)` walk steps instead of
`sqrt(pi n / 2)`.

## The instrument

`research/blindspots_20261018/instruments/blindspots_14_prime_m4_neg.rs` is a
copy of `blindspots_14_prime_m4.rs`. The original stays untouched, because the
recorded cards and replay cards pin its sha256
`f6696fc0cafff9120189072a7785452422d68c7944a2812d4667e27c8872b376`. The copy has
these changes, and nothing else:

1. **Three new counters** in the free slots 12 to 14: `canon`, `lookahead` and
   `cyctest`, defined below. `counts_json` prints the slots from 12 up only when
   they are nonzero. Before, it skipped the zero-valued unused names `x12..x15`.
   The rows of every old subcommand are therefore byte-identical. `native_total`
   adds the three counters. They are zero in every old subcommand, so the old
   native totals do not change.
2. **One new subcommand, `rho-neg`** (`rho_neg_run`, `neg_step`, `canon`,
   `cmd_rho_neg`).
3. **More `selftest` checks for `rho-neg`**, appended after the existing rho
   checks. The existing checks are unchanged and run in the same order.
4. The file header describes the variant and lists the new subcommand.

`rustfmt --check` and `clippy-driver` report no warnings on the new file, as on
the original.

## The `rho-neg` algorithm (exact)

Everything not listed here is the code of `rho_run`, unchanged: the same counted
instance setup outside the window, the same `make_jumps` (16 jumps
`T_j = alpha_j G + beta_j Q`, four random bits per scalar, built from the counted
doubling chains of `G` and `Q`), the same RNG draw order, the same random start
`aG + bQ` counted in `setup_gadd`, the same fresh-start chain (two random jumps
added to the chain point per walk), the same partition `part(x)`, the same
`theta = max(bits/4, 2)`, the same distinguished-point rule `x mod 2^theta = 0`,
the same x-keyed table with the same `+-` collision algebra, the same cap of
`20 * 2^theta + 64` steps per walk, and the same verification
`cv.mul(cand, G) == Q` inside the counted window.

* **Classes.** The walk runs on classes `{P, -P}`. The representative is the
  point with the smaller ordinate (`y <= p - y`). `canon(P)` does one `F_p`
  negation and one comparison, and is counted once as `canon` (native). When it
  negates, it negates the coefficients `(a, b)` mod `n`. The walk start and every
  candidate point are canonicalised.
* **Step with look-ahead** (Bos, Kleinjung and Lenstra, ANTS 2010). From a
  canonical `w`, take `j = part(x(w))` and `C = canon(w + T_j)`, one counted
  addition. If `canon` negated and `part(x(C)) = j`, the next step would return
  to `w`, a fruitless 2-cycle. That partition test is counted as `lookahead`
  (native, done only on negated candidates). The walk then refuses `C` and tries
  `j+1, j+2, ...` mod 16. Every refused candidate's addition stays counted in
  `gadd`. If all 16 are refused, the step is `canon(2w)`, one counted doubling.
  The rule depends only on the class of `w`, so the walk is a function on
  classes.
* **Fruitless-cycle detection.** Look-ahead does not catch cycles of length 4,
  6 and so on, or the rare 2-cycles with `2w = T_k - T_j`. Inside each walk,
  Brent's check compares every new class with a saved class. The saved class is
  replaced at distances 1, 2, 4, and so on. Each comparison is counted once as
  `cyctest` (native). A walk can revisit a class only inside a cycle that holds
  no distinguished point.
* **Escape.** On a repeat, if the coefficients differ, the walk first tries the
  self-collision as a solution (same algebra and the same counted verification).
  It then scans the cycle once (`len - 1` counted transitions and `cyctest`
  comparisons), takes the smallest class in it by `(x, y)`, and continues from
  `canon(2 * min)` with one counted doubling. Every walk that falls into the same
  cycle therefore leaves it at the same class. The escape point is
  distinguished-point-tested like any step. Escapes do not count towards the
  walk-length cap.
* **Counting.** `gadd` holds every addition: setup, starts, accepted steps,
  refused look-ahead candidates, cycle scans, escape doublings and verification.
  `dptest` holds one test per transition, steps and escapes alike. `lookup` and
  `insert` hold the table operations, as in `rho`. The native total is
  `lookup + insert + dptest + canon + lookahead + cyctest`. The other native
  counters are zero.
* **Row.** `"kind":"rho-neg"` has the fields of a `rho` row (`bits`, `run`,
  `ok`, `setup_gadd`, `walks`, `counts`, `gae` = `counts.gadd`, `native`, `n`,
  `sqrt_pi_n_over_2`, `ms`). It also has `sqrt_pi_n_over_4`, `lookahead`,
  `steps` (accepted transitions), `la_rejects`, `la_exhausted`,
  `fruitless_cycles`, `fruitless_2cycles`, `cycle_len_sum`, `escape_gadd` and
  `abandoned_walks`. The additions behind the diagnostic fields are already in
  `gadd`. The diagnostics only break them out and are never added a second time.
* **Seeds.** `rho-neg --bits B --runs 16` keeps the `--seed 20261018` base but
  draws from streams distinct from `rho`. The target is
  `d = 1 + Rng(seed ^ (B << 32) ^ 0x4E47 ^ run * 0x9E3779B9).below(n - 1)`, where
  `rho` uses tag `0x4410`. The walk seed is `seed + run * 104729 + 0x4E4547`,
  where `rho` uses `+ 3`. The offset is not a multiple of 104729, so no `rho-neg`
  walk seed equals any `rho` walk seed.

## Measured in the same binary and process family as arm (a)

One card builds the new file once with the command the item's cards use:
`prlimit --as=4294967296 rustc --edition 2021 -C opt-level=3 -C codegen-units=1
-C target-cpu=native -o <bin> research/blindspots_20261018/instruments/blindspots_14_prime_m4_neg.rs`.
The card records the source and binary sha256 in its receipt. From that one
binary, as separate processes, it runs:

1. `selftest --deep`;
2. the continuity controls (below) at the four design cells;
3. `rho-neg --bits B --runs 16` for `B = 16, 20, 24, 28`.

These are the curves frozen in `CELLS` (`find-curve --seed 20261018`), the same
ones as the recorded cells. The card writes outputs under
`research/blindspots_20261018/14-prime-m4-cells/raw/rho-neg/`, and never over a
recorded file.

## Continuity controls (must hold, or the `rho-neg` rows are not used)

At every design cell, in the same binary:

* `rho --bits B --runs 16` must reproduce `raw/rho-bB.jsonl`;
* `fullic --bits B --arm a --runs 16` (default `b_x`) must reproduce
  `raw/fullic-a-bB.jsonl`.

Reproduce means the rows are identical field by field, except `ms`. A mismatch
stops the comparison and is reported as an instrument defect. Arm (a) and plain
`rho` then share a binary with `rho-neg`, and arm (a)'s recorded counts carry
over to the gap.

## Decision rule (gap)

At each cell and each `w in {0, 0.1, 1}` GAE per unpriced native operation
(AGENTS.md section 5a):

* `S_a(w)` = mean over the 16 arm-(a) full-IC runs of `gae_total + w * native_total`
  (the recorded rows, confirmed by the continuity control);
* `rho_neg(w)` = mean over the 16 `rho-neg` runs of `gae + w * native`;
* **gap** = `S_a(w) / rho_neg(w)`, reported as the distance remaining and never
  as pass or fail.

The order of arm (a) and `rho-neg` is stated only where it holds at all three
weights. Otherwise it is "unresolved at the boundary". The matched reference for
the gate is `rho-neg` (`kind: negation`). Plain `rho` is still reported beside
it, priced at the same three weights.

Pricing note, fixed now. `canon`, `lookahead` and `cyctest` are priced as native
operations, like `dptest`. That works against `rho-neg`: at `w = 1` an `F_p`
negation and comparison is charged as a full group addition. At high `w`,
`rho-neg`'s priced cost can therefore exceed plain `rho`'s, although it uses
about `1/sqrt(2)` as many group additions. Where that happens, the row reports
it as measured, and a statement of order against "matched rho" must hold
against both `rho-neg` and plain `rho`. The counters are in every row, so a
reader can re-price them.

**Expectations (pre-run, from `selftest` only, not data).** `selftest --deep`
on the build at the commit of this addendum gave these mean accepted steps:

| bits | runs | mean steps / `sqrt(pi n / 4)` |
|--|--|--|
| 16 | 400 | 1.062 |
| 20 | 200 | 1.010 |
| 24 | 40 | 1.042 |

Plain-over-negation online GAE was 1.401, 1.483 and 1.331 (`sqrt 2 = 1.414`).
At 16 bits the 400 runs found 140 fruitless cycles (4 of length 2), with 574
escape GAE in total. With look-ahead off (positive control, 50 runs), 289
2-cycles were detected and escaped, and all 50 logarithms were recovered. The
gap is expected to shrink by about `sqrt 2` at `w = 0` and to stay above 1 at
every cell and weight, because arm (a) was 6 to 31 times plain rho.

## What is NOT changed

* `blindspots_14_prime_m4.rs`, its sha256, and every recorded and replay card,
  receipt, raw row, certificate, figure and report;
* arm (a), arm (b), arm (c) and arm (d), their cells, bases, targets, seeds,
  caps and verdict rules (`PROTOCOL.md` Decision rules 1 to 6);
* the unit (GAE), the existing native set and the pricing band;
* arm (b)'s full-IC rows. They ran in separate builds (cards 09 to 12), so they
  stay outside this reference's `same_binary_or_process` scope.
