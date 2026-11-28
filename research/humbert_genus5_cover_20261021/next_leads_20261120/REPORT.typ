#set document(title: "Genus-5 cover against a matched rho")
#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[Genus-5 cover against a matched rho:\ the old "below rho" was mostly a weak rho; the best IC arm reaches 0.77× of a 4-thread rho at q = 32003]\
  #v(2pt)
  Study `research/humbert_genus5_cover_20261021/next_leads_20261120` · 2026-11-20 · follows `levers_20261116` (PR \#4065)
]

#v(6pt)
*Question.* `levers_20261116` left four leads:
- re-balance the factor base now that relations are cheap;
- speed up setup (23 % of the IC time);
- build a rho matched to the 4-thread IC arm;
- an L2 rerun.

The first three were built in isolated crate copies and checked by independent verifier agents
(workflow `wf_51f8d293-995`); all three were accepted. The L2 rerun is blocked: isolab-eks needs
an AWS identity this session does not have. [pcc:unverified]

*Scope and standing.* Toy fields only. The campaign is 3 values of $q$ × 5 seeds × 10 arms = 150 runs
under `PROTOCOL.md`, frozen before data, with one binary (`bin/humbert-v7`, sha256 prefix
`a4b55b71`) and serial runs in rotated order. There is no addendum. The host is a 4-vCPU shared
VM, a different container from v6 (L0). Every wall time is exploratory and an unverified
assertion [pcc:unverified]. No leaderboard row: there is no replay certificate or
`VALIDATION.json`.

= Correctness and counted identity

- All 120 IC runs are solved with $x P = Q$, and all 15 matched-rho runs are solved with
  abandoned_walks = 0.
- The 7 failures are all the old scalar rho (`rho_s`) stopping at its hard cap of $2 dot 10^8$
  steps (2 at $q = 24001$, 5 at 32003). Its actual times are therefore censored. Its expected
  time, from measured throughput, is not affected. [pcc:unverified]
- *C0:* `comb6` reproduces the v6 `comb` counted fields on 15/15 instances. `comb7` and
  `comb7_aa`, with `HUMBERT_SETUP_FAST`, equal `comb6` on 30/30 runs. The same instance gives
  the same relations, matrix and $x$, so the setup lever is pure speed.

= Predictions against the data

#let rt = csv("results/ratios_v7.csv").slice(1)
#figure(
  text(size: 8pt, table(
    columns: 6, align: right,
    [*q*], [*metric*], [*median*], [*min*], [*max*], [*n*],
    ..rt.map(r => (r.at(0), r.at(1), str(calc.round(float(r.at(2)), digits: 3)), str(calc.round(float(r.at(3)), digits: 3)), str(calc.round(float(r.at(4)), digits: 3)), r.at(5))).flatten()
  )),
  caption: [Paired per-instance ratios (`results/ratios_v7.csv`, L0). S1 = t_setup comb7/comb6, S2 = t_ic comb7/comb6, AA = comb7_aa/comb7, dNN = dlpf:0.NN/comb7, M1 = rho_m/rho_s steps per second.],
)

- *S1: holds.* Setup with SETUP_FAST is 0.057–0.066× (bound 0.20); setup is now about 0.1 s.
- *S2: holds.* Paired t_ic comb7/comb6 is 0.654, 0.744 and 0.786, outside every A/A range (0.966–1.214). [pcc:unverified]
- *F1: holds.* At $q = 32003$, d25, d30 and d35 are 0.828, 0.827 and 0.834 of comb7 (max 0.884). The
  cost model predicted 6–17 %; the measured gain is about 17 %. At 16001 only d30 and d35 help
  (0.91). d25 is slower there, because relation collection grows faster than LA shrinks. [pcc:unverified]
- *F2 (two-sided).* lp40/lp50 is 1.26, 1.18 and 1.12, so lp's optimum stays at 0.5. lp50/comb7 is 1.10, 1.01
  and 1.03; with dlpf re-tuned, dlpf is ahead again.
