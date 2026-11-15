# RESULTS: PKM fall-degree study, the `t = 12` bracket (jobs `pfd-x-t12`, Amendment 7 B)

**Reading (applied by hand from the rule text; the Amendment 7 D scorer patch is not committed):** at Kummer `m = 2`, `P1`, `t = 12`, the bracket gives `d_F = 6` on both draws. The median is 6 and `c*(12) = 6`. Amendment 7 D.5 reads **"holds at t = 12"**. `[pcc:unverified]`

- Cap 5 does not refute on either draw (so `d_F >= 6`). Cap 6 refutes on either draw (so `d_F <= 6`).
- Both draws have `grid_zeros = 0`: neither is decomposable.
- The cell predicts the same value as `t = 11`, so it cannot tell "C-step" from "flat at 6". `t = 13` and `t = 14` stay undecided (see "What this does not show").

Every number below is `[pcc:unverified]`: no pcc claim id exists for any of them.

Evidence classes: **measured** = read from the rows under `results/X1..X4/`; **derived** = a comparison or the floor in `c*` applied to those rows by the scratchpad script of the archive in section 6.

## 1. What was run (measured)

Four command jobs on Iso Lab EKS Spot, one capped F4 run each (`f4_fp_tower`, `cap:12:<draw>:<cap>:12800000000`), all on the frozen image and binaries of `pfd-a`. The cell is Kummer `m = 2`, `p = 2013265921` (`P1`), `t = 12` (`N = 24`), random targets, draws 0 and 1 (`target_index`), `--budget 86400`, `--max-dense 12800000000`.

| job | card (launch) | draw | cap | request file | `protocol_sha256` (request) |
|:--|:--|--:|--:|:--|:--|
| `pfd-x-t12-k0-c5-20261108` (X1) | `X1-1` | 0 | 5 | `requests_r2f/pfd-x-t12-k0-c5.json` | `cc6573af0782aa2e...` |
| `pfd-x-t12-k1-c5-20261108` (X2) | `X2-1` | 1 | 5 | `requests_r2g/pfd-x-t12-k1-c5.json` | `bbd17530ee06205a...` |
| `pfd-x-t12-k0-c6-20261108` (X3) | `X3-1` | 0 | 6 | `requests_r2g/pfd-x-t12-k0-c6.json` | `bbd17530ee06205a...` |
| `pfd-x-t12-k1-c6-20261108` (X4) | `X4-1` | 1 | 6 | `requests_r2g/pfd-x-t12-k1-c6.json` | `bbd17530ee06205a...` |

- **Two protocol hashes.** X1 was launched from `requests_r2f/` (hash `cc6573af...`). X2 to X4 were launched from `requests_r2g/` (hash `bbd17530...`), after Amendment 13 (R2-B) added the `m = 4` null cells. Per its provenance note, Amendment 13 changes no field of any other request except provenance. Each `receipts/X*/launch/request.json` equals the request file named in the table (compared, derived). For each cell, the `r2f` and `r2g` request differ only in `provenance.protocol_sha256` and `provenance.note`; the `job` block (command, image, cpu, memory, `seconds`) is identical (compared, derived).
- **Resources per request:** cpu 12, memory 56Gi, `seconds` 90000, `isolation_profile` standard, `retries` 0, no instance pin (Amendment 12, price guard). `host.txt` of every pod: `nproc 12`, `rayon: 12`.
- **Image:** `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:0d6b2ecd6df3f5f092fe4b6179986b041eb7d6be3b94e52d92809bdab2ff3ae8`. `binary.sha256` (each pod): source revision `f13a690df016d0a457bd40579fbb9d054f486adf`, `pkm_tower_pilot f78f3959...`, `prime_fall_degree_tools 7e8a6703...`, `pfd_grid_zeros 962833611...`.
- **Node type:** not recorded. `host.txt` gives Intel Xeon Platinum 8488C, `MemTotal` about 61.8 GiB (64776596 to 64776600 kB), kernel `6.18.54-121.166.amzn2023.x86_64`. Any instance type named from that is an inference.
- **Pods and dates (UTC):** X1 pod `9f2460de...` 2026-11-10 00:27:37 to 00:27:48; X2 pod `16cf635f...` 01:25:40 to 01:25:52; X3 pod `b6596acd...` 01:34:20 to 02:04:44; X4 pod `6cb84cac...` 02:25:14 to 02:54:36. All four exited 0; job state `succeeded`.
- **Cards and receipts.** Launch cards `X1-1`, `X2-1` and `X4-1` have receipts (status `completed`). `X1-1` and `X4-1` show the `wait-short` step with exit 41, which I read as the job still running at the end of the 900 s wait (inferred from the job times above); the collect cards `X1-V01`, `X3-V01`, `X4-V01` and `X2-C01` (receipts, all steps exit 0, including the S3 hash check and `upload-raw`) took the results. **Card `X3-1` (launch of the cap-6 draw-0 job) has no receipt:** a container restart killed it during its wait step after the launch was recorded. The job itself is recorded by `receipts/X3/launch/result.json` (launcher `74aeef5a...`, pod `b6596acd...` Running at launch time) and by the collect card `X3-V01`.

