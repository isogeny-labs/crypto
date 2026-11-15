# Red-team review, round 3: draft Amendment 18 (R2-H), capped runs on reserved RunPod CPU pods

**What was reviewed.** Commit `a1c9a4b79c` on `claude/serene-davinci-ujoi58`. During the review the branch head moved to
`ba8821ef02`, but only through merges with main. `git diff --quiet a1c9a4b79c ba8821ef02` is empty for every path
reviewed here:
- `research/pkm_falldegree_unblock_20261107/`;
- `research/prime_fall_degree_20261027/`;
- `research/open_leads_execution_20261108/`;
- the smoke2 bench record.

**Conventions.** Paths are relative to `research/pkm_falldegree_unblock_20261107/runpod_capped/` unless they say
otherwise. "Draft" means `AMENDMENT_R2_H_runpod_capped.md`. Every number here is `[pcc:unverified]`.

**What I did not do.**
- I edited no repository file and made no commit.
- I made no RunPod, AWS or other paid call, and created or touched no pod.
- I did not read `rpc_state/`.

**What I ran.** All of it was offline:
- the test suite, plainly;
- the suite again on a `git archive` copy of `a1c9a4b79c`;
- the request validator;
- `fetch.sh --classify-dir` on the smoke2 receipts, and on five synthetic receipt sets I built in the scratchpad
  (`scratchpad/rt3/probe_scripts/probes.sh`).

`git status` was clean before and after.

## Verdict: freeze with corrections

**What is right.**
- Nothing that is computed changes. I checked this myself: the binaries, `entry_r2.sh` against `entry.sh`, the stage
  strings, `--budget`, `--max-dense`, 12 threads, and the `-m112` derivation.
- All 13 earlier MUST items are closed in the code. The tests reproduce from the commit: 163/163 with the layer, and 117
  plus 46 skips without it.
- The second smoke ran the current scripts byte for byte.
- The offline classifier gives the same verdict on the smoke2 receipts as the one recorded.

**What must change first (three items below).**
- Two defects in how "host-side evidence" grants an automatic relaunch. Each can be fixed with a few lines in
  `fetch.sh` `decide()` and settled with an offline test.
- Text that is stale after the second smoke. It now misstates the evidence on host heterogeneity: `cpu5m` ran on two
  different CPU models.

**What the corrections need.**
- No new pod. The pod-facing code is unchanged.
- A Validator should check two things before the hashes go into the preregistration:
  - the corrected diff is confined to `decide()`, the text and the tests;
  - the suite, re-run with probes P1 and P2 added as cases, passes.
- Bump the classifier version, for example to `a18-r4`, and re-record the section 10 hashes.

## Closure of round 1 (M1-M7) and round 2 (N1-N6)

