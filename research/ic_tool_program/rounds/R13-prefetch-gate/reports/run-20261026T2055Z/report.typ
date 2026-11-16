#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#show raw: set text(size: 8.5pt)

= R13 shared-host A/A preflight

*Status: stopped resumably after 60 busy-machine refusals.* This is a failed
isolation start, not an IC cost comparison. The frozen protocol is
`research/ic_tool_program/rounds/R13-prefetch-gate/PROTOCOL.md`; raw records
are in that round's `runs/` tree. Run report: `REPORT.md` beside this PDF.

- *Host:* EC2 `i-03d5e15a586275e77`, `m8i-flex.8xlarge`, Intel Xeon
  6975P-C, 32 logical CPUs; CPU 2's SMT sibling is CPU 18.
- *Source:* clean worker snapshot `381415d41320bde44fff7c214ba2bf5169238c84`;
  base `261482646ee89b332dca69e2f296b0774c25d2c2`, candidate
  `efc6336d55c59cceb92cb82de73598ebe15ff3ca`, rustc 1.99.1.
- *Binaries:* base SHA-256 `df0c4df05632d002d9d7be4b3ebeaf675b15495e16b6c0fe8592bfd6cbc7f96d`;
  candidate `2537d8b4d356cb3bc7534dd48edd859a9b20451418681eb32d9fad02b7d79c9f`.
- *Untimed controls:* 24/24 frozen holdouts regenerated; native v0 pin held
  90/90 with matching names. The earlier strict v0 schema failure remains
  preserved. The shared worker passed 114 native `icprog` tests.
- *Requested timing:* five A/A rounds, then five ABAB rounds over the M1
  suite and fresh holdouts, one cold `ic price --single-target` process per
  public target, same-target signed-Frobenius rho and verified scalar.

#figure(image("preflight.svg", width: 100%), caption: [Executed preflight path,
R13 shared host, 2026-10-26. Counts come from `runs/aa/refusals.log` and
`runs/pin/pin.json`. No timed performance value was produced.])

== Observations and next gate

The first launch recorded five SMT-topology refusals. The corrected launch
used `ICPROG_BENCH_CPUS=2,18` from 20:55:55 UTC. All 60 quiet-host attempts
for the first A/A row refused machine contention. The final refusal at
21:48:33 UTC recorded 64.09 other-process CPU seconds in a two-second
window, above the fixed 0.20-second limit; PSI some avg10 was 73.59 against
5.0. The runner then stopped resumably (`aa-return.log`).

*Timed output:* zero A/A price files, zero suite or holdout timed files.
No cold ratio, `S`, interval, candidate decision or Table A row follows.
The refusal log has 65 entries (five topology, 60 busy-machine). The exact
run tree is also preserved at
`s3://setec-astronomy-artifacts/checkpoints/crypto/codex__ic-single-target-tournament-20261026/20261026T212010Z-567a3f37aab0/worker-runs-final/`.

The next discriminating step is a quiet shared-host window or one
owner-authorized on-demand worker after AWS removes the explicit IAM deny.
On a new host, repeat source, host-class, holdout, pin and isolation checks
before the frozen paired schedule. Do not pool hosts. The leaderboard and
canonical performance graphs have no new admitted value to render.
