# RESULTS: PKM fall-degree study, the `t = 13` bracket (jobs `pfd-x-t13`, Amendment 7 B)

**Reading (applied by hand from the rule text; the Amendment 7 D scorer patch is not committed):** at Kummer `m = 2`, `P1`, `t = 13`, `c*(13) = 6`. Draw 1 gives `d_F = 6`. Draw 0 gives only `d_F >= 6`. Amendment 7 D.5 reads **"undecided at t = 13"**. It is not "holds at t = 13". `[pcc:unverified]`

- Draw 1: cap 5 does not refute and cap 6 refutes (`inconsistent true`). So `d_F = 6 = c*` for draw 1.
- Draw 0: cap 5 does not refute, so `d_F >= 6`. Cap 6 has no row: the cell was launched three times and each launch was killed by an EC2 Spot interruption. Draw 0 has a one-sided bound only.
- The median over the two draws is a bound that includes `c* = 6`. D.5 calls that "undecided".
- One draw agrees with `c*`. The other is unresolved for lack of a completed run. No row measures a disagreement with `c*`.
- Both draws have `grid_zeros = 0`: neither is decomposable.

Every number below is `[pcc:unverified]`: no pcc claim id exists for any of them.

Evidence classes: **measured** = read from the rows under `results/Y1/`, `results/Y2/` and `results/Y4/`; **derived** = a comparison or the floor in `c*` applied to those rows by the scratchpad script of the archive in section 6.

## 1. What was run (measured)

Three command jobs completed, one capped F4 run each (`f4_fp_tower`), on Iso Lab EKS Spot, on the frozen image and binaries of `pfd-a`. The cell is Kummer `m = 2`, `p = 2013265921` (`P1`), `t = 13` (`N = 26`), random targets, draws 0 and 1 (`target_index`), `--budget 86400`.

| job | card (launch) | draw | cap | request file | `--max-dense` | memory | `protocol_sha256` (request) |
|:--|:--|--:|--:|:--|--:|:--|:--|
| `pfd-x-t13-k0-c5-20261108` (Y1) | `Y1-1` | 0 | 5 | `requests_r2g/pfd-x-t13-k0-c5.json` | 27,900,000,000 | 112Gi | `bbd17530ee06205a...` |
| `pfd-x-t13-k1-c5-20261108` (Y2) | `Y2-1` | 1 | 5 | `requests_r2g/pfd-x-t13-k1-c5.json` | 27,900,000,000 | 112Gi | `bbd17530ee06205a...` |
| `pfd-x-t13-k1-c6-20261108` (Y4) | `Y4-1` | 1 | 6 | `requests_r2h/pfd-x-t13-k1-c6.json` | 12,800,000,000 | 56Gi | `a10b9393ef23d5e7...` |
| `pfd-x-t13-k0-c6` (draw 0, cap 6) | `Y3-1`, `Y5-1`, `Y6-1` | 0 | 6 | see section 2a | | | |

- **Two memory sizes, two protocol hashes.** The cap-5 jobs were submitted at 112Gi (Amendment 7 A). The cap-6 job of draw 1 was submitted after Amendment 14 and asks for 56Gi with `--max-dense 12800000000`. Each `receipts/Y*/launch/request.json` equals the request file named in the table (compared as JSON, derived). Amendment 14 says the limit only stops a run and does not steer it; the two cap-5 rows and the cap-6 row each end below their own limit, so the limit did not act in any of them (section 2b).
- **Resources per request:** cpu 12, `seconds` 90000, `isolation_profile` standard, `retries` 0, no instance pin. `host.txt` of every pod: `nproc 12`, `rayon: 12`.
- **Image and binaries:** `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:0d6b2ecd6df3f5f092fe4b6179986b041eb7d6be3b94e52d92809bdab2ff3ae8`. `binary.sha256` (each pod): source revision `f13a690df016d0a457bd40579fbb9d054f486adf`, `pkm_tower_pilot f78f3959...`, `prime_fall_degree_tools 7e8a6703...`, `pfd_grid_zeros 962833611...`. These equal the t = 12 note's.
- **Node types** (`host.txt` and `SPOT_LOSSES.md`):

