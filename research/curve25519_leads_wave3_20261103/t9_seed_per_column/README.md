# T9: selecting the PKM isogeny tower seed by J ÷ columns (results, 2026-11-04)

**Class:** exploratory lead. It has (a) an exact census, (b) an engine
measurement in counted F4 multiply-adds, and (c) an end-to-end ecbench
session in the counted unit S. It is a constant-factor change to the
algebraic arm `pkm-iso-t6-g4`. **No row here is a leaderboard claim**: there
is no validation gate (`VALIDATION.json`) for this lead. **Rho remains
ahead.**

- Protocol: [PROTOCOL.md](PROTOCOL.md), frozen at `505ecbb154`. It was not
  edited.
- Runner: [scripts/run.sh](scripts/run.sh), frozen. Stage (c) ran with one
  added flag, `--allow-busy`; see Deviations.
- Parent study: `research/curve25519_isogeny_seed_20261103` (T4/T5).

## Headline

- Seed 1308 is picked by J ÷ columns. On 16 fresh targets with full solves,
  its end-to-end S is **0.5605× that of seed 0 [95% 0.421, 0.747]** at
  w = 0.1 and w = 1, and 0.6431× [0.509, 0.813] at w = 0.
  - These ratios hold **with the seed search amortised** over targets on
    the same curve. Charged to a single target, the search adds
    13,528,559 group operations (37,349 in S): s1308 ÷ seed 0 becomes
    1397.6× at w = 0, 0.586× at w = 0.1 and 0.563× at w = 1, so the order
    does not hold across the band (validator E3).
  - The model predicted 0.497×. Measured ÷ model = 1.13.
- Against seed 1093 (picked by J), seed 1308 is **0.9047× [0.740, 1.106]**
  at w ≥ 0.1. P7's threshold was ≤ 0.90, so **P7 fails**, by 0.005. At
  w = 0 the ratio is 0.7231× [0.638, 0.820].
  - `ecbench band` orders s1308 < s1093 at every weight. The paired
    interval at w ≥ 0.1 includes 1.
- The run replicated T5. Seed 1093 is 0.6195× seed 0 [0.489, 0.785] at
  w ≥ 0.1. With 16 targets the interval now excludes 1; T5 measured
  0.641× on 6 targets.
- **Distance remaining** for the best PKM arm (s1308) at w = 0.1:
  - 6.23·10⁴× behind proj-a;
  - **4.73·10⁵× behind rho-neg**.

## (a) Census, exact (re-run of `scripts/select_seeds.py` on the parent census)

`data/census_t9.json` is byte-identical to the design-time
`analysis/census_t9.json` (`cmp` passed). The table below is the verbatim
`data/census_t9.txt`.
- Jc = the J/cols-selected seed. J = the J-selected seed. orc = the seed
  with the lowest M = columns ÷ covered of the 2,000.
- "better" = the number of seeds with M strictly lower than the Jc seed.
- "search" = the group operations of the 2,000-seed search (build +
  selector).

