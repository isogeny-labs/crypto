# Full-D18 local positive control: censored at 300 seconds

The frozen source at commit `dc5600493668544691a5884f5dcd474192a4d921`
ran the prespecified `L0_POSITIVE_PROTOCOL.md` diagnostic on the primary
Koblitz `a=0` GF(2^83) full-D18 base. The binary SHA-256 was
`c155f955b6bf3c8aafe003a154ca1804f6053b3e8296c198adfa347b12a5ad16`.
The fresh S3 object GET and custody wrapper are retained here. The wrapper
binds the committed producer manifest by SHA-256; `audit.py` independently
rehashed the full 160,747,529-byte object and checked the wrapper, GET
checksums, binary/source identities, phase records and cap receipt.

The worker re-imported all 5,418,738 labeled points into 32,643 signed-
Frobenius columns in 59.933 seconds. Its pinned four-summand control circuit
then built in 0.422 seconds with 329,726 variables, 11,425,240 total CNF
clauses and 3,901 XOR rows. The cap receipt records
`UNKNOWN_wall_cap` at 300.007 seconds of whole-process time; there was no
solver-returned phase, SAT model, group relation, rank update, or normal
result JSON. `audit.json` reports `PASS_CENSORED_CAP_RECEIPT`, meaning the
inputs and censoring were verified, not that the control succeeded.

The external supervisor used a 310-second cap. `/usr/bin/time -l` recorded
301.40 seconds real, 287.93 user and 8.68 system, then failed to read
`kern.clockrate` in the sandbox, so its exit code 1 and absent peak-RSS
measurement are not used to infer a worker failure mode. The worker's durable
cap file is the direct evidence of its 300-second cutoff. The host was a
shared 48 GiB arm64 Mac with no enforced one-CPU or hard-memory cgroup.

This control is censored before a relation result. It does not estimate
natural-query yield or full IC runtime; `selected_best_total_runtime` remains
`null`. The next useful access test should first make the planted control
finish under a hard-cap Linux worker, or benchmark an equivalently checked
lower-cost encoding. The frozen [Linux protocol](../NATURAL_PROTOCOL.md)
still requires a successful positive control before interpreting its natural
arm.