| job | node type | CPU (`host.txt`) | `MemTotal` | pod start to end (UTC, `progress.txt`) | pod |
|:--|:--|:--|:--|:--|:--|
| Y1 | `r6i.4xlarge` (`SPOT_LOSSES.md`) | Intel Xeon Platinum 8375C | 129804652 kB | 2026-11-10 03:09:48 to 03:10:05 | `9253a575...` |
| Y2 | not recorded in `SPOT_LOSSES.md` | AMD EPYC 7R13 | 129114476 kB | 03:19:25 to 03:19:42 | `e7e4ad4c...` |
| Y4 | `m8i.4xlarge` (`SPOT_LOSSES.md`) | Intel Xeon 6975P-C | 64776640 kB | 07:12:18 to 10:25:08 | `2f855278...` |

  For Y2, any instance type named from the CPU model is an inference.
- **Cards and receipts.** Launch cards `Y1-1`, `Y2-1` and `Y4-1` and collect cards `Y1-C01`, `Y2-C01` and `Y4-V01` have receipts with status `completed` (`cards/*.receipt.json`). The jobs are recorded in `receipts/Y1/`, `receipts/Y2/` and `receipts/Y4/`.

## 2. The three rows (measured)

Source: `results/Y{1,2,4}/<pod>/runs/X-m2-t13-k<draw>-c<cap>.jsonl`. `grid_zeros` from the `.gridzeros.jsonl` beside each.

| row | draw | cap | `degree_reached` | `inconsistent` | `basis_len` | `timed_out` | `oversize` | `max_cols` | `max_dense_entries` | `muladds` | `wall_s` | `grid_zeros` | refutes |
|:--|--:|--:|--:|:--|--:|:--|:--|--:|--:|--:|--:|--:|:--|
| Y1 | 0 | 5 | 5 | false | 2922 | false | false | 16572 | 53833099 | 179052499158 | 16.8 | 0 | no |
| Y2 | 1 | 5 | 5 | false | 2922 | false | false | 16572 | 53833099 | 179052499158 | 16.8 | 0 | no |
| Y4 | 1 | 6 | 6 | true | 1 | false | false | 177063 | 4717115340 | 288586444518022 | 11570.0 | 0 | yes |
| (none) | 0 | 6 | no row | | | | | | | | | | no row |

- Each row has `zero_dim true`, `full_rank_exit true`, `planted_ok null` (random target), and its `x_r` equals the one in the matching `.gridzeros.jsonl` row (`1118191946` for draw 0, `1132329660` for draw 1).
- **Compared with `t = 12` (derived).** The cap-5 rows Y1 and Y2 equal X1 and X2 of the t = 12 note in `max_cols`, `max_dense_entries`, `muladds`, `basis_len` and `steps` (22). Wall time differs (16.8 s here, 10.8 s and 11.8 s at t = 12, on other pods). The cap-6 row Y4 differs from X4: `max_cols` 177063 against 103785, `max_dense_entries` 4717115340 against 1357804954, `steps` 32 against 30, `wall_s` 11570.0 against 1762.8. Both end with `basis_len 1` and `inconsistent true`.

### 2a. Draw 0, cap 6: no row (measured)

- The cell `pfd-x-t13-k0-c6` was launched three times and killed each time by an EC2 Spot interruption: `pfd-x-t13-k0-c6-20261108` (card `Y3-1`, `r7a.4xlarge`, about 10 min), `pfd-x-t13-k0-c6-20261110-r1` (card `Y5-1`, `r6a.4xlarge`, about 20 min) and `pfd-x-t13-k0-c6-20261110-r2` (card `Y6-1`, 56Gi under Amendment 14, `m7i.4xlarge`, 3 h 51 min). `SPOT_LOSSES.md` (rows 1 to 3 and the section on the cell) has the times, the audit-log events and the node types.
- Each attempt wrote no result row (the row files in S3 are 0 bytes). **Nothing from the three lost attempts is admitted or analysed in this note.** I did not read their partial logs for it.
- Owner decision 3 allowed one relaunch; owner decision 7 allowed one third attempt. Both were used. The cell is back with the owner.