```
 k t | seed0 cols/cov | J-sel seed cols/cov | J/c-sel seed J cols/cov | Jc/s0  J/s0  Jc/J  Jc/orc | better | search
16 5 |  13/   56 |  348  22/  172 | 1597   162  18/  160 | 0.485 0.551 0.880 1.000 |    0 | 5.023e+06
16 6 |  28/  198 | 1381  40/  614 | 1381   632  40/  614 | 0.461 0.461 1.000 1.000 |    0 | 1.075e+07
16 7 |  68/ 1038 |  483  82/ 1630 |  278  1678  74/ 1514 | 0.746 0.768 0.972 1.000 |    0 | 2.795e+07
18 5 |   9/   14 |  700  25/  194 |  997   188  19/  186 | 0.159 0.200 0.793 1.000 |    0 | 5.990e+06
18 6 |  30/  230 | 1277  45/  554 | 1236   480  36/  480 | 0.575 0.623 0.923 1.000 |    0 | 1.259e+07
18 7 |  61/  912 |  918  84/ 1696 | 1955  1680  78/ 1644 | 0.709 0.740 0.958 1.000 |    0 | 3.135e+07
20 5 |  18/   72 | 1550  26/  214 | 1307   210  22/  210 | 0.419 0.486 0.862 1.000 |    0 | 6.935e+06
20 6 |  25/  172 | 1093  46/  528 | 1308   500  36/  498 | 0.497 0.599 0.830 1.000 |    0 | 1.353e+07
20 7 |  65/ 1188 |  804  86/ 1846 | 1594  1854  83/ 1840 | 0.824 0.851 0.968 1.001 |    1 | 3.276e+07
22 5 |  13/   40 | 1034  21/  224 | 1034   224  21/  224 | 0.288 0.288 1.000 1.000 |    0 | 8.319e+06
22 6 |  32/  270 | 1070  46/  568 |  284   458  34/  458 | 0.626 0.683 0.917 1.000 |    0 | 1.576e+07
22 7 |  62/  956 |  454  81/ 1906 |  454  1914  81/ 1906 | 0.655 0.655 1.000 1.000 |    0 | 3.686e+07
24 5 |  17/   76 |  797  24/  190 |  978   190  20/  190 | 0.471 0.565 0.833 1.000 |    0 | 9.405e+06
24 6 |  39/  364 | 1490  41/  514 | 1340   380  29/  380 | 0.712 0.744 0.957 1.000 |    0 | 1.757e+07
24 7 |  59/  900 |  702  86/ 1864 |  702  1864  86/ 1864 | 0.704 0.704 1.000 1.000 |    0 | 3.869e+07
```

These are facts, not predictions, and they match PROTOCOL.md:
- Jc/J ≤ 1 in all 15 cells. The two rules pick the same seed in 4 cells.
- The Jc seed has the lowest M of the 2,000 in 14 of 15 cells. At (20, 7)
  it is 1.001× the lowest.

## (b) Engine: `curve25519_family f4cost 20 <form> 6 4 100 2026111506 3600 2 full 0 30000000`

| arm | columns | covered | solving degree | terms | f4_muladds | muladds_per_target | muladds/relation at exact coverage | E = cols × that | capped | undecided | hits (verified) |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|:--|--:|:--|
| seed 0 (`isogeny`) | 25 | 172 | 5 | 16 | 71,009,400 | 710,094.0 | 541,665,483.03 | 1.3542·10¹⁰ | false | 0 | 0 (0) |
| seed 1093 (`isogeny@1093`) | 46 | 528 | 5 | 16 | 70,384,751 | 703,847.51 | 174,899,441.01 | 8.0454·10⁹ | false | 0 | 1 (1) |
| seed 1308 (`isogeny@1308`) | 36 | 498 | 5 | 16 | 71,009,508 | 710,095.08 | 187,081,535.71 | 6.7349·10⁹ | false | 0 | 0 (0) |

Derived values (exact arithmetic on the JSON fields):
- muladds_per_target s1308 ÷ s0 = **1.0000015**;
- E(s1308) ÷ E(s0) = **0.49735**;
- E(s1308) ÷ E(s1093) = **0.83712**;
- E(s1093) ÷ E(s0) = 0.59412.

C0 check: the seed-0 and seed-1093 outputs match the parent files on every
field except `wall_s`.

## (c) End-to-end: ecbench session `ECBS1hce736ca27358` (k = 20, 16 targets × 2 rounds, target_seed 2026111509)

- 160 of 160 records verified, 0 censored.
- Replay audit (`data/ecbench/verify.txt`): "audit OK: 160 records, 160
  verified, 16 replays reproduced, 0 problems". The replay seed is
  3858208747647974989 and the auditor binary is `54737834…29641b`.

`ecbench band --ranks` (verbatim): S = (GAE + w × unpriced native operations) ÷ √r, as a mean.

| arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) |
|:--|--:|--:|--:|--:|
| rho-neg | 32/32 | 1.607 | 1.724 | 2.779 |
| proj-a | 32/32 | 12.150 | 13.069 | 21.341 |
| pkm-iso-t6-g4 (seed 0) | 32/32 | 26.735 | 1453724.741 | 14537006.794 |
| pkm-iso-t6-g4-s1093 | 32/32 | 23.775 | 900613.893 | 9005924.949 |
| **pkm-iso-t6-g4-s1308** | 32/32 | **17.192** | **814819.507** | **8148040.344** |

Band verdicts (search amortised; see the charged-search table below):
- s1308 < seed 0 at every weight;
- s1308 < s1093 at every weight;
- s1093 < seed 0 at every weight;
- proj-a < every PKM arm, and rho-neg < every PKM arm, at every weight;
- rho-neg < proj-a.

`ecbench compare` gives B ÷ A, the 95% linearised-t interval (df 15, 32
pairs on 16 workloads), and the cluster-bootstrap interval in brackets
after it:

| B ÷ A | w = 0 | w = 0.1 | w = 1 |
|:--|:--|:--|:--|
| s1308 ÷ seed 0 | 0.6431 [0.5085, 0.8132] {0.5035, 0.8168} | **0.5605 [0.4208, 0.7467]** {0.4158, 0.7525} | **0.5605 [0.4208, 0.7467]** {0.4158, 0.7525} |
| s1308 ÷ s1093 | 0.7231 [0.6378, 0.8199] {0.6233, 0.8324} | **0.9047 [0.7398, 1.1065]** {0.718, 1.1373} | **0.9047 [0.7398, 1.1065]** {0.718, 1.1373} |
| s1093 ÷ seed 0 | 0.8893 [0.7341, 1.0773] {0.7348, 1.0821} | 0.6195 [0.4891, 0.7847] {0.485, 0.7885} | 0.6195 [0.4891, 0.7847] {0.485, 0.7885} |
| s1308 ÷ proj-a | 1.4150 [1.3098, 1.5287] | 62347.0134 [55941.0938, 69486.4869] | 381802.2968 [343095.5273, 424875.8210] |
| s1308 ÷ rho-neg | 10.7005 [9.1832, 12.4686] | 472666.9011 [395061.8808, 565516.4677] | 2932319.4036 [2423846.1347, 3547459.9488] |

Unpriced F4 elimination multiply-adds summed over 32 solves, from the
compare JSON (`solver_F_p_multiply-adds_(elimination_only)_uncharged`):
- seed 0: 1.6843·10¹¹;
- s1093: 1.0434·10¹¹;
- s1308: 9.4403·10¹⁰.

### Seed search, reported separately

- Both rules read the same 2,000-seed census. At (20, 6) the search costs
  13,528,559 group operations (build + selector). ÷ √r = **37,349.06 in S**.
- The rows below charge the whole search to one target. These sums are
  computed from the band means plus 37,349.06. ecbench does not report them.

| | S(w=0) | S(w=0.1) | S(w=1) |
|:--|--:|--:|--:|
| s1308, search amortised (as measured) | 17.192 | 814819.507 (0.5605× seed 0) | 8148040.344 (0.5605×) |
| s1308, search charged to one target | 37366.25 | 852168.57 (0.586× seed 0) | 8185389.40 (0.563×) |
| s1093, search charged to one target | 37372.83 | 937962.95 (0.645× seed 0) | 9043274.01 (0.622×) |

- Charged to one target, the search loses at w = 0 against seed 0, which
  needs no search. It still wins at w = 0.1 and w = 1, so it is not ordered
  across the band.
- Amortised over targets on the curve, s1308 is ordered better at every
  weight.

## Predictions (frozen rules; no retuning)

