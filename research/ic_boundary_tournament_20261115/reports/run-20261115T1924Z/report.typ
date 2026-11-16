#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt, font: "Arial")
#set par(leading: 0.65em)
= Four-rung single-target tournament
#text(size: 15pt, fill: rgb("#8b3a23"))[Queued provisioning snapshot]

Job `ic-boundary-single-20261115-1912` was created on November 15, 2026 at 19:24:05 UTC. Its sole pod remains Pending. Karpenter's worker launch calls are refused by AWS with `MaxFleetCountExceeded`. No comparison process has started.

#image("queue.svg", width: 100%)

#table(columns: (1fr, 1fr, 1fr, 1fr), inset: 7pt,
  [Rung], [Eval requested], [Tune per K], [Completed],
  [n=61, a=0], [64], [4], [0],
  [n=71, a=0], [16], [1], [0],
  [n=73, a=0], [16], [1], [0],
  [n=83, a=1], [16], [1], [0])

== Frozen limits and checks
One sequential Job; fixed m8a.2xlarge; pod requests/limits 4 CPUs and 24 GiB; Spot-only L2 pool; zero experiment retries. USD 20 exception authorized. Original deadline: November 16, 2026 at 08:11:12 UTC (00:11 Pacific). Preparation and queue time consume the same 13-hour window.

Autolab Python tests: 52/52 passed. ShellCheck passed. Image smoke verified Rust 1.99.1, Python 3.11.2, AWS CLI 2.40.3, required utilities, frozen source and local dependency manifests. Native build, timings, replay, claim checks, A/A coverage and achieved isolation remain untested for this job.

== Source and evidence
Algorithm revision: #text(size: 8pt)[`a10cb7b4f8e48f094efe70c31d9a24b6d72aee4a`].
Launch revision: #text(size: 8pt)[`ee2052860bcf1b02c08485fa65a84caea75e3be3`].
Image digest: #text(size: 7.3pt)[`35af97d51319db07a1092ff8b8101b9c57c696720489fd88f623e1c71e111f13`].
Job UID: `751ec9e0-1e39-4c6e-a592-cc76c66a59a9`.

This report sits beside `job-snapshot.json`, `pods-snapshot.json`, `nodeclaims-snapshot.json` and `fleet-error.jsonl`. The Markdown report links the exact request, manifest, image receipt, runner, watcher and AWS API documentation. All local prelaunch receipts remain in the autolab run directory.

AWS requires explicit IDs to describe instant fleets; an empty default listing is not a complete inventory. Five historical candidates were already absent. The internal fleet-count cause remains unresolved. No infrastructure limits or other workloads were changed.

There are no comparable algorithm measurements, timing ratios, uncertainty intervals or ledger changes from this snapshot. Canonical algorithm charts remain unchanged. The original Job stays queued with its original deadline; the bounded watcher does not relaunch it.
