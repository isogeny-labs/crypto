# Amendments (the frozen rule in PROTOCOL.md is unchanged)

## A1 — 2026-11-13 00:40Z — RunPod extension and msolve cross-check
Reason: coordinator relayed that the user asked to run on RunPod; pod bdr-chase-big (L3 share: 8 threads,
48 GB) has msolve 0.6.5-1build2. New cells, added before running them:
- pcdeg (same binary, sha256 recorded in MANIFEST) on the pod, 1 thread per process, <= 8 concurrent,
  prlimit 6 GB each, timeout 3 h: m=2 bool k=8,9 (A/A 2 draws); b3 k=5; b4 k=4; mix k=7; m=3 bool k=5,6;
  m=3 b3 k=3; m=3 mix k=4; m=4 bool k=3,4; extra draws (4/cell) on cells already run locally.
- Exhaustive-target census (all r in F_p) at m=2 bool k=4,5 for the Hypothesis-G clause (ii) test
  (distribution of D over every target, satisfiable and unsatisfiable). Added because clause (ii) is about
  the count of low-degree targets and needs every target, not samples.
- Second solver: msolve F4 (-v 2) on the same polynomial system written in the ordinary ring with explicit
  digit equations prod(d-c), on overlapping cells (m=2 bool k<=7, b3 k<=4, m=3 bool k<=4). Observables: msolve
  max step degree and unsat verdict (GB = {1}). Cross-check rule: msolve's verdict must agree with the grid
  zero count; msolve max degree is reported beside the PC degree D (a different degree object: ordinary
  ring, F4 strategy), and D <= msolve max degree is expected; any violation is reported.
- Families `linb3` (base-3 single linear equation) and `linrand` (random nonzero weights) for the theory
  section; `lin` (binary weights) and `sym` (unit weights, IPS check).
Kill/survive rule: unchanged; pod cells are read against the same rule.

## A2 — 2026-11-13 01:25Z — msolve cost ladder (added after the first 25 cross-check systems)
Reason: msolve reproduced D exactly on 25/25 overlapping systems, and its largest F4 matrix on univariate bases
was tiny (41 x 87 at B = 16) while it exceeded the quotient-ring column count on Boolean digits (47052 x 52140 at
k = 7 against 9908). The cost translation in PROTOCOL.md uses cols(D) of the quotient ring; that charge is not
what F4 pays. New cells (msolve only, systems dumped by pcdeg; --cmax 0 = dump without the closure where the
closure is too slow): uniint/unirand m=2 B in {32,64,128,256}; uniint m=3 B in {8,12,16,20,24}; bool m=2 k=8
(1 system); b3 m=2 k=5; b4 m=2 k=3,4; mix m=2 k=4,5,6; bool m=3 k=5. Observables: msolve max degree, largest
matrix, elapsed time. Reported beside (not instead of) the preset cols(D) translation; the frozen rule is read on
the preset translation, and a second reading with the msolve matrix size as T_dec is reported separately.
Operational notes: /usr/bin/time and bc are absent on the pod, so the first msolve pass ran through msrun.sh
(msolve's own "overall(elapsed)" is the time). The first pod driver stopped launching jobs after a job was
killed (xargs semantics); remaining jobs ran under a second driver. m=2 bool k=9 was cancelled (needs ~22 GB
dense rows, above the 6 GB per-process cap). Locally, lin k=14 (draw 2) and mix k=6 (draw 2) hit the 1200 s
timeout (censored, no value).

## A3 — 2026-11-13 02:00Z — disclosed additions made after seeing data (recorded late, not pre-registered)
- uniint/unirand m=2, B = 48, 64, 96, 128 re-run with cmax 140/200 (`runs/m2u.jsonl`): the registered runs
  used the default cmax = 40 < B + 2 and are censored (D > 40). Parameter fix only.
- m=3 b4 k = 1, 2 (local, `runs/m3.jsonl`): added after the m=3 b3 cells to get a second non-saturated m=3 family.
- Second draws of m=2 b4 k=4 and mix k=6 (local, `runs/m2x.jsonl`): A/A for cells that had one draw.
- Theory work (THEORY.md addendum, jstar.py, lemmaF_check.py) was developed after the first rows; its
  predictions were then checked against all rows, including the earlier ones. These are post-hoc fits of a
  proved bound to existing data plus out-of-sample checks on the cells that finished later
  (m=2 bool k=8, b3 k=5, b4 k=4, mix k=6/7, m=3 bool k=5, mix k=4, b4 k=2; census b3 k=3, bool k=5; all msolve
  dump-only systems).

## A4 — 2026-11-13 02:50Z — upper-half test D = ffd at m = 3, Boolean, k = 6..8 (requested by coordinator)
Reading fixed in advance: certified D = ffd (14, 14, 15 for k = 6, 7, 8) supports the upper half; any certified
D > ffd refutes "D = ffd" for that cell and is reported (it changes the projection); censored -> bound only.
Feasibility stated before running: at c = ffd the closures seen so far jump to (nearly) the whole ring, i.e.
dim A_{<=14} = 261,156 columns at k = 6 (dense rows ~136 GB in pcdeg), far above the 48 GB share, so the D <= ffd
certificate is expected to be censored by memory. Runs (pod, 1 thread each, unsat targets, p of 24 bits):
(a) pcdeg k = 6, 7, 8 with cmax = ffd - 1: certifies 1 not in V_{ffd-1} (lower half, cross-check of Theorem F);
(b) pcdeg k = 6 at c = 14 with a 30 GB cap and progress logging: rank growth beyond dim f*A_{<=2} = 172 before
the cap (a fall at ffd) is reported as partial evidence only, never as a certificate;
(c) msolve on k = 6 (one system) with the remaining memory, 3 h cap: a "no solution" with max degree 14 would
certify D_F4 = 14 = ffd for that system (F4 degree, ordinary ring) — the only route to an upper-half certificate.
Binary: pcdeg rebuilt with stderr progress lines only (closure code unchanged).
A4 outcome (03:25Z): lower half certified at k=6 (rank V_13 = 19 = dim A_{<=1}, 1 not in V_13) and k=7 (rank 22, 1 not in V_13);
k=8 not run (needs ~17 GB of dense rows for a theorem-covered check). Upper half censored: pcdeg k=6 at c=14 reached rank
10,000 of 261,156 columns in 2,357 s (> dim f*A_{<=2} = 172, so falls do occur at c = ffd = 14), stopped by the lab;
msolve k=6 reached F4 degree 14 (matrix 2,036,323 x 2,294,979) and crashed rc=139 at the 24 GB cap after 1,254 s, no verdict.
Records: runs/up_k6.*, runs/up_k7.*, podout/up/ (sha256 da9de0bb...f8daf).