| item | status | evidence (file:line; test I ran) |
|:--|:--|:--|
| M1 classes conflict, producer choice | Closed | One table, `fetch.sh:157-233`. Abort resolves for the owner: `common.sh:118`. D01-D42 and C01-C19 pass. |
| M2 attempt limit, overwrite | Closed | Allow-list: `common.sh:45-47`, `run.sh:18`. Order: `common.sh:125-155`, `run.sh:48-49`. Serial rule: `run.sh:110-111`. Directories never reused: `run.sh:27-28,169`. Name with a ledger row refused: `run.sh:31`. Refusal once `cost.json` exists: `fetch.sh:248`. Tests A01-A30, B01, B16, E01-E24, C02 pass. |
| M3 no backstop for 90,000 s / US$40 | Closed by owner decision 14, which accepts session plus reaper (`DECISIONS.md` row 14). | `reap.sh` acts only past the hard deadline: `reap.sh:28-34` (F01-F07). Residuals are S1, S2 and S5. |
| M4 Amendment 14 OOM branch | Closed, and seen on a pod | Boot copy loop: `run.sh:177-196`. Smoke2 copied the v2 `memory.events` (`oom_kill 0`), `memory.peak` and `memory.max` (`receipts_dev/pkm-x-dev-smoke2/admission-lines-1.txt`). `oom_kill` is read at `fetch.sh:107-112` (C10, C11, D10-D12). |
| M5 client hash | Closed | I ran `git show d2a16c723d:…/runpod/rp.sh | sha256sum` and got `b4b60199…`, equal to `rp_pinned.sh`. Pins checked at `run.sh:62-66`. Tests A13, A14, G-pin. |
| M6 rule 7 file, vacuous reserved check | Closed | `run.sh:218-224`, `fetch.sh:139-142` (B05, B06, D23). Smoke2's `pod-create.json` has `machine.secureCloud: true` and `costPerHr` 1.04, equal to `price.jsonl`. |
| M7 host dependence | Code closed; text now stale (see MUST R3) | One flavor, no fallback: `common.sh:18`, `run.sh:113-121` (B02). The second smoke host was AMD EPYC 9575F; the first was a Threadripper 7960X. |
| N1 abort buys a relaunch | Closed | `lostlike` requires LOST with relaunch `auto` (`common.sh:118`). Tests A25, A29, E03, E24. |
| N2 no information counted as loss | Closed | Positive evidence only: `fetch.sh:160-164`. Collect decision: `fetch.sh:379-389`. Tests C06, C08, C08b, C12, D07, D08, D08b, D15, D21, D24b, D24d. |
| N3 manufactured loss, discarded tree, reaper | Closed as written, with the owner-accepted caveat. Two new holes in the relaunch rule: MUST R1 and R2. | (a) `lastStatusChange` is recorded in check-ins (`fetch.sh:276-279`), and relaunch `auto` needs host-side evidence before any logged action (`fetch.sh:178-187`); tests C05, C13, D07b, D24, D24c, D38, D39. (b) Intent record, numbered collections and orphan recovery: `fetch.sh:337-348,393-398`; tests C18, C18b, D41, D41b, and my probe P5 (ADMITTED). (c) Reaper only past the hard deadline: `reap.sh:28-34` (F05, F06). |
| N4 memory rule versus Amendment 14 | Closed | `fetch.sh:206` and `common.sh:120` require `oversize false`. Tests D37, D37b, E23. |
| N5 tests not reproducible, logs dropped | Closed | `.gitignore:6-9`; fixture logs are tracked. `git check-ignore` does not ignore `receipts_dev/**/run.log`. From a `git archive` of `a1c9a4b79c`: 163/163 with the layer, 117 plus 46 SKIP without it. |
| N6 state dies with the session | Closed in code; one smoke claim unsupported (S3) | Ledger-row refusal: `run.sh:31` (A27, B16). Push reminder: `common.sh:58`. Token recovery: `fetch.sh:261-268` (C17). |

## New MUST items

### R1. Host-side evidence is not tied to the loss (wrong against owner decision 13, "positive host-side evidence of loss")

**Where.** `fetch.sh:180-184` takes the *first* check-in, before any logged action, whose `lastStatusChange` differs from
start and does not say "by user". Draft §4.3:357-363 says the same.

**The defect.** Any non-user status change recorded earlier in the run makes every later LOST `auto`. That includes a
loss whose own status change names the user.

**Probe P1.**
- Check-in 1: the runner is running, and `lastStatusChange` reads "Migrated: host maintenance".
- Check-in 2: the runner is idle, and `lastStatusChange` reads "Restarted by User: …".
- Result: `LOST P3, relaunch auto`, with host_side citing check-in 1.

So the "by user" exclusion, the one safeguard the route has against an unlogged session restart, is defeated whenever an
unrelated RunPod-side change happened first.

**Fix.**
- Count a host-side record only if it is dated after the last check-in that saw the runner alive under `s0` (running or
  done), and at or before the first loss evidence.
- If the loss check-in has an HTTP 200 record whose `lastStatusChange` names the user, the relaunch goes to the owner.

**Cheapest test.** Add P1 as a D-case expecting `relaunch owner`. C05, C13, D07b and D24 must still give `auto`.

### R2. A loss after an observed completion gets an automatic relaunch (unsupported by decision 3 and 13; outcome choice)

