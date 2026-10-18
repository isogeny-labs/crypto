# N83 four-pair root batching stage screen

`KIC_INDEX_ROOT_PAIR_BATCH=4` groups four adjacent representative pairs into
one Montgomery inversion. The root solver fills a contiguous result buffer,
then the index consumes those roots in the original left/right/relative order.
The default is one pair. The flag requires `KIC_INDEX_ROOT_BATCH=1` and accepts
sizes 1 through 8. The index builder remains opt-in, and this is an index-stage
measurement with `candidate_id: null`.

Freeze source SHA-256
`0b3fe1952e5ea193373b539a1c3149c25945b0c56f585ed7e626e204078f12d1`,
runner SHA-256
`4c0eac85981f88c2ce0d2531a627e17b8c25bbd495f960fee00dc643f197219f`,
analyzer SHA-256
`b6283f45ffb2a5ee235c56e246d53dd39e828b05b6c8a77a4c4ea051ee0e0c68`,
and K512 base dump SHA-256
`7b97fbdb231418a5caccae429b2bb62162865a4a1421399b53573eff32d47b33`.
The base has 84,992 usable points, 512 orbit columns, and BLAKE3 base hash
`fc53690f6fa19ef8765a7e4ad09d6f475a582e69fdefc43dd77f5b401f9b3058`.

Run fresh processes in pair-1, pair-4, pair-4, pair-1 order, each capped at
180 seconds, with the sparse canonical, root-only 11-byte, indexed five-byte,
75% load, symmetric S3 flags used in the earlier sparse K512 pilot. Freeze
the same source, base, and host for all four arms. The analyzer requires
successful exits, the stated base identity, 10,879,232 regular states,
21,756,398 indexed roots, and identical full BLAKE3 state and table digests.
Those digests must also equal the earlier sparse K512 panel's
`73ce87c21a6bb20472daafff1a1c72a4cebb78c53d63beada5c7d76f635e2687`
state digest and
`67dac08b8514076544f8ca53e2306ba39118c90f1ce6a3edc308a1fff2e75a70`
table digest.
The first local launch stopped before an arm when sandboxed `sysctl` denied
the load-average read; its partial host files and failure receipt are retained
under `local_pilot1`. The second launch produced four matching digest rows,
but sandboxed macOS `/usr/bin/time -l` returned status 1 after each child when
it could not read `kern.clockrate`; its raw rows and failure receipt are under
`local_pilot2`. The runner now records host load through `uptime` and uses
the child process exit status directly.
Record failed and capped arms. Report paired index-build ratios only when all
four arms pass. A larger-base cloud index test and a complete same-point
IC/rho panel remain the promotion gates.
