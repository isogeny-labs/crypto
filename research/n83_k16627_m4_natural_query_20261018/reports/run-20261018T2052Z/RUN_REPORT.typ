#set page(paper: "a4", margin: 13mm)
#set text(font: "New Computer Modern", size: 9.3pt)
= N83 K = 16,627: spot-host natural-query run

*Interval:* 2026-10-18 20:52:39 to 2026-10-19 01:29:24 UTC. *Protocol:*
`REMOTE_PROTOCOL_V2.md`, frozen before P2/A1/B/A2. Public primary curve
`icv1-f2m83-tm6151469093347-debefd74`, fixture 0, policy
`public_x_hash`, seed 2026100801, m = 4. Retained base: 16,627
signed-Frobenius columns, 2,760,082 points, compressed-object BLAKE3
`1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`.
Worker source commit `65392fb874584a0a5845515f0f8de0510b9c702c`; x86_64 worker
SHA-256 `326afc55077c080daae40244da5dfdecf2916c65c587260c3fca09f2a17b1887`.

#image("sequence.svg", width: 100%)

== Observed arms

#table(
  columns: (1fr, 1fr, 1.15fr, 1fr),
  inset: 3pt,
  [*Arm*], [*Exit/status*], [*Process wall*], [*Peak bytes*],
  [P2 preflight], [0 / PREFLIGHT_ONLY], [43.737231 s], [2,569,699,328],
  [A1 selectors], [0 / PASS], [60.034319 s], [2,569,474,048],
  [B natural], [124 / UNKNOWN_budget], [1,790.010187 s], [2,569,474,048],
  [A2 selectors], [0 / PASS], [68.854539 s], [2,569,539,584],
)

One CPU, a hard 4 GiB cgroup, zero swap and network denial were enforced on
shared EC2 `i-01f85a6c905181498`; wall times are L0 diagnostics. P2 imported
the base in 34,768.419 ms and validated the public point in 2.940 ms. A1/A2
imported it in 43,121.059/48,937.666 ms and recorded solver fields of
15,923.239/19,098.983 ms. Each returned the same group-checked rank-one
constructed-selector relation. A separate local native replay checked both
receipts and rejected changed-x and changed-row inputs.

B loaded the base and entered relation collection at 61.656 s, then wrote
`UNKNOWN_budget` at its 1,790-second internal limit. No attempt, progress,
relation or rank receipt exists for B. `memory.events` records zero OOM
events. A2 began 4 h 02 m 28 s after B ended because the local session was
interrupted. Different workloads and the shared host prevent a timing ratio.
The 100-file `spot-raw-files.sha256` manifest binds complete raw outputs and
the control replays. P1's earlier cgroup-path infrastructure failure is
retained separately. The full path-by-path report is `RUN_REPORT.md`.