**How it happens.**
1. A check-in records the runner `done` under `s0`. Its `status-<stamp>.json` carries `progress.txt`, so the pilot's end
   time is in the receipts. That end time shows whether the pilot used its 86,400 s budget, that is, whether the row will
   be `timed_out`.
2. The transfer check fails (`fetch.sh:411-412`, exit 6, pod kept).
3. The pod then disappears (not listed, GET 404).

**Result.** `LOST P3, relaunch auto` before the run deadline (probe P2), and `LOST P6, auto` after it (probe P3). I ran
both.

**Why this is not what decisions 3 and 13 cover.** On EKS this case cannot happen: `entry.sh` syncs the row at exit, so a
reclaim after completion loses nothing. Decision 3's relaunch is for a reclaimed computation. Here the receipts show that
the producer saw the completion, and a read of the timing, before the loss.

**How it differs from the accepted caveat.** Like that caveat, this needs an unlogged deletion. Unlike it, the case is
visible in the receipts and costs nothing to close.

**Fix.** The relaunch goes to the owner when any check-in dated before the first loss evidence shows `live.state == "done"`
with `live.started == s0`.

**Cheapest test.** Add P2 as a D-case expecting `relaunch owner`. D24 and C13 must still give `auto`, because their
earlier check-ins show `running`.

### R3. The text to be frozen is stale after the second smoke and misstates the host evidence (wrong)

| place | what the draft says | what the committed smoke2 receipts show |
|:--|:--|:--|
| §2.4, lines 108-109 | whether the pod has the v2 or the v1 cgroup files "is **not known**" | v2: `fstype` is `cgroup2fs`, and `memory.max` is 128,000,000,000 (`cgroup-prestart.json`, `admission-lines-1.txt`) |
| §2.7, lines 150-152 | "The smoke pod's host was an AMD Ryzen Threadripper 7960X"; `MemTotal` 263,400,384 kB | host `AMD EPYC 9575F 64-Core Processor`, `MemTotal` 527,605,596 kB (`admission-lines-1.txt`) |
| §4.2, line 348 | before start the session sees the host's thread count and memory, "not its CPU model" | runner `nproc` 128 and `MemTotal` 527 GB (`status-ready.json`, `run.log` "runner ready"). These separate the two observed hosts. The pre-start lever stays closed by N6; the wording is wrong. |
| §9, point 5 | lists as future checks the ones smoke2 made | cgroup files, `memory.max`, `lastStatusChange` at creation, `env` in GET (but see S3), the `--delete RECORD` path and `reap.sh` were all exercised |

**Why it matters.** Section 7's control ("one flavor is pinned"; RunPod "may still" vary the CPU model) is now contradicted
by an observation. `cpu5m` ran on a Zen 4 HEDT part and on a Zen 5 server part. At t = 14 the wall time is projected at
about 75,900 s against an 86,400 s budget, so the CPU model can decide `timed_out`.

`SMOKE.md`'s second-smoke section does not mention the CPU model either.

**Fix.**
- Update §2.4, §2.7, §4.2, §7 and §9.5, and `SMOKE.md`, from these receipts.
- State that the flavor does not pin the CPU model: two smokes, two models.
- Say that the two t = 14 draws, and any `-r1`, may land on different CPUs.

**Cheapest test.** Read `receipts_dev/pkm-x-dev-smoke2/admission-lines-1.txt` and `status-ready.json`.

## SHOULD

- **S1. The reaper takes no lock (the statement is wrong).**
  - Draft §3:162-163 says `run.sh`, `fetch.sh` and `reap.sh` each take a lock per name. `reap.sh` has no `flock`.
  - Its direct DELETE (`reap.sh:46-50`) can run while a session `fetch.sh` is collecting, for example after `fetch.sh`
    gave up on the 900 s lock wait (exit 2).
  - Fix: lock the name around the direct delete, or correct the text.
