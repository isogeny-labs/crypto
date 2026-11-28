# Projective Semaev-chain parameter sweep — producer report

2026-11-28 · draft PR #5365 · all results [pcc:unverified].

## Scope and custody

The protocol was committed before the first run (`52768af12b4be4c044011ad567dd7ec8f9d41e02`). It fixes 36 public synthetic cells: p=97,193,257, curve y²=x³+2x+3, Kummer t=2,3,4, and m=4,5,6,7. Four deterministic targets per cell give 144 cold attempts. There are no random streams or exclusions. Each decision arm and the independent group-law reference rebuild the factor base for each target. Target selection is evaluator-only. The arm returns only a target-x verdict; it receives no planted signs or scalar.

The primary panel was completed before the fixed-leaf stress addendum was committed (`d2b050a5a8c31931c35b78759dc7a716e1c818a5`). The addendum is explicitly adaptive: its fixed leaf order was chosen to exercise the identity chart after the primary panel produced no affine verdict misses. It fixes all 36 cells and the P,Q selection rule before its own run; no pair is selected by outcome.

Native C source is `native/sweep.c` and `native/fixed.c`. This environment has GCC 14.2.0 on Linux 6.18.57 x86_64, but no repository checkout, Rust toolchain, EKS client or AWS credentials. The local run is an L0 correctness and discrete-work diagnostic; the repository's F4 engine was not run. Research-KB and Conductor tools were unavailable; source review used the existing PKM, summation-split and tree-gluing notes named in `PROTOCOL.md`.

## Primary panel

| m | cold targets | reference positive | projective/ref mismatches | affine misses | infinity-chart targets |
|---:|---:|---:|---:|---:|---:|
| 4 | 36 | 35 | 0 | 0 | 36 |
| 5 | 36 | 30 | 0 | 0 | 36 |
| 6 | 36 | 35 | 0 | 0 | 36 |
| 7 | 36 | 34 | 0 | 0 | 36 |
| Total | 144 | 134 | 0 | 0 | 144 |

The 10 other targets were reference negatives; there were no censored rows. Across the 144 cold attempts, the projective and affine arms each evaluated S3 45,728,400 times. The projective arm recorded 1,344 Kummer-candidate checks and 245,056 ordinate tests; the reference made 838,320 group-addition calls. The maximum single-target S3 count was 1,600,082. These counts describe this root-propagation implementation, not an F4 matrix cost.

## Fixed-leaf identity stress panel

The frozen ordered leaves are (x(P),x(P),x(Q),…,x(Q)); the planted target is x([m−2]Q). Thus the first two signed leaves can cancel. All 36 rows were reference positives and all projective verdicts matched; the affine-only arm missed 23. No row was censored.

| m | cells | affine misses | projective S3 evaluations | affine S3 evaluations |
|---:|---:|---:|---:|---:|
| 4 | 9 | 6 | 7,915 | 6,564 |
| 5 | 9 | 6 | 13,705 | 11,100 |
| 6 | 9 | 6 | 22,390 | 17,277 |
| 7 | 9 | 5 | 31,878 | 24,257 |
| Total | 36 | 23 | 75,888 | 59,198 |

The projective arm reached infinity in every fixed-leaf cell and made 16,690 more S3 evaluations than the affine arm in total. The separate reference made 1,798 group-addition calls. This panel shows a real correctness difference for fixed ordered leaves; it does not establish that the affine variable-leaf solver misses targets in the primary panel.

## Exact checks and limitations

- Both primary and fixed raw CSVs were reproduced byte for byte by a second deterministic run. Separate AddressSanitizer/UndefinedBehaviorSanitizer builds produced byte-identical CSVs for both panels. LeakSanitizer alone could not run under this environment's ptrace; the sanitizer runs used `ASAN_OPTIONS=detect_leaks=0`.
- The program stops on any projective/reference mismatch and on a missing planted fixed-leaf target. It compared every selected verdict, not every possible target or every fixed tuple. All checks are producer-run and unsigned.
- The Fp-projective chain covers genuine rational-point witnesses here. Formal S_(m+1) roots with arbitrary Fp input abscissae can require Fp² intermediates, as prior `summation_split/FINDINGS.md` records.
- Finite root propagation is not symbolic F4. Neither chart counts nor input degree establish a solving-degree bound, p^{o(1)} per-attempt work, a rank result, or a PDP speedup. No timing or external-tool superiority claim is made.
- The shared `docs/session-logs/2026-11-28.md` append was rejected by automatic approval review because the connector would republish the entire large shared log with unrelated session metadata. No retry through an indirect write was made. The task protocol, raw rows and this report are retained in this draft PR.

## Reproduce and hashes

```sh
gcc -std=c11 -O2 -Wall -Wextra -Werror native/sweep.c -o sweep
./sweep > raw.csv
gcc -std=c11 -O2 -Wall -Wextra -Werror native/fixed.c -o fixed
./fixed > fixed_raw.csv
gcc -std=c11 -O1 -g -Wall -Wextra -Werror -fsanitize=address,undefined -fno-omit-frame-pointer native/sweep.c -o sweep_san
ASAN_OPTIONS=detect_leaks=0 ./sweep_san > sanitized.csv
cmp raw.csv sanitized.csv
```

Run from `research/projective_chain_sweep_20261128/`; build `fixed_san` analogously and compare to `fixed_raw.csv`. The replay uses the same binaries a second time.

| File | SHA-256 |
|---|---|
| `raw.csv` | `5498b8b0048a408e34463ce353a8754357efe440804638ddc1cbfc1f0ff7abd3` |
| `fixed_raw.csv` | `c18fa7fef0c23006a3eae43cc6ad239eea4b99efa85f776d711164a856b4013e` |
| `native/sweep.c` | `ac36fec06f6a70045690f116b588714d09a93190a4c758efb326df696f4bf65a` |
| `native/fixed.c` | `4a46210910b075646edbf747184d10664e87c00bbbb40b90616c0a95894c7ea1` |

Next technical step: implement the chart-complete projective m≥4 system in the repository's native F4 path, with cold per-target setup, extraction, and independent relation-rank measurement under a separately frozen protocol.