### 2b. Amendment 14 conditions for the 56Gi run Y4 (measured, then compared)

| condition (Amendment 14, sections 3 and 4) | Y4 | holds |
|:--|:--|:--|
| memory request 56Gi, `--max-dense 12800000000` | `argv: cap:13:1:6:12800000000`; request `memory "56Gi"`; `MemTotal` 64776640 kB | yes |
| `oversize false` | `oversize false` | yes |
| `timed_out false` | `timed_out false` | yes |
| `max_dense_entries` below the limit | 4,717,115,340 against 12,800,000,000 (about 37 percent of the limit) | yes |
| not killed by the memory limit (`OOMKilled`) | exit 0, job state `succeeded`; the run wrote its row | yes |
| peak memory | 20,618 MB (the `peak` value in the F4 log lines, largest over the log) | recorded |

- By Amendment 14 section 4, a 56Gi run that completes with `oversize false` and `timed_out false` is the cell's row, whatever it reads. No `-m112` re-run applies to Y4.
- Peak memory of 20,618 MB is about 32 percent of the pod's `MemTotal` (derived).

## 3. The bracket and the D.5 line (derived, by hand)

Amendment 7 B at `t = 13`: `c* = 4 + floor((13 - 2)/4) = 6`. Run cap `c* - 1 = 5`; if it does not refute, run cap `c* = 6`; if that refutes, `d_F = c* = 6`.

| draw | cap 5 | cap 6 | `d_F` |
|--:|:--|:--|:--|
| 0 | does not refute (Y1) | no row (lost three times) | `>= 6` |
| 1 | does not refute (Y2) | refutes (Y4) | 6 |

- Draw 1 is exact and uncensored: no `timed_out`, no `oversize`.
- Draw 0 is a lower bound only. The rule has no value for a draw whose cap-6 run did not finish for a reason outside the computation.
- The median of a draw with `d_F >= 6` and a draw with `d_F = 6` is a bound that includes `c* = 6`. Amendment 7 D.5 ("undecided at t": a bound that includes `c*`, or no median). **Reading: undecided at t = 13.**
- It is not "holds at t = 13". "Holds" needs a median that equals `c*`, and the median here is not exact.
- Plainly: draw 1 agrees with `c*`. Draw 0 is unresolved because its run never completed. It is not unresolved because of any measured disagreement.
- Stage A's row for `t = 13` (closure one-sided, `>= 6`) is unchanged in its reading. The `t = 12` row is as in the t = 12 note ("holds at t = 12"). `t = 14` is unchanged.

## 4. What this shows and does not show

Shows (all `[pcc:unverified]`): on draw 1 at `t = 13`, capped F4 at degree 5 does not reach `1` and capped F4 at degree 6 does. On draw 0, capped F4 at degree 5 does not reach `1`. Under the Amendment 7 B and D.5 rule text, the cell reads "undecided at t = 13". The one exact draw equals the registered C-step value 6.

Does not show, each stated once:

