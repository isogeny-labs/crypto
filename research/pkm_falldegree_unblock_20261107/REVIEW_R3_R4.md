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

## Re-review of R4 revision 2

Reviewer: Red Team, 2026-11-14. Reviewed: commit `46e371d09d6a1027b34d52dbf51b533609356032` on
`aburan28/loving-heisenberg-rbzzbu` (working tree clean). `examples/pkm_tower_pilot.rs`, `examples/prime_fall_degree_tools.rs`
and the preregistration are unchanged since `507593ee09`. Nothing was edited and no `P1` isogeny cell was run. Every study
number is `[pcc:unverified]`; **derived** marks this review's arithmetic.

### R2.1 Verdict: freeze with edits

The instrument and the route plumbing are now in good shape: the argv, the wrapper, G3 through the wrapper, the certificate,
the recount, the on-host G-AUX and a reproducible patch. What remains blocking is in two places:
- `iso/score_iso.py`, which is part of what gets frozen, mis-reads five cases I constructed (R2.4 N1).
- The cost and run plan, which leans on an R3 cost stop that does not exist (R2.4 N2).

Both are fixed by the exact edits in R2.5. R4 also cannot run until R3 is frozen with the edits listed in R2.3b Q4.

### R2.2 What this re-review ran (measured (review), `[pcc:unverified]`)

- `python3 -I iso/score_iso.py --selftest`: "selftest: pass".
- `python3 -I iso/certify_m2.py --selftest iso/selftest/planted-iso-p0.jsonl iso/selftest/planted-kummer-p1.jsonl
  gaux/v3/A-m2-iso-p0-mestre.jsonl`: "40 rows, 0 mismatches". `iso/selftest/cert.jsonl` holds:
  - 12 planted P0 isogeny rows, all decomposable;
  - 12 planted P1 Kummer rows, all decomposable;
  - 16 random-target P0 isogeny rows, all with 0 solutions (all 16 refuted in stage A, so this agrees).
- `git apply gaux/aux_order_f13a690df0.patch` on the f13a690df0 version of the pilot applies cleanly and gives
  `9c63c61a0db94842e8f3c9240b07c154b89e6ffbb403c8e3472fd220970e374d`.
- **G3 through the wrapper, at P0** (no P1 row). In a scratch directory I ran the frozen `prime_fall_degree_tools gate3`
  (`7e8a6703…`) with `PILOT = iso/pilot_mestre.sh` (which execs `/usr/local/bin/pkm_tower_pilot_r4`, `705b151b…`, left
  installed by the integration test), `RAYON_NUM_THREADS=12`, `--max-n 20 --jobs 2 --budget 900`, over the 16
  `gaux/v3/A-m2-iso-p0-mestre.jsonl` rows. Result: "G3: 32 capped runs, 0 disagreements"; 32/32 `agree`, 32/32 `seed_match`,
  0 timeouts.
- **Edge cases of `score_iso.py`** (synthetic rows, own scratch script that imports the committed module):

  | case | input | `score_iso` output | correct reading |
  |:--|:--|:--|:--|
  | A | t = 10: closure reached cmax 7 without 1 (`d_F ≥ 8`); DA refuted at D = 6 | `≥8`, P-ISO-2 "grows", G2 0 violations | contradiction of the G2 theorem (`d_F ≤ D`): instrument failure, R4 stops |
  | B | t = 13: both closures `≥6` (cmax 5, no 1), **no certificate row** | `≥6`, "grows" | R4 §5 requires "certified inconsistent": undecided |
  | C | t = 12: draw 0 exact 5, draw 1 has **no row** | median 5, "flat at 5" | one-draw cell; R4 defines no reading, so it should be undecided |
  | D | t = 12: closure exact 5 on both draws; `X-iso-m2-t12-k0-c5` completed and did **not** refute (`d_F ≥ 6`) | 5, "flat at 5"; conflict silent | closure/capped-F4 disagreement: G3-type failure, R4 stops |
  | E | t = 12 draw 0: closure `≥6` (x_R 1120); an `X…-c6` row from a **different system** (x_R 999) refutes | exact 6, "grows" | rows of different systems must not combine (the frozen scorer keys by `x_r`) |

