# Round C: cold batched setup for strong rho

Status: prospective local code/count protocol, frozen before candidate
implementation or candidate output on 2026-11-18. Round ID:
`ghs-strong-rho-r3-batched-window2-setup`.

This round asks one bounded engineering question: can the 32 jump points and
initial lockstep walkers be constructed with fewer fully charged field
operations by sharing inversions across exact simultaneous scalar
multiplications? It does not amend the pending Round A performance panel and
cannot move the restricted-family frontier or leaderboard.

## Baseline, lead, and candidate

The baseline is `origin/main` revision
`ad2a014c0022cf000845014499a9649b6c2cef2d`. Its Round B
`strong_rho.rs` SHA-256 is
`0bb8da07393873aa2b935403b86f5b7a0c24a52fff2d598a145a9fcca103229c`;
`main.rs` is
`b2244ffece12855622a88f9a2af4926b1fac6113b9519fdf11e2bc7449fa27c8`.
The baseline constructs every point independently as
`[a]G + [b]Q` with two affine double-and-add scalar multiplications and one
affine addition.

The candidate is a shared width-2 simultaneous multiplication. A cold table
contains every `[i]G + [j]Q`, `0 <= i,j < 4`; its construction is charged.
For each base-4 digit, all live outputs are doubled twice and then receive
their selected table entry. Each doubling/addition layer uses one Montgomery
batch inversion across outputs. The candidate must retain the sampled
coefficient arrays verbatim. If a tentatively batched initial walker is the
identity, the implementation must restore the RNG and counters and execute
the baseline sequential retry path; RNG reordering is forbidden.

This is a standard fixed-window/simultaneous-multiplication engineering lead,
not a new rho algorithm. The repository research-KB wrapper was queried first
but reported no executable. Bounded repository fallback found the existing
Montgomery-batch accounting in `src/cryptanalysis/jv_rho_strong.rs` and the
matched GHS cost model in `research/ghs3_vs_rho_20261023/PROTOCOL.md`.
No novelty or literature-absence claim depends on the unavailable KB.

## Frozen workload and accounting

The deterministic control is the existing public synthetic `q=223`, instance
seed 1, target index 0, target policy `ic-repeat`, arm `strong-b8`, calibration
triples 1, single thread. Candidate and in-source legacy-setup control use the
same executable and algorithm seed. The target multiplication, lookup-table
construction, jump construction, walker construction, canonicalisation, and
all exceptional fallback work are setup and are charged cold.

Primary evidence is the phase-separated operation vector. For any
nonnegative inversion price `R`, setup field cost is

```text
M-eq(R) = fq_mul + R * fq_inv.
```

`fq_add`, extension-field operation kinds, batch inversions, group operations,
and native counts are reported beside it rather than hidden. A candidate is
strictly lower in setup counted work only if its `fq_mul` and `fq_inv` are no
greater than baseline and at least one is lower; this establishes lower
`M-eq(R)` for every `R >= 0` without choosing a post-output calibration.
Walk and verification deltas must be zero. Wall time is diagnostic only and
is not an acceptance signal.

## Requirements-to-evidence matrix

| Requirement | Evidence | Pass condition |
|:--|:--|:--|
| Complete batch group law | deterministic standalone self-test | mixed additions, doublings, identities, and inverse pairs equal the frozen curve law point-for-point |
| Exact simultaneous multiplication | candidate-vs-legacy tests over deterministic coefficient panels | every `[a]G+[b]Q` point is identical, including zero coefficients |
| Same setup state | paired setup trace/test | `ma`, `mb`, all 32 jump points, every initial walker point/coefficient, and post-setup RNG state are identical |
| Same ECDLP work | paired `q=223` JSON records | same answer, steps, DPs, restarts, look-ahead retries, cycles, cycle additions, walk field counts, verification field counts, and all walk native counts |
| Lower cold setup cost | paired setup operation vectors | candidate componentwise no worse in `fq_mul` and `fq_inv`, at least one strict; table/precomputation included |
| Honest failures | retained stderr/status/raw records | wrong points, state divergence, nonzero deltas outside setup, or compile/test failures remain visible |
| Frontier discipline | report and dashboard unchanged | no throughput/frontier claim without checkpoint, isolation, complete matched grid, independent validation, PCC, and publication gates |

## Execution and stop rules

Compile the standalone driver with `rustc --edition 2021 -O`. Run `selftest`
before the paired control. Retain exact commands, compiler and host metadata,
source/binary hashes, stdout, stderr, and exit status. The conductor CLI was
checked before edits and was absent.

Stop on a wrong point, coefficient/RNG mismatch, wrong logarithm, divergent
walk identity, candidate setup `fq_mul` or `fq_inv` regression, checkpoint
refusal, or missing raw output. Failures remain evidence. The checkpoint gate
is required before any profile, timing panel, larger diagnostic, paid run, or
frontier claim. The `q=223` deterministic code/count control is development
validation only.
