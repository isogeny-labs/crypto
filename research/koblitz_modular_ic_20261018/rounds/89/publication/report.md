# R89 bidirectional rank-32 transport: rejected pole bound

The frozen `transport89-01` attempt reached column 1 in each direction and
failed to find a polynomial RR basis coordinate vector through pole power
51. The native log contains two Singular errors. The strict driver returned
failure even though the process exited zero and subsequently printed a
misleading completion marker. Accepted column images: 0 of 64. The diagram
records the actual stopping points; no timings or product counts are inferred.

The input was the R50 rank-32 PID basis, its degree-37 common denominator,
and the R85-R88 rational diamond maps. The source revision was
`a624ea54235799a37e5c2488679d3e1ea7323b5e`. The one-thread native
attempt used seed 89, a 4 GiB virtual-memory cap and a 1200-second cap.
It was guarded by donor hashes and source-byte pins. The saved source and
native log are in the raw archive described in `../CM.md`.

The new bound calculation in `../BOUND-ERRATUM.md` gives 61 as a uniform
upper bound from R27 cusp valuations and the actual R50 relative degree.
The next experiment must freeze the repaired bound and make the first
native error stop the script before any completion marker.

Sources: `../PROTOCOL.md`, `../runs.sha256`, `../results.json`, R27
`CM.md`, R50 `NEXT_SOURCE_NORMALIZATION.md`, R85-R88 `CM.md`.
