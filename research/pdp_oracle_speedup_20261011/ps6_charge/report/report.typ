#set page(margin: 1.7cm, numbering: "1")
#set text(size: 9.2pt)
#set heading(numbering: "1.")
#show raw: set text(size: 7.8pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[PS-6 results report: the pairs-and-solve kernels charged in the counted unit] \
  #text(size: 10pt)[PDP oracle speedup program, `research/pdp_oracle_speedup_20261011/ps6_charge/` --- 2026-10-31 \
  Frozen protocol `protocols/PS-6.md`. Class: engineering measurement; stage diagnostics for the kernel.]
]

= Scope and what ran

Three decomposition oracles of the same pipeline (`ic.pipeline`, factor base `binary-subspace:dimension=l`, `l = n/3`, three summands, random targets) were charged in ecbench's counted unit `S` (group-addition equivalents over $sqrt(r)$) on the boundary ledger's char2 rungs, the random binary curves of $n = 15, 18, 21, 24, 27$ (registered ICV1 slugs in the study README), one target per run, the same curve, subgroup, factor base, targets and seeds for every arm: *(a)* the baseline `semaev_decomp::decompose` (`semaev-s4-baseline`), *(b)* the PS-2 group-law kernel (`semaev-s4-grouplaw`), *(c)* the library's meet-in-the-middle pair table (`mitm`, and its negation-folded variant), against the matched `rho.negation`.

*Sessions.* Five counted sessions (8 targets, 2 rounds, 1 round at $n = 27$; 432 runs, all verified) recorded 2026-10-18 23:27--23:44 UTC by the binary of commit `9e7e9a8a5`; five timed sessions (8 targets x 5 rounds + 1 warm-up, 4 targets at $n = 27$; 1,296 runs, 1,080 of them measured, all verified) recorded 2026-10-31 19:25--20:16 UTC by the binary of commit `7ded1ca84`, with an A/A control arm. The base branch was merged between the two groups, so the counted sessions were replayed by the later binary: every replay reproduced. One host class: Intel Xeon 2.10 GHz KVM guest, 4 vCPU.

*The unit.* The group-law kernel's pairs have no pinned conversion in `docs/ic/calibration.json`, so they are counted, recorded and left out of `S`: the kernel row is a *lower bound*, and its ratio to the baseline is not a speedup. Every arm's unpriced native operations are priced at *0, 0.1 and 1 GAE each* (AGENTS.md section 5a, the same way for every arm, rho included) and an order is ranked only where it holds at all three weights. Four further kernel points use conversions that are *not pinned* and are not ranked.

= Observations

#figure(
  image("s_over_floor.svg", width: 96%),
  caption: [`S / floor` per rung, mean over targets with 95% bootstrap intervals; shaded: the section 5a band for each arm (unpriced operations at 0 to 1 GAE; dotted edge = 1 GAE). Source `raw/summary.json` from the sealed counted sessions.],
)

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto, auto),
  align: (right,) * 8,
  table.header[*n*][*rho (negation)*][*S4 baseline*][*kernel, w=0 (recorded)*][*kernel, w=0.1*][*kernel, w=1*][*MITM*][*MITM, neg.-folded*],
  [15], [2.10], [1,270], [33.8], [44.4], [139.7], [57.9], [56.1],
  [18], [1.97], [3,041], [26.9], [55.0], [308.3], [65.9], [60.2],
  [21], [1.31], [8,560], [22.9], [94.6], [740.0], [73.6], [69.9],
  [24], [1.29], [19,067], [16.3], [211.1], [1,964], [93.9], [80.9],
  [27], [1.09], [114,574], [22.4], [1,012], [9,915], [197.0], [182.4],
)

Mean `S` over 16 targets-by-rounds (8 at $n = 27$), floor `S` = 0.886. Intervals are in `raw/summary.md`; e.g. at $n = 27$: baseline 107,540--121,184, kernel w=0 21.1--23.5, kernel w=1 9,306--10,486, MITM 186.5--206.8. The baseline's `S` at $n = 27$ (114,574) is the boundary ledger's row (115,459) reproduced in ecbench. Rho is itself a lower bound (its canonicalisations are counted, not charged); at w = 1 it reads 3.6, 3.5, 2.4, 2.5, 2.1.

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto),
  align: (right,) * 7,
  table.header[*n*][*baseline / rho*][*kernel / rho, w=0*][*kernel / rho, w=1*][*MITM / rho*][*kernel / MITM at w = 0, 0.1, 1*][*order kernel vs MITM*],
  [15], [605], [16.1], [38.4], [27.6], [0.58, 0.77, 2.41], [unresolved],
  [18], [1,544], [13.6], [87.4], [33.5], [0.41, 0.83, 4.68], [unresolved],
  [21], [6,560], [17.5], [302], [56.4], [0.31, 1.29, 10.1], [unresolved],
  [24], [14,806], [12.6], [800], [72.9], [0.17, 2.25, 20.9], [unresolved],
  [27], [105,128], [20.5], [4,721], [180.8], [0.11, 5.14, 50.3], [unresolved],
)

