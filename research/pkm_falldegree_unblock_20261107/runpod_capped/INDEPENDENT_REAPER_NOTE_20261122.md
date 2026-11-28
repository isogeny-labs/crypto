# Note on the hourly independent reaper (2026-11-22, after its first pass)

Every statement is `[pcc:unverified]`. `INDEPENDENT_REAPER.md` is a procedure file whose hash Amendment 33 section 10
records; it is not edited. This note records how the stored task differs from it, and why.

- **First pass (2026-11-22, fired 00:41Z, session `cse_01AXUKfTwsFADek3cPKmD7hs`).** It cloned the branch, ran
  `reap.sh` with a fresh `RPC_STATE`: exit 0, one `pkm-x-` pod listed (`pkm-x-t14-k0-c7-d54`, id `46duu2mnh5dyz6`,
  hard deadline 2026-11-23T01:40:15Z), not due, no action. Deleting would have worked: it needs only the RunPod API.
- **What failed.** Its one-line receipt commit could not be pushed: the routine's sessions do not have
  `isogeny-labs/crypto` among their authorized sources, so the git proxy answers 403 (also for a `reaper/` branch).
  The pass also amended its own unpushed commit to change the author, which the prompt now forbids.
- **Change to the stored prompt (task `trig_01RRGvQp8VdUtEKBjAn99RHH`, 2026-11-22T00:57Z).** One push attempt, no
  retries, no other branches, no amend; whether or not the push works, the pass prints every file it created or changed
  under `runpod_capped/` verbatim in its report, and the main session commits those files from the transcript. Push
  notifications only when a pod was deleted, the exit code is not 0, or the pass failed early.
- **Why this matters for the classifier.** If a pass deletes the study pod while the main session is down, its receipts
  reach the branch only through the main session copying them from the transcript. Until they are copied, a `fetch.sh`
  run would see the pod gone with GET 404 and no logged session action (relaunch `auto` under owner decision 13); the
  main session reads the reaper's transcripts before any such classification and records the reaper's delete.
- **Cost.** The first pass used about US$1.04 of model usage by its own run record; up to about 26 passes run before
  the hard deadline.
