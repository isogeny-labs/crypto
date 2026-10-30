# Complete XOR-cache public card

The card ran from **2026-10-20 01:16:34 to 02:00:10 UTC** and ended
`COMPLETE_BOUNDED`. All sixteen public attempts returned
`UNKNOWN_search_cap` at 10,000 conflicts, with normal exits and no OOM.
Four constructed controls and their separate general-field replays passed.

The native [audit](audit.json) passes **294 files**. All four ABBA cells
have exact matched search counters, identical repeated per-setting visit
counts, and cache-on visits plus skips equal cache-off visits. See the
[report](../../reports/run-xor-cache-natural-20261020/RUN_REPORT.md).
Natural yield and useful rank remain censored, rather than recorded as zero.

Archive SHA-256:
`cf3310c7cb597c144b30a427683ed41d37cf4e2ec2425945d259a1173e776abd`.
Manifest SHA-256:
`55a87e7e0ca6561c48d1ad558ec8b3d8ba385e4cb4c26a6b6188688732eea4ee`.
Audit SHA-256:
`da7f5df53baaeba77091cb7aea843375b019adaf65533a20a416d634ae71f100`.

From this directory, `shasum -a 256 -c archive.sha256`, then
`tar -xJf runs.tar.xz` and `shasum -a 256 -c files.sha256`. From the
repository root, reproduce into a new output:

```sh
cargo run --release --example n83_xor_cache_audit -- \
  research/n83_prebuild_cost_m23_20261019/xor-cache-v2/final \
  cf3310c7cb597c144b30a427683ed41d37cf4e2ec2425945d259a1173e776abd \
  55a87e7e0ca6561c48d1ad558ec8b3d8ba385e4cb4c26a6b6188688732eea4ee \
  /tmp/n83-xor-cache-audit-new.json
```

The audit checks the exact archive/manifest and full raw inventory,
source/input/binary identities, mounted input sources, image/resource
policy, serial container order, control/replay bindings, natural
phase/progress completeness, counter monotonicity, matched endpoints and
visit conservation. This is a receipt/work audit; the separate general-field
replays supply the control arithmetic checks. No natural relation object
was returned in these capped searches.

`collect-xor-cache-final.sh` made a sorted GNU tar archive with fixed
2026-10-20 mtime, uid/gid zero and normalized modes, compressed with xz
-6 -T1. Earlier control and failed-launch archives remain unchanged.
All extracted raw files are ignored locally and retained in the archive.