- *M1: holds.* Matched rho throughput is 13.9–14.5× the scalar reference: 25 M against 1.8 M
  steps/s. Of that, about 4× comes from threads and the rest from batched affine walks and Barrett
  arithmetic. The negation map halves the expected steps on top of this. [pcc:unverified]
- *M2: holds.* 15/15 solved. steps/expected median 1.09, range 0.24–2.06; the single-solve scatter is
  why the expected time is the primary reference.

= Distance remaining to the matched rho

#figure(image("figures/v7_ic_vs_rho.svg", width: 74%), caption: [Median IC time per arm, and the median expected time of the matched (rho_m) and the scalar (rho_s) rho, same instances (L0).])

#figure(image("figures/v7_gap_to_rho.svg", width: 70%), caption: [Median per-instance IC time / expected rho time. Solid: against the matched rho. Dotted: against the scalar rho used before.])

Best IC arm over the matched rho's expected time (per-instance median) [pcc:unverified]:

#align(center, table(
  columns: 5, align: right,
  [*q*], [*best arm*], [*IC / matched rho (expected)*], [*IC / matched rho (actual)*], [*IC / scalar rho (expected)*],
  [16001], [d35], [1.68], [1.63], [0.084],
  [24001], [d30], [1.19], [1.34], [0.057],
  [32003], [d35], [0.77], [1.19], [0.039],
))

What this means and does not mean:

- *The old headline was mostly a weak reference.* Against the scalar rho, the same IC arms sit at
  0.04–0.09 of rho's time. Against the matched rho, they sit at 1.68, 1.19 and 0.77. Earlier
  "below rho" statements in this line (`la_solver` §5, `levers` §4) are superseded by this table. [pcc:unverified]
- *The crossing is a slope effect, not a constant.* Over 16001–32003 the median time slope is about
  1.33 for the re-tuned IC arms (comb7: 1.55) and 2.42 for the matched rho's expected time. The
  q-span is a factor of 2 and there are 5 instances per $q$, so these slopes are rough. The
  expected crossing of a known attack on toy fields is not a break of any curve.
- *Actual solve times* are noisier: matched rho got lucky at 32003 (median 5.35 s against 8.47 s
  expected), and against actual times the best IC arm is still 1.19×. The frozen primary reference
  is the expected time.
- *§5a band.* In counted units the order flips across the band: priced at $w$ GAE per native mulmod,
  the best IC arm costs 0, 2.30 and 23.0 times the matched rho's group operations at
  $w = 0, 0.1, 1$ ($q = 32003$). No ranking is made in counted units. The wall-time ratio implies
  $w approx 0.03$ GAE per mulmod on this host, inside the band. The ranking above is a
  same-host wall-time ranking only. [pcc:unverified]

= Where the time is now

#figure(image("figures/v7_phases.svg", width: 100%), caption: [Phase medians (L0). With setup gone, comb7 at q = 32003 is 73 % LA; d35 moves work from LA (3.5 s) to relations (2.4 s).])

= Leads this opens

+ *LA is again the largest phase* for the balanced arms (d35 at 32003: 3.5 s LA, 2.4 s relations). A block
  method or a better threaded SpMV would move the balance point further down.
+ *Strengthen rho further before any claim:* a precomputed table across instances, SIMD K4, and the
  $FF_(q^4)$ Frobenius, if the curve's structure allows it. Each makes the gap honest.
+ *Larger $q$* (48 000–64 000) at matched rho, to test the slope gap with more than a factor 2 in $q$.
+ *L2 rerun with a replay certificate* via isolab-eks, before any leaderboard row.

= Reproduce

```
cargo build --release && cargo test --release          # 19 tests
BIN=next_leads_20261120/bin/humbert-v7 BIN_SHA256_PREFIX=a4b55b71 PRIMES=16001 \
  ./next_leads_20261120/run_v7.sh next_leads_20261120/campaign/q16001   # likewise 24001, 32003
python3 next_leads_20261120/analyze_v7.py
```
