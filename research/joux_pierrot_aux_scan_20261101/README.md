# Auxiliary-curve literature leads: existence scan and status

2026-11-01 · follow-up to the auxiliary-curve open items 1–7 (Conductor
T-5750) · scan by the script below (Python, no dependencies)

Three literature leads name auxiliary curves for index calculus. This note
records, for each, what the programme already holds and one decisive fact
established here. Class: literature/status note, no run priced.

## 1. Joux–Pierrot elliptic bases (`literature/ffdlp_transfer.json`, lever 6)

The suggested route (`research/ic_mega_pass_20261031/literature/ffdlp_transfer.json`):
a base in `F_{2^n}` defined by translation-by-Q structure on an auxiliary
ordinary curve `E'` with a point `Q` of order `n` — the Couveignes–Lercier
elliptic-period representation — with factor base = points of the target
curve whose `x`-coordinate is degree ≤ 1 in this basis; measure relation
yield and the Boolean fall degree of the descended `S_3` against a random
subspace.

**Existence scan (this note).** `#E_t(F_{2^k}) = 2^k + 1 − a_k` for the four
ordinary isogeny classes of `F_2` (`t = ±1, ±2`, `a_k = t·a_{k−1} − 2·a_{k−2}`)
was tested for divisibility by the toy primes and `n = 131` (`scan.txt`):

| n | first k with n \| #E(F_{2^k}) (t) | n = 131 |
|---|---|---|
| 13 | k = 3 (t = +2), #E = 13 | never, k ≤ 20 |
| 17 | k = 16 (t = ±2) | — |
| 19 | none, k ≤ 20 | — |
| 23 | k = 11 (t = −1) | — |
| 29 | k = 7 (t = −2, −1) | — |
| 31 | none, k ≤ 20 | — |

Facts and corrections to the suggested route:

- The premise as written ("`E'/F_2` or `F_4` with a point of order n") fails
  for every listed `n`: Hasse gives `#E(F_2) ≤ 7` and `#E(F_4) ≤ 9`. The toy
  must use `E'` over a larger base (`F_{2^4}`, say) or accept the order-`n`
  point over an extension; then the represented field is
  `F_{(2^k)·n}`, not `F_{2^n}` (for `n = 13`, `t = +2`: `F_{2^39}`).
- `n = 19` and `n = 31` do not divide `#E(F_{2^k})` of any `F_2`-descended
  ordinary class for `k ≤ 20`; the toy list should drop them or extend the
  base-field scan.
- **ECC2K-130 (`n = 131`): no `F_2`-descended ordinary class has
  `131 | #E(F_{2^k})` for `k ≤ 20`.** If (to verify against Joux–Pierrot §3)
  the order-`n` point must be rational over the representation's base field,
  then at prime `n = 131` the elliptic-period base does not exist over
  `F_2` (Hasse: `#E'(F_2) ≤ 7`) and exists over `F_{2^k}` only with the
  represented field `F_{(2^k)·131}`. Since 131 is prime there is no
  factorisation that lands in `F_{2^131}` — the route would be closed at
  ECC2K-130 by rationality alone, independent of yield.

Next step (cheap): verify the rationality condition in the paper, then run
the toy at `n = 13` over the base `F_{2^3}` (`F_{2^39}`) or `n = 29` over
`F_{2^7}` (`F_{2^203}`), comparing descended-`S_3` fall degree and yield
against a random subspace, as pre-registered.

## 2. Spaenlehauer, deterministic root finding over finite fields with
elliptic curves (arXiv 2610.32508; `literature/arxiv_recent.json`,
`novelty_vs_programme: new_to_programme`)

An auxiliary curve with smooth group order is used as the root-finding
engine (Mignotte–Schnorr recursion) — auxiliary curve in the *solver*, not
the factor base. The corpus already notes: because the auxiliary curve is
chosen, not the target, no ECDLP leverage is expected (the target order is
prime). Toy route recorded in the corpus: the `S_3`/`S_4` root-finding
step. Obstruction named: the recursion needs the auxiliary curve's group
order to be smooth — at cryptographic `p` a smooth-order curve in the Hasse
interval is a `p^{o(1)}` event, so this is a toy-size instrument only.
Status here: unchanged; a pilot would slot beside
`research/pdp_root_finder_20261012`.

## 3. Elliptic SNFS framing (`research/elliptic_snfs_survey_20261016`, PR #1951)

The survey places both PKM towers (Kummer, isogeny) as shape C of an
"elliptic SNFS", with open asymptotic cost. Its one open shape-C
experiment is `backlog/XP-20261015-pkm-m3-n21-m4-n20` (encyclopedia §8,
lead 1): whether the Kummer tower's F4 solving degree stays at 7 at
`m = 3, N = 21` and `m = 4, N = 20` — blocked on hardware (≈256 GB / ≈512 GB;
a disk-backed reducer would unblock it). The `m = 2` ladder re-measurement
`research/pkm_kummer_remeasure_t1_14_20261101/` (this task, item 1) is the
affordable half of the same question.

## Files

- `scan.txt` — the divisibility scan, plain text, reproducible from the
  code included at its top.
