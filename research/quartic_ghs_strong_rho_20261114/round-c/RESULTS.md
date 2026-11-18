# Round C partial result: exact reciprocal reduction and width sensitivity

Status: reciprocal-reduction implementation and producer-side
correctness/count controls complete; batch-width retune protocol frozen and
one deterministic sensitivity cell complete; profile, isolated throughput,
full-grid retune, and frontier assessment blocked. These results have not been
independently admitted `[pcc:unverified]`.

## Implementation status

`FastF3` now precomputes `floor(2^64/q)` once and reduces each canonical
`a*b < q^2 < 2^62` with a high product and at most one subtraction. The hot
`fm` path no longer executes a run-time remainder by `q`, and it still bumps
`fq_mul` once per call `[pcc:unverified]`.

The candidate source SHA-256 is
`77ec7654292f107b041adc34d2b7e016d453311c5c3c602c6fe64423d45e32fc`.
It is layered on the Round B five-product cubic square; no walk, partition,
cycle, distinguished-point, or accounting rule changed.

## Correctness and count result

The standalone self-test made 5,791,514 reducer/reference comparisons:
5,691,264 exhaustive products for every modulus 2 through 257, 250 boundary
operand products, and 100,000 deterministic random products over the ten
frozen boundary moduli. Every reciprocal residue equalled an independent
widened `u128` remainder `[pcc:unverified]`.

The same binary also passed 200 deterministic cubic-field comparisons,
batch inversion, ordinary and batched curve addition, scalar multiplication,
and planted-log recovery at widths 1, 4, 8, 16, and 32
`[pcc:unverified]`. Exact output is in [`selftest.txt`](selftest.txt).

Source-bound baseline and candidate binaries then ran the public synthetic
`q=223`, seed 1, target 0, `ic-repeat` cell at widths 4, 8, and 16. After
removing only calibration, `wall_s`, and the deliberately different source
binding, the normalized instance/rho files are byte-identical: both have
SHA-256
`2f28d058b934027b2d4a849d51e4c8be1a919957ba50d962dc57fa15f9f9af0e`
`[pcc:unverified]`. Therefore each same-width pair has the same recovered
scalar, steps, distinguished points, restarts, look-ahead behavior, cycles,
group counts, every field count, and every native count in this cell. The raw
and normalized records remain beside this report.

## Prospective width sensitivity

The following values describe the candidate's single deterministic `q=223`
cell. They are count/correctness sensitivity, not a throughput ranking or a
new primary-width selection `[pcc:unverified]`.

| arm | steps | DPs | look-ahead | cycles | cycle adds | total `fq_mul` | total `fq_inv` | batch invocations | group adds | band ops |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| strong-b4 | 2,481 | 155 | 92 | 1 | 5 | 175,852 | 2,867 | 703 | 4,814 | 9,707 |
| strong-b8 | 1,487 | 76 | 53 | 2 | 10 | 149,558 | 2,619 | 231 | 4,011 | 5,049 |
| strong-b16 | 1,008 | 57 | 34 | 0 | 0 | 151,572 | 2,954 | 89 | 4,005 | 3,189 |

All three arms recovered scalar 1,765,657 with zero restarts. Differences
between widths arise from their distinct deterministic batching schedules;
this one cell does not estimate expected work or throughput. Width 8 remains
the frozen Round A primary.

## Performance and retune status

Checkpoint push and gate both exited 1 because the runtime has no
`ecc-challenges-new` AWS profile, upload token, or checkpoint token file. The
first invocation also encountered the full overlay's `/tmp`; it was repeated
with `TMPDIR` on `/dev/shm`, exposing the clean transport refusal retained in
[`checkpoint-gate.txt`](checkpoint-gate.txt).

Per the frozen stop rule, no profile, A/A control, interleaved A/B timing,
standard-isolation run, or full Round A grid was started. Raw development
`wall_s` and one-triple calibration fields are retained for audit but excluded
from the normalized comparison and from every conclusion. Reciprocal-reduction
throughput, the optimal batch width, and any frontier or leaderboard change
remain unmeasured `[pcc:unverified]`.

## Provenance and test status

- Baseline revision:
  `ad2a014c0022cf000845014499a9649b6c2cef2d`; baseline source SHA-256
  `0bb8da07393873aa2b935403b86f5b7a0c24a52fff2d598a145a9fcca103229c`.
- Protocol freeze commit:
  `a62240111e15ce310e37f3f2d754493c3d642062`.
- Shared `main.rs` SHA-256:
  `b2244ffece12855622a88f9a2af4926b1fac6113b9519fdf11e2bc7449fa27c8`.
- Baseline binary SHA-256:
  `636df5139d45efc3b2d9a284a49cdd24540befe05ef75bc19943b5db36539a89`.
- Candidate binary SHA-256:
  `4327d6ebac858b43870a75aee742b2fa4c5167caf00df2fc7fe0e7b53218b228`.
- Compiler: `rustc 1.100.0 (0ef1bc386 2026-11-09)`, LLVM 23.1.5,
  `x86_64-unknown-linux-gnu`; host details are in [`metadata.txt`](metadata.txt).

The edited standalone source compiled with `rustc --edition 2021 -O`, and its
source-bound self-test passed. The required `cargo test --release --lib` first
stopped at an omitted sparse path dependency. After materializing the declared
path dependencies, a 300-second bounded retry timed out while compiling third-
party dependencies, before compiling the root library; no library-test verdict
is claimed. Both attempts are retained in
[`full-test-status.txt`](full-test-status.txt).

Conductor was unavailable (`conductor: command not found`). The research-KB
wrapper was queried for reciprocal/Barrett reduction and Pollard-rho batch
width, but no executable was available. Repository precedents used instead are
listed in the frozen protocol; no novelty or absence claim depends on the
failed searches.

## Next discriminating work

Restore checkpoint transport, profile the source-bound baseline and candidate,
then run the frozen A/A and at least five interleaved A/B observations on one
standard-isolation host. A width retune must cover the preregistered sensitivity
sizes and seeds; the q=223 development cell cannot promote b4 or b16. Only a
complete validated grid can change the matched GHS frontier.
