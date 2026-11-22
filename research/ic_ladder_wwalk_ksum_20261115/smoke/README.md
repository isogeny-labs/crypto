# Local smoke test (toy sizes, non-frozen seeds): pipeline only, never evidence

Command: `TOY=1 SIZES="16 20" pod/run.sh <study dir> <workdir>` with the release `ecbench` built from the working tree after the
k-sum change (sha256 in `BINARY.sha256`). It runs train -> select -> test for two toy sizes (`prime_search` curves, model-optimum
bases, 3-4 targets, seeds 20261115900+), the k = 32 positive-control session (non-frozen seed), `verify --replay-all` on every
session, `analysis/evaluate.mjs`, `analysis/memory.mjs` and `SHA256SUMS`, in about 14 s on 4 shared vCPUs
(`runlog.tsv`). Every run verified; every replay reproduced (`verify.txt` rc=0); the A/A pairs are exactly 1.
The toy S values and the registered thresholds mean nothing at 16-20 bits (the evaluator marks them false); the toy
positive-control session gave wwalk-c2-x06 S(w = 0) = 1.940 on 24 non-frozen targets, a sanity check of the instrument only.
Files: `results.md` (evaluate output), `memory.txt`, `select.txt`, `params-toy/`, `runlog.tsv`, `verify.txt`, `SHA256SUMS`.
