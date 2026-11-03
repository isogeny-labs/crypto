# Native replay of the relocated quotient-calibration preflight

**Run: `run-20261025T1845Z`. Destination: `aburan28/crypto`. Source-only
replay; no target calibration, inverse fiber or transfer was evaluated.**

The user requested that the work be redone and pushed to `aburan28/crypto`.
The retained design came from `aburan28/ghs-trapdoor` commit
`b314cf87940cfb7199cd98261ba6d3910f275c90` (PR #10). The native source and
protocol were frozen locally in commit
`a0cd24e1c7f4381e9ce59cfce760eb5eb8cea3c5` before this command-line replay.

## Command, host and scope

From the crypto repository root:

```sh
cargo run --release --locked \
  --manifest-path research/quotient_calibration_20261025/native/Cargo.toml \
  -- --output research/quotient_calibration_20261025/reports/run-20261025T1845Z/native-preflight.json
```

Exit status: 0. Host: Linux 6.18.55 x86_64, five available logical CPUs;
Rust 1.99.1 (`58cf8f9c3`), Cargo 1.99.1 (`5f94df478`). Execution used
ordinary shared-worker conditions, with no isolation admission. No algorithm
timing, memory-cost comparison or speedup is reported. Cargo's build/test
durations in logs are tooling diagnostics, not performance measurements.

The exact case is `y²=x³+248x+42` over
`F7[z]/(z³+z²+1)`, integer encoding `a0+7a1+49a2`, generator `(0,176)`,
order 373, kernel 153 with the frozen basis and seed `(0,0,0)`.
All coefficients, source coordinates and chart conventions remain in the
manifest and protocol; no new curve registry alias or cross-model identity
is inferred from the field size. The native code checks the degree-18
working modulus by the Rabin criterion over the prime field before use.

## Observations

| Check | Recorded outcome |
| --- | --- |
| Historical manifest bytes | SHA-256 `cfca312b80fe7a35da6cd8d31ee44b195885aad5fbd6b1de07b1fc9dfe33dc65`; exact copy. |
| Pilot and sections | 32 source points, 64 section columns per point; exact sample rank 8. |
| Full normalized section matrix | SHA-256 `52f1938e51d9baca3e9d1753a90516611468ed1472c08203decbdd0a41ab87fe`; matches historical output. |
| Selected coordinates | Indices 0–7; rank eight on the pilot. |
| Projective frame | Scalars 1–8 and 11; all nine eight-vector minors have rank eight. All frozen vectors and weights match. |
| Source arithmetic | Source order 373 checked; normal 2-Weil cocycles: 16 checks, normal 3-Weil cocycles: 81 checks. |
| Challenges | All 32 coordinate pairs and their P+G partners reproduced exactly; 99 stream counters consumed. No discrete-log labels or target images computed. |
| Controls | Altered reference hash rejected before evaluation; dependent frame rejected; all nonzero F343 inverses and source generator order checked. |
| Historical dependency custody | Four full upstream input reports fetched and all four SHA-256 digests matched. These reports are not evaluator inputs. |

The source receipt is [native-preflight.json](native-preflight.json).
[input-hashes.sha256](input-hashes.sha256) pins the four upstream reports;
their immutable URLs and sizes are in [PROVENANCE.json](../../PROVENANCE.json).
`bash research/quotient_calibration_20261025/verify-upstream.sh NEW_DIRECTORY`
repeats download and byte verification using shell, curl and sha256sum.

The [diagram](flow.svg), with [editable source](flow.dot), distinguishes
executed source checks from unexecuted target gates. The
[PDF](REPORT.pdf) embeds the same diagram and observations.

Raw validation-log publication is blocked, separately from the native
source replay: the 67,540-byte archive exceeds the repository data-file
threshold and automatic approval review rejected its S3 upload. Its exact
hash and contents are in [ARCHIVE_PUBLICATION.json](ARCHIVE_PUBLICATION.json).
It remains local and unpublished; no durable raw-log publication is claimed.

## Limits, failed attempts and remaining work

This is a native cross-implementation source replay on one worker. It is
not an independent-host map verification, a target calibration pass, a
global section-identity certificate or a faster DLP solver. The target's
frame has not been evaluated. Fitting nine frame pairs cannot validate a
map; 32/32 frozen target matches and all divisor/sign/group/control gates
remain required. Profile A still inherits the certified anchor. Profile B
still requires normalized target Weil functions and a global identity.

The original Python/FLINT/Sage code is historical provenance; no Python
research code was executed here. The replay fixes the certified seed and
kernel rather than rerunning their search. It matches a finite source
sample and does not prove a global section identity.

Initial root test attempts could not resolve the path dependency
`isogeny_algos` because it was outside the sparse profile. That tracked
directory was materialized and the unchanged root test command restarted.
The bounds test initially also lacked a tracked historical session outside
the sparse profile; materializing its study made the focused rerun pass.
The unchanged ecbench weighted-comparison test separately fails its
zero-weight legacy-ID assertion. This is tracked in
[the named problem](../../../../problems/ecbench-weighted-zero-legacy-id-contract.md)
and [VALIDATION.md](VALIDATION.md); no unrelated library code is changed.
The first strict-Clippy attempt found four style diagnostics; those were
fixed and strict Clippy passed. The first formatting command found rustfmt
uninstalled; rustfmt and Clippy were installed in the workspace toolchain.
No failed arithmetic check was suppressed or challenge replaced.

Conductor's CLI and `.conductor/WORKFLOW.md` are unavailable in this fresh
environment; the pre-edit check returned `command not found`. The
repository's `docs/CONDUCTOR.md` documents this fail-open installation case.
Changes use an isolated feature branch and study directory; no shared
Conductor reservation is claimed. A focused CI workflow is the only added
path outside the study, apart from the required named problem and index row
recording the baseline validation blocker.

Canonical graphs checked: `docs/index-calculus-scoreboard.html`,
`docs/ic/progress-timeline.json`, `docs/ic/LEADERBOARD.md` and
`docs/ic-leaderboard.html`. They remain unchanged: this preflight adds no
cost, IC/rho comparison, new speed result or changed frontier claim. Fixed
fixture cardinalities are not daily progress measurements. No benchmark
index or performance exporter entry is warranted by this source replay.

The next target action and every original acceptance criterion remain in
[PROTOCOL.md](../../PROTOCOL.md). The relocated protocol/native replay is the
delivered scope; target execution remains future work, not a hidden
completion claim.