### R2.3 Edit by edit

| edit | status | evidence and what is left |
|:--|:--|:--|
| E4.1 argv, wrapper, pins | **mostly closed** | Every argv is in `iso/entry_iso.sh`; it matches `run.sh`/`entry_r2.sh` flags with `--kinds isogeny` and `BIN` = wrapper. Runner `iso` mode installs and hash-checks the three binaries (`iso/PINS.json`) and `repo.tar`; rayon is pinned at 12. Integration test (`g1 gaux`, local) passed and used the same `entry_iso.sh` and `server.py` bytes as `46e371d09d` (no change since `1e1e0f596b`). **Open:** the entry script, wrapper, certifier, scorer, `compare.py`, `pfdA_summary.json` and `verify.py` are not pinned. They arrive in `repo.tar` built from "the job's commit", any commit, and the runner checks `repo.tar` only against the hash the client computed. `entry_iso.sh` records some of those hashes in `binary.sha256` but compares them with nothing. R4 §7 says `runner_request.json` carries the commit; `start-iso` does not send it. |
| E4.2 G2, G3 | **closed in mechanics; G2 incomplete** | G3 through the wrapper works (R2.2: 32/32 at P0). `score_iso` G2 is a faithful port of the frozen check (exact closure `d_F` against a refuting, completed D; slightly stricter, since it requires `inconsistent`). But the new interval logic turns a bound-form violation into a reading (case A), which the frozen scorer never did. |
| E4.3 decomposability | **closed in the certifier; not enforced by the scorer** | `certify_m2.py` is exact. S3 is symmetric, so counting `U ∈ V` roots of `S3(x1, x_R, U)` per `x1` equals the `V × V` count, including the `x1 = x_R` linear case. It agrees with brute force on 40 rows. `score_iso` treats a draw with no certificate as inconsistent (case B), contrary to R4 §5. |
| E4.4 readings | **text closed; implementation gaps** | P-ISO-1 and P-ISO-2 rules and their aggregation match the text in code, apart from cases B and C. |
| E4.5 scorer, bracket | **partly closed** | Scorer named and self-tested; `c*_iso = 6`, cap 5 first. Open: the scorer's SHA-256 is not pinned; cases A, D, E; the per-draw source label that Amendment 7 D.2 requires ("closure", "=D (F4)", "bracket") is not produced. |
| E4.6 reproducible binary | **closed, one precondition missing** | `git apply` patch and patched-source hash verified (R2.2); tests on frozen + patch recorded (`gaux/TESTS_FROZEN.md`). The artifact-store copy of `705b151b…` "waits on S3". R4 should say no job starts until it exists. |
| E4.7 on-host control, recount, panic | **closed** | `gaux` stage in every job, compared by `compare.py`. `naive_count.c`/`recount.py` equal Mestre on the 8 P0 curves (`iso/selftest/recount-p0.json`). Exit 3 on "Mestre: #E ambiguous". Minor: `panicked()` scans `runs/*.log` and `gates/*.log` but not `gates/gaux/*.log`. A gaux panic still stops the job, through `compare.py` failing, but with exit 1 instead of 3. |
| E4.8 resources, cost, disclosure | **disclosure closed; cost plan not** | §0 now lists the bracket results, the Kummer null, stage C and the rows made while preparing. §6 cost: see Q3 and N2. |

### R2.3b The coordinator's four questions

**Q1. Is the bound combination in `score_iso.py` faithful to Amendments 3, 5 and 7?**
- **Each bound is registered.**
  - "Refuting uncapped F4 at D ⇒ `d_F ≤ D`" is Amendment 3's own sentence ("which gives `D`, so `d_F ≤ D`") and the §4 G2 theorem.
  - "Cap C refutes ⇒ `d_F ≤ C`" is §3's soundness argument and needs no gate.
  - "Cap C completes without refuting ⇒ `d_F ≥ C + 1`" is the converse. It needs G3, and above `N = 20` it is the same extension
    Kummer's Amendment 7 B makes.
  - "Stopped by budget or `--max-dense` ⇒ no bound" (`f4_done`) is Amendment 7 B.