- **S2. Worst-case pod life is about 31 h, not 30 h.**
  - "At most every 4 h" (§2.6) is ambiguous. It should read "at intervals of at most 4 h".
  - A reaper pass can add about 50 minutes: up to 15 min of lock wait, 2 × 15 min of fetches, and the polls.
  - Pod life is therefore about 26 h + 4 h + 0.8 h. The guard `price × 30 h <= 40` admits prices up to US$1.333/h, at
    which 30.8 h costs more than US$40.
  - This does not bind at US$1.04/h. Fix: set `WORST_HOURS` to at least 31, or require a reaper interval of at most
    3.5 h.
- **S3. Two SMOKE.md claims are not supported by any committed receipt.**
  - "`GET /pods/{id}` returns `env` (keys `PUBLIC_KEY`, `RUNNER_TOKEN`), so token recovery works". Every receipt redacts
    `env`, and no recovery ran: `fetch.log` has no "recovered" line.
  - Fix: record the key names (not the values) in a receipt, or exercise the recovery once at the next dev pod. Until
    then, call the claim "observed by the session, not in receipts".
- **S4. Two RunPod behaviours that the relaunch rule depends on are unobserved.**
  - (a) What GET returns for a deleted pod. Form 2 of host-side evidence, and tests C13 and D24, assume 404. Fix: add
    one GET after the confirmed delete in `delete_phase` and save it as a receipt.
  - (b) The "by user" exclusion is a deny-list built on one observed string. Probe P4 ("Exited: stopped via API") gives
    `auto`. Disclose this.
- **S5. Pods and costs that no script handles.**
  - A pod whose pre-start deletion was not confirmed (exit 11) cannot be deleted by any route script until creation plus
    26 h. `fetch.sh` needs `receipts/NAME`, which was moved to `.prestart-k`, and `reap.sh` waits for the hard deadline.
    The pod idles for up to 30 h and blocks the serial rule.
  - Pods with no ledger row:
    - a pod created by a timed-out create that appears after the six name polls (the `nocreate` path);
    - a pod whose `run.sh` was killed between the create response and `pod-create.json`.
  - Fix: let `reap.sh` delete at once any pod recorded in a `.prestart-*` directory with `confirmed_gone: false`, or
    any pod with no matching receipts that is younger than 26 h.
- **S6. The US$400 study cap (unpriced).**
  - The ledger has `complete_as_of never` and none of the study's EKS or `pfd-b-rp` rows, so the draft cannot show that
    the remaining budget covers the three runs (US$93.60).
  - Only this route's `run.sh` consults the ledger. Amendment 16 and 17 launches do not, so concurrent launches on other
    routes can push the study past US$400.
  - Open rows of other routes should carry their own ceiling (US$40), not "30 h" (§5).
  - Fix: state the spend to date at the freeze, and have other routes check the same ledger, or reserve their worst case
    in it.
- **S7. A wrong statement about `/start`.** Draft §3:221 says "`/start` refuses any hash other than the frozen ones". In
  fact `server.py:69-72` compares the uploaded files against the hashes *sent in the request*. The actual protection is
  that `run.sh` calls `/start` immediately, and admission rules 1 and 5 check the pins. Correct the text.
- **S8. A loss after crossing 56 GiB is not defined.**
  - Suppose a 56Gi attempt's partial tree shows a `VmHWM` above 56 GiB and the run is then LOST (P3). The route
    consumes the cell's relaunch and runs `-r1` at 56Gi.
  - EKS would have hit `OOMKilled` first and given the `-m112` run without consuming the relaunch.
  - Fix: state the rule in §4.2, even if it is "partial trees are never read".
- **S9. A secret could reach `run.log` on a failed create.** On a create failure, `run.sh:211` logs the response with a
  top-level `del(.env?)` only, or the raw first 500 bytes. A response that echoes the request body would put the pod token
  in `run.log`. Fix: scrub the token value from anything written to receipts.
- **S10. The check-in window can be 5 minutes.** "At least one [check-in] in the hour between the run deadline and the
  hard deadline": that interval is only 5 minutes when the runner starts 55 minutes after creation. Fix the wording.
