# Item 10: program search over the decomposition kernel (assessment after repair pass R2, stage 2)

## Verdict: INCONCLUSIVE (level: constant; the frozen rule still has no input)

The frozen decision rule (`PROTOCOL.md`, "Decision rule") is defined on **held-out** data (128 natural + 128
planted targets per cell, 2000-resample bootstrap intervals for `Q(B)/Q(A)` and `Q(B)/Q(C)` at w = 0, 0.1, 1),
and on the controls (null, yield conservation, a finalist's family passing the yield model). The held-out,
finalist, ablation, conservation and summary cards (14-18, 21-r2, 22-r2) have not been frozen or run, so no
per-cell ADVANCES / MATCHES / REGRESSES / TRADES can be assigned; the rule's own "otherwise" branch applies. This
assessment adds what stage 2 delivered: **all seven searches (b and c at n = 31, 37, 41; p at n = 31) are now
complete at 256 evaluations, every receipt completed, every relay verbatim, every archive hash equal to the
receipt's output hash.** Nothing here is a speedup claim and nothing is credited to the search.

What changed against the previous assessment (below): the n41 arm-b search was **not** at a plateau at 128; its
best training Q fell from 296.4 to 195.1 (evaluation 174). The earlier sentence "all three search arms reach their
plateau by evaluation 128" is withdrawn.

Distance to matched rho: **unchanged and not measured by this item.** It is a one-stage diagnostic (S_4
pair-and-root kernel, toy factor bases, l = 9, n = 31, 37, 41); it computes no S, no rho ratio and no GAE, so the
section 5a band (0, 0.1, 1 GAE per unpriced native operation) has no cross-method row to rank. The kernel's
unpriced native counters (word XORs, hash probes, trace tests, pair iterations) are priced at 0, 0.1 and 1 counted
operations inside Q for every arm, and the training ratios below hold at all three weights (they differ by under
one part in ten between w = 0 and w = 1). Even a full success would be a constant factor on the relation phase,
not the exponent; the distance to rho is whatever it was before this item.

## Method (unchanged from PROTOCOL.md)

Counted fitness Q(pi, w) = cbar_N(w) . B_r / (T_eff . phi) on 6 natural + 32 planted training targets; (1+1)-ES with
elite archive (b), 256 distinct random valid programs (c, negative control by design), the same engine from a
handicapped start (p, n = 31). Incumbent = evaluation 0 of the arm-b archive. Stage 2 resumed each chain from its
committed 128-line archive (bound by sha256, copied with `cp -t`) and appended 128 more evaluations; the first
128 lines of every new archive are byte-identical to the stage-1 archive (checked), as the first 192 lines of
n31-b are to the card-04 archive.

## Numbers (TRAINING set only; measured, from `raw-r2/state/*.jsonl`)

"ratio" = best finite training Q of the arm / the incumbent's Q on the same targets. These are selected-on
quantities (best of 256 on 6 natural targets), carry no interval, and are exactly what the rule forbids reading as
a verdict.

| cell, arm | evals | best program (training) | idx | Q at w=0.1 | ratio at w = 0 / 0.1 / 1 | phi | Q=inf (of which dominated) |
|:--|--:|:--|--:|--:|:--|--:|:--|
| n31 b | 256 | affine-aligned-s2 lift1 stop0 root0 q=b1 sqt1 ab=2 bt=1 | 94 | 314.3 | 0.0753 / 0.0755 / 0.0767 | 0.94 | 10 (2) |
| n37 b | 256 | affine-aligned-s1 lift1 stop1 root0 q=b1 sqt1 ab4 bt1 | 79 | 331.8 | 0.1014 / 0.1016 / 0.1031 | 0.31 | 10 (5) |
| n41 b | 256 | affine-aligned-s1 lift1 stop1 root0 q=b1 sqt1 ab4 bt1 | 174 | 195.1 | 0.0531 / 0.0533 / 0.0541 | 0.53 | 2 (1) |
| n31 c | 256 | affine-aligned-s1 lift1 stop0 root0 q=general sqt0 pm=full ab4 bt0 | 216 | 362.1 | 0.0869 / 0.0870 / 0.0873 | 0.31 | 72 (53) |
| n37 c | 256 | affine-aligned-s1 lift1 stop0 root1 q=general sqt1 ab4 bt1 | 158 | 528.4 | 0.1594 / 0.1619 / 0.1763 | 0.31 | 89 (71) |
| n41 c | 256 | affine-aligned-s1 lift1 stop0 root1 q=b1 sqt1 pm=full ab4 bt2 | 56 | 389.5 | 0.1058 / 0.1063 / 0.1092 | 0.53 | 82 (72) |
| n31 p | 256 | affine-aligned-s2 (same program as n31 b idx 94, found at idx 64) | 64 | 314.3 | start 15505 -> 314.3 | 0.94 | 5 (2) |

Incumbent training Q at w = 0.1: n31 4162.2, n37 3264.2, n41 3663.4 (phi = 1).
Search versus random at equal evaluations (256 each, training, w = 0.1), best-b / best-c: **0.868 (n31), 0.628
(n37), 0.501 (n41)**. Descriptive, no interval, same targets used for selection in both arms; it is the quantity
that the held-out `Q(B)/Q(C)` comparison will test and it does not yet support ADVANCES over MATCHES. At n31 the
two arms are within 14% of each other on training data and the gap could be selection noise.
Search consumes far fewer capped evaluations than random (Q=inf: 2-10 versus 72-89 of 256), which is a cost-of-
search observation, not a quality-of-finalist one.

Running-best training Q (w = 0.1): n31 b 4162 -> 314 (by 128, unchanged to 256); n37 b 3264 -> 332 (by 128,
unchanged); n41 b 3663 -> 296 (128) -> 195 (256); c: n31 503 -> 362, n37 655 -> 528, n41 389.5 (unchanged).

## Controls (state as of now)

* Instrument correctness (selftest, `raw/selftest.txt`, first run, kept): 400 random programs against brute force at
  n = 17, l = 6, bad = 0; S_4 vanishes on x(P1 +- P2 +- P3) at n = 17, 31, 41; handicap checks all PASS.
* **Positive control for the search (arm p), now final at 256 evaluations: PASSES on training data.** Best training Q
  314.3 is at most the incumbent's 4162.2 and at most half of its own start Q 15505 (ratio 0.0203).
* Negative control, arm c: complete. `affine-anti` never has a finite Q in any archive (every occurrence is
  `inf`), as predicted. Random search over this program space already finds large training gains (0.087, 0.162,
  0.106 of the incumbent), so most of the training gain is the space, not the search.
* Not run: null (incumbent on two held-out halves), yield conservation at (17,6) and (19,7), finalists, ablation,
  reference arm a1. A finalist whose family violates the yield model cannot support a verdict; that check has not
  happened.

## Observations that the held-out stage must test (not findings)

1. All six arm-b and arm-c winners at l = 9 are `affine-aligned` bases with `lift = 1`. The `T_eff` credit this gives
   in Q (n31: 2.90e6 versus 1.11e6; n37: 3.21e6 versus 1.60e6; n41: 3.50e6 versus 1.35e6; factors 2.6, 2.0, 2.6)
   is the yield model, not a cheaper kernel. Whether the model holds at those bases is the conservation check.
2. `lift = 1` (do not scan abscissae without a point) is shared by every winner; how much of the gain it alone
   explains is the a1 reference arm (card 14), not yet run.
3. The aligned winners give up planted retention (phi 0.94, 0.31, 0.53 for the b winners): Q divides by phi, and
   held-out phi is what counts. The n41 winner changed between 128 and 256 evaluations, so the finalist is not
   stable under search length at that cell.

## Deviations and caveats

* Staging, not a protocol change: outputs under `raw-r2/`, searches split in two 128-evaluation cards
  (`PROTOCOL.md` "R2 stage 2"); question, arms, cells, metric and rule unchanged.
* Relay files for this stage are `raw/relay-8..13.txt` (relay-1..7 belong to earlier stages and are not
  overwritten). `check-relay` on all six: `relay_verbatim: true`, no findings. Two of my first transcriptions
  (relay-9, relay-13) failed with one missing line each; both were my own copying errors (a gene digit and a
  dropped trailing line), corrected against the returned text and then verified; they are not executor deviations.
* Card 05-r2's after-the-fact lint failure (see below) was fixed in the stage-2 card text; the stage-2 cards pass.
* Host L0 (shared 4-vCPU container, two lanes at once): wall times (87-291 s per 128) are descriptive only. Toy scale:
  GF(2^n), n = 31..41, l = 9, sparse regime (about 0.02 raw S_4 roots per natural target); a replica of the
  incumbent kernel, not the crate's `SubspaceOracle` (Deviation 1); no result transfers to cryptographic sizes.
* Not a performance result in the sense of AGENTS.md: no held-out measurement exists, so nothing was copied to
  `benchmarks/` and the leaderboard was not regenerated.

## Next steps (not done here)

Freeze stage 3 (`make_r2_cards.py 3`: cards 14-18 and 21-r2, which bind the seven archives above by sha256; they
can be frozen now), run and commit them, freeze stage 4 (card 22-r2), then apply the rule to the held-out summary
and replace this verdict.

## Files

`graph.svg`, `graph.png` (source `make_graph.py`, data `raw-r2/training-stage-summary.json`), `REPORT.pdf`,
`raw/relay-1..13.txt`, `raw-r2/receipt-card-*-r2.json`, `raw-r2/state/*.jsonl`, `raw-r2/state-1/*.jsonl`.

---

## First run (superseded)

Previous assessment (R2 stage 1, kept verbatim below; its verdict was INCONCLUSIVE and so is this one, but its
numbers for n37/n41 arm b and all arm c are superseded by the 256-evaluation table above, and its plateau claim
is withdrawn):

> 
### Verdict: INCONCLUSIVE (level: constant; no verdict can be computed yet)

The frozen decision rule (`PROTOCOL.md`, "Decision rule") is defined on **held-out** data
(128 natural + 128 planted targets per cell, 2000-resample bootstrap intervals for
`Q(B)/Q(A)` and `Q(B)/Q(C)` at w = 0, 0.1, 1). The held-out, finalist, ablation, conservation and
summary cards (14-18, 21, 22) have not run: stages 2-4 are not frozen or executed. Stage 1 of 4 is
complete, and the search archives are only partly complete (see the table). So no per-cell ADVANCES /
MATCHES / REGRESSES / TRADES can be assigned, and the rule's own "otherwise" branch applies.
Nothing here is a speedup claim, and nothing is credited to the search.

Distance to matched rho: **unchanged and not measured by this item.** It is a one-stage diagnostic
(S_4 pair-and-root kernel, toy factor bases, l = 9, n = 31, 37, 41); it computes no S, no rho ratio and
no GAE, so the section 5a pricing band (0, 0.1, 1 GAE) has no cross-method row to rank. The unpriced native
counters (word XORs, hash probes, trace tests, pair iterations) are priced at 0, 0.1 and 1 counted
operations inside Q for every arm, the same way. Even a full success would be a constant factor on
the relation phase, not the exponent.

### First run proper (before the R2 pass)

The first run left no assessment: no README existed. It produced receipts for card 01 (selftest, completed) and
card 04 (n31 arm b, evaluations 0..191, completed), both kept and not re-run. Cards 02 and 03 were cut off
without a receipt; a partial `raw/state/n31-p.jsonl` (151 evaluations) has no receipt and is **not used**
(its evaluations 0..127 agree field for field with the R2 card-02 archive, which is consistent with the
evaluations being pure functions of the index; noted, not relied on). Cards 05-18 never ran. No first-run verdict exists to supersede;
the status was "incomplete, nothing assessable".

### What ran (R2 pass, all relays checked against receipts)

| card | content | status | archive (sha256 prefix) |
|:--|:--|:--|:--|
| 01 (first run) | build + selftest | completed | `raw/selftest.txt` |
| 04 (first run) | n31 arm b, evals 0..191 | completed | `raw/state/n31-b.jsonl` 23792d0b |
| 02-r2 | n31 arm p, evals 0..127 | completed | `raw-r2/state-1/n31-p.jsonl` 83fac4f4 |
| 05-r2 | n31 arm b, evals 192..255 (copy of card 04 archive, bound by sha, then appended) | completed | `raw-r2/state/n31-b.jsonl` 8d4e27e9 |
| 06-r2 | n37 arm b, evals 0..127 | completed | `raw-r2/state-1/n37-b.jsonl` 0c8c0d54 |
| 07-r2 | n41 arm b, evals 0..127 | completed | `raw-r2/state-1/n41-b.jsonl` 59c6c7fc |
| 08-r2 | n31 arm c, evals 0..127 | completed | `raw-r2/state-1/n31-c.jsonl` e44de45a |
| 10-r2 | n37 arm c, evals 0..127 | completed | `raw-r2/state-1/n37-c.jsonl` 0154f724 |
| 12-r2 | n41 arm c, evals 0..127 | completed | `raw-r2/state-1/n41-c.jsonl` ebaf9b23 |

Relay check (`tools/task_card.py check-relay`, reports in `raw/relay-1..7.txt`): cards 02, 06, 07, 08, 10, 12-r2
`relay_verbatim: true`, no findings. **Card 05-r2: check-relay returned `CARD FAIL`, not a relay finding:**
its `copy-card-04-archive` command names the destination file `raw-r2/state/n31-b.jsonl`, which the card itself
created, and the linter (run after the fact) now reports it as an unpinned repository file. Checked by hand
instead: the relayed card sha, receipt sha (e8a6d9a4...) and output sha (8d4e27e9...) equal the on-disk values,
the bound input `raw/state/n31-b.jsonl` still hashes to 23792d0b..., and the first 192 lines of the new archive are
byte-identical to it. This is reported as a deviation of the check, not repaired; the data are usable.
Not run yet: cards 03, 19, 20, 09, 11, 13 (stage 2: the second 128 evaluations of p-n31 b-n37 b-n41 c-n31 c-n37 c-n41),
14-18 and 21 (stage 3), 22 (stage 4).

### Numbers (training set only, Q at the stated weight; measured, from the archives)

Training = 6 natural + 32 planted targets per base. The incumbent is evaluation 0 of each arm-b archive
(Q at w = 0.1: n31 4162.2, n37 3264.2, n41 3663.4; planted retention phi = 1). "ratio" is the best finite training Q of
the arm divided by the incumbent's Q on the same targets. These ratios use the targets the programs were
selected on, have no interval, and are exactly the quantity the rule forbids reading as a verdict
(selection among 128-256 candidates on 6 natural targets, so they are optimistically biased).

| cell, arm | evals done / 256 | best program (training) | Q at w=0.1 | ratio at w = 0 / 0.1 / 1 | phi | dominated, Q=inf |
|:--|:--|:--|--:|:--|--:|:--|
| n31 b | 256 | affine-aligned-s2 lift1 stop0 root0 quartic=b1(1) sqt1 ab=2 bt=1 (idx 94) | 314.3 | 0.0753 / 0.0755 / 0.0767 | 0.94 | 2, 10 |
| n37 b | 128 | affine-aligned-s1 lift1 stop1 root0 q1 sqt1 ab4 bt1 (idx 79) | 331.8 | 0.1014 / 0.1016 / 0.1031 | 0.31 | 3, 5 |
| n41 b | 128 | affine-aligned-s2 lift1 stop1 root0 q1 sqt1 ab4 bt1 (idx 66) | 296.4 | 0.0807 / 0.0809 / 0.0821 | 0.34 | 0, 0 |
| n31 c | 128 | affine-rand-s1 lift1 stop0 root0 q1 sqt0 pm1 ab4 bt1 (idx 99) | 502.9 | 0.1207 / 0.1208 / 0.1219 | 0.44 | 26, 39 |
| n37 c | 128 | sub-low lift1 stop1 root1 q0 ab4 bt1 (idx 28) | 655.4 | 0.1979 / 0.2008 / 0.2172 | 0.50 | 48, 55 |
| n41 c | 128 | affine-aligned-s1 lift1 stop0 root1 q1 sqt1 pm1 ab4 bt2 (idx 56) | 389.5 | 0.1058 / 0.1063 / 0.1092 | 0.53 | 39, 44 |
| n31 p | 128 | affine-aligned-s2 (same program as n31 b idx 94, found at idx 64) | 314.3 | start 15505 -> 314.3 | 0.94 | 0, 0 |

Running-best training Q (w = 0.1, recorded): n31 b 4162 -> 343 (64 evals) -> 314 (128) -> 314 (256); n37 b 3264 -> 418 -> 332;
n41 b 3663 -> 297 -> 296. All three search arms reach their plateau by evaluation 128 or earlier.
Search versus random on equal evaluations (128 each, training, w = 0.1): best-b / best-c = 0.625 (n31, b at 256),
0.506 (n37), 0.761 (n41). Descriptive only: no interval, no held-out, and arm c is half complete.

### Controls (state as of now)

* Instrument correctness (selftest, `raw/selftest.txt`): 400 random programs against brute force at n = 17, l = 6,
  bad = 0; quartic forms equal the resultant; S_4 vanishes on x(P1 +- P2 +- P3) at n = 17, 31, 41; `#E` and
  `2E = ker delta` at n = 13..41; handicap checks 1.996 (pairs=full, 1.95-2.05), 0.213 (lift, 0.20-0.30), 0.234
  (abort 1/8, 0.22-0.25), 1.542 (generic root > 1.3), 0.9858 (b1 < 1): **all PASS**.
* Positive control for the search (arm p), provisional at 128 of 256 evaluations: best training Q 314.3 is at most the
  incumbent's 4162.2 and at most half of its start 15505. The pass criterion is met so far (ratio 0.0203 to its start),
  but the arm is not finished and the criterion is on the final 256, so it is recorded as "met provisionally".
* Negative control, arm c: half complete (128 of 256). Its best training Q is also far below the incumbent's
  (ratios 0.12, 0.20, 0.11), so random search over this program space already finds large training gains; whether
  the search beats it is exactly the `Q(B)/Q(C)` comparison that needs held-out data.
  `affine-anti` never receives a finite Q wherever it was drawn (arms b n31, b n37 and the c archives): consistent
  with its prediction.
* Null (incumbent on two held-out halves), yield conservation at (17,6) and (19,7), finalists, ablation: **not run**.
  A finalist whose family violates the yield model cannot support a verdict, and that check has not happened.

### Observations that the final held-out stage must test (not findings)

1. The best training programs share `lift = 1` (the incumbent's `lift = 0` scans abscissae without a point):
   the best `sub-low` program in the n31 b archive (lift1, otherwise incumbent-like) already has Q 834 (ratio 0.20).
   The reference arm a1 (incumbent plus `lift = 1, stop = 1`) is a card-14 output and has not run, so how much of the
   gain is "do not scan non-rational abscissae" is **not yet separated**; the rule reports `Q(B)/Q(A1)` for this.
2. The winners are `affine-aligned` bases. In Q this earns `T_eff` credit (n31: 2.90e6 versus 1.11e6; n37: 3.21e6
   versus 1.60e6; n41: 3.10e6 versus 1.35e6, factors 2.6, 2.0, 2.3), which is the yield model, not a cheaper kernel;
   whether the model holds at those bases is the conservation check, which has not run. Part of the training gain
   is therefore contingent on an unrun check.
3. The aligned bases give up planted retention (phi 0.94, 0.31, 0.34 for the three winners): Q divides by phi. The
   held-out phi is what counts.

### Deviations and caveats

* Staging, not a protocol change: the R2 pass put all outputs under `raw-r2/` and split searches into 128-evaluation
  pieces across cards. The addendum to `PROTOCOL.md` records this; question, arms, cells, metric and rule unchanged.
* The relay check of card 05-r2 fails only the after-the-fact lint (above).
* The partial first-run `raw/state/n31-p.jsonl` (151 evaluations, no receipt) is not used.
* Host L0 (shared 4-vCPU container, two lanes at once): wall times in the archives are descriptive only; the
  verdict quantity is counted operations. Toy scale: GF(2^n), n = 31..41, l = 9, sparse regime (about 0.02 raw
  S_4 roots per natural target); a replica of the incumbent kernel, not the crate's `SubspaceOracle` (Deviation 1);
  no result here transfers to cryptographic sizes.
* Not a performance result in the sense of AGENTS.md: no held-out measurement exists, so nothing was copied to
  `benchmarks/` and no leaderboard section was regenerated. Re-evaluate after stage 3.

### Next steps (not done here)

Commit the stage-1 outputs (this commit), run `make_r2_cards.py 2`, execute stage-2 cards, then `3` and `4`;
then apply the rule to the held-out summary (`raw-r2/summary.json`, card 22) and replace this verdict.

### Files

`graph.svg`, `graph.png` (source `make_graph.py`, data `raw-r2/training-stage-summary.json`), `REPORT.pdf`,
`raw/relay-1..7.txt`, `raw-r2/receipt-card-*-r2.json`, `raw-r2/state*/`.
