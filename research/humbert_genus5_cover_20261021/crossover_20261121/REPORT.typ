#set document(title: "Genus-5 cover crossover test")
#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[The extrapolated genus-5 crossover does not happen:\ IC/rho rises from 1.21 to 2.00 over q = 65521…131071]\
  #v(2pt)
  Study `research/humbert_genus5_cover_20261021/crossover_20261121` · 2026-11-21 · follows `slope_20261120` (PR \#4848)
]

#v(6pt)
*Question.* `slope_20261120` fitted IC time slope 1.21 against the SIMD rho's 2.00 over $q = 16001…65521$.
It extrapolated a crossover near $q^* approx 92 000$, beyond the crate's $q < 2^(16)$. The crate now accepts
$q < 2^(17)$. An independent verifier found no defect, and every non-timing field at $q < 2^(16)$ is
byte-identical. This study tests the extrapolation directly. [pcc:unverified]

*Scope and standing.* Toy fields only. 5 values of $q$ × 5 seeds × 6 arms = 150 runs under
`PROTOCOL.md`, frozen before data. One smoke pair at $q = 131071$ is disclosed there: it ran before
freezing and is not data. One binary (`bin/humbert-v9`, sha256 prefix `26f1620a`), serial runs in
rotated order, the same 4-vCPU shared VM (L0, load1 median 2.4). Every wall time is exploratory and
an unverified assertion [pcc:unverified]. No leaderboard row.

= Correctness and counted identity

- *Z:* 150/150 runs solved (IC: solved and $x P = Q$; rho: the log, abandoned_walks = 0). No rho [pcc:unverified]
  run reached the $2 dot 10^10$ step cap.
- *C0:* at $q = 65521$, d35 and d50 reproduce the v8 `best8` and `d50_8` counted fields on 10/10
  instances. The r < 2^62 rule changed no instance there. d35_aa equals d35 on 25/25.

= The tested prediction

#let rt = csv("results/ratios_v9.csv").slice(1)
#figure(
  text(size: 7.5pt, table(
    columns: 7, align: right,
    [*q*], [*arm*], [*median t_ic*], [*IC/rho exp*], [*min*], [*max*], [*wins*],
    ..rt.map(r => (r.at(0), r.at(1), str(calc.round(float(r.at(3)), digits: 2)), str(calc.round(float(r.at(8)), digits: 3)), str(calc.round(float(r.at(9)), digits: 3)), str(calc.round(float(r.at(10)), digits: 3)), r.at(12) + "/" + r.at(2))).flatten()
  )),
  caption: [Per-instance t_ic / SIMD-rho expected time, same instance (`results/ratios_v9.csv`, L0). "wins" counts instances below 1.],
)

*X: falsified.* At $q = 131071$ the best arm (dlpf:0.35) is at a median 2.00 [1.71, 2.44] of the
matched rho's expected time, 0/5 instances below 1. The best-arm ratio rises monotonically:
1.21, 1.48, 1.68, 1.79 and 2.00 at $q = 65521$, 81001, 98009, 115001 and 131071. The A/A repeat
moves by at most 0.05. Above 65521 no instance and no arm is below the matched rho. [pcc:unverified]

#figure(image("figures/v9_gap_to_rho.svg", width: 72%), caption: [Median per-instance IC time / expected rho time per arm.])

= Why the extrapolation failed

Two effects, both measured here:

+ *LA becomes quadratic.* At a fixed factor-base fraction, the median LA time of d35 grows with slope
  1.98 in $q$, while relation collection grows with slope 1.05. Lanczos costs about $n dot "nnz"$
  with both proportional to the base size, hence $q^2$. Up to 65521 relations dominated, so the
  fitted IC slope (1.21) was a sub-asymptotic mixture. Here the best-arm slope is 1.51 and rising.
  Smaller bases flatten it (d20 1.16, d25 1.29), but their constant is higher in this range. The
  known genus-$g$ double-large-prime optimum (base shrinking with $q$, exponent $2 - 2 slash g$) is
  not reached by a fixed-alpha arm. [pcc:unverified]
+ *Instances above 2^16 have small r relative to q^4.* The kept rule r < 2^62 forces cofactors
  $h >= q^4 slash 2^62$, so the median r/q^4 falls from 0.050 at 65521 to 0.0104 at 131071. Rho's
  expected time follows $sqrt(r)$ and grows only with slope 0.85 here, against 2.00 below 2^16 where
  r/q^4 was roughly constant. IC's cost depends on $q$ (the base), not on r. Per instance the
  comparison is fair, since both arms attack the same r. But it is a comparison on instances whose
  group is 20–100× smaller than $q^4$, which favours rho. With r ∝ q^4 (not representable within
  r < 2^62 here) rho's slope would be about 2. Against the measured LA slope of about 2 at fixed
  alpha, the gap would then stay roughly constant rather than close. [pcc:unverified]

#figure(image("figures/v9_ic_vs_rho.svg", width: 74%), caption: [Median IC time per arm and median SIMD-rho expected time (L0). The rho curve is flattened by the shrinking r/q^4.])

#figure(image("figures/v9_phases.svg", width: 100%), caption: [Phase medians for d25 and d35 (L0). LA overtakes relation collection above q ≈ 98000.])

*d50 at large q.* At $q ≥ 115001$, dlpf:0.5 hits the instance-intrinsic empty-column precheck on
6 of 10 instances (163–2316 extensions, 27–442 s of filtering). This is the effect seen in
`levers_20261116` §3: large bases have columns that few relations hit. d35 needed no extensions
anywhere. d50's large-q medians (67.7 s and 82.9 s) are driven by this.

= What this means for the genus-5 line

- Against an honestly strong rho on the same instances, this crate's genus-5 IC does not cross
  below rho anywhere in $q < 2^(17)$. The two earlier sub-rho statements were weak references,
  and the slope extrapolation was a sub-asymptotic fit.
- Crossing would need an alpha schedule that shrinks with $q$ (the $2 - 2 slash g$ regime) and an
  LA better than quadratic in the base, for instance by filtering or structured Gaussian
  elimination before Lanczos. It would also need instances with r ∝ q^4, which means lifting the
  r < 2^62 limit (u128 or multi-limb LA).
- §5a: same-host paired wall time only. In counted units the order flips across the GAE band, as in v7 and v8.

= Reproduce

```
cargo build --release && cargo test --release          # 42 tests (1 ignored)
BIN=crossover_20261121/bin/humbert-v9 BIN_SHA256_PREFIX=26f1620a PRIMES=65521 \
  ./crossover_20261121/run_v9.sh crossover_20261121/campaign/q65521   # likewise 81001 98009 115001 131071
python3 crossover_20261121/analyze_v9.py
```
