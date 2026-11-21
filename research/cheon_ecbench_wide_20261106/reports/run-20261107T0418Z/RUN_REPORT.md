# Run report: Cheon wide chunk sessions (36 to 56 bits)

Study: [`../../REPORT.md`](../../REPORT.md); protocol and amendments: [`../../PROTOCOL.md`](../../PROTOCOL.md).

- **What ran:** 22 `ecbench run` chunk sessions, one per family and size, each 24 targets × 2 arms = 48 runs, at L0 (operation counts only), `--allow-busy` on the CPUs of the container's cpuset, then `ecbench verify --replay-all`.
- **Hosts:** a shared 4-core cloud container, replaced once at about 05:40Z; binaries per [`../../sessions/BINARY.txt`](../../sessions/BINARY.txt).
- **Wall times are descriptive only** (concurrent lanes, shared host); no wall figure is admitted.

| session | session id | records | verified | replays reproduced | binary | finished (UTC) |
|:--|:--|--:|--:|--:|:--|:--|
| `minus-b36` | `ECBS1h3a395867cb39` | 48 | 48 | 48 | `8c2639b20e7f` | 2026-11-07T04:15:23Z |
| `minus-b38` | `ECBS1h029db5d972b1` | 48 | 48 | 48 | `8c2639b20e7f` | 2026-11-07T04:15:25Z |
| `minus-b40` | `ECBS1h712e678e43a1` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:20:01Z |
| `minus-b42` | `ECBS1ha3c909049801` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:20:12Z |
| `minus-b44` | `ECBS1h313856868ba8` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:20:57Z |
| `minus-b46` | `ECBS1h487b1de96e13` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:22:22Z |
| `minus-b48` | `ECBS1h6da0a0881fb1` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:25:27Z |
| `minus-b50` | `ECBS1h480f99f38f14` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:29:40Z |
| `minus-b52` | `ECBS1h70b5c62472e9` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:40:52Z |
| `minus-b54` | `ECBS1h854c777af927` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T07:06:40Z |
| `minus-b56` | `ECBS1h6c6c02758331` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T07:47:29Z |
| `plus-b36` | `ECBS1hbe229c4ff2e5` | 48 | 48 | 48 | `8c2639b20e7f` | 2026-11-07T04:15:23Z |
| `plus-b38` | `ECBS1h882056aefa19` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:19:58Z |
| `plus-b40` | `ECBS1h7b2c4f8f4377` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:20:02Z |
| `plus-b42` | `ECBS1h66a00f31083b` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:20:32Z |
| `plus-b44` | `ECBS1heac4390f80db` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:21:17Z |
| `plus-b46` | `ECBS1h0ddaee371bcb` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:22:38Z |
| `plus-b48` | `ECBS1h912efb5320bd` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:24:56Z |
| `plus-b50` | `ECBS1h97f80fa86853` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:31:14Z |
| `plus-b52` | `ECBS1hddc7d95a3caa` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T06:42:09Z |
| `plus-b54` | `ECBS1h560fbce3aad9` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T07:04:13Z |
| `plus-b56` | `ECBS1h0848d852a611` | 48 | 48 | 48 | `d4df6333bcf9` | 2026-11-07T07:57:35Z |

Interrupted sessions (kept, never fitted): `minus-20261106` (526 records), `plus-20261106` (525), `minus-20261107` (550), `plus-20261107` (547), `minus-b54.interrupted-1`; each audited with `--allow-interrupted`, 0 problems.