| # | rule | measured | outcome |
|:--|:--|:--|:--|
| C0 | seed-0 and s1093 engine outputs equal the parent's on columns, covered_points, f4_muladds, solving_degree_max, poly_terms_per_specialisation, hits, hits_verified | all equal (only `wall_s` differs) | **held** |
| P1 | s1308: columns 36, covered 498, degree 5, terms 16, not capped, 0 undecided, hits_verified = hits | 36, 498, 5, 16.0, false, 0, 0 = 0 | **held** |
| P2 | muladds_per_target s1308 ÷ s0 ∈ [0.90, 1.10] | 1.0000015 | **held** |
| P3 | E(s1308) ÷ E(s0) ≤ 0.55 and E(s1308) ÷ E(s1093) ≤ 0.90 | 0.4974 and 0.8371 | **held** |
| P4 | 32/32 per arm, 160/160 total, audit 0 problems | 32/32 for each of the 5 arms; 160/160; 16 replays reproduced, 0 problems | **held** |
| P5 | S(s1308) ÷ S(seed 0) ≤ 0.60 at w = 0.1 and w = 1 | 0.5605 and 0.5605 | **held** |
| P6 | upper 95% bound < 1 at w = 0.1 and w = 1 | 0.7467 and 0.7467 | **held** |
| P7 | S(s1308) ÷ S(s1093) ≤ 0.90 at w = 0.1 and w = 1 | 0.9047 and 0.9047 | **failed** |
| P8 | point S(s1308) ÷ S(seed 0) < 1 at w = 0 | 0.6431 | **held** |
| P9 | point S(s1093) ÷ S(seed 0) ≤ 0.75 at w = 0.1 | 0.6195 | **held** |
| P10 | all three PKM arms rank behind proj-a and rho-neg at w = 0.1 and 1 | band: proj-a < each, rho-neg < each, at every weight | **held** |

Reported for P7 but not predicted:
- The 95% interval of s1308 ÷ s1093 at w ≥ 0.1 is [0.7398, 1.1065]. It
  **includes 1**.
- The band order is "pkm-iso-t6-g4-s1308 < pkm-iso-t6-g4-s1093 at every
  weight".

## Exploratory (labelled; not preregistered, no claim)

- **Measured against the model.**
  - s1308 ÷ seed 0: measured 0.5605, model 0.497, ratio 1.13.
  - s1093 ÷ seed 0: measured 0.6195, model 0.599, ratio 1.03. T5 had
    1.07.
  - The engine E ratios come close to the model: 0.4974 and 0.5941.
  - The gap between model and measurement is larger for s1308. A plausible
    cause, untested: the solve does not need exactly `columns` relations,
    and both the overhead and the relation yield are noisy at 16 targets.
- **Ratio of F4 elimination totals.**
  - s1308 ÷ s0 = 0.5605; this term dominates S at w ≥ 0.1.
  - s1308 ÷ s1093 = 0.9047.
- **Per-target ratios at w = 0.1** (16 workloads):
  - s1308 is cheaper than seed 0 on 14 of 16 (range 0.219–2.209);
  - cheaper than s1093 on 9 of 16 (0.417–1.646);
  - s1093 is cheaper than seed 0 on 14 of 16.
- **At w = 0, s1308 is cheaper than s1093 on 15 of 16 workloads**
  (0.441–1.05). This is the counted-GAE term only; it excludes the unpriced
  F4 work.
- **Wall time** is diagnostic only, at L0. s1308 ÷ seed 0 end-to-end wall
  is 0.5594 [0.434, 0.721]. No timing claim is made.

## Distance remaining (this session's band; s1308 is the best PKM arm)

| | to proj-a (generic baseline) | to rho-neg |
|:--|--:|--:|
| w = 0 | 1.415× [1.310, 1.529] | 10.70× [9.18, 12.47] |
| w = 0.1 | 6.23·10⁴× [5.59·10⁴, 6.95·10⁴] | **4.73·10⁵×** [3.95·10⁵, 5.66·10⁵] |
| w = 1 | 3.82·10⁵× [3.43·10⁵, 4.25·10⁵] | 2.93·10⁶× [2.42·10⁶, 3.55·10⁶] |

- At w = 0.1, seed 0 on these targets is 8.43·10⁵× behind rho-neg, and
  s1308 is 4.73·10⁵× behind.
- **Rho remains ahead** on every track, by more than five orders of
  magnitude at w ≥ 0.1.
- The result covers one curve (k = 20), t = 6 and G = 4.

## Costs

- The session counted 199,276 GAE for s1308, 275,579 for s1093 and 309,887
  for seed 0, each summed over 32 runs.
