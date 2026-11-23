# Run51 status note (evidence-backed), 2026-11-18T03:5xZ

All statements here are [pcc:unverified]; no pcc claim was filed or checked.
The only existing pcc id near this run is PCC-20261117-74aa6e (Unit51 source
controls, standing UNVERIFIED per the session log); it covers the source, not a
result.

## Timeline from repository evidence

| Time (UTC) 2026-11-17 | Event | Evidence |
|---|---|---|
| 12:26:57 | Source frozen, `f5fdb2d04c` "Integrate audited five-variable Nagao descent into deferred graph IC" | PR #4537 commit list; `origin/measure/unit51-source` = `f5fdb2d04c2efdc89381d74ecae9da9bcea2d511` |
| 12:29:44.784 | Pod `7zki4wnz56fzad` created, 4 CPU/8 GiB CPU5c, actual 0.14 USD/h; detached guardian, absolute deadline 14:29:44Z | `docs/session-logs/2026-11-17.md` line 27025 |
| 12:30:41 | Sole dispatch POST accepted 202; runner status 200 `started=true`, unfinished | same, line 27025 |
| 12:34:27 | Last PR #4537 science commit `e394138bb7` "Checkpoint frozen Runpod51 invocation and guarded allocation" (touches only the session log) | `gh api .../pulls/4537/commits`, `.../commits/e394138bb7` |
| 14:29:44 | Guardian absolute deadline (deletion expected if the guardian process was alive) | session log line 27025 |
| 15:14:40 | PR #4537 merged (auto-merge); body still says "No51natural-stream result yet." | `gh api .../pulls/4537` |
| 18:00:30 | `main` head `833a35d21e`; no later commit under `research/extension_ic_graph_20261115` | `gh api .../commits?path=...` |

The session log's last Run51 instruction (line 27029) is "Poll authorized
capture status, collect originals, delete, then interpret ...". No entry after
it records polling, collection, a qualifier exit code, `deletion.json`,
`CUSTODY.txt` or an independent `GET 404`. Compare Run50, whose directory
`runpod-deferred-pair-20261117-50/` holds `deletion.json` (DELETE 204, GET 404)
and `CUSTODY.txt`; no `runpod-unit-deferred-20261117-51/` directory exists.

## What is known lost / recoverable

- Lost from this machine: the original returned archive, `qualified-output`,
  `summary.json`, the private dispatch log and the guardian log. Their stated
  locations (`/workspace/ic-runpod-runs/runpod-unit-deferred-20261117-51`,
  `/tmp/ic-descent51-native`) are absent here.
- Possibly recoverable: (a) the dispatching Codex host's local copies, if that
  host still exists; (b) the pod's own volume, only if the pod still exists
  (the runner wrote to its workspace; Run50 used an authorized capture endpoint
  on the pod).
- Recoverable without compute: source (`measure/unit51-source`), protocol,
  source/build hash manifests (`validation/unit51-*.sha256`), control logs. The
  worker/checker binaries themselves were LOCAL in `/tmp/ic-descent51-native`
  and are not here, but are rebuildable from the pinned argv files
  (`validation/unit51-worker.argv.shell`, `unit51-checker.argv.shell`) given the
  pinned Root41 libraries, which are also absent here.

## Billing exposure (bounded estimate, not an observation)

Pod status could not be read (REST 502, GraphQL 401). Two cases:

- Guardian worked: lifetime 12:29:44Z-14:29:44Z, 2.0 h x 0.14 USD/h = about
  0.28 USD; within the registered 0.80 USD cap.
- Guardian did not run (for example the detached process lived on a host or
  session that ended): pod still billing at 0.14 USD/h since 12:29:44Z, about
  15.4 h x 0.14 = about 2.15 USD by 03:51Z 2026-11-18, rising about 3.36 USD per
  day, plus any volume storage. This would exceed the registered 0.80 USD
  lifetime exposure.

Which case holds is **not known**. That uncertainty is the main open risk and
is why a read-only check by someone with a working credential is the first
next step.

## What was not checked

- Runpod pod existence/status/cost (API unavailable to this session).
- S3/custody stores: no AWS CLI or credentials here; Run51's protocol says
  large originals stay LOCAL and no artifact-store upload was planned, so S3 is
  not expected to hold it.
- Whether `git fetch origin` would reveal unindexed objects: the fetch failed with
  `No space left on device`; `git ls-remote --heads` (2452 heads) was used instead.
