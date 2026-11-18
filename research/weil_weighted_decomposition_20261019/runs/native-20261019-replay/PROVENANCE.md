# Deterministic replay provenance

Runtime source fc13c52e82202b44a13f9e32dca2fac2401e320e, primary protocol
6095144c. The source snapshot is ../../sources/primary.rs. The replay
started after the primary finished, using the same release binary, inputs,
seeds and budgets; only --out changed. config.json records exact argv.
The process exited 0 and stdout.log retains its case progress output.

records.jsonl matches ../native-20261019-primary/records.jsonl byte for
byte; ../../validation/replay.txt gives the hashes. Native PARI 2.17.2
also rechecked the fixtures from this replay, with exit status 0:
pari_checks.jsonl and pari.log. These are repeated fixture checks on the
same host, not additional independent curves or independent-host replay.

Analysis uses source 040b8e1a. The producer is the same AMD EPYC 9V74 KVM
container, four-CPU cgroup quota and 16 GiB, L0 concurrent. Host observations
were captured after the run in the same container. No wall time is admitted.
The primary/replay binary hash was not captured before rebuilding; the
committed source, Cargo.lock and source revision are retained provenance.