Ratios of sums over targets, each arm and rho priced at the same weight. Baseline, kernel and MITM are above rho at every rung and every weight. Baseline above kernel and above MITM at every weight. Kernel against MITM: below at w = 0, above at w = 1; the order changes inside the band, so it is *unresolved* at every rung.

#figure(
  image("band_order.svg", width: 100%),
  caption: [Sum `S`(A) / sum `S`(B) at w = 0, 0.1, 1 GAE per unpriced operation, per rung (source `raw/summary.json`, `order_across_band`). A band that straddles 1 is an unresolved order.],
)

*Stage ratios, baseline over kernel* (sum `S` baseline / sum `S` kernel over the same targets; paired; intervals over targets): w = 0: 37.6, 113, 374, 1,171, 5,119; w = 0.1: 28.6, 55.3, 90.5, 90.3, 113.3; w = 1: 9.1, 9.9, 11.6, 9.7, 11.6 at $n = 15 .. 27$. The estimates that are not pinned: each kernel pair priced as a baseline pair, 3.6, 3.6, 4.2, 3.4, 3.7 (the pair-count ratio); the pinned baseline-pair price times the measured kernel/baseline per-pair time ratio ("anchored"), 10.8, 17.8, 28.7, 31.2, 42.3; the measured kernel pair over a measured addition, 15.2, 26.0, 43.7, 53.6, 74.7. The measured kernel/baseline time ratio per pair falls with $l$: 0.264, 0.177, 0.137, 0.107, 0.088 (`raw/calibrate.jsonl`, 11 interleaved repetitions each, one reserved core).

#figure(
  image("exponents.svg", width: 92%),
  caption: [Fitted exponent $e$ of total operations $T = S sqrt(r) tilde r^e$ over the five rungs, with 95% bootstrap intervals over targets (the slope of `S`/floor is $e - 1/2$).],
)

#table(
  columns: (auto, auto, auto, auto),
  align: (left, right, right, right),
  table.header[*arm*][*e*][*95% interval*][*e minus the baseline's (paired)*],
  [rho (negation)], [0.405], [0.332--0.465], [-0.660],
  [S4 baseline], [1.063], [1.049--1.078], [0],
  [kernel, w = 0 (lower bound)], [0.436], [0.424--0.447], [-0.628 (-0.634 to -0.623)],
  [kernel, w = 0.1], [0.895], [0.882--0.908], [-0.168 (-0.172 to -0.165)],
  [kernel, w = 1], [1.036], [1.022--1.050], [-0.027 (-0.028 to -0.026)],
  [kernel, pair priced as a baseline pair (not pinned)], [1.059], [1.045--1.073], [-0.004],
  [kernel, anchored estimate (not pinned)], [0.892], [0.878--0.906], [-0.171 (-0.173 to -0.170)],
  [MITM], [0.645], [0.633--0.657], [-0.418 (-0.421 to -0.416)],
  [MITM, negation-folded], [0.639], [0.626--0.653], [-0.424],
)

The baseline's $e = 1.063$ reproduces the ledger's exponent (1.06, five sizes). The intervals resample targets only; they carry neither the five-rung model nor the calibration spread.

*Wall time* (timed sessions, solve wall per run; descriptive, a practicality note):

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto),
  align: (right,) * 7,
  table.header[*n*][*baseline ms*][*kernel ms*][*MITM ms*][*kernel / baseline (paired median, 95%)*][*MITM / kernel*][*A/A (baseline control / baseline)*],
  [15], [14.5], [5.8], [5.1], [0.385 (0.363--0.444)], [0.885 (0.806--0.941)], [0.999 (0.966--1.037)],
  [18], [41.2], [6.9], [5.6], [0.163 (0.154--0.179)], [0.807 (0.779--0.860)], [1.014 (0.984--1.056)],
  [21], [749], [33.8], [13.8], [0.044 (0.043--0.046)], [0.418 (0.389--0.468)], [1.019 (0.993--1.055)],
  [24], [2,521], [86.0], [23.3], [0.035 (0.034--0.036)], [0.272 (0.261--0.283)], [1.006 (0.983--1.017)],
  [27], [46,151], [1,156], [132.5], [0.025 (0.024--0.025)], [0.117 (0.113--0.126)], [0.999 (0.991--1.010)],
)

