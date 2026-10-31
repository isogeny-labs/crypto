---
title: "Data request: the fall-degree paper (treatise Ch. 6 core)"
date: 2026-10-29
geometry: margin=2cm
---

## Purpose

This is a list of the data the paper still needs, one item per gap. Each item names:

- the claim it supports;
- what is on record now;
- what is missing;
- how to get it: a new run under a protocol record in `protocols/`, collecting output from a study already running, or work that needs a person (no compute).

**Status.** Nothing here has been run. The three protocol records are `draft`. Each validates against `experiment.protocol/v1` with 0 findings. None is frozen. Freezing waits on the prerequisites listed per protocol. A run then goes through the model tiers (director card, mechanical executor) on `isolab-eks`, after `scripts/checkpoint.sh init` (CLAUDE.md). Loading these files does not authorise a run.

![Data on record (filled) and requested (open, stars, circles) for the m = 2 binary descent](coverage.png)

## The paper's hypotheses and the data each needs

The paper proves:

- the first fall degree is a constant of the presentation: 2 for binary $S_3$ descent, 4 for PKM towers;
- the last fall degree is not bounded by it.

The hypothesis it cannot yet state is that **the last fall degree grows**. The table lists what would move that question, and what the paper's existing tables need to be complete and reproducible.

| # | Claim in the paper | On record | Gap | How | Priority |
|---|---|---|---|---|---|
| R1 | Thm 6.11: $d_F \ge 5$ certified (n = 33–45); Table 6.1 | 13 checked certificates; R39a/b and C21a/b closure A only | Certificates for R39a/b and C21a/b; random V at n = 41–51; polynomial V at n = 37, 39, 47–51 | **FD-BINARY-CLOSURE-EXT**, claims C1, C3 | 1 |
| R2 | Open problem: does $d_F$ reach 6? | R35a is exactly 5; no $W_5$ has ever been computed | Degree-5 closure on the 8 certified instances at n = 33–37 | **FD-BINARY-CLOSURE-EXT**, claim C2 | 1 |
| R3 | Table 6.3: KW's $D_{\rm reg}$ against our $d_F$ | KW published values only; no F4 step degree on our instances | $D_{\rm reg}$ on the same systems; reproduce KW at n = 34–48 | **FD-F4-STEP-DEGREE** | 1 |
| R4 | Table 6.4 and the C-step prediction $d_F=4+\lfloor(t-2)/4\rfloor$ | t $\le$ 10 medians session-reported, raw data lost | t = 1–14 re-run with committed rows; m = 4 rise/flat/fall reading | **Collect output** from the running relaunch: `research/prime_ic_program_20261029/fall_degree_relaunch/` (Amendments 7–8). No new protocol. | 1 |
| R5 | Every certificate "any reader can check" | sha256 values in `certificates.txt`; certificate files not public | Publish certificate files and `cert_check.py` with hashes | Aggregation: artifact store plus a public bundle | 1 |
| R6 | Table 6.3 transcription; which subspace KW used | Read from a photographed page | Check against the PDF; note KW's subspace and z | Literature (person); read HKY 2015 and KW 2019 in full | 1 |
| R7 | Closure times in Table 6.1 | Host not recorded | Host recorded for new rows | Inside FD-BINARY-CLOSURE-EXT (instance type, CPU, peak RSS per row) | 2 |
| R8 | Conclusion item 3: a measured crossover at k = 4 | m = 15 only, n = 2 per arm | 8 fresh targets per arm at m = 14–17, priced across the band | **K4-CHAR2-REPLICATION** | 2 |
| R9 | PKM raw records from the first run | Lost on the owner's Mac (`RESTART.md`) | Upload the Mac worktree | Owner action | 3 |

Out of scope for this paper: the frontier chapter's 9 unvalidated leaderboard rows. They are tracked by the validation gate, not here.

## Protocols

| protocol | question | arms and controls | size | route | prerequisites before freeze |
|---|---|---|---|---|---|
| [`fd-binary-closure-ext`](protocols/fd-binary-closure-ext/protocol.json) | Is $d_F \ge 5$ certified at n = 39–51? Does $d_F$ reach 6 at n = 33–37? | closure A (lfdclose2) against an independent certificate check; A/A on thread count; positive: known refuting instances; negative: satisfiable instances | ~36 instances plus 8 D = 5 closures; ~100 h | isolab-eks spot, search class, 64–128 GiB for D = 5 | extend `mktable.py` for D = 5 rows; pin its sha256; time one D = 5 closure (n = 33) |
| [`fd-f4-step-degree`](protocols/fd-f4-step-degree/protocol.json) | What is F4's $D_{\rm reg}$ on the same systems? Does it reproduce KW? | Boolean F4 (`pq_f4_f2`) against KW's published values; A/A on variable order; positive: d_F = 3 instances | ~135 instances; up to 72 h per large cell | isolab-eks spot, up to 256 GiB | write `examples/f4_f2_sysfile_degree.rs` (reads `.sys`, reports step degrees); pilot at n = 34, 40; check the KW transcription |
| [`k4-char2-replication`](protocols/k4-char2-replication/protocol.json) | At k = 4, is IC below the strongest rho at m = 16 and 17, across w = 0, 0.1, 1? | `ic-wied` against `rho-strong`; `rho-neg` regression; A/A | 4 arms × 4 m × 8 targets; ~30 h | isolab L2 measurement pool | build `rho.negation_strong` (ecbench-extend); `ecbench verify --replay` preflight |

**Protocol details.** Each record states:

- its claims, each with a class;
- its hypotheses, with dated predictions marked out-of-sample or calibration;
- a variable ledger in which every comparison is one-factor, absolute or A/A;
- the seed rule;
- the metrics, with exact or counted quantities as primary;
- stopping and invalidation rules;
- the verdict vocabulary;
- what is inadmissible.

**Seeds.** The k = 4 seeds are listed explicitly. The red team's plan wrote `sha256("gaudry-k4-redteam-v2" || m || i)` without a byte encoding. This record fixes it as UTF-8 `gaudry-k4-redteam-v2|<m>|<i>` before any data. The other two protocols derive their seeds from the protocol hash after freeze.

## What comes back, and where it goes

Every run writes `experiment.export/v1` beside its records. Each result has a fixed destination:

- `check-export --verify-files` and the `flow` count go in the run report.
- New certified rows are appended to `crypto-autoresearcher:analysis/semaev-m2-last-fall-degree/results/`. `papers/ecdlp-index-calculus-treatise/figures/make_tab_fall_degrees.py` then regenerates Tables 6.1–6.3, with a measured $D_{\rm reg}$ column in 6.3.
- R4 output updates `figures/data-new.json` and Table 6.4.
- R8 reaches the paper only through the validation gate.

No outcome promotes a hypothesis by itself: review-evidence and the PR decide.
