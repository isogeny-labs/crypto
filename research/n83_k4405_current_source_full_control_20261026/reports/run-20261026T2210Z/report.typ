#set page(paper: "a4", margin: 1.65cm)
#set text(font: "Helvetica Neue", size: 10pt)
#set par(leading: 0.65em)

= N83 K4405 current-source resource control

*Run:* `evidence-01`, started approximately 2026-10-26 22:10:56 UTC.  \
*Status:* `BOUNDED_UNKNOWN_HOST_SWAP_ABORT`.  \
*Candidate:* `IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h76ff6ddee28c`.

The K4405 wide3 producer was run on earlier public N83 fixture zero on a local macOS host. The wrapper admitted the host at 53% system-wide free memory and sampled the child process and host during a 1,200-second outer cap. An operator sent SIGTERM to the worker process group after observing host swap pressure. The terminal receipt records exit code -15 and 873.169 seconds of wrapper wall time. This is a bounded resource control; the source, executable, point, flags, receipt, and samples are retained in `../../evidence-01/`.

== Recorded resource trace

#figure(
  image("resource_trace.svg", width: 100%),
  caption: [K4405 `evidence-01` resource samples (n = 1,613). Child RSS is recorded every sample; host free memory is recorded periodically. Dashed lines show the configured 24 GiB child RSS cap and 10% host-free stop gate. The manual stop was prompted by host swap pressure, which this original wrapper did not monitor.],
)

The largest sampled child RSS was 11,680,759,808 bytes (10.88 GiB after byte-to-GiB conversion). Recorded host-free memory ranged from 33% to 54%. The final sample was at 872.624 seconds. The post-stop `vm.swapusage` line in `manual_abort.json` reads `used = 12092.38M`; the record does not attribute swapped pages to an individual process. The producer emitted no rank, base dump, relation matrix, or target receipt before termination.

== Reproduction and evidence

- Frozen run configuration: `../../evidence-01/config.json`.
- Terminal receipt and operator stop: `../../evidence-01/receipt.json` and `../../evidence-01/manual_abort.json`.
- Resource samples: `../../evidence-01/samples.json`; its SHA-256 is bound by the receipt. `plot.py` verifies that digest before rendering `resource_trace.svg` and `resource_trace.png`.
- Exact executed wrapper: `../../evidence-01/runner_v1.py`; the historical file is preserved byte for byte. The source and binary digests are in the configuration.

The graph displays recorded values only. The run's local host did not have a Tier 0 or Tier 1 timing receipt. The separately sealed cross-family candidate comparison remains pending on eligible hosts.
