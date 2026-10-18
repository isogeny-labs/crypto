# Constant-time probe, base vs new (callgrind, all threads)

Inputs (`inputs/`): two L1 secret keys with one valid and one foreign
(FO-rejected) ciphertext each, written once by the **base** build
(`ct_probe_files gen`, eigenx fc02c6b); both builds read the same files.

Per case: `valgrind --tool=callgrind --dump-line=no ./ct_probe_files run|prep <in> <k> <valid|foreign>`.
`secret` = total Ir of `run` − total Ir of `prep` (derived), i.e. the instruction
count of `decaps_core` over all threads. Builds: base = eigenx fc02c6b,
new = eigenx 1eeb2d3 + examples/ct_probe_files.rs (library unchanged).

`summary.txt` holds the measured `run`/`prep` totals and the derived
difference; `*.out.*.txt` the shared secrets (identical across builds).
Earlier probe variants (main thread only via --toggle-collect; per-thread
files) were discarded: the first misses worker threads and the second
cannot separate phases because valgrind reuses thread ids.
