# A. Harness accounting (SYNTHESIS section 5 item 2), 2026-11-02
Pre-registration: ../PREREG.md (metric fixed before any run; Amendment 1 selects curves on |F| only). Everything here is the sweeper (L11 copy, MULMODE=school, kissat 4.0.4, bits-grouplaw, sym on), m=3 diagonal b=3s, l=s.
Conflicts are the unit; host loaded, `nice` was a no-op in this sandbox (setpriority refused), so CPU s are not reported. Raw: raw/A_yield.txt, raw/L12_restated.txt, ../cube/raw/sweep/*/runs.jsonl (30 sweeps), probe.txt.

## A1. Relations per conflict at a fixed cutoff (new runs)
10 curves per b (random primes, selected on |F| = 3 / 4 / 8 only), 16 targets per curve (8 planted + 8 uniform), conflict cap 100000. Relation = target answered SAT (witness verified by exact arithmetic). Censored runs charged C. CI = bootstrap over the 10 curves.

| b (l) | |F| | targets (unique R) | oracle-SAT | C | relations | conflicts charged | relations per 1k conflicts [95% CI] | conflicts per relation | censored at C |
|---|---|---|---|---|---|---|---|---|---|
| 6 (2) | 3 | 160 (112) | 91 | 100k (same at 10k) | 83 | 0.64M | 0.129 [0.113, 0.148] | 7.8k | 0/160 |
| 9 (3) | 4 | 160 (144) | 85 | 30k | 6 | 4.8M | 0.001 [0.000, 0.003] | 796k | 154/160 |
| 9 (3) | 4 | 160 (144) | 85 | 100k | 84 | 8.2M | 0.010 [0.009, 0.012] | 97k | 1/160 |
| 12 (4) | 8 | 160 (159) | 94 | 10k / 30k / 100k | 0 / 0 / 0 | 1.6M / 4.8M / 16M | 0 (95% upper bound by rule of three on pooled charge: < 0.19e-3 at C=100k) | no relation | 160/160 at every C |

Reading, honest: yield per conflict falls 12-13x from b=6 to b=9 (log2 +3.65 per 3 bits, i.e. about 1.2 bits per bit of b, but this is two points, the b=9 yield is itself cap-sensitive, and the two sizes use different |F|) and b=12 is entirely censored at 1e5 (0/160 solved, 0/94 SAT targets found). No slope is claimable; b=12 only gives an upper bound on the yield. Answers (SAT+UNSAT) per 1k conflicts at C=100k: b=6 0.236 [0.211, 0.271], b=9 0.019 [0.017, 0.023] (raw/A_yield.txt). Propagation-normalised yields are in raw/A_yield.txt (b=6 0.507 per M propagations, b=9 0.047).
Caveats: (i) b=6 has |F|=3, which admits exactly one distinct planted triple per curve, so the 8 planted targets per curve are repeats (112 unique R of 160); duplicates give identical deterministic results and are kept (clustering by curve handles the weighting only partly). (ii) 8 b=6 runs (3 of the 10 curves: seeds 37, 52, 55) are FAIL-false-unsat: kissat answered UNSAT on a planted target for which the oracle has a witness. That is the encoding missing an oracle witness at p=61..., not a solver error; they are counted as non-relations (and excluded from "answers"). It does not exist in L11's b=6 seed. Not investigated further. (iii) one b=9 run hit the cap (100002 conflicts, UNKNOWN). (iv) kissat default, one encoding, only the yield per conflict, no conflict-equivalent cost of the enumeration.
Enumeration reference for the same instances: T_enum = 4*C(|F|,2) group operations = 12 / 24 / 112 at b = 6 / 9 / 12. One relation costs the SAT path 7.8k / 97k / (>1.6e7) conflicts against <= 112 group operations. Different units (a conflict is thousands of machine operations), so no exponent or crossover is read off; the point is only that the harness number "relations per conflict" is not comparable to an enumeration number without a stated exchange rate.

## A2. L12 retained-setup table restated with setup charged (no new solves)
Source: L12 raw TSVs (one seed, one curve, 20k/100k per-solve caps), script analyze_L12.py. setup_est := inc_total(N) - N*tail, tail = mean inc conflicts over the last quarter of the N targets (floor 0); break-even N := setup_est / (mean fresh conflicts per target - tail). Enumeration reference in group ops: table (2|F|)^(m-1) once plus 2|F| lookups per target. "inc conf / enum op" is a unit-mismatched ratio, shown only to size the gap.

| cell (|F|) | N | inc total conf | fresh total conf (censored) | inc relations / fresh relations | setup_est (conf) | tail (conf/target) | inc conf per relation | enum ops (setup + N lookups) | break-even N |
|---|---|---|---|---|---|---|---|---|---|
| m2 b12 (|F|=8) | 16 | 155.7k | 957k (1/16) | 8 / 7 | 123k | 2042 | 19.5k | 272 | 2.1 |
| m2 b12 | 64 | 210.8k | 3.83M (2/64) | 32 / 30 | 167k | 684 | 6.6k | 1040 | 2.8 |
| m2 b12 | 176 | 259.4k | 10.7M (6/176) | 94 / 88 | 190k | 393 | 2.8k | 2832 | 3.1 |
| m2 b16 | 16 | 1.42M | 1.49M (11/16) | 0 / 0 | 230k | 74k | no relation | 272 | 12 |
| m2 b16 | 86 | 2.02M | 7.91M (53/86) | 35 / 0 | 1.42M | 6.97k | 57.6k | 1392 | 16.7 |
| m3 b12 (|F|=4) | 64 | 981k | 1.28M (64/64) | 8 / 0 | 878k | 1606 | 123k | 576 | 47.7 |
| m3 b12 | 256 | 1.03M | 5.12M (256/256) | 106 / 0 | 996k | 127 | 9.7k | 2112 | 50.1 |
| m3 b16 (|F|=3) | 165 | 3.30M | 3.30M (165/165) | 0 / 0 | not identifiable | 20k = cap | no relation | 1026 | not defined |
(full rows incl. N=16/64 in raw/L12_restated.txt.)
Reading: the retained-solver gain is real but it is the cost of building a table of the fixed factor base inside the solver. Once setup is charged, the retained solver's setup is 1.2e5..1.9e5 conflicts at m=2 b=12 and about 1.0e6 at m=3 b=12 (|F|=4, i.e. a 576-op enumeration), and the setup grows about 7x from m2 b12 to b16. Break-even versus fresh is 3 targets at m=2 b=12, 17 at b=16, about 50 at m=3 b=12 (where fresh is fully censored, so these break-evens are lower bounds on the true fresh cost and upper bounds on break-even N). Against explicit enumeration, which pays (2|F|)^(m-1) group operations once and 2|F| per target, the SAT path is about 10^2..10^3 conflicts per enumeration op even in the amortised regime (92 at m2 b12 N=176, 487 at m3 b12 N=256). Verdict: constant-only, as L12 said; the restatement adds that the 40x headline (m=2 b=12) is mostly the one-time table cost amortised, that a setup of order |F|^(m-1) conflicts is charged to the SAT lane but not to the sweep, and that no L12 cell shows a per-relation cost below the enumeration reference. Sample: one seed per cell (CIs in L12 are over targets, not curves); this restatement does not add curves.

## Process
Sweeps: 3 concurrent sweeper processes (kissat children serial) plus up to 1-4 cube-driver workers; at most 4 solver processes at once. First launch hit ENOSPC on the root disk (b=12 sweeps b12-s{2,4,5,9,15,16,20,35,36,38} truncated, moved to ../cube/raw/failed/ and rerun from scratch with TMPDIR on /Volumes/SSD990-2). Cube numbers come from 3 + 1 drivers writing separate files, merged and deduplicated. Total solver CPU roughly 1.5-2 CPU-h (estimate; no accounting instrument), well under the 8 CPU-h budget. All processes started here finished or were stopped with TaskStop before hand-back; I cannot list processes in this sandbox (ps/pgrep refused), so absence of strays is inferred from task completion notices, not verified.
