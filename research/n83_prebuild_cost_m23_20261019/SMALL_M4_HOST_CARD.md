# Frozen SSH host card: small-m4-v1

Frozen before controls or public queries on 2026-10-19 UTC. Execute
`run-small-m4-card.sh /home/ec2-user/n83-m23-20261019/small-m4-v1` only after
the source, executable, script and input manifest below verifies.

- Source commit: `611aad60e4d9952d6239a3382a77184da3640eeb`.
- Source bundle SHA-256: `8610cf8b0127622c612c787da04583b5befaeadfb06c7b04772309db55455b81`.
- Linux worker SHA-256: `3048550afc4ad1bdad1e390f74802112dfc6fadf96048cbff823f8f65364b0c3`.
- Linux general-field replayer SHA-256: `359968c67994521198c4b8b69f599b0f937bf080c2f0c26046346e021fc23b22`.
- Worker source SHA-256: `fcb7898efa4f72999fdf9301720fd7056d2ae9beb44800e6a31809ecc6318f5a`.
- M4 module SHA-256: `d8c2fc23d93f32a78ed7cf7c98994266721308f76d74546e78d089ba1e4a2e7f`.
- Replayer source SHA-256: `d261edc668b8f7fdb3ebfb16ae42d724b3ab9e16b6cc787aa1d6f94670934dc4`.
- Card SHA-256: `70827381430f9ef514476d997091311a5c09132cbbf4136ff31674dfe76b7442`.
- Docker wrapper SHA-256: `2bd6992965af930c5038563dba2cbd6e5a18e0c426a20f00665779e31ba3006f`.

Both binaries were built with `cargo build --release --example
n83_prebuild_cost_m23 --example n83_small_m4_replay --jobs 2` in a separate
remote target directory. The first attempt failed because inherited
sparse-checkout rules excluded the committed m4 module. After
materializing the study directory, the same source revision built
successfully; preserve both build logs.

Use the existing authorized m8i.8xlarge Spot runner
`i-01f85a6c905181498`, CPU 31, one CPU quota, 4 GiB memory, zero swap,
no network, read-only inputs and root filesystem. The pinned image is
`amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f`.
CPU 31 shares a physical core with CPU 15. Other research jobs and a data
migration were observed on this host during preparation; it earns L0.
Record current containers, CPU topology and uptime at card launch.
Stop no other workload. No new cloud capacity is authorized by this card.

The preparation script retains the pinned public corpus, manifest and
archive hashes from `SMALL_BASE_PROTOCOL.md`. The validation sidecar is
not mounted. Two preflights and two positive controls with separate
general-field replay precede K1,D1,D2,K2. Caps and missing files retain
their explicit status. The script must stop on failed controls, invalid
receipts, missing cap/progress evidence or interruption. Each natural
arm has its own 1,790-second inner and 1,800-second outer cap.

Maximum scheduled solver budget is four natural arms plus two controls,
each at most 1,800 seconds, with separately capped preflights/replays.
No target beyond public corpus index 0 is scheduled by this card. The
remaining corpus, natural useful rank, full costs and matched rho remain
open in the original requirement ledger.
