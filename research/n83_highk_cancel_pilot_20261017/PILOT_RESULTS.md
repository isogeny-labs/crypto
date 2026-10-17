# High-K cancellation implementation result

On the previously exposed N83 point, the K2816 root11 factor base built
467,456 subgroup-usable points, reached rank 2,816, and independently
replayed the four-point target relation and scalar under two-worker
cooperative cancellation. The separate K3072 process reached its declared
900-second cap during construction. These results support dispatching the
already sealed fresh-point Tier 1 factorial comparison; they do not select
a factor base.

| Arm | Local producer status | Independent status | Base points | Rank | Cold process | Target online | Probes |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| K2816 cancel | `PASS_PRODUCER` | `PASS_INDEPENDENT_REPLAY` | 467,456 | 2,816 | 421.283 s | 13.529209 ms | 19,510 |
| K3072 cancel | `TIMEOUT` | `TIMEOUT` | — | — | >900 s cap | — | — |

The K2816 online interval comprises target PDP 13.147207 ms, relation check
0.023084 ms, descent 0.000791 ms, and recovery check 0.357834 ms, with zero
target-query time in this implementation. The source SHA-256 is
`8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829`.
The K2816 base hash is
`9208460bf63be634f0aee6091abaaed855e05004fc2894e28a9c1fd394c35ec4`;
its ordered base dump SHA-256 is
`e3b9a30d57ee61a31ae5b80d585ba6aee55475654062ad8dda17693a38adc5cb`.
The root index contains 658,165,181 entries and allocates 5,368,709,120
bytes. The 900-second K3072 process exited with status `-9` after 901.228
wall seconds; its zero-length producer outputs are preserved.

The local host is macOS arm64. Its inherited process sampler cannot read
`/proc/<pid>/status` and mis-scales `wait4` RSS; resource status is
`UNVERIFIED_MAC_RSS`. The times above are ordinary-host implementation
diagnostics. The 24 fresh public points, exact four candidate identities,
resource envelope, and selector were frozen before this result in
[FRESH_PROTOCOL.md](FRESH_PROTOCOL.md). The cloud comparison uses cgroup
receipts and same-point rho pairs on four independent Tier 1 hosts.

The raw local record is stored under
`/Volumes/SSD990/crypto/artifacts/n83_highk_cancel_pilot_20261017_raw`.
SHA-256: `analysis.json`
`5689abf7fd0bb3183045a70e3096aab2957d63d9b97edc18a95adb00f3554243`,
`source.json`
`94436658ae194273e058867ae1afcc5294970dcbe92a9062a80263c9a2c42a97`,
K2816 `usage.json`
`6128fe5c97a05c9a7144bef3301fc9f482bca7f72cff7b622617b5fe7f751869`,
and K3072 `usage.json`
`b39bef5bd8ada6dd68b9ec917ee38070fc8405205bc6383adbfcbc00b22cd395`.
