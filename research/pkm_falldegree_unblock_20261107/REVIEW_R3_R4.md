# Red-team review of draft Amendments R3 (RunPod route) and R4 (2-isogeny arm at P1)

Reviewer: Red Team (independent, adversarial), 2026-11-14. Reviewed evidence: commit `507593ee0982f24b611ddf2af79310990dfa8ee8`
on `aburan28/loving-heisenberg-rbzzbu` (equal to the pushed remote head; working tree clean when read), PR #4098. Nothing in
the drafts, the preregistration, the results or the code was edited. No P1 isogeny cell was run.

Every study number quoted here is `[pcc:unverified]`. Numbers marked **derived** are this review's arithmetic on those
numbers; numbers marked **measured (review)** come from the checks in section 2 and are `[pcc:unverified]` as well.

## 1. Verdicts

| draft | verdict | one-line reason |
|:--|:--|:--|
| R3 (RunPod route, proposed Amendment 15) | **freeze with edits** (blocking: E3.1 to E3.9) | The binaries and entry script are preserved, but the text claims invariances the route does not have (wall-budget censoring, memory/OOM semantics, job time limit), promises a cost guard nothing implements, and re-attempts two cells whose retry allowance is exhausted without quoting owner authority. |
| R4 (isogeny arm at P1, proposed Amendment 16) | **freeze with edits** (blocking: E4.1 to E4.8) | The instrument (`mestre_exact_order`) is correct and leaves the cell stream untouched. The protocol around it is not executable as written (the R3 runner refuses its binary and no frozen command runs its cells), it omits the registered gates G2/G3 for the new cells (and the frozen `gate3` would hang on them), it wrongly says no decomposability certificate exists, and its readings are incomplete. |

Neither draft should be frozen in its current text. Both can be frozen once the listed edits are made, the runner changes
in E3.2 to E3.5 are implemented and re-reviewed, and the owner decisions in E3.1 are recorded.

## 2. What this review ran (measured (review), `[pcc:unverified]`)

- `cargo test --release --example pkm_tower_pilot mestre` at HEAD (main + patch, glibc): `mestre_exact_order_equals_point_count`
  ok, `mestre_and_naive_build_the_same_isogeny_tower` ok (2 passed, 17.9 s).
- Patch application: `git apply --check gaux/aux_order.patch` against the f13a690df0 version of `examples/pkm_tower_pilot.rs`
  (blob `b5781d44`) **fails** at hunk 4 (`patch failed: examples/pkm_tower_pilot.rs:2639`). GNU `patch -p1` applies it with
  "Hunk #4 succeeded at 2218 with fuzz 2 (offset -574 lines)". The resulting file has SHA-256
  `9c63c61a0db94842e8f3c9240b07c154b89e6ffbb403c8e3472fd220970e374d`, equal to the producer's build tree in the session
  scratchpad (`build4/examples/pkm_tower_pilot.rs`), and the fuzzed hunk lands in the intended place (between `--kinds` and
  `--m`). The patch was generated against main's file (blob `9efea80a01`), which carries three cover-map commits the frozen file
  does not.
- Binaries in the session scratchpad: `frozen_bin/*` equal all four `image/image.json` hashes; `build4/.../pkm_tower_pilot` is
  `705b151b…`. These are the only copies found; the session log records that S3 access is blocked.
- Independent re-comparison of G-AUX (own script, not `compare.py`): `gaux/` (main, 4 threads) and `gaux/v2/` (frozen + patch,
  4 threads) differ from the admitted values on the same 6 draws with **identical** values (for example t = 6, draw 0:
  admitted 55,685,928, both runs 55,512,274; t = 8, draw 0: 6,980,649,315 against 6,976,152,323); `gaux/v3/` (12 threads) has 0
  mismatches on all 8 fields; within each run the A and DA rows of a cell carry the same `aux_a`, `aux_b`, `aux_order`,
  `aux_curves_tried` and `V`. `diag/`: D = 5 at 1, 4 and 12 threads; `f4_muladds` 54,927,560 / 55,512,274 / 55,685,928 at
  t = 6 draw 0. v3 ran `RAYON_NUM_THREADS=12` on this 4-vCPU sandbox and still matched, so the count follows the rayon thread
  count, not `nproc`.
- Mestre termination premise: for every curve class at p = 29, 31, 37, 101, 229, 233, 241, 257 (all j, every twist of j = 0 and
  j = 1728) I computed the exponents λ(E), λ(E^d) by full enumeration and counted the Hasse-interval candidates `n` with
  `λ(E) | n` and `λ(E^d) | 2p + 2 − n`. Ambiguous classes: p = 29: 14 (all j = 1728); every other prime: 0 (683 classes at 229,
  767 at 257). This matches the Cremona–Sutherland bound (joint determination for prime p > 29) and shows the check is
  sensitive. Script (non-durable): `/tmp/claude-0/-home-user/673e57eb-93cf-5f68-aaf7-d9c809d76d05/scratchpad/rt/mestre_joint.py`.

