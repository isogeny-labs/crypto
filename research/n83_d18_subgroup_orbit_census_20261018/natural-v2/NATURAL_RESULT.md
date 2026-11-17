# N83 m=5 paired-orbit Spot result (bounded)

The frozen `PROTOCOL.md` panel completed on one `c7i.2xlarge` Spot host in `us-west-2b` on 2026-10-19. The ten raw arm directories and their immutable `raw-files.sha256` manifest are in this directory. `audit_m5.py` reports `AUDITED_BOUNDED`; all manifest hashes and the five mutation tests pass. This panel does not select a factor base: `candidate_id: null` and `selected_best_total_runtime: null`.

## Exact inputs and environment

Both bases have 16,627 signed-Frobenius orbit columns and 2,760,082 labeled points. U (`public_x_hash`) and D (`direct_subgroup_x_d18`) are distinct point sets on the exact `icv1-f2m83-tm6151469093347-debefd74` primary subgroup. Their S3 objects are under `s3://crypto-autoresearcher/factor-bases/icv1/etc/`; the exact object URIs, BLAKE3 identities, manifests, and replay requirements are pinned in `natural-v1/PROTOCOL.md`. Fresh S3 HEADs record AES256 storage, and host downloads matched the exact SHA-256 archive hashes: U `2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369` (83,701,045 bytes), D `347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7` (81,844,623 bytes).

The worker was built from source commit `5604fb5ed63fa8dcfcc5e838b517637f0075eaec`; the six compiled source files match that commit and the x86_64 binary SHA-256 is `c8f6849ee98d2552709e232a85f3933c1735326f78f57648f1e96ac84c23b705`. The pinned image is `amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f`. Each arm used one CPU, 4 GiB hard cgroup memory, zero swap, no network, read-only inputs, and a fresh output directory. The natural arms had a 600-second internal watchdog and 610-second outer cap. Each preflight constructed 257,268 variables, 7,409,527 clauses, and 4,980 XOR rows within the frozen bounds. No arm was OOM-killed.

## Complete arm panel

| Order | Arm | Fixture | Outcome | Host wall (s) | Process CPU (s) | Cgroup peak (MiB) | Durable decisions |
|---:|---|---:|---|---:|---:|---:|---:|
| 1 | p-u0 | 0 | PASS_PREFLIGHT | 36 | 35.948 | 2452.1 | — |
| 2 | p-d0 | 0 | PASS_PREFLIGHT | 37 | 36.228 | 2529.6 | — |
| 3 | p-u1 | 1 | PASS_PREFLIGHT | 36 | 36.028 | 2452.3 | — |
| 4 | p-d1 | 1 | PASS_PREFLIGHT | 37 | 36.260 | 2530.1 | — |
| 5 | a-u | 0 | PASS_CONSTRUCTED_CONTROL | 37 | 36.107 | 2452.4 | 0 |
| 6 | a-d | 0 | PASS_CONSTRUCTED_CONTROL | 37 | 36.781 | 2530.5 | 0 |
| 7 | n-u0 | 0 | UNKNOWN_budget | 600 | 599.929 | 2451.9 | 39,316 |
| 8 | n-d0 | 0 | UNKNOWN_budget | 600 | 599.948 | 2530.0 | 26,640 |
| 9 | n-d1 | 1 | UNKNOWN_budget | 601 | 599.959 | 2530.1 | 26,449 |
| 10 | n-u1 | 1 | UNKNOWN_budget | 600 | 599.956 | 2451.3 | 39,098 |

The positive controls pinned five known representatives and proved the selector, original-field group lift, signed-orbit row and rank-insertion path. Both independently replayed on local arm64; altered point and row receipts were rejected. An initial local replay invocation used an outdated executable and failed its CLI check; the corrected build and replay are retained. These constructed controls do not estimate natural yield. Each natural search made observable decisions until the watchdog, with 57 progress samples and a durable `UNKNOWN_budget` receipt. A capped search is neither a zero-yield observation nor an UNSAT certificate. It cannot rank U against D or establish useful rank.

## Charged accounting and limits

The four natural arms consumed 2,401 host-wall seconds at their caps, plus 220 seconds for four preflights and two controls. Every arm also charged roughly 35–36 seconds of factor-base import as included in its wall time. The host launched at `07:46:28Z`, completed its last receipt at `08:47:36Z`, and entered user-initiated termination at `08:49:33Z`: the provisioned interval was about 63 minutes 5 seconds, including setup, package installation, the first build failure from missing `openssl-devel`, the corrected build, transfers and gaps. The observed Spot price API record (`host-setup/spot-price-observed.json`) is a point observation, not an invoice or exact full-window price. Source transfer and build logs are retained. AWS confirms the instance terminated and the temporary key pair and security group were deleted; see `host-setup/cloud-cleanup.json`.

There was no checked natural relation or useful-rank estimate from these four censored searches. Partial-relation graph cost, linear algebra, individual logarithm, scalar verification and matched same-point rho have therefore not been measured in this panel. Full cold and online index-calculus runtime remain unknown. The v5 and v6 enumerators catalog possible frozen configurations; their billions of ordinal cases are design coverage, not executed benchmarks. Existing WDSat subspace-ANF code (`src/cryptanalysis/wdsat_oracle.rs`) and quadratic FES path (`src/cryptanalysis/mq_fes.rs`, m=2 and n<=24) are prior art but do not furnish a validated drop-in solver for this m=5 orbit-domain model. The next empirical decision requires a separately frozen way to obtain checked natural relations and useful rank, followed by complete graph/LA/individual-log and matched-rho accounting; the present cap does not justify simply extrapolating decision counts to relation yield.