40, 40, 40, 40 and 20 paired runs per row. Isolation: 209--220 of 288 runs earned L2 at $n <= 21$ (the baseline's long runs less often: 18 of 40 at $n = 21$, 6 of 40 at $n = 24$, 0 of 20 at $n = 27$; one hypervisor steal tick on the run CPU is enough to lose L2, and a long run is likely to have one), so ecbench marks the $n = 27$ comparisons that include the baseline `descriptive`. Rho's wall is the same at every $n$ (38--42 ms) while its `S` is about 1, so it is dominated by a fixed cost, not by the walk. The measured $n = 27$ kernel/baseline wall ratio, 0.025, is within 6% of the anchored estimate's `S` ratio, 1/42.3.

#figure(
  image("wall.svg", width: 90%),
  caption: [Median wall time of the solve per run by arm and rung (timed sessions; levels in the title). Source `raw/summary.json`.],
)

*Memory (not in `S`).* The library's pair table at $n = 15 .. 27$ has 220, 1,016, 3,721, 18,711 and 69,118 entries (about 30--39 bytes per entry at $n >= 21$), 0.0--2.1 MiB of resident growth; every run's peak resident set is 100--108 MiB (the process). The baseline's span and the kernel's base list are a few kilobytes. Keys grow as $|B|^2$, so the table is the limit at larger $l$ (PS-8 measured 16.4 M keys at $l = 13$).

*Correctness.* Unit tests (4, release): the group-law formulas match `FastCurve::add` for $a in {0, 1}$, translation by the 2-torsion point is $x |-> sqrt(b)/x$ on every rational point of four random curves, the kernel's roots equal an exact pairwise enumeration, and the kernel returns the baseline's *witness* on all 60,128 non-degenerate targets of 30 random curves (22,920 with a witness; 9,404 degenerate targets skipped, which the oracle sends to the baseline). In the sessions the audit arm compared 27,938 witnesses with the baseline's and found 0 mismatches, and all 72 (workload, round) groups have identical targets tried, relations found, matrix rank and recovered scalar across the baseline, kernel and audit arms. All 1,512 measured recovered scalars were re-verified by arithmetic that shares no code with the harness ($[d]G = Q$, $[d+1]G != Q$), and `ecbench verify` replays reproduce: pre-merge audits replayed all 96 runs at $n <= 21$ and 12 runs at $n = 24, 27$; the post-merge binary replayed all 96 runs of each session at $n <= 24$ and 12 of 48 at $n = 27$.

= Reading (interpretation; labelled, scope: these five rungs, this host class, this unit)

- *Observation, stated again as a reading.* The kernel lowers the S4 oracle's cost against the baseline at every rung and weight. How much depends on the price of a kernel pair: about 3.4--4.2 if a pair costs what a baseline pair does (the smaller candidate set alone), about 9.1--11.6 at 1 GAE per pair, 11--42 at the anchored estimate, and 38--5,119 if pairs are free. None of these is a pinned figure, and a speedup is not claimed. It is a *stage diagnostic*.
- *Slope.* The finding rule asks whether the ratio to the floor changes slope. The kernel's fitted slope is below the baseline's over these rungs, by 0.03 (1 GAE per pair) to 0.17 (anchored estimate) with intervals that exclude 0 and by 0.004 if a pair is priced as a baseline pair (no change). What moves the slope is the per-pair cost ratio, which falls with $l$ (0.264 to 0.088) while the pair-count ratio stays 3.4--4.2; the likelier cause is a row set-up amortised over longer rows. If so the difference should vanish as the ratio flattens; five rungs ending at $l = 9$ do not show that. It is reported as an observation, not as a change of exponent.
- *Against the incumbent.* The meet-in-the-middle table is cheaper in `S` than the baseline at every rung (by 22.0 at $n = 15$ to 582 at $n = 27$ in `S`) and its exponent is 0.645 against the kernel's 0.89--1.04 at the priced points. Whether the kernel is above or below it in `S` is unresolved inside the band at every rung. At the anchored estimate the kernel is above it at every rung (2.0 to 13.7 times).
- *Against rho.* Every index-calculus arm is above matched rho at every rung and every weight; the gap grows with $n$ for the baseline, for MITM and for the kernel at w >= 0.1; the kernel's lower bound (w = 0) stays at 12.6--20.5 times rho.
- AGENTS.md section 3 class: engineering for the oracle stage (the cost fell, the ratio to the floor still rises with $r$).

= Not claimed, gaps

Nothing about ECC2K-130 and nothing about the $m = 83$ gate (an enumerator would need $l tilde 28$ at $m = 3$). No end-to-end speedup of the kernel: its pairs are unpriced in the canonical unit. One host class; no second host replayed these sessions; the validation artifacts were produced by the study's author. Five rungs ($n <= 27$, the ledger's char2 S4 ladder); no larger rung, since the baseline's cost grows about 18--20 times per three bits. The PS-3 bit-sliced arms were not charged (PS-3 found them no faster than the scalar kernel per pair). The `pdp-opcount` counters of PS-0 are absent from the merged library, so no counted carry-less-multiply ratio was measured; the wall-measured ratio stands instead. Conductor's control plane was unreachable. Files: `README.md` beside this report lists the requirement table, commands, deviations and every record path.