- **Intersecting the intervals reproduces every value Amendment 7 D.2 names:** "=D (F4)", the bracket, and the closure one-sided
  bound.
- **It adds exact values only where two valid bounds meet**, for example closure `> 5` with a refuting D = 6 and no cap-5 row.
  That is sound, but it is a generalisation the text should state, and D.2's per-row source label is missing.
- **Not faithful in three respects:**
  - it has no conflict detection (cases A and D);
  - it does not key by `x_r` (case E; the frozen `Key` includes `x_r`);
  - a missing draw yields a one-draw median (case C).

  With interval logic, a conflict becomes a reading instead of a stop. That is the decisive difference from the frozen scorer.

**Q2. Is the G3 directory layout within Amendment 11's scope?**

Yes.
- `gate3` reads `DIR/runs/*.jsonl` with prefixes `A-`/`B-`/`C-` and writes `DIR/gates/g3.jsonl`, so the copy into
  `gates/g3dir/runs` is what it needs.
- The copied rows are random-target, tower-control, `m = 2` closure rows. `--max-n 20` keeps exactly t = 1..10 and drops
  `A-m2-iso-p1-big` (N ≥ 22). That is Amendment 11's scope (prefix `A-`, not naive, `N ≤ 20`, `m ≤ 6`), restricted to R4's new
  cells.
- Running `gate3` on the job's own `runs/` would select the same rows: `CAP-`, `CERT-`, `DA-` and `X-` do not match `C-`/`A-`.
- The layout works mechanically (R2.2: 32/32 at P0).
- The 900 s budget per capped run is wall-clock, and `gate3` counts a timeout as a disagreement (Amendment 11 stop rule), so host
  speed matters (F4, E3.6). R4 does not say so.

**Q3. Are the §6 projections and ceilings honest?**
- **Expected figures:** plausible. US$2–7 for `iso-a` at 2–6 h. `pfd-a` ran the larger Kummer stage A in 3.5 h with the same
  request: 12 vCPU, 56Gi, 172,800 s (`requests_r2c/pfd-a.json`).
- **At-limit figures** (US$50 for `iso-a`, US$26 per `iso-x`) assume a 16-vCPU / 128 GB pod at US$1.04/h. The table asks for
  "≥ 12 vCPU / 56Gi". A 12-vCPU memory-optimised pod (96 GB) at about US$0.78/h gives about US$37 at the 172,800 s limit
  (**derived**). Pinning that pod would bring `iso-a` under US$40 before idle time, and removes the need for any cost stop.
- **Not honest in full:**
  - Idle billing after the job is not priced.
  - R4's own worst case is not totalled against the US$400 study ceiling. The bracket can take up to four cap steps (4..7) × 2
    draws × t = 11..13, each about US$26 at the limit (**derived**), so the worst case far exceeds US$400 even though the
    expected cost is small.
  - "Until then `iso-a` runs under the US$40 stop of R3's cost guard (E3.2)" relies on a stop that no code implements. The runner
    has no deadline, so the 172,800 s and 90,000 s time limits are not enforced either. The sentence also contradicts E3.2 as
    written: do not start a job whose projection exceeds US$40; ask the owner.
  - "Censored-by-cost … rows up to the stop are kept" is a new censoring class with no reading. Combined with case C it can turn
    a stopped job into a one-draw "flat at 5".

**Q4. Which of R3's edits does R4 still need before it can run?**
- **E3.1:** the owner's decision on reserved-pod ceilings. R4 asks for it separately. R4 needs no retry authority, since its cells
  are new.
- **E3.2:** a runner deadline enforcing 172,800 s and 90,000 s, plus the pre-launch cost projection with refusal, the US$400
  running total and the balance check.
- **E3.3:** memory floor and OOM classification. The X runs inherit Amendment 14's `-m112` rule, which needs `OOMKilled`.
  `entry_iso.sh` records `memory.max`, but nothing refuses an undersized pod, and `rp.sh create` still allows `g` flavors.
