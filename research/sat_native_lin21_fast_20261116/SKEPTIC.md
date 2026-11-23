# SKEPTIC review: lin21 wall-gap pilot (`lin21-fast`), 2026-11-16

Independent review, written without the producer's session. Every number below is an
unverified assertion of this review `[pcc:unverified]`; nothing here is a benchmark row.
All computation was done with Rust (`rustc -O`) and shell tools; no Python. Wall times were
taken on the same shared 4-core container, at most two processes at a time.
Scratch (not in the repository):
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-W3/`.

## Verdict

**Weaker than stated.** The rewrite itself held up: no soundness defect was found in about
19,500 differential instances and about 62,000 learnt clauses, and the counters reproduce
exactly. The speed-up over the original solver (about 60 to 70 times at l = 9..12) is
supported. The headline "gap closes" is not:

- The rule is met literally (largest ratio of medians 24.5, threshold 30), and the producer
  reports correctly that 30 lies inside the A/A band.
- The denominator is a slow scan. The plain scan in this harness costs 1,506, 2,099 and
  2,702 ns per leaf at n = 29, 35, 41. The lab's standalone port of the same 2+1 leaf
  (`research/scan_floor_leaf_20261116/scan_leaf.rs`, rerun here on this machine) costs 273,
  349 and 428 ns per leaf on the same number of leaves per target (517, 2,009, 8,193). That
  is 5.5, 6.0 and 6.3 times faster. Against that scan `lin21-fast` is about 113, 148 and 159
  times slower at l = 9, 11, 13, and the ratio grows with l.
- As frozen (every learnt clause kept), the ratio will rise beyond l = 13.

Plain supported claim: `lin21-fast` decides the same lin21 instances as the original
`LinCdcl` with the same verdicts and nearly the same conflict counts, about 60 to 70 times
faster at l = 9..12. It is 21 to 25 times slower than this harness's own bit-serial scan and
roughly 110 to 160 times slower than the lab's faster native leaf. The scan remains the best
arm; no index-calculus cost and no distance to Pollard rho changes.

## 1. Reproduction: yes

`run.sh <scratch>/l21fast-l5 "5 9" 1` (66 s; both `cmp` gates passed, selftest passed).

- Selftest output is byte-identical to the producer's `raw/selftest.jsonl`.
- For all 41 targets of l = 5 and l = 9 (205 rows: scan, lin21, lin21-fast, lin21-fast-b,
  lin21-prof) the status, the conflict count and the scan's leaf count are identical to the
  producer's `raw/rows.jsonl` (`diff` empty).
- Wall medians, mine against the producer's:

| l | scan ms | lin21 ms | lin21-fast ms | fast / scan | lin21 / fast | A/A paired |
|---|---|---|---|---|---|---|
| 5 (mine) | 0.0111 | 20.9 | 0.589 | 53.0 | 35.5 | 0.88 [0.64, 1.15] |
| 5 (producer) | 0.0109 | 20.4 | 0.523 | 47.9 | 39.0 | 0.90 [0.62, 0.98] |
| 9 (mine) | 0.781 | 1083 | 16.0 | 20.5 | 67.8 | 0.99 [0.64, 1.39] |
| 9 (producer) | 0.778 | 1061 | 16.0 | 20.6 | 66.3 | 0.98 [0.61, 1.51] |

- I recomputed every median and ratio of the README's main table (l = 5..13) from
  `raw/rows.jsonl` with `sed`/`sort`/`awk`, independently of `summary.rs`: all agree to the
  printed digits (fast / scan 47.9, 40.1, 24.9, 24.8, 20.6, 23.7, 24.5, 23.2, 25.0).
- sha256 of `fastcdcl.rs`, `l21fast.rs`, `prof.sed`, `lincdcl_prof.rs`, `run.sh`,
  `base_lin21_verbatim.rs` and `examples/pdp_lin_cdcl.rs` equal the protocol's. `summary.rs`
  differs, as disclosed.

Not rerun: l = 6..8, 10..13 with the full arm set (time). I ran `lin21-fast` on three
targets each at l = 11 and 13 inside the deletion test below; conflicts matched the README
medians (3,066 and 12,352 against 3,066 and 12,351).

## 2. Soundness of the rewrite: no defect found

### (a) Differential tests (seeds not used by the pilot)

Harness: `<scratch>/dt/dt.rs`. It includes the original solver unchanged and a copy of
`fastcdcl.rs` patched by `sed` only to expose `assigns`/`clauses`, to log learnt clauses and
to add an optional deletion switch (off by default; 41 diff lines, `<scratch>/dt/fastp.rs`).

| Test | Instances | Result |
|---|---|---|
| A. lin21 encoding, seed 991177, (n, l) = (10,3), (12,3), (13,3), (14,4), (13,4), (15,4), (17,5), (20,6); uniform and planted targets | 3,180 (1,256 decomposable, 1,924 not) | 0 verdicts differ from the pair oracle (fast and original); 0 bad models; 0 non-liftable models on admissible targets; 0 panics |
| B. random clause + XOR + gate formulas, n = 2..4, at most 16 variables, brute-force enumeration of all models; blocks disjoint within a gate | 8,000 (6,735 satisfiable) | 0 verdicts differ from brute force (fast and original); 0 bad models; 0 panics |
| B'. the same with blocks overlapping inside a gate | 4,000 (3,281 satisfiable) | 0 / 0 / 0 |
| C. larger random formulas, 70..149 variables, n = 3..6, fast against original, 3,000-conflict budget | 4,300 (4,229 decided by both, 2,056 SAT) | 0 verdicts differ; 0 bad models (models checked against the constraints with an independent field multiplication); 0 panics |

Total 19,480 instances, no counterexample.

### (b) Learnt clauses

Reading: a row's mask is the OR of the unit slots of every row XOR-ed into it, and a gate
row carries the masks of the known block and of every assigned bit it substituted. The mask
of an implied literal is therefore a superset of the unit literals that support it, all of
them earlier on the trail. A superset gives a weaker but still implied clause. Cutting at
decisions and clause-propagated literals is the usual "decision-side" resolution and is
sound. Checkpointing is consistent: the level-L matrix is modified only while the solver is
at level L, and unit slots are a stack truncated with the trail. I found no path to an
unsound clause.

Tests:

- B and B': every learnt clause checked against every model of the formula by brute force:
  56,546 learnt clauses, 28,350 of them on satisfiable formulas (the non-vacuous case),
  **0 violated**.
- A: on 38 decomposable lin21 targets, each of 137 learnt clauses of `lin21-fast` was added
  negated to a fresh instance of the ORIGINAL solver: all 137 came back UNSAT (implied).
- C: 4,995 sampled learnt clauses checked the same way: 0 refuted, 0 undecided.
- The overflow fallback (bit 63), which the pilot never exercised, was exercised in C:
  771,220 overflow slots on 471 instances; 1,886 of the checked clauses came from those
  instances, 0 refuted.
- Positive control for the test itself: a mutant with the gate guard mask set to 0 gave
  164 violated learnt clauses, 17 wrong verdicts and 962 panics in 3,000 B instances, and 17
  refuted clauses in 500 C instances. The tests can see an explanation bug.

### (c) Gate folding

Division is taken only when the known operand is non-zero (`if known != 0`) and uses
`gf_inv`, so y = z / a is an equivalence there. With a = 0 the code falls through to the row
form, where all coefficients are zero and the rows state z_k = 0; y stays free, which is
correct. The forward product needs no condition. The multiplication-row cache is refreshed
when the key changes or is 0. In B and B' the three paths ran 24,121 (forward), 19,930
(divide) and 57,444 (rows) times, with zero-valued blocks frequent at n = 2..4, against
brute force: no mismatch.

### (d) The 21 targets with different conflict counts

All 21 are SAT targets (21 of the 42 SAT targets; listed by `awk` from `raw/rows.jsonl`).
The differences are -3..+1 conflicts (for example l = 12: 3,897 against 3,898). 17 of the 21
return the same (x1, x2, x3) as the original; 10 of the 42 SAT targets return a different
model. This is explained by different (larger, decision-only) reasons changing activities
and the point at which the satisfying leaf is closed. It is not a sign of different
semantics; verdicts and model checks agree.

One correction: "conflicts equal on non-decomposable targets" is an observation, not an
invariant. In test A, 3 of 1,924 non-decomposable targets had different conflict counts
(one each at (10,3), (13,4) and (17,5), the last being the pilot's own shape n = 3l + 2).
"Same search" should read "the same search on almost every UNSAT target".

## 3. Fairness of "x above the plain scan"

### The scan baseline is slow: the ratio flatters the solver by about 5.5 to 6.3 times

| n | l | leaves per target | this harness's scan, ns per leaf | standalone port, ns per leaf (rerun here) | harness / port | lin21-fast / port scan |
|---|---|---|---|---|---|---|
| 29 | 9 | 516 / 517 | 1,506 | 272.7 | 5.5 | 113 |
| 35 | 11 | 2,008 / 2,009 | 2,099 | 349.0 | 6.0 | 148 |
| 41 | 13 | 8,192 / 8,193 | 2,702 | 427.6 | 6.3 | 159 |

Harness figures: median scan wall of the non-decomposable targets divided by the row's
`leaves` field. Port: `scan_leaf --mode breakdown --cells 29:9,35:11,41:13`, baseline arm,
"no" class (its README reports 279.7, 348.9, 420.1). Last column: the producer's median
`lin21-fast` wall divided by leaves x port ns per leaf.

Caveats on my own comparison: the port is built with `-C target-cpu=native` (carry-less
multiply) and uses the half-trace leaf, while the pilot builds every arm with plain
`rustc -O` and a bit-serial field multiplication. The README does list "a native target CPU"
under "Not covered". I rebuilt the pilot harness with `-C target-cpu=native`: neither its scan
nor `lin21-fast` got faster (5 targets, within noise), so the flag alone does not close the
difference; the scan implementation does. The pilot's rule names "the plain scan" of this
file, so the rule is not violated. But the sentence "21 to 25 times slower than the plain
native scan" describes a gap to a scan that the lab already beats by about 6 times, and the
gap to the faster scan widens with l (113, 148, 159) because the port's per-leaf cost grows
more slowly (x1.57 from n = 29 to 41) than the harness scan's (x1.79).

### Noise and the 30x threshold

- The scan is a median of 21 repeats; each solver arm is one run. Single-run noise therefore
  falls on the numerator only. In my own runs the same configuration on the same target took
  89 ms and 139 ms (l = 11) with a second process active.
- The A/A control is a fixed order (the second run always follows the long `lin21` run), so
  it mixes order effects with noise. It is not an interleaved or randomised A/A.
- With the producer's per-target A/A range, ratio x [min, max] contains 30 at every l in
  9..12. In my rerun of l = 9 it did not ([13.0, 28.3]). 5 of the 64 single-target ratios at
  l = 9..12 exceed 30.
- The ratio of medians itself (20.6 to 24.5, per-cell A/A medians 0.89 to 1.00) is probably
  below 30 for this harness's scan; that much I accept. "Gap closes" as a general statement
  is not justified.
- The protocol's predictions were written after tuning on seed 777 with the same shapes and
  already state "expected rule outcome: gap closes". The rule guards against target-specific
  tuning only; it was not a blind test of the threshold.

### Trend

- +0.057 bits per unit of l over four medians, with per-cell noise of about 10 %, is not
  distinguishable from zero. It is evidence neither for nor against divergence.
- The mechanism is evidence. Clause propagation is 5.7, 6.4, 8.8, 13.2, 23.2 % at
  l = 9..13. From l = 12 to 13 its absolute time grows about 4.4 times while the rest grows
  about 2.2 times (shares x median walls). Extrapolated, clause propagation is about a third
  of the time at l = 14 and about half at l = 15, and the frozen solver is at about 30x of the
  harness scan at l = 14 and about 39x at l = 15. So as frozen the ratio diverges; the README says so in words but
  the decision line does not.
- The divergence is removable. With my deletion patch (drop a learnt clause once it is no
  longer the reason of an assigned literal; keep the last 1 or 64), on three
  non-decomposable targets per size:

| l | keep all: median wall ms / clause-propagation share | keep last 1 | keep last 64 | conflicts (sum, all three settings) |
|---|---|---|---|---|
| 9 | 15.5 / 5.9 % | 15.1 / 2.8 % | 15.3 / 3.5 % | 4,638 = 4,638 = 4,638 |
| 11 | 89.9 / 8.8 % | 96.9 (min 83.4) / 2.8 % | 86.0 / 3.3 % | 18,394 each |
| 13 | 509 / 20.2 % | 414 / 4.2 % | 423 / 4.3 % | 74,102 each |

  Conflict counts are identical with deletion on every target: the learnt clauses do no
  pruning on these UNSAT instances (consistent with the lin21 pilot). With deletion the l = 13
  ratio to the harness scan is about 20 to 21 instead of 25, and the clause-propagation share stays
  near 3 to 4 %. "Learning disabled" in the strict sense was not built; keep-last-1 is the
  closest sound variant I could make in the time. Few targets, single machine, loaded: treat
  the wall figures as indicative.

## 4. Decision rule and the `summary.rs` change

- The decision follows from the pre-written rule as worded: medians <= 30x at l = 9..12,
  0 verdict differences, 0 censored rows, and the A/A clause is reported as the protocol
  requires.
- Protocol order: `PROTOCOL.md` was committed alone at 02:55:26Z (`dfe87cd298`); the run
  started 02:56:01Z (`raw/started_utc.txt`). The earlier snapshots `e0fdc0a272` (02:31Z:
  `base_lin21_verbatim.rs`, `lincdcl_prof.rs`, `prof.sed`) and `7a808c71b6` (02:40Z:
  `fastcdcl.rs` with 1,190 lines, `l21fast.rs` with 302) hold development versions. The
  frozen solver and harness were not committed before the run; they are bound only by the
  sha256 values in the protocol, which match the files now in the tree. That is adequate but
  weaker than a commit.
- `summary.rs`: the pre-change file (sha256 34d8e712...) is not in Git, so "6 lines, no
  number changed" cannot be checked by diff. My independent recomputation from the raw rows
  matches every median and ratio in the main table, and the rule's inputs are those medians,
  so the change cannot have moved the decision. The A/A-band lines it prints were
  spot-checked for l = 9 (20.6 x [0.61, 1.51] = [12.6, 31.1]).

## Problems, in order of weight

1. The scan baseline in the harness is 5.5 to 6.3 times slower per leaf than the lab's
   standalone port on the same machine; against that port the solver is about 110 to 160
   times slower and the gap widens with l. "Gap closes" should not be repeated without this.
2. As frozen, the ratio rises beyond l = 13 (learnt clauses never deleted); the rule's window
   l = 9..12 ends just before this matters. Deleting clauses removes it without changing a
   conflict count.
3. Wall evidence is one run per arm and target on a loaded machine; the threshold lies inside
   the A/A band (disclosed), and the A/A is fixed-order.
4. The threshold was tested after tuning on the same shapes with the outcome already
   predicted.
5. "Same conflict counts on non-decomposable targets" is empirical (3 exceptions in 1,924
   fresh targets), not structural.
6. Frozen code was hash-bound, not committed, before the run; the pre-change `summary.rs` is
   not recoverable.
7. No research-kb query, benchmark record, pcc claim or PDF (disclosed by the producer; the
   pilot is correctly labelled unverified).

## Not checked

The `FAST_PIVOT` variants and `restarts = true`; fields with n > 41 or more than one word;
full reruns of l = 6..8 and 10..13; the profiling copy's timer placement; whether the
standalone port and `scan21` return identical leaf sets (only leaves per target were
compared: 516/517, 2,008/2,009, 8,192/8,193); a strict learning-free variant.