- **S11. The upload check names an older record version.** `upload-check.txt` names the bench record by SHA-256
  `a639abc4…`. The committed record is `e0fdd2ca…`, edited when it was closed. Its 19 `raw[]` SHA-256s equal the
  manifest; I checked that. Their presence in S3 I took from the record.

## Focus answers, briefly

1. **Choosing an outcome after seeing a result.**
   - Closed in the receipts, except R1 and R2.
   - Remaining, and accepted by owner decision 13: an unlogged deletion while the run is still running is
     indistinguishable from a reclaim.
   - P0 (unparsable receipt) and P0b (collections disagree) give NO_EVIDENCE, which goes to the owner and never gives an
     automatic relaunch.
2. **Comparability.** Nothing computed changes beyond what the draft discloses, except that the CPU-model heterogeneity is
   now observed (R3).
3. **Deadline and cost.**
   - A pod can outlive 30 h only if the session and its reaper stop (accepted under decision 14), and by the S2 margin.
   - The US$400 cap is enforced only at this route's launches (S6).
   - `reap.sh` touches only pods whose names start with `pkm-x-` (`common.sh:105`, F01, F07). `pkm-x-` appears nowhere
     else in the repository at this commit.
4. **Secrets.**
   - No pod token or API key is in committed receipts, tests or branch history: I scanned for 48-hex strings and
     `Bearer` or `X-Token` values.
   - The smoke1 receipts keep `consumerUserId`; `SMOKE.md` discloses this.
   - The token appears in curl argv (in `rp_pinned.sh`); this is disclosed. See also S9.
5. **Smoke2 claims.**
   - Supported: launch commit and hashes, flavor, data center, price, boot, upload hashes, cgroup v2 and its limit, the
     memory figures, the `lastStatusChange` text, the single collection, transfer true, ADMITTED P2d, the
     upload-gated delete (HTTP 204, gone), 188 s and US$0.0543, and the reaper's empty list.
   - Not supported by receipts: `env` in GET and "token recovery works" (S3).
   - Omitted from `SMOKE.md`: the CPU model (R3).
   - The offline verdict matches the recorded one: `fetch.sh --classify-dir` gives a JSON identical to `outcome.json`,
     facts included. It uses the git-excluded tree, which equals `local-sha256-1.txt`.
6. **Reproducibility from the commit.**
   - Reproduced from the commit: the hashes, the pins, the `entry` diff, the `-m112` derivation (`cmp`), the validator
     (VALID ×6; notes are 1,964 characters), and the counts.
   - The classification of the smoke2 tree needs the S3 raw tree, or the local copy I used.

## Verified myself versus taken from the text

**Verified myself.**
- The section 10 SHA-256s of all six files and of `server.py`.
- The `rp_pinned.sh` equality with `d2a16c723d`.
- The `entry.sh` and `entry_r2.sh` diff: header, usage line and `m4n` branch only.
- The `image.json` and `binaries.sha256` pins.
- The `-m112` re-derivation, and the validator output.
- Amendment 7 A's `--max-dense` arithmetic at 56Gi and 112Gi.
- The pilot's peak line format and unit (`VmHWM` kB / 1024), and the chunk size at `f4_fp_tower.rs:1694` (both at
  `f13a690df0`).
- The cited 20,618 MB, 1,762.8 s and 11,570 s values.
- The suite, three ways.
- The offline reclassification of smoke2, and the probes P1-P5.
- The boot template's SHA-256 against `pod-request.json` and against `run.sh`.
- That no route script changed between the smoke's launch commit `fd5a92da08` and `a1c9a4b79c`.
- The bench record's `raw[]` against the manifest.
- The secret scans.

**Taken from the text or receipts, not verified.**
- That RunPod's live API behaves as the receipts record.
- The S3 upload of the raw files.
- The owner's acceptance of the caveat in decision 13. `DECISIONS.md` row 13 quotes the question with an ellipsis, and
  the caveat is not in the recorded text.
- The completeness of the study's past spend.
- The proxy's handling of credentials.