## 3. Findings, ordered by severity

Class: **wrong** (the text or code states something false), **unsupported** (claimed without evidence), **unpriced** (a cost or
risk left out).

### High

**F1. R4 cannot run on the R3 route as written. (wrong)**
- R3's runner pins the installed `pkm_tower_pilot` to the frozen hash: `rp.sh start` sends `image/image.json`
  `study_binary_sha256` (`f78f3959…`, `runpod/rp.sh:29`), and `server.py:71` refuses any mismatch. R4's binary is
  `705b151b…`, so `/start` returns 409.
- No frozen command runs an R4 cell. `entry_r2.sh` has no isogeny or `--aux-order` stage; its `cap:` stage hard-codes
  `--kinds kummer` (`image/entry_r2.sh:51`); `run.sh cap-A` reads only `DA-m2-tower.jsonl` (`run.sh:84`); `run.sh hky-A` runs
  isogeny only at P0. R4's table gives cell ranges and "uses `run.sh`'s flags", not argv. Amendment 13 froze its command
  verbatim; R4 does not.
- Any fix needs a new entry script (or a new `run.sh`), which R3's "byte-identical `entry_r2.sh`" and its SHA list exclude.
- Cheapest test: `rp.sh start` against a local `server.py` with the R4 binary uploaded (expect 409); `grep -n isogeny image/entry_r2.sh`.

**F2. R4 omits the registered gates G2 and G3 for its new cells, and the frozen `gate3` cannot run them. (wrong)**
- §4 G2 applies to "every Kummer/isogeny `m = 2` cell `t ≤ 10`". Amendment 11 scopes G3 to closure rows with prefix `A-`,
  `N ≤ 20`, `m ≤ 6`. `A-m2-iso-p1` at t = 1..10 is inside both. §4 says any gate failure stops the study. R4 mentions neither.
- R4's CAP and X rows (capped F4 at t = 11..13) are valid `d_F` instruments only through G3's equivalence, which has never been
  checked on isogeny systems at P1 (Amendment 3 and stage A ran it on isogeny at P0 and Kummer at P1).
- The frozen `gate3` rebuilds each system by calling `PILOT --engine tower --p … --kinds isogeny …` without `--aux-order`
  (`examples/prime_fall_degree_tools.rs` at f13a690df0, lines 501-508). The default is `naive`, so at P1 every curve tried costs an
  `O(p)` Legendre count, about 2·10⁹ symbols, and a t = 10 tower tries on the order of 10³ curves (**derived** from P0's
  `aux_curves_tried` = 404 at t = 8). `--budget 900` bounds only F4, not the build, so the gate does not time out; it runs for
  days, on a reserved pod billed by the hour with no deadline (F3).
- Cheapest fix: a committed wrapper used as `BIN`/`PILOT`
  (`exec /usr/local/bin/pkm_tower_pilot --aux-order mestre "$@"`), with its SHA-256 pinned; `gate3` and `run.sh` both take the
  pilot path from outside, so no frozen tool changes.

**F3. R3's cost guard is not implemented and, at RunPod reserved prices, would cut registered budgets. (unpriced; wrong claim)**
- R3 §4: "a job whose `costPerHr × seconds limit` exceeds US$40 stops at the hour that reaches US$40". Neither `server.py` nor
  `rp.sh` implements a stop, a job time limit, or a deadline. On EKS the job time limit (at most 176,400 s, Amendment 9) was
  enforced by Kubernetes; on this route nothing enforces it.
- Price (**derived** from `research/fall_degree_bounds_20261008/e14/pod-r2/launch.json`: 32 vCPU / 256 GB at US$2.08/h, about
  US$0.065 per vCPU-hour at 8 GB per vCPU): a 112Gi request needs a memory-optimised pod of 16 vCPU (128 GB), about US$1.04/h.
  The `m = 4` and `m4n` jobs have a 172,800 s closure budget and a 176,400 s time limit: about US$50 per job, above the US$40
  ceiling. R3's guard would stop them at about 38.5 h, short of the registered 48 h budget. That contradicts R3's opening
  sentence ("does not change any … budget"), and the stop is a new censoring type ("censored-by-cost") that §2 and Amendment 7 B
  do not define.
- The eight `m = 4` and `m4n` jobs alone come to about US$400 at worst (**derived**), with stage B (about US$37 at 12 vCPU for
  48 h), the cap runs and R4 on top. R3 gives no per-job projection and no running total against the US$400 study ceiling.
- Also unpriced: the pod bills after `entry_r2.sh` exits until `rp.sh delete`. At the check-in cadence (about 50 min, then about
  4 h) that is hours of idle time per pod. AGENTS.md §9a requires charging idle attempts.
