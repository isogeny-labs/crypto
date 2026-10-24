# N83 K1107 disjoint-point root-layout diagnostic: local root11 lead

On one new public N83 point, the four fresh-process ABBA runs completed with
identical 183,762-point factor bases and independently checked scalar results.
The local macOS arm64 root11 median was 52.940 seconds; root22 was 64.435
seconds, 11.494 seconds slower. The difference arose mainly in the rank phase:
root11 took 33.598/33.512 seconds and root22 took 45.377/45.006 seconds.
Pair spreads were 0.242% and 1.984% respectively. Same-point rho completed in
2.720 seconds. These are exploratory process times on this host, with one point
and two repetitions per layout. `selected_best_total_runtime` remains null;
the five-base GitHub confirmation and a controlled broader holdout are separate.

| Arm | Whole child (s) | Setup (s) | Rank (s) | Target (s) |
| --- | ---: | ---: | ---: | ---: |
| K1107/root11 A | 52.876 | 18.866 | 33.598 | 0.119 |
| K1107/root22 A | 65.074 | 19.491 | 45.377 | 0.115 |
| K1107/root22 B | 63.795 | 18.558 | 45.006 | 0.113 |
| K1107/root11 B | 53.004 | 19.096 | 33.512 | 0.118 |

The fixture was generated before timing from the domain in `generate.py` and
was checked against all 16 frozen finalist points. `audit.py` rehashed every
recorded raw file, checked the source commit and binary/input receipt,
recomputed the scalar-to-point multiplication, and checked all four IC outputs
and rho against it. A copy with one changed byte in the root22 B target result
was rejected at the file-hash gate. Its machine-readable result is `audit.json`. The factor
base dump SHA-256 was
`08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc`
in all four children. A fresh download of the previously custodied K1107
object at
`s3://crypto-autoresearcher/factor-bases/icv1/etc/n83_current_source_finalists_20261019/a2/objects/ccdbf619478990570bfba982c7d1bdfe361ed4530c32407ece4a7ad1be5c42a5.jsonl.gz`
had gzip SHA-256
`ccdbf619478990570bfba982c7d1bdfe361ed4530c32407ece4a7ad1be5c42a5`
and decompressed SHA-256 equal to every new dump. The exact factor base is
therefore already stored in the requested S3 prefix.

The raw 57 MiB run directory is currently local at
`/private/tmp/n83-disjoint-root-layout-raw-20261024`; its tar.gz archive is
`/private/tmp/n83-disjoint-root-layout-raw-20261024.tar.gz` with SHA-256
`d67727a84a87ce966e1b839ae476d3b2dae2462dc10d4c9e5e30c20561bad139`.
Automatic approval review rejected exporting this broader raw archive to S3;
its reason was that permission to store factor bases did not specifically
authorize the full diagnostic bundle. It also rejected the prepared Modal run
because that would have sent project-derived binaries and inputs to a remote
volume. No Modal benchmark ran. Raw replay therefore requires this local
archive; the committed audit is a digest-bound summary, not a substitute for
the raw files.

Validation before publication: `cargo test --release --lib --locked` passed
4,162 tests with 117 ignored when local loopback sockets were permitted;
the two relevant release examples passed 21 and 4 tests. The four frozen
finalist Python suites and the boundary-autolab suite passed (52 boundary
tests). The initial sandboxed library run had nine unrelated Cairn/network
socket-permission failures; the rerun passed. The first test build exhausted
the system volume; moving Cargo output and temporary files to the workspace
SSD resolved that host issue before the passing run.
