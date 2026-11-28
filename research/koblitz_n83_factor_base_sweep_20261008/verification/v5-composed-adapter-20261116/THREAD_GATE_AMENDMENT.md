# V5 one-thread admission amendment

Filed after the first binding receipt and before a second binding check. The
first receipt at source `cdc078c51511d97a1320d11a9be210cd816064ee`
remains unchanged. A code audit found that the library can use Rayon outside
the serial one-query relation loop. The V5 cell declares one thread, so the
new `primary-v5-retained-chain-cold` route rejects an absent or non-one
`RAYON_NUM_THREADS` before writing its binding sidecar or starting the worker.
The pure gate is checked by the exporter example test for `1`, absent, and
`4`. The later cold-run protocol must also pin host CPU/cgroup limits and
confirm the actual worker thread count.

Run one new read-only binding check with the revised source and
`RAYON_NUM_THREADS=1`, using [`run-binding-attempt-2.sh`](run-binding-attempt-2.sh).
Keep its raw outputs in `attempt-2/`; do not replace the first receipt. The
expected binding values and stop conditions are those of [`PROTOCOL.md`](PROTOCOL.md).
The second check still makes no S3 request and does no solver work. It cannot
verify the future cgroup, relation loop, rank, or total runtime.
