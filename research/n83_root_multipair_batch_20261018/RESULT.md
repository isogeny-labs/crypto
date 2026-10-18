# Four-pair N83 root batch screen

The opt-in four-pair implementation preserved all 10,879,232 K512 compact
states, 21,756,398 indexed roots, the historical full state digest
`73ce87c21a6bb20472daafff1a1c72a4cebb78c53d63beada5c7d76f635e2687`,
and the historical full table digest
`67dac08b8514076544f8ca53e2306ba39118c90f1ce6a3edc308a1fff2e75a70`.
The focused first-witness parity test and all 21 Koblitz example tests passed.

The clean [ABBA attempt](local_pilot3/analysis.json) ran on an ordinary local
ARM64 host with a recorded launch load average of 77.83. All four child exits
were zero. Index-build milliseconds in pair-1, pair-4, pair-4, pair-1 order
were 44,076.762, 48,199.690, 38,739.230, and 33,711.368. The two paired
pair-1/pair-4 ratios were 0.914 and 0.870; means were 38,894.065 ms for
pair-1 and 43,469.460 ms for pair-4. The four-pair option showed no stage
gain in this screen and is not promoted to the K2816 cloud panel.

These are index-stage diagnostics, with `candidate_id: null`; no one-target
IC/rho comparison was run for the four-pair option. The host was heavily
loaded, so the measured difference does not establish a stable cross-host
slowdown. The checked source, runner, analyzer, exact base and hashes are in
[the protocol](PROTOCOL.md). The first launch's blocked `sysctl` receipt and
the second launch's four failed `time -l` wrapper statuses are preserved in
`local_pilot1` and `local_pilot2`; the latter four child JSON rows matched
the full historical digests but were not admitted as successful runs.