- **E3.4:** done for rayon in `iso` mode. vCPU ≥ 12 is still unchecked.
- **E3.5:** durability. Results are still on the container disk, and S3 is blocked; R4 §7 says so.
- **E3.6:** the host-dependence sentence. It applies to R4's 1,800 s closure, 900 s G3 and 86,400 s X budgets.
- **E3.7:** satisfied for R4 by its own `gaux` stage. No speed calibration.
- **E3.8:** runner and image identity. `server.py` and the base image are still unpinned and unrecorded.
- **E3.9:** a loss rule. R4 says nothing about a lost pod (relaunch or not, attempt count).

### R2.4 New findings introduced by revision 2

- **N1 (high, wrong): `score_iso.py` reads conflicts and gaps as results.** Cases A to E in R2.2. Cases A and D hide what §3
  calls gate failures. Case E combines different systems. Case B contradicts R4 §5. Case C reads a one-draw cell.
- **N2 (high, unpriced): the cost plan depends on an unimplemented stop and adds an undefined censoring class.** See Q3.
- **N3 (medium, unsupported): the bundle is not pinned.** Any commit's `repo.tar` passes. R4 §7 says the commit is in
  `runner_request.json`, but `rp.sh start-iso` keeps it only in the client's `$TOKDIR`.
- **N4 (low): untested stages.** The integration test exercised `g1 gaux` only. `cap-A` and `x:` were not run end to end. Their
  argv mirror the frozen `run.sh`/`entry_r2.sh`, and their output names match the scorer's regex, so the risk is low. `g3` is now
  covered by R2.2 at P0. `cap-A` takes D from any DA row, including a timed-out one, as the frozen `run.sh` does. That is harmless
  to scoring, because the scorer reads the cap from the file name and ignores stopped runs.
- **N5 (low):** `panicked()` misses `gates/gaux/*.log` (E4.7 row above). `naive_count` has no pinned build or hash, and the
  recount host and order are unstated: readings should not be reported before the recount passes.

### R2.5 Exact edits for freezing R4 (blocking: R2-E1 to R2-E7)

- **R2-E1 (scorer: conflicts).** In `score_iso.score()`, after combining:
  - flag a draw as `conflict` when `L > U`, or when a closure-exact value lies outside `[L, U]`;
  - extend G2 to "closure lower bound `L` > refuting D at t ≤ 10";
  - make `readings()` return `"instrument conflict"` and a non-zero exit when any conflict or G2 violation exists. R4 §3 then
    treats it as a gate failure (R4 stops).
  - Add cases A and D to `--selftest`.
- **R2-E2 (scorer: certification).** A draw with no refutation (no exact value and no `U`) enters a median only if a certificate
  record with `solutions == 0` exists for its `(t, k, x_r)`. Otherwise it is `uncertified` and its `t` reads "undecided". Add
  case B.
- **R2-E3 (scorer: missing draws).** For every `t` in 1..13, expect `k ∈ {0, 1}`. Only a certificate-excluded (decomposable) draw
  may be absent, as Amendment 7 C. A draw missing for any other reason (lost or stopped job, ladder stop) makes its `t` read
  "undecided". Add case C.
- **R2-E4 (scorer: identity).** Key draws by `(t, k)` and require every source (A, DA, CAP, X, CERT) to carry the same `x_r`, and
  the same `aux_a`/`aux_b`/`aux_order` where present, with `p = 2013265921` and `aux_order_method = "mestre"`. A mismatch is an
  instrument failure. Add case E. Print the source of `L` and `U` per draw ("closure", "=D (F4)", "bracket"), as Amendment 7 D.2
  requires.