## 2. The four rows (measured)

Source: `results/X{1..4}/<pod>/runs/X-m2-t12-k<draw>-c<cap>.jsonl`. `grid_zeros` from the `.gridzeros.jsonl` beside each.

| row | draw | cap | `degree_reached` | `inconsistent` | `basis_len` | `timed_out` | `oversize` | `max_cols` | `muladds` | `wall_s` | `grid_zeros` | refutes |
|:--|--:|--:|--:|:--|--:|:--|:--|--:|--:|--:|--:|:--|
| X1 | 0 | 5 | 5 | false | 2922 | false | false | 16572 | 179052499158 | 10.8 | 0 | no |
| X2 | 1 | 5 | 5 | false | 2922 | false | false | 16572 | 179052499158 | 11.8 | 0 | no |
| X3 | 0 | 6 | 6 | true | 1 | false | false | 103785 | 42213325264923 | 1824.0 | 0 | yes |
| X4 | 1 | 6 | 6 | true | 1 | false | false | 103785 | 42213325353310 | 1762.8 | 0 | yes |

- Each row has `zero_dim true`, `full_rank_exit true`, `planted_ok null` (random target), and its `x_r` equals the one in the matching `.gridzeros.jsonl` row (`150938225` for draw 0, `1002906059` for draw 1).
- **Repeat of stage A (derived).** Rows X1 and X2 equal stage A's `results/R9/*/runs/CAP-m2-t12-k{0,1}-c5.jsonl` in `x_r`, `basis_len`, `max_cols`, `muladds`, `degree_reached` and `inconsistent`: a repeat of an exact computation. Wall time differs (stage A 17.3 s and 17.1 s on another pod, with `--budget 1800 --max-nnz 2e9`; here 10.8 s and 11.8 s). For RESULTS.md U5 (whether the cap-5 step had to be repeated under the Amendment 7 A limits; the director's call), the repeat gives the same value.
- `muladds` is the count of `F_p` multiply-adds. It is equal between each cap-5 row and its stage A repeat, and it differs between the two cap-6 draws only from the 9th digit on.

## 3. The bracket and the D.5 line (derived, by hand)

Amendment 7 B at `t = 12`: `c* = 4 + floor((12 - 2)/4) = 6`. Run cap `c* - 1 = 5`; if it does not refute, run cap `c* = 6`; if that refutes, `d_F = c* = 6`.

| draw | cap 5 | cap 6 | `d_F` |
|--:|:--|:--|--:|
| 0 | does not refute (X1) | refutes (X3) | 6 |
| 1 | does not refute (X2) | refutes (X4) | 6 |

- Median `d_F` = 6, exact. No row is censored: no `timed_out`, no `oversize`.
- Amendment 7 D.5: "holds at t (median equals `c*`)". **Reading: holds at t = 12.** Stage A's table 5 row for `t = 12` ("undecided", bracket started) is now resolved by these rows. The change is in the `t = 12` row only. `t = 11` (6, `=D (F4)`), `t = 13` and `t = 14` are unchanged.
- The closure bound of stage A (`d_F > 5`) agrees with the cap-5 step.

## 4. What this shows and does not show

Shows (all `[pcc:unverified]`): on two random draws at `t = 12`, capped F4 at degree 5 does not reach `1` and capped F4 at degree 6 does. Under the Amendment 7 B and D.5 rule text, that reads "holds at t = 12". The reading is consistent with the registered C-step value 6.

