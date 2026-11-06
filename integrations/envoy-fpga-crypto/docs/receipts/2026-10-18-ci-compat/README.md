# Ubuntu RTL tool compatibility receipt

Run batch: `envoy-fpga-20261018-ci-compat`. Parent revision:
`e6369b99328ab2b6ee301679c133691000c1f31a`, on
[PR #2122](https://github.com/aburan28/crypto/pull/2122).

The PR's push and pull-request `host-and-rtl` jobs passed 16/16 core groups,
then failed source RTL lint with Ubuntu 24.04's Verilator 5.020. The original
local run used Verilator 5.032, so it had not established compatibility with
the CI toolchain. Both public job logs are retained:
[117581611720](https://github.com/aburan28/crypto/actions/runs/39095715556/job/117581611720)
and
[117581612715](https://github.com/aburan28/crypto/actions/runs/39095715838/job/117581612715).

The unchanged parent reproduces seven `BLKLOOPINIT` errors and four
`WIDTHEXPAND` warnings locally with the Ubuntu Verilator package. Raising the
static loop-unrolling limit alone removes the errors and retains the four
fatal warnings. The final fix sets `--unroll-count 256` for the 200-byte
SHAKE copy/erase loops and explicitly zero-extends both NTT multiplier
operands. It preserves source `-Wall`, assertions, randomized initialization,
the test cases, and the synthesis structural checks. No warning is suppressed.
The CI job also prints its tool versions before validation.

| Local verification of the corrected source | Result |
|---|---|
| `make test lint-rtl test-rtl test-rtl-synth`, Verilator 5.020 / Yosys 0.33 / sv2v 0.0.13 | Exit 0: 16/16 core groups, 15/15 source RTL groups, same 15/15 groups after synthesis |
| Verilator 5.032 source lint with the corrected Makefile | Exit 0, fatal `-Wall` and assertions enabled |
| Scoped workflow, actionlint 1.7.7 | Exit 0 |
| Required `cargo test --release --lib --locked` | Exit 101 before tests: the same 26 errors in unchanged root sources |

The Ubuntu RTL packages were extracted into a user-owned directory on a
Debian 13 host. GCC was 14.2.0 and OpenSSL was 3.5.7; this is not the GitHub
Ubuntu runner environment. The new commit's CI result must be checked
separately. The source tests use seed `0x4650433252544c` and unchanged
independent OpenSSL/polynomial oracles. The synthesis replay repeats the
same cases; it is not a formal equivalence proof or 30 independent specifications.
No new optional BoringSSL/liboqs/live-Envoy suite or hardware deployment was run.

The public [summary manifest](manifest.json) records aggregate outcomes,
tool-package and current project-file hashes, and hashes identifying the
new raw archive and detailed run manifest. Those two files remain local
pending explicit owner approval for this new upload; **the new archive is
not included in this commit**. Earlier approval covered the already uploaded
Oct 18 validation bundle, which remains unchanged and available.

The frozen detailed run manifest records commands and hashes of all 22 raw
records in `runs.tar.xz`. The local archive includes both failing public
module jobs, the unrelated failing workflow/name job logs, local failed
attempts, successful validation, the synthesized netlist, tool versions,
and the root Rust failure.
Local package-relocation failures and an actionlint invocation from the wrong
directory are retained and identified as setup errors. There were no timed
cryptographic performance measurements.

Verify the public summary. After the new raw archive is approved and obtained,
verify its checksum and extract it:

```sh
sha256sum -c manifest.json.sha256
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

Reproduce on Ubuntu 24.04 with Verilator 5.020, Yosys 0.33, sv2v 0.0.13,
a C++17 compiler, Make and OpenSSL development files:

```sh
make test lint-rtl test-rtl test-rtl-synth
```

The earlier Oct 17/18 receipts, raw archive and frozen run manifest are
unchanged. This receipt describes a subsequent compatibility fix. Existing
workflow duplicate-key failures and curve-registry/name failures are outside
this PR's changed paths and were left unchanged. No failed, queued or neutral
check has been treated as a passing merge gate. See the
[new report and PDF](../../reports/2026-10-18-ci-compat/README.md).

The unrelated failing jobs remain readable at their original public links:
[workflow parsing](https://github.com/aburan28/crypto/actions/runs/39095715540/job/117581611811)
and [curve names](https://github.com/aburan28/crypto/actions/runs/39095715534/job/117581611882).

Complete crypto controllers, entropy/key custody, FPC1/F2 integration, an AFI,
physical validation and measured acceleration remain unfinished. The original
[implementation report](../../reports/2026-10-18/README.md) remains a historical
record of the earlier source and validation batch.