- Unpriced native operations for the same arms: 9.444·10¹⁰, 1.044·10¹¹ and
  1.685·10¹¹. The band prices them at w = 0, 0.1 and 1.
- Seed search: 13,528,559 group operations (37,349 in S), reported
  separately above.
- Engine runs: three f4cost runs of about 4 s each. Wall time is diagnostic.

## Deviations

1. **`--allow-busy` on `ecbench run`.**
   - The first stage-(c) attempt was refused before any execution: "host is
     not quiet (PSI cpu some avg10 45.84 > 5.00)". Other leads were running
     on the shared 4-core host.
   - The protocol fixes isolation at L0. ecbench's `--allow-busy` records
     the busy preflight and keeps runs below L2. That is consistent with L0,
     and it does not change counted units.
   - Stage c was re-run with a copy of the frozen `run.sh` that differs in
     two things: the added `--allow-busy`, and an absolute `D=` path. The
     copy is `data/run_c_allowbusy.sh.used`.
   - The refused attempt's outputs are kept in
     `data/ecbench_attempt0_refused_busy_host/`, and its log in
     `data/run_attempt0.log`. That attempt produced no session directory and
     no runs, so nothing was mixed in.
   - Stages a and b were not re-run.
2. **Resume checkpoint.**
   - The protocol makes `.resume/RESUME.md` (`scripts/checkpoint.sh init`)
     the main session's precondition.
   - This executor may write only inside the lead directory, so it did not
     fill or push the checkpoint. `.resume/RESUME.md` currently describes a
     different effort.
   - This is recorded for the main session.
3. **No git, no rebuilds**, as instructed.
   - Binary hashes matched the frozen ones before every stage: run.sh
     checks them, and `verify` reports the auditor binary as `54737834…`.
4. **Wall-clock measurement quality is 0/100** (L0, busy host). Every
   decision above is in counted units.
5. **Two summary figures are computed outside ecbench**, from its outputs
   without changing them:
   - the "search charged" rows: band mean + 37,349.06;
   - the per-target win counts: from the compare JSON `workloads`.
6. **No report, graph or PDF** (research-visuals) was produced in this
   step. The task asked for the README and SHA256SUMS only.

## Files

- `data/census_t9.{txt,json}` — (a).
- `data/engine_k20_t6_g4_s{0,1093,1308}.{json,stderr}` — (b).
- `data/ecbench/` contains the stage-(c) outputs:
  - `s1/`, the session;
  - `band.{txt,json}`;
  - `compare_*` × 15 (`.txt` and `.json`);
  - `verify.{txt,json}`;
  - `run.log`, `spec.json`, `load_{start,end}.txt`.
- `data/run.log` (both invocations), `data/binary.sha256`,
  `data/SHA256SUMS`. SHA256SUMS covers every file under `data/` except
  `run.log` and itself.

## Validator errata (applied)

Independent validation: pass_with_errata (`validator-report.json`).

- **E1.** The "cmp passed" check of `data/census_t9.json` compared a file
  with its own copy, because `select_seeds.py` regenerates
  `analysis/census_t9.json`. The validator reproduced both census files
  byte for byte from an independent reimplementation, so the census itself
  is confirmed.
- **E2.** `SHA256SUMS` leaves out three `run.log` files (the frozen
  `find ! -name run.log`). Their hashes were recorded in
  `validator-report.json`.
- **E3.** The headline qualifier was added: the S ratios against seed 0
  hold with the seed search amortised; charged to one target, the order
  does not hold at w = 0.
- **E4.** `--allow-busy` was added to the frozen `ecbench run` command after
  the busy preflight refused attempt 0. There was no dated addendum before
  the data; this is disclosed here and in Deviation 1. Counts are
  unaffected (producer 16 replays, validator 4, engine reruns
  count-identical).
- **E5.** The binaries are identified by hash only. Three source files
  changed between `505ecbb` and the results commit (the T11 prime-interval
  code, which does not touch the isogeny path).
- **E6.** Classification label (AGENTS.md §3) left to the core tier.
  Constant-factor component result.
