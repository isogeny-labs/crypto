# Prime31 B8 public-target D9 coefficient extraction

**2026-11-22, local development diagnostic. [pcc:unverified].** The positive residual was registered before this run in `RR9_D9_POSITIVE_KRYLOV_PROTOCOL_20261122.md`; the source and exact JSON are pinned by `receipt.json`. This is one positive coefficient extraction and independent arithmetic replay, with no natural relation-rank panel or complete logarithm solve.

## Input and method

The original ordinary, cofactor-one Prime31 curve has `p=2147483647`, `a=847514846`, `b=2087444724`, prime subgroup order `n=2147446831`, and factor-base abscissas `(0,1,2,3,5,6,8,9)`. The only target input to the extractor was the registered public residual `R=(1911579686,548820096)`. It formed the original eight RR9 quadrics in seven normalized coefficients and the degree-nine Macaulay operator `M` with 27,456 rows, 11,440 columns and 240,240 nonzeros. It ran scalar Wiedemann/Berlekamp–Massey on `M^T D M` with fixed seed `0x0d990003` and applied the quotient of the connection polynomial by `t` to the same seeded starting vector. No sign mask, known scalar, supplied chart coefficients or support index entered discovery.

The scalar recurrence had linear complexity 11,440 and zero constant coefficient. This alone is not a root certificate. The candidate vector passed a direct `M v = 0` check on every row and had a nonzero constant-monomial coordinate. Normalizing and reading the seven degree-one coordinates gave

```
[455835358, 518652820, 754716693,
 932383545, 62510441, 1884059825, 1142049404]
```

The Rust producer checked all eight original equations, recomputed the full polynomial identity and checked the eight signed points sum to the public residual. The separate Python verifier in `verify_d9_positive_krylov.py` independently rebuilt the base polynomial, the original RR9 coefficient identity, each curve ordinate and the signed group sum from the output JSON. Its exact replay is `independent_replay.json`. These checks establish a valid coefficient witness for this one input; they do not certify uniqueness of the chart or behavior on other natural residuals.

## Measured local costs and controls

| Arm or stage | This run |
| --- | ---: |
| Krylov positive full wall, including construction and checks | 30.345545625 s |
| Of that, scalar moments | 20.125402292 s |
| Projection and remaining work, by subtraction | 10.220143333 s |
| Scalar moment operator field multiplications | 11,883,322,880 |
| Projection Gram applications | 11,439 |
| Additional projection operator field multiplications, derived from the same operator count | 5,941,142,064 |
| Existing generic B8 4+4 setup, same local host | 25,584 ns; 128 group additions |
| Existing generic B8 4+4 registered-positive query, same local host | 459 ns; one group addition |
| Generic B8 natural public panel | 0 hits among 1,024 distinct targets |

The generic query's fixed positive is its first lookup and its submicrosecond clock is a single diagnostic observation; it is not a stable speed estimate. The complete 30-second Krylov cost and its roughly 18 billion counted operator field multiplications make the D9 linear-algebra phase the immediate runtime bottleneck at B8. The generic control uses an explicit 16-by-16 signed-support table and serves only as a correctness and cost reference. Its 1,024-target panel supplies no useful B8 relation yield. The one positive Krylov input was selected as a registered positive control, so it cannot estimate natural relation yield either.

## Interpretation and next discriminating action

This source demonstrates a public-target coefficient extractor for the fixed B8 positive RR9 chart, with independent original-curve verification. Its full cost is much higher than the same-host generic B8 control, and B8 support is too sparse to supply relation rank for the full objective. It is a mechanism result rather than a single-target index-calculus crossover.

The next test is a target-conditioned selected-root or quotient solver that avoids the 11,440-column D9 Krylov operator. Freeze the same one positive and two negative public controls, require original-equation/full-point positive replay and negative unit or exhaustive quotient certificates, and charge setup, failed calls, peak memory and wall time. A useful successor then has to handle B31 multiplicities and the frozen natural-target panel without enumerating supports; only a naturally useful rank stream warrants integrating the complete single-target solver and comparing it with optimized rho on fresh identical public targets.

## Provenance and prior work

- Raw positive output: `positive.json`; same-host original generic control: `generic_control.json`; independent verifier output: `independent_replay.json`; exact file hashes and commands: `receipt.json`. These are producer-development records without signed PCC or CI replay.
- Existing controls and problem scope: `../RR9_D9_BLACKBOX_RESULT_20261117.md`, `../RR9_B8_GENERIC_MITM_RESULT_20261117.md`, `../../native/M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md`, and `../../native/M8_NATURAL_F4_CI_OUTCOME_20261115.md`.
- Prior methods: Petit–Kosters–Messeng, *Algebraic approaches to ECDLP over prime fields*, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Yokoyama et al., *Complexity bounds on Semaev's naive index calculus method for ECDLP*, J. Math. Cryptol. 14 (2020), DOI 10.1515/jmc-2019-0029. The latter's lower bound uses statistical assumptions for the naive method and is not asserted for this extractor.