- RunPod pods also stop when the account balance runs out. The e14 launch records here record `balance_usd` and
  `other_pods_usd_per_h` at launch; R3 checks neither.
- Decision 4 sets "US$40 of **Spot** compute per job"; applying it to reserved compute is an extension the owner has not made.

**F4. "No reading depends on the host" is wrong for wall-budget censoring, and so for the decisive cells and G3. (wrong)**
- The study's budgets are wall-clock (Amendment 9 keeps them so and notes that fewer threads can censor a system 14 threads would
  finish). A slower RunPod CPU, or a pod with fewer than 12 vCPU running 12 rayon threads (R3 allows this: "whatever the pod's
  vCPU count"), raises budget censoring.
- The decisive C-step cells are close to their budget. Cap 6 took 1,824 s at t = 12 (Xeon 8488C) and 11,570 s at t = 13
  (Xeon 6975P-C). At the same ratio t = 14 cap 6 needs about 73,000 s; at Amendment 7 B's tenfold growth, about 116,000 s
  (**derived**). The budget is 86,400 s. Whether t = 14 gets a reading at all depends on host speed.
- `gate3` counts a budget timeout (900 s) as a disagreement, and Amendment 11 stops all PKM jobs on any disagreement. Stage B's G3
  at `m = 5, 6` has never run. On a slower host a timeout can stop the study.
- Cheapest test: the first job on each pod type reruns the admitted row `cap:12:0:6:12800000000` (1,824 s on EKS) and compares
  every field and the wall time.

**F5. Two R3 jobs re-attempt cells whose retry allowance is exhausted, without quoted owner authority. (unsupported)**
- `pfd-x-t13-k0-c6`: lost three times. Decision 7 allowed one third attempt; "If it is lost again the cell goes back to the
  owner" (`DECISIONS.md`; `SPOT_LOSSES.md`: "the cell is back with the owner"). R3 schedules `pfd-x-t13-k0-c6-rp`, a fourth
  attempt.
- Stage B: lost twice, so decision 3's one relaunch is used (`STAGE_B_LOST.md`). Decision 8 chose the split option, which became
  R2-G. R3 declares R2-G unused and runs stage B as one job, a third attempt.
- AGENTS.md §9a: "Never bypass a budget stop through another pool, account, region, provider or a renamed retry." The owner's
  2026-11-14 instruction ("unblock the fall-degree study and root solver. Run this on runpod") may authorise both. R3 must quote
  it as the authority for each cell and give the attempt number, or remove the two jobs.
- R3 §4 also writes a new reserved-pod rule ("relaunched at most once") as if decision 3 extended to it. It does not say whether
  the count is per cell across routes, as Amendment 14 says it is across memory sizes. With the earlier losses, the rule as written
  allows a fifth attempt of t13-k0-c6 and a fourth of stage B.

**F6. Amendment 14's memory rules cannot be applied on this route. (wrong)**
- Amendment 14's `-m112` trigger is `oversize true` or `OOMKilled`, and "Exit 137 is classified by the pod and container reason,
  never by the exit code alone". The runner records only exit codes. RunPod gives no container reason, and the runner does not read
  `/sys/fs/cgroup/memory.events` (`oom_kill`).
- Nothing checks that the pod's memory is at least the registered request (R3 §3 requires it). The RunPod create API has no memory
  field for CPU pods: memory is RAM per vCPU times the vCPU count, set by the flavor. `rp.sh:20` falls back to `cpu5g`/`cpu3g`
  (general purpose, half the memory per vCPU of the `m` flavors in these records): 12 vCPU there is about 48 GB, below 56Gi
  (about 60 GB).
- `host.txt`'s `MemTotal` and `nproc` in a container report the host, not the cgroup limit. `server.py` defaults
  `rayon` to `os.cpu_count()` (host CPUs) when the field is missing (`server.py:82`).

### Medium

**F7. R4 says the isogeny arm "has no decomposability certificate". One exists. (wrong)**
- `research/pkm_tower_pilot_20260924/verify.py` counts, for every isogeny row, the ordered pairs in `V × V` with
  `S3(x1, x2, x_R) = 0`, taking `V` from the row (the pilot writes `V` into every isogeny row for exactly this check,
  `examples/pkm_tower_pilot.rs:975-977`). At t = 13 that is 6.7·10⁷ pairs: seconds in Rust, minutes in Python (**derived**).
- Without it, t = 13 (λ = 0.0333 ≥ 0.01) falls under Amendment 7 C and Amendment 10: a closure that never contains `1` is labelled
  "possibly decomposable", not `d_F > cmax`. That conflicts with P-ISO-2's "a censored median `≥ 6` → grows", and R4 does not
  resolve the conflict.
- Size of the risk (**derived**): the expected decompositions per draw sum to 0.044 over t = 9..13; over both draws the
  probability that some draw is consistent is about 8.5%, and both draws at t = 13 about 0.1%. Under the lower median, one
  consistent draw cannot raise a median unless the other draw is also ≥ 6.
- With the certificate, the λ < 0.1 cut (Amendment 1) is not forced. Kummer ran t = 14 (λ = 0.133) under its certificate, and t = 14
  is the Kummer arm's decisive cell. Adding t = 14 for isogeny is the owner's choice, but R4's stated reason for excluding it is false.

**F8. P-ISO-1 and P-ISO-2 are not complete decision rules. (unsupported)**
- P-ISO-1 gives "refuted" only. It has no "holds" (all eight medians exact and equal to P0's) and no "undecided" (a censored
  median and no exact difference).
- P-ISO-2:
  - "every exact median at t = 9..13 is 5" can be read as "all five exact and 5" or "those that are exact are 5".
  - An exact median below 5 falls to "undecided", although it is a distinct outcome.
  - The per-t readings have no aggregation rule over the range (for example "grows" at t = 10 and 5 again at t = 13).
- Median arithmetic must be named. The committed scorer's `lower_median` (`prime_fall_degree_tools.rs:111` at f13a690df0) marks a
  median censored when a censored value is at or below it, so `{6, ≥6}` reads `≥6`, as `RESULTS_T13_BRACKET.md` did. Plain
  minimum arithmetic gives 6. R4 must say which.
- Asymmetry, stated before data: for two draws the lower median is the minimum. `{5, ≥6}` reads exact 5, "flat", so "flat at 5"
  allows one draw per t to have grown, while "grows" needs both draws. R4 should report the per-draw count of `d_F ≥ 6` next to the
  median reading.
- §6 H-LFD "as written" will read "grows" from t = 1 alone (3, then 5), exactly as it did at P0 (`RESULTS.md` line 196). R4 should
  say up front that P-ISO-2, not H-LFD, answers the isogeny question, so the two headlines are not read against each other.

**F9. R4's scorer and bracket are not specified. (unsupported)**
- R4 §2: "the scorer [is] as frozen". The committed `prime_fall_degree_tools score` has no Amendment 7 D patch (no CAP/X
  handling). Stage A and the brackets were scored by a scratchpad script archived in S3
  (`results/R9/analysis/scripts-archive.json`, SHA-256 `8b746a89…`), applied "by hand".
- R4's new file names (`A-m2-iso-p1`, `CAP-iso-m2-…`, `X-iso-m2-…`) are not shown to parse in either scorer.
- The X bracket is "Amendment 7 B", whose `c*` is the Kummer C-step value. R4 never states `c*` for the isogeny arm. It happens to
  give cap 5 first at t = 12, 13, but that must be written down.

**F10. R3 loses partial evidence on pod death and does not persist results off the pod. (unpriced)**
- On EKS `entry_r2.sh` synced to S3 every 300 s; partial logs of lost jobs survived, and `SPOT_LOSSES.md` relies on them. Here the
  `aws` shim copies into `/results` inside the same container. RunPod's API spec says the container disk "is wiped when the Pod
  restarts". `rp.sh` attaches no volume, and `/results` is fetched only when the job ends.
- `/results.tar.gz` is built in memory (`io.BytesIO`) inside a pod that may be near its memory limit. A fetch during a run risks
  an OOM kill.
- S3 access is blocked today (session log, 2026-11-14), so the `runs/<job>/runpod/<pod id>/` upload R3 promises cannot happen yet.

**F11. R3's integrity checks cover the installed files only. (unsupported)**
- The expected hashes come from the client's request body. The runner holds no pin, so any JSON the client sends passes.
- `server.py` itself (passed base64 in `dockerStartCmd`) and the base image (`python:3.12-slim`, a mutable tag) are not hashed or
  recorded.
- `build-info.json` is unchecked but copied into `host.txt`. For an R4 job it would describe the frozen `pkm_tower_pilot`
  (`f78f3959…`) while `binary.sha256` records `705b151b…`.
- No in-pod manifest of result hashes exists, so the fetched tarball cannot be checked against what the pod wrote.

**F12. No positive control on the new host for either amendment. (unsupported)**
- G-AUX ran on the 4-vCPU sandbox, not on a RunPod pod. R3 runs registered jobs without first reproducing any admitted row.
- Cheap controls with known answers:
  - `cap:12:0:5:12800000000` (11 s; X1 row: `basis_len` 2922, `max_cols` 16,572, `muladds` 179,052,499,158).
  - `cap:12:0:6:12800000000` (30 min; also a speed calibration, F4).
  - For R4, G-AUX check 3's 16 P0 isogeny draws (about 1 min).

**F13. The R4 binary is not reproducible from committed material and not durable. (unsupported)**
- `aux_order.patch` does not apply to f13a690df0 with `git apply`; it needs GNU `patch` fuzz 2 (section 2). R4 says "frozen source
  plus `gaux/aux_order.patch` only" but records neither the patched-file hash (`9c63c61a…`) nor the apply command.
- G-AUX checks 1 and 2 ran on main + patch, not on the frozen + patch source that is the instrument (the Mestre code is byte-equal
  in both, so this is low risk).
- `705b151b…` exists only in the session scratchpad; S3 is blocked.

### Low

**F14. No null arm for the isogeny systems at P1. (unpriced interpretation)**
- The pilot's null keeps the cell's tower equations and randomises S₃'s coefficients (`system()`, `Control::Null`), so
  `--kinds isogeny --controls null` is a shape-matched isogeny null.
- The Kummer null read "shape-driven" at t = 9, 10 (`pfdA_summary.json` `H_null`). Without an isogeny null, "flat at 5" cannot be
  attributed to the Semaev system rather than to the isogeny tower ring.
- Either add `A-m2-iso-null-p1` at the same t, or state that H-null is not read for this arm and that P-ISO-2 says nothing about
  Semaev-specificity.

**F15. R4's disclosure is incomplete.** It lists stage A medians and gives Kummer P1 t = 12 as `≥6`. The main session had also seen:
- `RESULTS_T12_BRACKET.md`: t = 12 is exact 6, "holds".
- `RESULTS_T13_BRACKET.md`: draw 1 = 6, draw 0 `≥ 6`.
- The Kummer null readings and stage C.

R4's descriptive comparison uses these values, so all of them should be listed, with their current state.

**F16. A Mestre panic is not given a reading.** The loop is bounded (64 points), so it cannot hang. If it ends ambiguous it panics
and kills the whole invocation (every later t of that ladder). The probability is negligible (at most about 5·10⁻¹⁰ per curve,
**derived** from the chance that 32 random points miss a full q-part), but the rule should say: panic = instrument failure, stops
R4, not a censored row.

**F17. The G-AUX "stop" rule was triggered and then rerun.** R4 §3.3 says "Any mismatch stops this amendment's runs. It is
reported, not explained away." Attempt 1 mismatched; the gate was rerun twice under changed configurations. This is acceptable:
- the draft was not frozen;
- attempt 1 violated Amendment 9's 12-thread pin;
- all attempts are kept.

The frozen text should still pin the passing configuration (binary `705b151b…`, `RAYON_NUM_THREADS=12`) and say that a mismatch
under that configuration is final.

**F18. G-AUX check 3 compares eight derived fields from an uncommitted scorer's summary, not raw admitted rows.** It does not
compare `aux_a`, `aux_b`, `aux_order`, `aux_curves_tried` or `V`. Equal `x_r` and `closure_muladds` make a different tower very
unlikely, so this is low risk, but "field by field" (session log) overstates the check.

**F19. Numbering.** R2-G and R3 both claim "Amendment 15". Mark R2-G superseded, with the reason and a pointer to decision 8 (F5).

## 4. Answers to the questions put to this review

### 4.1 R4 instrument validity

- **Correct and exact? Yes, by construction, for every curve the builder can draw at P1.**
  - If the routine returns, its answer is `#E`. By Lagrange, `L_e | #E` and `L_t | #E^d = 2p + 2 − #E`, so the true order is always a
    candidate, and the routine returns only when exactly one candidate is left.
  - `bsgs_multiple_in` returns only a genuine multiple. It always finds one, because `#E`, `#E^d` ∈ `[p + 1 − ⌊2√p⌋, p + 1 + ⌊2√p⌋]`
    and the table keeps the least `j` per point. `exact_point_order` is the standard prime-stripping reduction.
  - j = 1728 cannot be drawn (`b ∈ 1..p`). j = 0 (`a = 0`, probability `1/p` per curve) is covered (section 2). Singular curves are
    rejected before the count, and the twist by a non-residue `d` scales the discriminant by `d⁶`. `random_point` never returns a
    point with `y = 0`.
  - Overflow: the largest intermediate is `4p` ≈ 8.05·10⁹ < 2⁶⁴; the lcms divide `#E` or `#E^d`; `mulm` goes through `u128`.
  - BSGS window `w = ⌊√(2r + 1)⌋ + 1` ≈ 424 at P1 (**derived**).
  - Termination: no unbounded loop exists. Ambiguity after 64 points requires the sampled lcms to miss the exponents (F16).
  - The claim "unique for p > 229" is conservative; joint determination holds for p > 29 (section 2).
  - The only way to a wrong P1 tower would be a defect in the group law. That code is shared with the naive path and was exercised
    on the 772 + 16-draw checks.
- **RNG stream untouched? Yes.**
  - Mestre draws from a private `StdRng` seeded by `(p, a, b)`.
  - `ops` is a local `GroupOps` that no row reports.
  - The cell seed is `seed ^ kind<<40 ^ m<<32 ^ t<<16 ^ g` and does not depend on `--kinds` order, on `p`, or on other cells.
  - The builder uses `n` only for `2^t | n` and `n / 2^t`. With an exact `n`, the curves tried, `Q`, `R'`, `V` and the chain are
    those the frozen naive code would build, and so are the target curve and `x_R` drawn after the tower.
- **Does the P0 control test what P1 needs? Partly.** It tests three things:
  - the flag plumbing;
  - that both methods reach the same tower at P0 (8 towers, about 700 tried curves' divisibility verdicts, 8 exact counts);
  - that the musl frozen + patch build reproduces admitted engine counters at 12 threads.

  It cannot reveal:
  1. Mestre at a 31-bit `p`. No unit test exceeds 786,433. The risk is low but untested. Cheap fixes: a unit test against the
     library's independent E-only implementation (`src/cryptanalysis/ic_framework/prime_tower.rs`, `mestre_order`) on thousands of
     curves at a ~2³¹ prime, and a naive `O(p)` recount, or PARI/GP `ellcard` if available, of each **selected** P1 auxiliary curve
     from the rows' `aux_a`, `aux_b`, `aux_order` (13 curves, minutes each, **derived**). This is a row-validity check, not new data.
  2. Resource behaviour of isogeny systems at t = 9..13, larger than anything run at P0.
  3. The RunPod route itself (F12).
  4. The gate tooling path (F2).
- **Frozen f13a690df0 + patch rather than main? Yes, right.**
  - The Kummer P1 rows came from f13a690df0.
  - Main's F4 engine changed (`3ab8291287`), and main's pilot carries three cover-map commits.
  - The v1 = v2 equality on 16 draws does not establish main ≡ frozen in general.
  - Keeping every other code path byte-equal makes the Kummer–isogeny comparison one of factor base, not of instrument version.
  - The reproducibility gaps are in F13.

### 4.2 R4 design

- **t = 1..13 and m = 2 only.**
  - The t range follows Amendment 1's rule, but the stated reason ("no certificate") is false (F7).
  - m = 2 only is a fair reading of "first". R4 should say that every m ≥ 3 isogeny reading (stages B and C) stays at P0, so the
    prime confound remains there. At P1, m = 3 isogeny would be admissible to about t = 9 (λ < 0.1, **derived**), against t ≤ 5 at P0.
- **Falsifiable and unambiguous?** Not yet (F8). Two-draw medians and censored medians need the arithmetic named; "holds",
  "falls" and aggregation need rules.
- **Missing:**
  - an isogeny null (F14);
  - the decomposability certificate (F7);
  - G2/G3 (F2);
  - resources per job (memory, vCPU, time limit, projected cost), which R4 never states and which do not appear in R3's job table.
  - The thread dependence of `f4_muladds` is handled by the 12-thread pin. The evidence supports that the count depends on the rayon
    thread count only (section 2), so R3's "12 threads whatever the vCPU count" is sound for counts, though not for budgets (F4).
- **Disclosure.**
  - Adequate in form; incomplete in content (F15).
  - Seeing the P0 medians makes P-ISO-1 and P-ISO-2 the natural extrapolations. That is acceptable for a disclosed prediction.
  - The bias risk lies in the reading rules, not the predictions: a lower-median "flat" that tolerates one grown draw per t, and an
    H-LFD that reads "grows" regardless. Both should be fixed or flagged before data (F8).

### 4.3 R3 route

- **Byte-identical `entry_r2.sh` behind a shim.**
  - Preserved: every exact value of a completed run (`d_F`, `ffd`, `D`, `muladds` at 12 threads). The binaries are static musl,
    `target-cpu=x86-64`, hash-checked, and every `aws` call in `entry_r2.sh` is an `s3 sync`/`s3 cp` that the shim maps correctly.
  - Not preserved:
    - wall-budget censoring and G3 timeouts (F4);
    - memory semantics and OOM classification (F6);
    - the job time limit (F3);
    - off-pod persistence of partial evidence (F10);
    - the `nproc`/`MemTotal` records, which now report the host (F6).
- **Integrity checks.** Sufficient for the four binaries and two scripts against a client-supplied map. Insufficient for the route
  (F11).
- **Cost guard.** Not enforceable as written, and likely binding on the `m = 4` jobs (F3).
- **Reserved-pod loss and decision 3.** A new rule, not decision 3. It needs owner adoption, a per-cell attempt count across routes,
  and quoted authority for the two cells whose allowance is used (F5).
- **R2-G declared unused.** Running stage B as its registered single job is more faithful to the preregistration than the split, so
  the science is not harmed. But:
  - it reverses the option the owner chose in decision 8;
  - a single job of up to 48 h is lost entirely on one pod failure, with one relaunch allowed;
  - a cost-guard stop mid-job (F3) would lose every later stage, including `g3`.

  Record the reversal and its authority, and price the single-job risk.

### 4.4 The corrected diagnosis (007bdf84f3 → `gaux/GAUX.md`)

Accurate. My recomparison (section 2) confirms each point:
- main and frozen + patch give identical `f4_muladds` at 4 threads, so the F4 engine change is not the cause on these draws;
- 12 threads reproduces the admitted rows;
- `D` does not move with the thread count.

Limits:
- `diag/` covers t = 6, 7 only; t = 8 rests on v3.
- The 4-thread setting of attempts 1 and 2 is not recorded in their artifacts. It is inferred from their equality with
  `diag/DA-frozen-th4.jsonl`.
- "That rules out the engine change" holds for these 16 draws, not for the engine in general.

R4 does not depend on main, so this does not matter for R4.

## 5. Exact edits

### R3 (blocking unless marked optional)

- **E3.1 (owner decisions, before freezing).** Quote the owner's 2026-11-14 instruction, or a new explicit answer, as the authority
  for:
  - `pfd-x-t13-k0-c6-rp` (attempt 4 of the cell);
  - `pfd-b-rp` (attempt 3) and the reversal of decision 8's split;
  - applying the US$40 / US$400 ceilings, written for Spot compute, to reserved pods.

  Remove any job without a quoted authority from §3.
- **E3.2 (runner: deadline and cost).** `/start` takes `seconds_limit` (the registered job time limit) and `cost_per_hr`. The runner
  kills the entry's process group at `min(seconds_limit, 40 / cost_per_hr h − idle allowance)` and writes
  `runner_state.json: {"stopped": "deadline" | "cost"}`. Replace R3 §4's sentence with:
  - a pre-launch projection per job, `(seconds/3600 + idle allowance) × costPerHr`;
  - "if the projection exceeds US$40 the job is not started and goes to the owner" (as Amendment 12 does), instead of
    "stops at the hour that reaches US$40";
  - a running study total against US$400, checked before each pod is created;
  - the account balance read at creation, with a refusal if it is below the projected spend of all running pods.
- **E3.3 (runner: memory).** `/start` takes `memory_request_gib`. The runner refuses to start if `/sys/fs/cgroup/memory.max` (or the
  API's `memoryInGb`, if no cgroup limit is visible) is below it. Record `memory.max`, `cpu.max` and `memory.events` before and
  after each stage. Restrict `rp.sh create` to `cpu5m,cpu3m` for 56Gi and 112Gi jobs. Add: "OOM ⇔ exit 137 and an `oom_kill`
  increment in `memory.events`; that is Amendment 14's `OOMKilled`".
- **E3.4 (runner: threads).** Make `RAYON_NUM_THREADS=12` a constant of the runner (no `os.cpu_count()` default), and require
  `vcpuCount ≥ 12`.
- **E3.5 (runner: durability).** Write `/results` to the pod volume (`volumeInGb`, mounted path) or a network volume. Fetch and
  hash it at every check-in. Write a SHA-256 manifest of `/results` at job end. Stream the tarball to disk instead of `BytesIO`.
  No job starts while S3 upload is unavailable.
- **E3.6 (text: host dependence).** Replace "so no reading depends on the host" with: "Exact values of completed runs do not depend
  on the host. Censoring by a wall-clock budget does (Amendment 9), and so does a G3 timeout. Every row records the CPU model,
  `cpu.max` and wall time. A budget-censored row is reported with its host and is not rerun on another host."
- **E3.7 (positive control and calibration).** The first job on each pod flavor runs `cap:12:0:5:12800000000` and
  `cap:12:0:6:12800000000` and compares them field by field with X1/X3 (exact counts at 12 threads), recording wall time against
  1,824 s. A mismatch stops the route. The rerun rows are checks and enter no reading.
- **E3.8 (record identity).** Record in each receipt:
  - the SHA-256 of the base64 `server.py` payload;
  - the `python:3.12-slim` digest (and pin the image by digest in `rp.sh`);
  - the pod `machine` record.

  Upload `build-info.json` only when it describes the uploaded binaries.
- **E3.9 (losses).** Rewrite §4: "A reserved pod lost for any reason (host failure, a RunPod stop, a balance stop, a container
  restart) is recorded with its reason. Attempts are counted per cell across routes and sizes, including the Spot attempts; the
  next attempt needs the authority named in E3.1."
- **E3.10 (optional).** State that R2-G is superseded (not merely unused) and give the single-job loss risk in §1.

### R4 (blocking unless marked optional)

- **E4.1 (executable commands).** Add a committed entry script (or stages) and freeze each job's argv verbatim, as Amendment 13
  did, with `BIN` set to a committed wrapper that prepends `--aux-order mestre` (F2). Pin the SHA-256 of the binary `705b151b…`,
  the wrapper and the entry script. State in R3 or R4 that R4 jobs use this set and not `image.json`'s `pkm_tower_pilot`.
- **E4.2 (gates).** Add: "G2 applies to `A-m2-iso-p1` rows at t ≤ 10 (§4, as registered). G3 runs as
  `prime_fall_degree_tools gate3 DIR <wrapper> --max-n 20 --jobs 2 --budget 900` over the `A-m2-iso-p1` rows, before any CAP or X
  row at t ≥ 11 is read. Failure or disagreement follows Amendment 11's stop rule."
- **E4.3 (decomposability).** Replace "The isogeny arm has no decomposability certificate" with: "Every isogeny P1 draw is
  certified by the exhaustive `V × V` count of `research/pkm_tower_pilot_20260924/verify.py` (m = 2 branch), or a committed
  Rust port of it, on the row's `V`, curve and `x_R`. A draw with a solution is decomposable and is excluded as Amendment 7 C
  excludes Kummer draws; a certified-inconsistent draw is not labelled 'possibly decomposable'." Optionally, the owner may add
  t = 14 under that rule.
- **E4.4 (readings).**
  - P-ISO-1: "**holds** if all eight medians are exact and equal P0's; **refuted** if any exact median differs; **undecided**
    otherwise."
  - P-ISO-2: "**flat at 5** if all five medians t = 9..13 are exact and equal 5; **grows** if any median is exact and > 5 or is a
    censored bound ≥ 6 at a t whose draws are certified inconsistent; **below 5** if any exact median is < 5 and none grows;
    **undecided** otherwise. The range reading is 'grows' if any t grows, else 'below 5' if any t is below 5, else 'flat at 5' if
    all are flat, else 'undecided'."
  - Medians are `lower_median` of the committed scorer (a censored value at or below the median censors it).
  - Report alongside the reading the number of draws with `d_F ≥ 6` per t.
  - State that H-LFD is reported as written but is set by t = 1 (as at P0) and does not answer the isogeny question.
- **E4.5 (scorer and bracket).** Name the scorer by path and SHA-256 (the archived scratchpad scorer `8b746a89…`, or a committed
  one), and show it parses the R4 file names on synthetic rows. Set `c*_iso(t) = 6` at t = 12, 13 for the Amendment 7 B order
  (cap 5 first), or state the bracket order directly.
- **E4.6 (binary).**
  - Record the patched-source SHA-256 (`9c63c61a…`) and the apply command (`patch -p1`, fuzz 2), or commit a patch re-diffed
    against f13a690df0 that `git apply` accepts.
  - Store `705b151b…` in the artifact store by hash before any run.
  - Run the two unit tests on the frozen + patch source.
  - Pin the G-AUX pass configuration: binary `705b151b…`, `RAYON_NUM_THREADS=12`; any later mismatch under it is final.
- **E4.7 (route control and row validity).**
  - The first stage of the first R4 pod reruns G-AUX check 3 (16 P0 draws) and must pass before any P1 row is admitted.
  - After the run, recount each selected P1 auxiliary curve independently, by a naive count or PARI/GP `ellcard`, from `aux_a`
    and `aux_b`. A mismatch voids that cell's rows.
  - A Mestre panic is an instrument failure that stops R4.
- **E4.8 (resources, cost, disclosure).**
  - Give each job's memory, vCPU, time limit and projected cost, and add the jobs to R3's table.
  - Extend the disclosure to everything seen: `RESULTS_T12_BRACKET.md`, `RESULTS_T13_BRACKET.md`, the Kummer null and stage C, with
    Kummer t = 12 now exact 6.
- **E4.9 (optional).** Add the shape-matched isogeny null (`--kinds isogeny --controls null`) at t = 9..13 as a descriptive control,
  or state that P-ISO-2 cannot separate a Semaev-specific effect from a tower-ring effect.

## 6. What this review does not establish

It establishes neither that either amendment's cells will produce any particular reading nor that the route works on a real pod;
no pod was created and no cell was run. The RunPod prices are derived from one earlier launch record, not read today. The Mestre
exactness argument is a proof sketch checked by tests and by an exhaustive small-prime check, not a Lean proof.

Sources consulted outside the repository: RunPod CPU flavor documentation ([cpu types](https://docs.runpod.io/sdk/configuration/cpu-types),
[list CPU types](https://docs.runpod.io/api-reference-v2/catalog/list-cpu-types.md)); the RunPod OpenAPI spec the producer saved
in the session scratchpad (`PodCreateInput`: `interruptible`, `cpuFlavorIds`, `vcpuCount`, `containerDiskInGb` "wiped when the
Pod restarts").
