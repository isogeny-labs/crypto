# SKEPTIC review: Lead B, dual-rail gates on the ECC2K-130 S'4 instance (2026-11-15)

Independent review, one niced process at a time on the shared container, about 20 minutes.
Everything here is `[pcc:unverified]`: a local check, no isolab host, no replay certificate.
Producer files were not edited. Scratch (copies, my scripts, rerun rows):
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-B/`
(`own_verify.py`, `rails_check.py`, `shuf.py`, `b_d5.jsonl`, `b_d7.jsonl`, `b_d8.jsonl`, `b_d8cL.jsonl`).

## Verdict

**Reproduced: yes** for every number I reran (conflict counts identical to the row). The
producer's verdict **INCONCLUSIVE follows from the rule as written**. The numbers are right; the
framing "reduction 50x / 31x / 591x" is weaker than it reads, for the reasons in "Problems".

Supported plain statement:

> On the repository's S'4 CNF+XOR instance at d = 8 (8 non-decomposable targets, one solver
> build), adding dual rails cuts CryptoMiniSat's conflict count by 35x to 110x (arm b) while
> wall time changes by less than 1.3x; at d = 7 the cut is about 4x and the railed instance is 7x
> slower; at d = 9 neither the plain nor the railed free solve finishes in 45 s. No SAT arm decides
> a target faster than pair enumeration: at d = 8 every arm is 6x to 16x slower than a plain-Python
> pair enumeration and 280x to 780x slower than a table lookup. The linear-leaf arm is a different
> formulation that performs the 2+1 scan with the solver as leaf routine; it costs 1.6 to 2.6
> conflicts per scan leaf and is the slowest arm in time.

## 1. Soundness of the rail post-pass: sound (checked)

Argument: for t = a & rest, r = (not a) & rest gives t xor r = rest identically, so the XOR
t + r + rest = 0 is implied once r is defined; r is a fresh variable defined as an AND of existing
literals, so every model of the original extends uniquely. `findGates` can only return a "gate"
whose three defining clauses are literally in the formula, so a false detection is impossible; a
missed gate only loses rails.

Checked on real instances (`rails_check.py`, d = 5, 6, 7, 8, uniform target 0 and planted target 0;
S'4 with all rails, linear-leaf with selector rails, linear-leaf with all rails at d = 5):

- The railed instance's clause list and XOR list start with the original lists unchanged.
- Every added clause contains a fresh variable; every fresh variable has exactly the clause set
  of `v <-> AND(literals over lower-numbered variables)`; no fresh variable is left undefined.
  `created_rest_gates` is 0 everywhere (the two-input "rest" gate always already exists).
- One SAT call per added XOR: (original gate CNF, without the equation XORs) AND (rail
  definitions) AND NOT(XOR) is UNSAT for all of them: 1,045 / 1,648 / 2,425 / 3,394 XORs at
  d = 5..8 for S'4, 3,144 to 5,109 for the linear-leaf selector rails, 5,028 for all rails at d = 5.
  Zero exceptions. The XORs follow from the gate clauses alone.
- Two-copy SAT call: two extensions of the same original assignment that differ on a fresh
  variable is UNSAT (fresh variables are functions of the old ones).
- Model sets projected on (x1, x2, x3), by blocking-clause enumeration, arm a against arm b: equal
  on 4 planted + 2 uniform targets at d = 5 and at d = 6 (6 models = the 6 orders of the one
  planted triple; 0 models for uniform).

Rails cannot change a SAT/UNSAT answer. The producer's README says a direct equisatisfiability
argument "was not written"; the checks above supply it for the instances sampled.

## 2. Answers verified independently

`own_verify.py` imports nothing from the producer or the repository: own F_2[t]/(t^131+t^8+t^3+t^2+1)
arithmetic (shift-xor multiply, Euclid inverse, half-trace lift), and a different decision method
(table of all +-P_i +- P_j including i = j and the 2-torsion point (0,1), then 2(A+1) lookups of
R -+ P_k). Targets were read from `results.json`; the reruns in section 3 regenerate the same R
from the seed.

- A = 17, 34, 74, 131, 252 at d = 5..9, as stated.
- All 50 targets (d = 5..8: 8 uniform + 4 planted; d = 9: 2 uniform): my triple set equals
  `truth_triples` exactly; every planted target has exactly one decomposition, the planted one;
  no uniform target decomposes.
- No degenerate decomposition (repeated abscissa, or abscissa 0) exists for any target, so the
  S'4 instance, which does not exclude them, cannot answer SAT where pair enumeration says no.
- Every decided arm status in `results.json` (235 rows: 80 SAT, 155 UNSAT) agrees with my
  decision; no target has two different decided answers.

## 3. Reruns

| rerun | rows | result |
|---|---|---|
| `b_run.py b_d5.jsonl 5 a,b,c,a-scan,L-scan` + `analyze.py` (13 s) | 60 | R, status and conflicts identical to `results.json` on all 60; table identical (L-scan 79.2 = 0.583x, 2.56 per leaf) |
| d = 7, arms a, b, 8 uniform | 16 | identical conflicts on all 16 |
| d = 8, arms a, b, 8 uniform | 16 | identical conflicts on all 16 (a 183,160 to 320,914; b 2,722 to 14,068) |
| d = 8, arms c, L-scan, uniform 0 and 1 | 4 | c 7,919 and 7,402 (producer rows: same); L-scan 427 and 449 (same) |

The solver is deterministic, so identical counts show the pipeline is honest, not that the effect
is robust. Robustness check I added (`shuf.py`): same instances with clause and XOR order shuffled.

| d | runs | arm a mean | arm b mean | a / b pooled | a / b per run, range |
|---|---|---|---|---|---|
| 7 | 8 targets x 3 shuffles | 1,044 | 244 | 4.3 | 1.7 to 13.8 |
| 8 | 3 targets x 2 shuffles | 282,978 | 4,919 | 57.5 | 35 to 112 |

The d = 8 reduction for arm b holds under perturbation; the d = 7 ratio stays below 5.
Arms c and L-scan were not perturbed: treat their ratios as single-configuration figures.

## 4. Fairness

- **Options**: identical across arms (`dr.CMS`: `threads=1`, `verbose=1`, defaults; only
  `time_limit` / `confl_limit` differ). The panel also used defaults (`Solver(threads=1, confl_limit,
  time_limit)`), on pycryptosat 5.15; here 5.18.0.
- **Instrument**: the panel's figure is the smallest deciding `confl_limit` by bisection; this
  pilot reads the solver's verbose line. I ran the panel's `weil.cmsConflictBracket` on d = 7
  uniform 0..3: 929 / 746 / 1,285 / 703 against 931 / 749 / 1,287 / 705 from the verbose line; the
  same bisection on arm b: 260 / 179 / 375 / 171 against 262 / 181 / 377 / 173. Same instrument to
  within 3 conflicts. Not checked at d = 8 (too slow for the time box).
- **29.5x against the panel's 36.7x**: different targets (seed, 8 against 4) and solver version;
  the panel's mean 312,231 lies inside this pilot's per-target range and inside my shuffled range
  (140,826 to 404,229). Consistent, not a reproduction of the panel's cell.
- **Means against medians** (uniform targets, conflicts):

| d | arm | mean | median | min | max | median / pairs | a / arm by medians |
|---|---|---|---|---|---|---|---|
| 7 | a | 992.5 | 978.5 | 705 | 1,287 | 0.36 | - |
| 7 | b | 259.0 | 252.5 | 173 | 383 | 0.093 | 3.9 |
| 7 | c | 285.4 | 246.5 | 199 | 410 | 0.091 | 4.0 |
| 7 | L-scan | 234.5 | 228 | 212 | 268 | 0.084 | 4.3 |
| 8 | a | 251,039 | 249,782 | 183,160 | 320,914 | 29.3 | - |
| 8 | b | 5,025 | 3,794 | 2,722 | 14,068 | 0.45 | 65.8 |
| 8 | c | 7,988 | 7,888 | 7,402 | 8,785 | 0.93 | 31.7 |
| 8 | L-scan | 425 | 418 | 404 | 464 | 0.049 | 598 |

  No cell is carried by one target in the producer's favour. Arm b's d = 8 mean is pulled up by
  one target (14,068); the median is lower. The d = 7 leg fails by medians too.
- **Linear-leaf arm against its natural reference.** It is not a post-pass on the repository
  encoding: it is a different system (S3(x1,xR,u), S3(x2,x3,u), a 131-bit intermediate u, 4,303
  variables against 1,510) with the 2+1 scan imposed by the outer loop, and only selector rails.
  The protocol did name it in advance as a "c variant", so counting it in the rule is not post
  hoc, but "591x against the repository encoding" compares two algorithms, not two encodings.
  Against the scan it performs:

| d | conflicts (mean) | x pairs | x scan count 2^d-1 | x A |
|---|---|---|---|---|
| 5 | 79.2 | 0.58 | 2.56 | 4.66 |
| 6 | 124.0 | 0.22 | 1.97 | 3.65 |
| 7 | 234.5 | 0.087 | 1.85 | 3.17 |
| 8 | 425.0 | 0.050 | 1.67 | 3.24 |
| 9 (2 targets) | 817.5 | 0.026 | 1.60 | 3.24 |

  It is 1.6 to 2.6 times the scan count and 3.2 to 4.7 times A (the loop also visits the
  abscissae that do not lift). It is never below its own reference; the falling "x pairs" column
  is A / C(A,2), not solver behaviour. One leaf costs 52 ms at d = 8 (13.2 s / 255).
- **Censoring.** d = 9: arms a and b are both censored at 45 s on 2 targets, added post hoc; the
  ratio 34x there is not defined. Arm b's own trend is the informative part: 0.096x, 0.59x,
  >= 1.17x of the pair count at d = 7, 8, 9, so the railed free solve grows faster than the pair
  count (about 6x per dimension against 3.2x to 3.7x) and is not shown to stay near it. Arm c:
  0.11x, 0.94x, >= 3.8x. Censored conflict counts are coarse (values 767, 1,023, 1,535, 35,840
  suggest the counter is read at 2^k or 1,024 boundaries on time-out), so the lower bounds are loose.
  The a-scan control at d = 8 is a lower bound (22 capped leaves), which only strengthens the
  attribution to the rails (c / a-scan <= 0.031).

## 5. Against the real baseline: pair enumeration

Wall times from my reruns (load about 2, niced; solve time only, instance build excluded).

| d = 8, per non-decomposable target | conflicts (mean) | wall s | x plain-Python pair enumeration (0.82 s) | x table lookup (0.017 s) |
|---|---|---|---|---|
| pair enumeration, producer's `dr.pairDecision`, Python | - | 0.82 | 1 | 48 |
| table of +-P_i +- P_j built once per d (0.98 s), then 2A lookups, own Python | - | 0.017 | 0.02 | 1 |
| (a) repository S'4 | 251,039 | 5.80 (8 targets) | 7.1 | 340 |
| (b) + rails, free | 5,025 | 4.77 (8 targets) | 5.8 | 280 |
| (c) + rails, x1 loop | 7,660 | 9.35 (2 targets) | 11.4 | 550 |
| L-scan | 438 | 13.2 (2 targets) | 16.1 | 780 |

At d = 7: (a) 0.027 s, (b) 0.19 s, pair enumeration 0.27 s in Python, table lookup 0.010 s. Arm (a)
is faster than the Python loop there, but that compares compiled CryptoMiniSat with interpreted
Python; against the lookup it is 2.7x slower, and (b) is 7x slower than (a).

The solver arms run in C++; the baseline is interpreted Python. A compiled pair enumeration
would widen every ratio in this table by one to two orders of magnitude. Instance construction
(`buildS4`, `railify`, solver load, about 1.2 s per target at d = 8 from the run totals) is not in
any arm's wall time. The table baseline uses O(A^2) memory, which is trivial at these sizes.

So: the 50x in conflicts buys 1.2x in time at d = 8 (5.80 s to 4.77 s), nothing at d = 7 (7x
slower), and no arm is within a factor 5 of even the slow baseline.

## 6. Does INCONCLUSIVE follow

Yes. "Transfers" needs >= 5x and <= 10x pairs at d = 7 and d = 8; d = 7 gives 3.8x / 3.5x / 4.2x
(3.9 / 4.0 / 4.3 by medians, 4.3 pooled under clause shuffling for arm b). "Does not transfer"
needs < 2x at d = 8 for all three; they are 31x to 591x. Neither holds, so inconclusive. The rule
was applied as written and not bent towards "transfers"; that is to the producer's credit.

## Problems

1. The large effect exists at exactly one decided size (d = 8). d = 7 is a 4x effect below the
   threshold; d = 9 is censored in both arms on 2 targets. No scaling claim is supported, and arm
   b's multiple of the pair count rises at every step.
2. "Reduction 50x / 31x / 591x" is a ratio of solver counters. In wall time the reductions are
   1.2x, 0.6x and 0.44x (slower), and all arms lose to pair enumeration by 6x to 16x (Python
   baseline) or 280x to 780x (table lookup).
3. The 591x arm (L-scan) is a different formulation with the scan built in. It should be
   reported as its own arm against the scan count (1.6x to 2.6x) or A (3.2x to 4.7x), not as a
   post-pass result against the pair count.
4. One solver configuration and one clause order per instance; deterministic reruns do not
   measure variance. My shuffles cover arm b only (holds at d = 8, 35x to 112x).
5. Per-conflict cost differs by 40x to 1,300x between arms, and decisions and propagations are
   not exposed, so the conflict counter does not measure the same work in each arm even though it
   is the same counter.
6. Wall times exclude instance construction and the rail pass.
7. Protocol-before-data rests on file times only; development probes had already shown arm b
   below arm a at d = 7 before the protocol was written (disclosed).
8. d = 9 arms a and b are post hoc and the planned d = 9 cells were not run (disclosed).
9. Process: the producer's `pkill -f` terminated two runs of the sibling pilot C (disclosed);
   those files need a rerun by their owner.

Not checked, so default to weaker than stated: instrument equality at d = 8; arms c and L-scan
under perturbation; full reruns of c, a-scan, L-scan at d = 7 and d = 8 (2 of 8 targets at d = 8
only); all planted-target conflict figures beyond d = 5; every d = 9 conflict figure; the figure;
L-plain-scan, L-free, L-rail-free rows.