- **R2-E5 (pins).**
  - Add SHA-256 entries to `iso/PINS.json` for `iso/entry_iso.sh`, `iso/pilot_mestre.sh`, `iso/certify_m2.py`,
    `iso/score_iso.py` (after R2-E1 to E4), `gaux/compare.py`, `results/R9/analysis/pfdA_summary.json` and
    `research/pkm_tower_pilot_20260924/verify.py`.
  - Have the runner (or `entry_iso.sh`, before any stage) verify the unpacked files against them.
  - Send the commit in the `start-iso` body so `runner_request.json` records it.
  - Name the freeze commit in the amendment.
- **R2-E6 (cost and run plan).**
  - Set the `iso-a` and `iso-x` pod to 12 vCPU on `cpu5m`/`cpu3m` only. State the projection at the limit with an idle
    allowance, and R4's worst-case total against US$400.
  - Replace "Until then `iso-a` runs under the US$40 stop … censored-by-cost, and its rows up to the stop are kept" with: "No R4
    job starts while its projection, idle allowance included, exceeds US$40, or while the owner has not decided the reserved-pod
    ceilings. R4 defines no censored-by-cost reading. A job stopped for any reason leaves its unfinished draws missing, and R2-E3
    applies."
- **R2-E7 (preconditions).** Add to R4 §6: "No R4 job starts until (i) R3 is frozen with E3.2, E3.3, E3.5, E3.8 and E3.9 in its
  runner, (ii) `705b151b…` is in the artifact store and S3 upload works, and (iii) the owner has decided the reserved-pod
  ceilings." Add a loss rule for R4 pods: one relaunch under a new name, both attempts kept, and further attempts go to the owner.
- **R2-E8 (low, optional).**
  - Add the E3.6 sentence: wall budgets and G3 timeouts depend on the host; record the CPU model.
  - Scan `gates/gaux/*.log` in `panicked()`.
  - Pin `naive_count`'s build command and hash.
  - State that no reading is reported before the recount passes.

## Final re-review (R3 rev 2, R4 rev 3)

Reviewer: Red Team, 2026-11-14. Reviewed: commit `69142b169675ab7aaac31623604d7db9215e2e4e` on
`aburan28/loving-heisenberg-rbzzbu` (working tree clean). That commit contains R4 revision 3 (`9fe6e242a3`) and R3 revision 2.
`examples/pkm_tower_pilot.rs`, `examples/prime_fall_degree_tools.rs` and the preregistration are unchanged. Nothing was edited
and nothing was run at P1. Every study number is `[pcc:unverified]`; **derived** marks this review's arithmetic.

### F.1 Verdicts

| draft | verdict |
|:--|:--|
| R3 revision 2 | **freeze with edits**: four text edits, F-R3-1 to F-R3-4 (one has a two-line code fallback) |
| R4 revision 3 | **freeze with edits**: three edits, F-R4-1 to F-R4-3 (one scorer rule) |

Every earlier finding is closed in substance. What remains:
- two statements that overstate what was tested;
- one wrong statement about the session machine;
- an idle allowance that does not match the check-in cadence;
- one instrument contradiction the scorer still swallows (case F).

Once those are fixed, the first run is gated only by the owner decisions and external preconditions in F.6.

### F.2 What this re-review ran (measured (review), `[pcc:unverified]`)

- `python3 -I iso/score_iso.py --selftest`: "selftest: pass" and "selftest (review cases A-E): pass".
- `sha256sum -c iso/BUNDLE.sha256` at the repository root: all 17 pinned files match HEAD. `git ls-files` of `BUNDLE_PATHS`
  gives the same 17 files.
- `python3 -m py_compile runpod/server.py` and `bash -n runpod/rp.sh`: both pass.
- Further `score_iso.py` cases, synthetic rows, imported from the committed module:

  | case | input | output | correct reading |
  |:--|:--|:--|:--|
  | F | t = 9: draw 0 refuted (closure exact 5), but its certificate finds 2 `V × V` solutions | no conflict. Draw 0 is excluded as decomposable, the cell counts as complete, and it reads "flat at 5" from draw 1 | a refuted system cannot have a solution: this is `verify.py`'s own cross-check, so it is an instrument conflict and R4 stops |
  | G | certificate `x_R` differs from the rows' `x_R` | STOP | correct |
  | H | rows at `p = 786433` with no `aux_order_method` | admitted | R2-E4 asked for a `p` and `mestre` check. Low, because `entry_iso.sh` always uses the wrapper |
  | I | `{5 exact, ≥6 certified}` | median 5, "flat at 5", `draws_dF_ge_6` = 1 | as registered: lower median, disclosed in R4 §5 |
