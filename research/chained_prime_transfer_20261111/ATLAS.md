# The transfer atlas: covering a prime-field curve, and where the log can go

**Status:** research construction + census. Same characteristic unless stated.
No cryptanalytic advantage; every number is `[pcc:unverified]` until CI's pcc
gate signs the claims. Code: `src/cryptanalysis/chained_transfer/atlas.rs`,
binary `transfer_atlas`. This answers the owner's follow-up: *cover a target
curve, and ask what it can be mapped to — can the cover reach a different, or
easier, curve?*

## The question, made precise

Fix the target elliptic curve `E/F_p` whose discrete log we care about. A
genus-2 cover `C → E` has `Jac(C) ~ E × E'`; the complementary factor `E'` is
a *different curve of a different order*, and it changes as the cover changes.
So: by varying the cover, **what orders can `E'` take, and can the discrete
logarithm ever follow to one that is easier?**

The load-bearing fact: a DLP-preserving homomorphism `E → E'` exists **iff**
`E` and `E'` are isogenous, and over `F_p` isogenous **iff** `#E = #E'` (Tate).
So a map to a genuinely *different-order* curve cannot preserve the log —
`Hom(E, E') = 0` for non-isogenous curves. The atlas measures how hard this
bites.

## The reachable set, by direction

- **Up, to the cover `J(C)`.** `φ^* : E → J(C)` is an explicit DLP-preserving [pcc:unverified]
  embedding. It is no cheaper: genus-2 index calculus on `J(C)` over `F_p`
  costs `Ω(p)` against rho's `√p` (PR #3898, `RESULTS.md`).
- **Across, to the complement `E'`.** The cover offers exactly one other
  elliptic map, `φ2_* : J(C) → E'`, and the composite that would carry the log,
  `φ2_* φ1^*`, is **identically zero** — forced by the deck involution
  `σ:(x,y)↦(−x,y)`, since `φ2 ∘ σ = −φ2`. So the cover never moves the log to
  its complement. Reaching `E'` at all needs a *separate* isogeny `E → E'`,
  which exists only when `#E' = #E`, and then at identical hardness.
- **Down, to an anomalous or supersingular curve (easy DLP).** An isogeny
  preserves the trace, so no edge reaches a special curve unless the start is
  already special. The easy node is real — Smart's additive transfer solves a
  genuinely anomalous curve (positive control below) — but unreachable.
- **Down, to a binary curve.** No edge here changes the characteristic. There
  is no binary node in the reachable set; the only known bridge is a global
  lift of degree `≥ m` (`cryptanalysis:research/xedni_cross_field`).

## The cover sweep (parallel workflow)

Workflow `transfer-cover-sweep`: fix a target cubic, sweep the cover
`C_c : y² = F(x² + c)`, and record the complementary factor `E'(c)`. Four
agents (one per prime `p ∈ {1009, 2003, 4001, 8009}`) swept six targets each;
an adversarial agent then hunted for any cover that carries the log or reaches
an easier curve. **24 reports, 5,641 covers** `[pcc:unverified]`:

- The complement ranges over up to **141 distinct orders** (at `p = 8009`)
  across the full Hasse interval — many different-order curves reached.
- **3,479 of 5,641 covers have a *smoother* complement** than the target, and
  85 are supersingular. "Smoother" here means a smaller largest prime that
  divides the order exactly once (`atlas.rs`). With the ordinary largest
  prime factor the count is 3,469 (replay F11).
- `cover_never_carries_log` was true wherever it was tested. The sweep tests
  `φ2_*φ1^* = 0` on the first 40 covers of each report, 16 samples each, in
  the E1 → E2 direction only (replay F9). It is vacuous in the 7 reports that
  have no valid cover. The census below is the broader check: both
  directions on every curve, with at least 14,371 generic fibres per prime.
- `easier_and_reachable` is 0 in every report. This is a consequence of the
  theorem, not independent evidence (replay F8). "Reachable" is tested as
  `#E2 = #E`, and equal order forces equal trace, embedding degree and prime
  factorisation, so the counter cannot be nonzero.

The figure `cover-spectrum.svg` shows one target (`F_1009`, order 990): 504
covers, 63 distinct complement orders over `[948, 1072]`, **87** of them
smoother, 22 isogenous (reachable) and none both reachable and easier.
*Correction (2026-11-12):* the original note and figure said "470/504
smoother". That figure belongs to the order-998 target, not the order-990
one (replay F10).

### An adversarial catch, and the correction

The adversarial agent flagged one cell — target order `1010 = p + 1`, a
**supersingular** target — as `easier_and_reachable = 6`. This was a
definitional bug, not a counterexample: a reachable complement of a
supersingular target is supersingular too, but **not easier** than an
already-supersingular start. "Easier" must be measured *relative to the
target* (a strictly smaller largest prime, or special when the target is not).
With that correction the count is 0 everywhere, and the fix is covered by a
unit test. What the adversarial pass caught was a faulty definition, not a
mathematical counterexample. With the definition corrected, the counter
cannot be nonzero (see above).

## Census (random covers, not the sweep)

`transfer_atlas run`: 300 bielliptic curves per prime, `p ∈ {1009..8009}`
`[pcc:unverified]`:

| p | curves | same order | cross-map 0 | E1 anomalous | E1 supersingular | facts differ | free easier |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 1009 | 300 | 7 | 300 | 0 | 4 | 0 | 0 |
| 2003 | 300 | 4 | 300 | 0 | 2 | 0 | 0 |
| 4001 | 300 | 7 | 300 | 0 | 3 | 0 | 0 |
| 8009 | 300 | 3 | 300 | 0 | 2 | 0 | 0 |

`cross-map 0` = `φ2_*φ1^* = 0` in both directions on every sampled generic
fibre. The replay re-checked it and found it non-vacuous: 14,371–14,397 of
14,400 samples per prime were generic, and the control `φ1_*φ1^* = [2]` held
on all of them.

`facts differ` and `free easier` are zero by construction, not by
observation (replay F8). Node facts depend only on `(p, #E)`, and both
counters are tested only when `#E1 = #E2`. They restate the theorem, and the
observed evidence is the cross-map column.

## Positive control: the easy node is real

`find_anomalous_curve` + `smart_attack_anomalous` (reusing
`canonical_lift`): on a genuinely anomalous curve over `F_43`, Smart's additive
transfer **recovered the discrete log correctly** `[pcc:unverified]`. So the
down-edge target exists and is genuinely easy — it is just unreachable from a
non-anomalous start.

## Conclusion

A cover of `E` reaches a rich spectrum of different-order, often smoother,
complementary curves — and the discrete logarithm follows to none of them. The
log is trapped in `E`'s isogeny class: up costs more (higher-genus index
calculus), across is the zero map, and down (anomalous, binary) is barred by
the trace and the characteristic. See
`problems/prime-field-chained-transfer-no-advantage.md`.