- **One draw only.** Exactly one draw has a completed bracket. A single value has no spread, and no median over two draws exists.
- **The lost cell.** Draw 0 at cap 6 has no row. Whether cap 6 refutes on draw 0 is not known from any admitted data. The owner decides whether the lost cell gets another attempt. Owner decision 7 allowed one third attempt, and that attempt was lost.
- **Not a test that separates the readings.** `c*(13) = 6`, the value already seen at `t = 11` and `t = 12`. This cell cannot tell "C-step" from "flat at 6". `t = 14` is the level where `c*` changes to 7. It is the decisive cell (Amendment 7 E) and has no row.
- **Scorer.** The D.5 reading was applied from the rule text by hand and by a scratchpad script. The Amendment 7 D scorer patch is still not committed or reviewed (RESULTS.md section 8). The committed `prime_fall_degree_tools score` was not run on these rows.
- **G3.** G3 (capped F4 agrees with the closure where both run) passed in `pfd-a` (RESULTS.md section 1) and in the stage C G3-only job `pfd-g3c-20261109` (RESULTS_STAGE_C.md section 2a), on cells with `m <= 6`. G3's tool tests `N <= 20`; `N = 26` here is outside that range, so no G3 run covers this cell itself.
- **Scope of the measurement.** This is a fall-degree measurement on toy primes (`P1 = 2013265921`), `m = 2` only. It is not an index-calculus cost result: no leaderboard row, no rho comparison, no `S`, and no gain over the index-calculus baseline is claimed. `[pcc:unverified]`

## 5. Limits

- Wall seconds depend on the host and are context only; no timing claim is made. The three rows ran on three different node types. No L2, replay or A/A admission applies to these command jobs (README).
- The node type of Y2 is not recorded.
- The cap-5 rows ran at 112Gi and the cap-6 row at 56Gi (Amendment 14). The cap-5 rows equal their `t = 12` counterparts in every compared count, and Y4 ended well below its limit, so the size does not show in the data.
- Peak memory is read from the F4 log lines, not from a pod metric.
- The draw 0 cap-6 cell has no row (section 2a).
- No pcc claim was filed. Every number is an unverified assertion until a claim and a signed verdict exist.

## 6. Evidence paths

Under `research/pkm_falldegree_unblock_20261107/`:

- Rows: `results/Y1/9253a575-a872-49e5-886e-eaa87e0ff902/`, `results/Y2/e7e4ad4c-2d4b-4f65-96b9-6b10ededf985/`, `results/Y4/2f855278-d366-4b77-a295-e2a784e950ba/` (each: `runs/`, `host.txt`, `progress.txt`, `binary.sha256`). These directories are git-excluded; the raw files are in S3 by SHA-256 through the benchmark record below.
- Requests: `requests_r2g/pfd-x-t13-k{0,1}-c5.json`, `requests_r2h/pfd-x-t13-k1-c6.json`.
- Receipts: `receipts/Y1/`, `receipts/Y2/`, `receipts/Y4/`; `cards/CARD-pkm-falldegree-unblock-20261107-pfd-x-t13-*.receipt.json`.
- Spot losses: `SPOT_LOSSES.md`.
- `t = 12` rows compared: `results/X1..X4/`; `results/X12/analysis/pfdX12_summary.json`. Stage A values for `t <= 11`: `results/R9/analysis/pfdA_summary.json`.
- Benchmark record: `benchmarks/records/20261110T0305Z-pkm-fd-pfd-x-t13.json` (commands and raw entries; the scripts tarball was added to it, nothing else).
- This analysis: `results/Y13/analysis/` holds `pfdY13_fig1.svg` and `.png` (`d_F` against `t`, with the C-step step function; draw 0 at `t = 13` and the cell `t = 14` drawn as open lower bounds), `pfdY13_report.pdf`, `pfdY13_summary.json` (the three rows, the Amendment 14 check and the reading) and `scripts-archive.json`. The generator scripts are kept out of the repository, as in stage A, stage C and the t = 12 note, in a tarball (`scripts.tar.gz`) recorded in the benchmark record and stored in S3 by SHA-256. `scripts-archive.json` names it, with its SHA-256, size and S3 path.
- Rule text: `research/prime_fall_degree_20261027/PREREGISTRATION.md`, Amendment 7 B, D and E, Amendment 14.

## 7. Status of the scripts archive

`results/Y13/analysis/scripts.tar.gz` holds the scripts and their output (SHA-256 `73157c8bb102bbd9...`, 4845 bytes; `scripts-archive.json` has the full hash and the S3 path). It is an entry of the benchmark record above and is stored in S3 by SHA-256 (`tools/benchrec.py add-raw` and `upload`, run by the main session on 2026-11-10; the object was listed afterwards).