- **cgroup on the session machine.** It is cgroup **v1**. `/sys/fs/cgroup/memory/memory.oom_control` reads `oom_kill 0`, so the
  counter **is readable**. `server.py`'s `oom_kills()` reads only the v2 `memory.events`, and `cgroup_cpus()` reads only the v2
  `cpu.max` (the v1 `cpu.cfs_quota_us` is ignored).
- **RunPod API spec** (OpenAPI saved in the session scratchpad):
  - `volumeInGb`: "The data on the Pod volume is persisted across Pod restarts";
  - `volumeMountPath` defaults to `/workspace`;
  - `containerDiskInGb`: "wiped when the Pod restarts";
  - there is no balance endpoint (only `/billing/pods`, `/billing/endpoints` and `/billing/networkvolumes`).

### F.3 R3 revision 2: edit by edit

| edit | status | note |
|:--|:--|:--|
| E3.1 authority | **closed** | Authority table: only R4 on the route. The Kummer retries, `t = 14`, `m = 4` and the reserved ceilings wait for the owner. "Until then, no job on this route starts." |
| E3.2 deadline, cost | **closed in code; one gap** | Process-group kill at `seconds`, recorded as `deadline`. Refusal at `cost_per_hr × (seconds + 3600)/3600 > max_usd`. Running total and balance are procedural, the balance through the owner. **Gap:** §6 deletes pods "at the first check-in after their job ends", and the cadence allows about 4 h between check-ins, but the guard allows 1 h of idle. With 4 h of idle, `iso-a` at about US$0.78/h costs about US$40.6 (**derived**), over the ceiling. |
| E3.3 memory, OOM | **closed for v2 hosts** | Memory floor against cgroup `memory.max` (v1 fallback present); `m` flavors only; refusal when `oom_kill` is unreadable. The OOM counter has no v1 fallback, so on a v1 host every job is refused. That fails safe, but it blocks the route. |
| E3.4 threads, vCPU | **closed for v2 hosts** | Rayon is always 12. The CPU check ignores the v1 CFS quota (low). |
| E3.5 durability | **closed** | `RESULTS_DIR=/workspace/results` from `rp.sh create`; read by `server.py`; inherited by the shim (`${RESULTS_DIR:-/results}`) through `run()`'s environment. The wiring is correct. `/manifest` exists; the tarball is built on disk, though still read whole into memory to send (low). No job starts without S3. |
| E3.6 host dependence | **closed** | §4. |
| E3.7 Kummer control | **closed (text)** | No Kummer job is on the route. `rp.sh start` (Kummer mode) sends no `seconds` or `cost_per_hr`, so the runner would refuse it. It must be updated before the owner authorises any Kummer job (not blocking now). |
| E3.8 identity | **closed** | Image pinned by digest; `RUNNER_SHA256`, computed client-side, is recorded and checkable against `create.json`'s `dockerStartCmd`; `create.json` has `machine`; the commit is in `runner_request.json`. |
| E3.9 losses | **closed** | §7. See Q2 on reusing a pod. |

### F.4 R4 revision 3: edit by edit

