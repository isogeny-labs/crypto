# Primary K4096 orbit model and one bounded search

The frozen `a=0`, public-X-hash, seed-`2026100801` K4096 base was fetched
afresh from [its content-addressed S3 object](../k4096-primary-20261018/RESULT.md).
The 20,621,466 compressed bytes matched SHA-256
`ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad`
and BLAKE3
`a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779`.
HEAD and GET ETags, length, BLAKE3 metadata and AES256 encryption agreed. The
base contains 4,096 signed-Frobenius columns and 679,936 points. Its exact
location is `s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/objects/a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779.jsonl.gz`.
No S3 object was overwritten in this gate.

The first capacity launch used source commit `63c394a1be142f0a011d8b19e55243c663951316`
and exited after **16.480 seconds** with `PRODUCER_FAILURE_worker_exit`.
The exact public corpus was available in the fixtures directory, but the
capacity worker also required it at the panel root. Its missing-file stderr,
raw logs and absent `worker.json` are retained. The fail-closed preflight
then reproduced that missing-corpus rejection. A byte-identical copy of the
same public corpus was placed at the panel root; no base, binary or compiled
source bytes changed. [The amendment](AMENDMENT.md) froze one corrected
attempt under source commit `4ad69d5efb2fc6aec6082d30443ba18ba741d34f`.

The corrected five-summand `frobenius_orbit_phase` capacity call passed model
construction under one CPU, 4 GiB, zero swap, no network, a read-only root
and a 900-second cap. The **measured installed model** had 131,958 SAT
variables, 1,773,110 required orbit-domain clauses, 2,021,197 total clauses,
4,980 XOR rows, 82,668 AND gates and four S3 nodes. Its outer wall was
**19.015 seconds**, including a 16.441-second base import; construction
itself took 0.607 seconds. The cgroup peak was 801,611,776 bytes. These
counts match the frozen source prediction, but model construction alone says
nothing about natural solver yield.

The single conditional public fixture-zero search used the same base,
five summands and domain encoding, with one trial, at most 32 models and a
5,000-conflict cap. It returned **`UNKNOWN_solver_cap`**, with one SAT call,
exactly 5,000 conflicts, zero models, zero relations, zero independent rows
and rank zero **within that cap**. Its outer wall was **373.290 seconds** and
cgroup peak 1,065,365,504 bytes. The original public target and group
receipt independently replayed. A zero-row linear-algebra invocation after
the cap is recorded; it does not establish the base's natural rank or yield.

The independent [audit](evidence/audit.json) verified the fresh S3 bytes,
Git source blobs, binary, live Docker limits, all raw worker and outer
receipts, public group replay, and cleanup. A separate retained evidence copy
passed, while a coherently rehashed false model-clause count was rejected.
The three launched outer walls sum to **408.785 seconds**. For conservative
planning, the amended gate charges the full launched caps plus reserve:
900 + 900 + 1,200 + 300 = **3,300 seconds**. This is a new-stage envelope,
separate from K4096 construction and all earlier sweep work; it is not an
observed total IC runtime. The binary ran under Linux x86 emulation on an
arm64 host, so these stage walls are not native throughput comparisons.

Natural relation yield beyond the cap, useful rank growth, factor-base
column logs, individual-log extraction, verification and matched same-point
rho timing remain unmeasured. Keep `total_index_calculus_runtime_ms` and
`selected_best_total_runtime` null. This one capped call cannot select the
best factor base.
