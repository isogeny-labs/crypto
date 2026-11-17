# R90 rank-32 transport: bound-61 prefix and dimension failure

The sole attempt `transport90-01` checked forward column 1 at first
pole-clearing power 61, including a full R50 module product and a corrupted
coordinate rejection. Forward column 2 stopped at `S90*matrix(powerVector90)`:
Singular formed a 31-by-1 matrix because the 32nd vector component was zero.
The run was rejected by the strict driver; no reverse column was reached.

The source was frozen at `156f7cbe80488153878fb2a96f711df18faedcf2`.
One native Singular/FLINT thread, seed 90, 4 GiB virtual memory and a
1200-second cap were specified. The run's 18 raw members passed checksum
protected upload, fresh readback, and hash verification. The diagram depicts
the checked prefix and exact stopping point, with no invented timings.

Next: represent every extracted power vector as an explicitly sized 32-by-1
matrix, freeze a new source, and rerun the complete bidirectional check.
Evidence: `../CM.md`, `../results.json`, `../runs.sha256`, and the immutable
raw archive named there.
