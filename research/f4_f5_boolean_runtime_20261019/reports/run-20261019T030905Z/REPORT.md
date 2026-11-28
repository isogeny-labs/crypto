# Incremental serial row clearing

Run 20261019-030905-boolean-native-serial-cl-25ad47. Same-binary KIC_GF2_SERIAL_CLEAR=0/1; single-write/vector tables stay enabled in both arms. Five baseline A/A and five A/B pairs for each F4/F5 matrix at n17 seed230 and n23 seeds230/231/232. Four-worker seed230 controls keep the original parallel path. Exact raw/canonical rows, rank and counted XOR/profile work agree.

Host: Intel(R) Xeon(R) Platinum 8488C; rustc 1.99.1 (58cf8f9c3 2026-10-13); c7i.2xlarge on-demand, reserved physical cores/SMT siblings: one=3,7
four=3,7,2,6. The first condition queries the current Rayon pool. The parallel iterator remains selected with four workers; that does not establish unchanged cold runtime.

All eight one-worker cells qualify faster. Primary n23 paired ratios are1.0770 F4 and1.0652 F5. The n17 F4 four-worker control qualifies slower at0.8436, CI[0.8284,0.8504]; the other three multiworker cells are neutral. This result blocks a regression-free claim for the unrestricted guard. A subsequent protocol narrows activation to explicitly configured one-worker processes before querying the registry; startup/scheduling is a hypothesis, not a proved diagnosis.

Admission, first eligible of three attempts, five-pair exact bootstrap and max(2%, twice median absolute A/A noise) are unchanged. Rejected attempts and every slower/neutral/inconclusive result remain in serial-clear-summary.json. Negative-control movements are not interpreted as algorithm gains.

This is an incremental matrix-call experiment. It does not measure complete recursive roots, decomposition, or a whole IC/rho pipeline. The eighteen-arm solver table retains its prior source and serial-clear-off flags; no complete-solver benefit is inferred from this kernel result. Source: SERIAL_CLEAR_PROTOCOL.md, run-serial-clear.sh, analyze-serial-clear.rs and runs/serial-clear/ in the raw archive. Both kernel-test switch states pass on the native host before timing.

|Arm|n|Seed|Workers|Before ms|After ms|Ratio|Decision|
|---|---|---|---|---|---|---|---|
|matrix-f4|17|230|1|7.4833|7.1817|1.0370|faster|
|matrix-f4|23|230|1|61.3743|57.7376|1.0770|faster|
|matrix-f4|23|231|1|62.0068|57.9712|1.0693|faster|
|matrix-f4|23|232|1|61.4660|57.5380|1.0652|faster|
|matrix-f5|17|230|1|8.0175|7.7868|1.0280|faster|
|matrix-f5|23|230|1|65.6287|61.2560|1.0652|faster|
|matrix-f5|23|231|1|65.3784|61.3875|1.0729|faster|
|matrix-f5|23|232|1|65.1584|61.3274|1.0620|faster|
|matrix-f4|17|230|4|8.3357|9.8786|0.8436|slower|
|matrix-f4|23|230|4|65.1697|64.7849|1.0120|neutral|
|matrix-f5|17|230|4|9.2373|8.4551|1.0451|neutral|
|matrix-f5|23|230|4|64.3608|63.6913|1.0081|neutral|