| edit | status | note |
|:--|:--|:--|
| R2-E1 conflicts stop | **closed except case F** | Cases A and D give STOP. A refutation contradicted by the certificate (F) is not flagged. `main` still exits 0 when STOPPED (low; the readings say "STOPPED"). |
| R2-E2 certified-only admission | **closed** | Case B. |
| R2-E3 both draws | **closed** | Case C. |
| R2-E4 identity, sources | **closed except the `p`/`mestre` check** | Case E gives STOP; per-draw sources are listed. Case H is low. |
| R2-E5 pins | **closed** | `iso/BUNDLE.sha256` matches HEAD. The runner verifies the unpacked files and refuses a tampered pin (`iso/INTEGRATION.md`, second run); the commit is sent. Wording: a commit cannot name itself in `PREREGISTRATION.md`. Name the bundle commit, or the SHA-256 of `iso/BUNDLE.sha256`, in the frozen text (F-R4-3). |
| R2-E6 pods, ceilings, no censored-by-cost | **closed** | 12-vCPU `cpu5m`/`cpu3m`. US$37 at the limit, US$38.2 with the runner's one idle hour (**derived**). Refusal above US$40; censored-by-cost removed; the bracket worst case is stated and exceeds US$400, with a running-total stop. The worst case assumes 3 bracket steps, but Amendment 7 B steps until a refutation or a limit; the running-total rule bounds the spend anyway (low). |
| R2-E7 preconditions, loss rule | **closed** | §6. |

### F.5 The coordinator's three questions

**Q1. Is "OOM = stage exit 137 plus an `oom_kill` increment over the whole job" good enough for Amendment 14's `-m112` rule?**

For the X and `cap:` jobs, yes. Their only heavy process is one pilot run.
- An entry-logged "exit 137" means the pilot got SIGKILL while bash survived. The runner's deadline kill takes the whole process
  group, so it never produces that line.
- The only other SIGKILL source in the container is the OOM killer. The kernel counts `oom_kill` on the victim's memory cgroup,
  so a host-level OOM increments it as well.
- The two conditions together identify an OOM kill of that run.

Limits:
- (a) v2 only. On a v1 host the runner refuses every job, although the counter is readable (F.2).
- (b) For multi-stage jobs (`iso-a`, stage B) a whole-job delta cannot be attributed to a stage. Amendment 14 does not apply to
  them, but §3 states the rule without that restriction.

**Q2. Does the `/workspace` volume survive a container restart, and is `RESULTS_DIR` wired correctly?**
- **Survival:** per RunPod's own API spec, yes. Untested here. The volume is host-local (not a network volume), so it is lost
  with a host failure; §7 already says "fetched if reachable".
- **Wiring:** correct (F.3, E3.5). Data up to the last 300 s sync survives; `/work` (container disk) does not.
- **New hazard:** after a restart the runner comes back idle on the same volume. A new `/start` on that pod would merge a new
  attempt's files into the old attempt's `results/`. `iso/INTEGRATION.md`'s second run shows the same kind of mixing
  ("`progress.txt` also holds the first run's four lines").

**Q3. Does either text still overstate what is tested?**

Yes, in three places.
1. **R3 §3, "Tested locally (… the runner checks of 2026-11-14 22:01Z)".** No log of those checks is committed; commit
   `69142b1696` changes only the amendment, `rp.sh` and `server.py`. On that machine the committed runner refuses at the
   memory-floor or `oom_kill` step before it reaches the CPU check or starts a job. So "4 CPUs < 12" and the deadline kill could
   only be observed on a modified copy with checks disabled, as the coordinator's note says for the deadline test. The committed
   runner's success path, with every guard passing, has never run.
2. **R3 §3, "The `oom_kill` counter was not readable on that machine".** Wrong. It is readable through cgroup v1; the runner
   reads only v2.
3. **R4 §3, "It also passed through the runner, run on this machine as the integration test".** True only of the pre-guard
   runners (`1e1e0f596b`, `172ce02e28`). The runner R4 will use (`69142b1696`) has not run `gaux`.

### F.6 Exact edits

**R3:**
- **F-R3-1** (replace the "Tested locally" bullet and its last line in §3): "**Tested locally** on the 4-vCPU, 15 GB, cgroup-v1
  session machine, partly on modified copies of `server.py` with refusal checks disabled (logs not committed): the cost refusal
  (US$50.96 against US$40), the memory-floor refusal, the CPU refusal, the missing-time-limit refusal, a `deadline` kill after
  3 s, and the manifest endpoint. The committed runner, with all guards passing, has not yet run a job; its first run on a pod is
  that job's `gaux` (R4) or the E3.7 control (Kummer)." Delete "The `oom_kill` counter was not readable on that machine." Either
  add a cgroup-v1 fallback (read `oom_kill` from `/sys/fs/cgroup/memory/memory.oom_control`, and the quota from
  `cpu.cfs_quota_us`/`cpu.cfs_period_us`), or add: "The runner reads cgroup v2 only. On a cgroup-v1 pod it refuses every job."