Does not show, each stated once:

- **Scorer.** The D.5 reading was applied from the rule text by hand and by a scratchpad script. The Amendment 7 D scorer patch is still not committed or reviewed (RESULTS.md section 8). The committed `prime_fall_degree_tools score` was not run on these rows.
- **Not a test that separates the readings.** `t = 12` predicts 6, the value already seen at `t = 11`. This cell cannot tell "C-step" from "flat at 6". Amendment 7 E names `t = 14` (predicting 7) as the decisive cell. `t = 13` and `t = 14` remain undecided (median `>= 6`; stage A's closure bound, no F4 bracket row).
- **Sample.** Two draws only, both not decomposable (`grid_zeros = 0`). A median of two equal values has no spread.
- **G3.** G3 (capped F4 agrees with the closure where both run) passed in `pfd-a` (RESULTS.md section 1: 144 capped runs on 72 closure systems with `N <= 20`, 0 disagreements, caps `d - 1` and `d` only; U3 records that this is a subset of what the text implies) and in the stage C G3-only job `pfd-g3c-20261109` (RESULTS_STAGE_C.md section 2a: 98 capped runs, 0 disagreements, on the `m <= 6` cells). These runs are `m = 2`, inside the F4 engine's capacity (`m <= 6`, Amendment 11). G3's tool tests `N <= 20`; `N = 24` here is outside that range, so no G3 run covers this cell itself.
- **Scope of the measurement.** This is a fall-degree measurement on toy primes (`P1 = 2013265921`), `m = 2` only, `t <= 14`. It is not an index-calculus cost result: no leaderboard row, no rho comparison, no `S`, and no improvement over the index-calculus baseline is claimed. `[pcc:unverified]`

## 5. Limits

- Instance type not recorded (section 1). Wall seconds depend on the host and are context only; no timing claim is made. No L2, replay or A/A admission applies to these command jobs (README).
- X1 and X2 used different request protocol hashes (section 1); the cap-5 rows are equal in every compared field, so the choice does not show in the data.
- Card `X3-1` has no receipt (section 1). The job's data and its collect receipt are intact.
- No pcc claim was filed. Every number is an unverified assertion until a claim and a signed verdict exist.

## 6. Evidence paths

Under `research/pkm_falldegree_unblock_20261107/`:

- Rows: `results/X1/9f2460de-3e12-455b-b34b-f1bfd5c4f9c4/`, `results/X2/16cf635f-d0be-45ce-afb1-244fe5b0b838/`, `results/X3/b6596acd-b2ef-49fd-befc-b5929de42242/`, `results/X4/6cb84cac-2967-42f7-b858-a1965d3c9892/` (each: `runs/`, `host.txt`, `progress.txt`, `binary.sha256`). These directories are git-excluded; the raw files are in S3 by SHA-256 through the record below.
- Requests: `requests_r2f/pfd-x-t12-k0-c5.json`, `requests_r2g/pfd-x-t12-k{1-c5,0-c6,1-c6}.json`.
- Receipts: `receipts/X1..X4/` (`launch/`, `status/`, `collect-*`); `cards/CARD-pkm-falldegree-unblock-20261107-pfd-x-t12-*.receipt.json`.
- Stage A rows compared: `results/R9/*/runs/CAP-m2-t12-k{0,1}-c5.jsonl`; stage A values for `t <= 11`: `results/R9/analysis/pfdA_summary.json`.
- Benchmark record: `benchmarks/records/20261109T2346Z-pkm-fd-pfd-x-t12.json` (commands, raw entries, metrics).
- This analysis: `results/X12/analysis/` holds `pfdX12_fig1.svg` and `.png` (median `d_F` against `t`, with the C-step step function; `t = 13, 14` drawn as open lower bounds), `pfdX12_report.pdf`, `pfdX12_summary.json` (the four rows and the reading) and `scripts-archive.json`. The generator scripts are kept out of the repository, as in stage A and C, in a tarball recorded in the benchmark record and stored in S3 by SHA-256 (`scripts-archive.json`).
- Rule text: `research/prime_fall_degree_20261027/PREREGISTRATION.md`, Amendment 6, Amendment 7 B, C, D and E, Amendment 11.
