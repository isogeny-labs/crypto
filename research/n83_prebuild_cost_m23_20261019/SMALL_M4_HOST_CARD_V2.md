# Pre-execution launch-card hardening

This addendum is committed before any `small-m4-v1` execution. It replaces
only the shell card and preparation script from `SMALL_M4_HOST_CARD.md`.
The original card remains retained and was not executed. Binaries, source
revision, bases, public point, solver limits, host and query order stay
exactly as frozen there.

Use `run-small-m4-card-v2.sh` (SHA-256
`b6a5dc13c44b44bafab85ec06eb4f83d114dd6e6fcf529a87989a4a124c70fb4`),
installed under the run root as `run-small-m4-card.sh`, and
`prepare-small-m4-v2.sh`. Before advancing from any natural arm, this card
requires a nonempty, parseable progress stream bound to that base, natural
mode and target index, with numeric decision, conflict and XOR-time
counters. A cap receipt alone cannot pass. It also rejects a pre-existing
`runs/` before writing the input-check log. These checks enforce the
already frozen instrumentation and immutable-output requirements.

The SSH run root remains
`/home/ec2-user/n83-m23-20261019/small-m4-v1`. Transfer these two shell files,
run the v2 preparation script once, then launch the installed card once.
The native worker revision remains
`611aad60e4d9952d6239a3382a77184da3640eeb`.
