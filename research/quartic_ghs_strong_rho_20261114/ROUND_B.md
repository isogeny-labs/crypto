# Round B: five-product cubic squaring optimization

Status: prospective local optimization protocol, frozen before candidate
implementation or measurement on 2026-11-18. Round ID:
`ghs-strong-rho-r2-five-product-square`.

This round asks one bounded engineering question: can the matched strong-rho
comparator remove one base-field multiplication from every
`F_q[z]/(z^3-c)` square without changing the walk, answers, or any other
count? It does not execute or amend the pending paid Round A panel, and it
cannot by itself move the restricted-family leaderboard.

## Baseline and source

The baseline is `origin/main` revision
`9ec88b7637aac4ce816739155f12f518b393da37`, using the frozen Round A
`strong_rho.rs` with SHA-256
`73abd342613dee852add1e3459c94bc7f949801c94eb875a309cdfc4f6b0f5de`.
Round A remains reconstructible from its pinned merge revision and hashes.
Any candidate source is a new Round B implementation and must receive a new
hash; it is not relabelled as Round A output.

The lead comes from the repository's earlier strongest-rho accounting:
`src/cryptanalysis/jv_rho_strong.rs` prices a tuned squaring separately from
a generic Karatsuba product. The current cubic comparator instead implements
`sq(a)` as `mul_raw(a, a)`, which executes six `F_q` products. For
`a = a0 + a1 z + a2 z^2`, `z^3 = c`, the candidate computes

```text
m0  = a0^2
m2  = a2^2
m01 = a0*a1
m12 = a1*a2
ms  = (a0+a1+a2)^2
d1  = 2*m01
d3  = 2*m12
d2  = ms - m0 - m2 - d1 - d3
a^2 = (m0 + c*d3) + (d1 + c*m2)z + d2*z^2.
```

This is five `F_q` products plus additions. It is an algebraic specialization,
not a new rho method or asymptotic claim. Background sources remain Diem
(ANTS VII, 2006), Diem--Thome (J. Cryptology 21, 2008), and
Gaudry--Hess--Smart (J. Cryptology 15, 2002), as catalogued in the historical
`papers/ecdlp-index-calculus-program/refs.bib`. The research-KB wrapper was
unavailable in this runtime; no novelty or absence claim relies on that
failure.

## Frozen workload and evidence

The local diagnostic cell is the existing public synthetic instance
`q=4051`, instance seed `1`, target index `0`, target policy `ic-repeat`, arm
`strong-b8`, calibration triples `3`, single thread. A `q=223` self-test is a
correctness control only. Exact build commands, compiler and host metadata,
binary/source hashes, stdout, stderr, exit status, and profiling manifests are
retained under `round-b/`.

Profiling identifies cost but cannot establish a throughput result. The baseline is
profiled first with `tools/profile.sh`; Callgrind is preferred when supported
because its instruction count is deterministic. A separate unprofiled timing
screen uses an A/A noise floor followed by at least five interleaved A/B pairs
on the same host. Local wall time is diagnostic because this host is not the
frozen standard-isolation host. Deterministic operation counts are primary.

## Requirements-to-evidence matrix

| Requirement | Evidence | Pass condition |
|:--|:--|:--|
| Algebraic correctness | self-test plus candidate-vs-generic square tests | every tested square equals `mul(a,a)` |
| Same ECDLP work | paired JSON records | same instance, target, answer, steps, DPs, restarts, look-ahead, cycles, group additions, and every native count |
| Count reduction | paired field-count records | candidate `fq_mul = baseline fq_mul - f3_sq` in each phase; all other field counts equal except the explicitly derived addition delta |
| Profile-supported lead | immutable baseline profile manifest and summary | profiler succeeds and identifies arithmetic cost, or an unsupported manifest is retained; no timing claim is inferred from it |
| Local runtime screen | raw A/A and ABAB observations | all runs correct; report medians and every sample beside the A/A spread, labelled diagnostic |
| Frontier discipline | report and existing dashboard remain unchanged | no leaderboard move until the complete Round B grid passes Round A's same-target, isolation, validation, PCC, and publication gates |

## Stop and decision rules

Stop on a wrong square, wrong discrete logarithm, divergent walk identity,
unexpected count delta, checkpoint refusal, or missing raw output. Failures
remain evidence.

The candidate is accepted as a code/count optimization only if all
correctness and identity gates pass and it removes exactly one `fq_mul` per
recorded `f3_sq`. A wall-time median outside the A/A noise band is reported
only as a local diagnostic. Regardless of outcome, the restricted GHS-weak
family remains historically IC-led only under the already admitted historical
protocol; Round B does not re-establish the complete matched boundary.
