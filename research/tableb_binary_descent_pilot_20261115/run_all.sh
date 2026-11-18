#!/bin/bash
# pilot sessions, serialised on the shared CPU lock; cwd = repo root
export BENCHREC_SKIP="pilot, not data" RAYON_NUM_THREADS=1
R=research/tableb_binary_descent_pilot_20261115
for n in main21 main24; do rm -rf $R/sessions/$n; flock /tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/cpu.lock target/release/ecbench run --spec $R/specs/$n.json --out $R/sessions/$n --cpus none > $R/sessions/$n.log 2>&1; done
rm -rf $R/sessions/desc24; (ulimit -v 9000000; flock /tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/cpu.lock target/release/ecbench run --spec $R/specs/desc24.json --out $R/sessions/desc24 --cpus none > $R/sessions/desc24.log 2>&1)