- **F-R3-2** (§6 and the runner's idle term): replace "plus one hour of idle billing" and "Pods are deleted at the first check-in
  after their job ends" with: "The idle allowance is 4 h, the longest check-in interval in force, unless the pod is deleted within
  1 h of the job's end." Set the runner's `3600` to the allowance sent with the request (`idle_seconds`, default 14,400). Note:
  with 4 h, `iso-a` fits US$40 only at or below about US$0.769/h (**derived**).
- **F-R3-3** (§3, OOM sentence): append: "This classification is per job. It is used for Amendment 14's `-m112` rule only in jobs
  with one heavy stage (`cap:` and R4's `x:` jobs). In other jobs an `oom_kill` increment marks the job as lost to memory, and no
  stage is re-run on that basis."
- **F-R3-4** (§7): append: "A relaunch uses a new pod. The runner refuses `/start` when `RESULTS_DIR` is not empty." Add that
  check to `server.py`'s `/start`.

**R4:**
- **F-R4-1** (§5 conflicts and `score_iso.py`): add the conflict "a refuted draw (closure exact or any F4 refutation) whose
  certificate has `solutions > 0`". Implement it in `score()` and add case F to `--selftest`. Make `main` exit 2 when
  `readings["STOP"]` is true. Optional: reject rows with `p ≠ 2013265921` or without `aux_order_method = "mestre"` (case H).
- **F-R4-2** (§3, G-AUX bullet): replace "It also passed through the runner, run on this machine as the integration test" with
  "It also passed through the pre-guard runner (`1e1e0f596b`, `172ce02e28`) on this machine (`iso/INTEGRATION.md`). The guarded
  runner of R3 revision 2 has not run it; the `gaux` stage of the first job on a pod is that run."
- **F-R4-3** (§6, precondition 4): replace "The freeze commit is named in `PREREGISTRATION.md`" with "`PREREGISTRATION.md` names
  the SHA-256 of `iso/BUNDLE.sha256` and of `iso/PINS.json` as frozen. Each job's `repo.tar` is built from a commit whose two
  files have exactly those hashes, and the receipt records that commit." This avoids a commit having to name itself.
  Re-generate `BUNDLE.sha256` after F-R4-1 changes `score_iso.py`.

### F.7 What still gates the first run once the edits are in (owner decisions and external preconditions, not text defects)

1. **Owner:** whether US$40 per job and US$400 for the study apply to reserved compute (R3 §2, R4 §6.3).
2. **Owner:** a one-time confirmation that the RunPod balance covers the projected spend. The API has no balance endpoint
   (R3 §6).
3. **External:** S3 works. `705b151b…` is in the artifact store by hash, and the resume checkpoint and `bench.record/v1` are
   written (R3 §3, R4 §6.2).
4. **Procedure:** R3 and R4 appended to `PREREGISTRATION.md`, with the bundle pins named (F-R4-3).
5. **External, found at pod creation:**
   - RunPod accepts the digest-pinned image;
   - a 12-vCPU `cpu5m`/`cpu3m` pod exists at a live `costPerHr` the guard admits (≤ about US$0.816/h with a 1 h allowance, ≤
     about US$0.769/h with 4 h, for `iso-a`; **derived**);
   - the pod's cgroup exposes `memory.events` (v2) or the v1 fallback is added;
   - `memory.max` ≥ 56 GiB and at least 12 usable CPUs.

   The runner refuses otherwise, and such a refusal is not a loss.
6. **Not for R4:**
   - any Kummer job on this route needs the owner's answers in R3 §2 and an updated `rp.sh start`;
   - the `m = 4` cells at `t = 5, c = 7` and `t = 6, c = 6` still need their own amendment.
