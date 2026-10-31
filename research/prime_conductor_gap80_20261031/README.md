# Native log-norm optimization and 80-bit candidate evidence

This study publishes the native C trace optimization, its native checks, and
the frozen 80-bit source/kernel study. The shared trace functional avoids
three trace-only polynomial products; a reversed-modulus inverse is also
reused by three modular products. This is an implementation of established
polynomial arithmetic, with no novelty or fastest-implementation claim.

The native checker passed 96 independent direct-root comparisons and 96
noninvertibility rejection controls across three fields. PARI independently
checked the frozen source, kernel and prime subgroup and produced a verified
certificate for the 202-bit field prime. Publication-time build and
correctness checks passed; no new performance experiment was run.

## Code and immutable evidence

- [`native/`](native/): C baseline, optimized helper, deterministic native
  microbenchmark and direct-root oracle, native CM-family producer and PARI
  verification entry point. `native/build.sh` compiles but launches no study.
- [`parameters.json`](parameters.json): unchanged producer parameters. The
  prime is `729845429276318409687041` (80 bits); the source is
  `y^2 = x^3 - 140*x + 784` over the 202-bit prime specified there. The
  endomorphism-conductor argument uses the ordinary CM discriminant `-7`
  source and an inert prime-degree kernel. This is not an arithmetic
  conductor difference between curves over the rationals.
- [`p-primecert.txt`](p-primecert.txt): unchanged PARI field-prime certificate.
  The parameters also contain a Pocklington witness for the 80-bit prime.
- [`historical-summary.json`](historical-summary.json): byte-identical copy
  of the historical `results.json`, including every exclusion and open item.
- [`evidence.tar.gz`](evidence.tar.gz): original 58,863-byte source/evidence
  package, unchanged. [`evidence.tar.gz.sha256`](evidence.tar.gz.sha256)
  pins it. The archive's `gap80-native/SHA256SUMS` verifies 55 files.
- [`reports/`](reports/): descriptive run/results reports, plotted historical
  receipts and rendering source; copies are in
  [`benchmarks/20261031-gap80-native-diagnostic`](../../benchmarks/20261031-gap80-native-diagnostic/).

```sh
cd research/prime_conductor_gap80_20261031
sha256sum -c evidence.tar.gz.sha256
tar -xzf evidence.tar.gz
(cd gap80-native && sha256sum -c SHA256SUMS)
NATIVE_PREFIX=/path/to/flint-gmp-pari-prefix sh native/build.sh
native/build/check
(cd native/build && ./pari_verify ../verify80.gp)
```

The original Python/Sage producer and its clocks remain inside the archive
as historical provenance; they are not installed as a research execution path.
Research execution uses native source and thin shell build orchestration.
The report's Python renderer only lays out already-recorded values.
The native algorithms are byte-identical to the frozen C producer files.
The new build script uses relative source paths and a configurable prefix;
the GP script also replays the frozen field-prime certificate and writes its
regenerated certificate in the build directory rather than the former
absolute scratch path. Run it from `native/build` as shown above. Future performance experiments
must follow current native execution, checkpoint and EKS measurement rules.

## Historical measurements and their scope

These observations are retained diagnostics, not admitted benchmark rows.
The original host was AMD EPYC 9V74, Linux x86-64, FLINT 3.3.1; the Sage tile
harness adds Python orchestration to the native helper. Exact platform,
compiler, seeds and inputs are inside the archive.

| Frozen workload | Pairs | Baseline median | Candidate median | Baseline/candidate |
|:--|--:|--:|--:|--:|
| Native helper, polynomial degree 4,096 | 7, alternating order, unpinned | 0.469961 s | 0.395023 s | 1.189706 |
| Native helper, polynomial degree 16,384 | 7, alternating order, unpinned | 2.453493 s | 2.087622 s | 1.175257 |
| Same actual 80-bit-kernel 4,096-by-4,096 tile | 3 clean pairs, CPU 0 | 3.735434 s | 3.431381 s | 1.088610 |

Ratios are ratios of the separately recorded medians, not means of paired
ratios. The last row's arithmetic time reduction is 8.139711%; it covers a
partial tile only. All corresponding outputs agree exactly. Seven tile pairs
were recorded: pairs 1–3 were unpinned (pair 1 overlapped a selftest); pairs
4–7 were pinned, but pair 4 overlapped the independent root-oracle check.
Only pairs 5–7 enter the last row. No confidence interval was recorded;
individual measurements and their ranges are shown in the reports. Pinning
does not establish a fully isolated host or a general wall-time advantage.

The historical Sage fixtures matched traditional Vélu quotient coefficients
and six point images at degrees 19 and 10,009. The exact 80-bit kernel's
8-by-8 tile matched direct enumeration of 128 roots at 32 non-kernel scalar
contexts under Sage, Python-FLINT and the optimized native backend. Those
32 contexts are partial sums, not 32 completed isogeny evaluations.

## Acceptance status and next construction

| Requested obligation | Evidence/status |
|:--|:--|
| At least an 80-bit prime source/kernel candidate | Native and independent PARI checks; parameters and field-prime certificate retained |
| Faster code with checked outputs | Native optimization and equality checks; historical timing diagnostics above |
| Explicit 80-bit quotient equation and complete non-kernel images | Not computed |
| Matched endpoint ECDLP measurements | Not attempted: destination and transported base missing |
| Largest certified explicitly evaluated crossing | Not established; the published 263-bit comparison is not evidence of an evaluated crossing |

At block 4,096, this parameter set requires 10,875,544,558,802,641
rectangular tiles, plus 722,432,689,208 leftover roots. The optimized tile
does not establish feasible total work. The next obligation is a native
construction/evaluation method that completes the quotient and non-kernel
images under a recorded resource envelope. See the updated
[existing problem record](../../problems/conductor-gap-upper-floor-and-cost.md).

## Admission, prior work and publication checks

The validation gate requires independently admitted evidence for benchmark
table rows. This study has no admitted row or matched rho protocol and no
end-to-end measurement. Its historical clocks are therefore linked as
documentary diagnostics outside the admitted benchmark table. Canonical
IC/rho progress, leaderboard, registry and browser graphs are unchanged:
no completed crossing, registry addition, priced IC/rho result or admitted
performance promotion is supplied. The existing figures were checked for
scope, rather than populated with a partial-map result.

Sources checked for this publication: the
[gap60 archive](../prime_conductor_gap60_20261031/README.md), existing problem
record, native baseline/candidate and their FLINT API declarations,
[FLINT polynomial documentation](https://flintlib.org/doc/fmpz_mod_poly.html),
[Sage 10.9 square-root Vélu source](https://github.com/sagemath/sage/blob/10.9/src/sage/schemes/elliptic_curves/hom_velusqrt.py),
and [PARI elliptic-curve documentation](https://pari.math.u-bordeaux.fr/dochtml/html/Elliptic_curves.html).
The earlier source study's reviewed alternative-route references remain in
the gap60 archive; this publication makes no new literature or record claim.

Archive verification passed all 55 manifest entries. Native compilation,
the 96-case direct-root oracle and PARI verification passed in the publishing
runtime. Cargo is not installed: `cargo test --release --lib` returned 127;
no Rust/library execution path changed. Conductor is not installed, as
documented as nonblocking in `docs/CONDUCTOR.md`.
