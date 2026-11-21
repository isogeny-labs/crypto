# Prime31 composed window4 + public-y APS: fresh 24-Q exact-source result

2026-11-16 UTC. Development evidence **[pcc:unverified]**. This is an evaluated engineering change to the previously published Amadori–Pintore–Sala S3 one-relation index-calculus-derived algorithm; enumerating every pair-root support remains generic meet-in-the-middle equivalent. No new asymptotic ECDLP attack is asserted.

## Source, immutable protocol, data, and validation

- Prospective [protocol](APS_COMPOSED_WIN4_PUBLICY_PROTOCOL_20261116.md) frozen at commit `d6df1ec9f52e1c4021dfe9c7c12d833c8f88244e` before implementation.
- [Existing Rust solver example](examples/prime31_aps_one_relation.rs), opt-in `--aps-composed-win4-publicy-24q`; implementation commit `2eedb58a2e46604aae1dcef528da77d9ff6eb427`.
- [Pinned Rust 1.100 workflow](../../../.github/workflows/prime31-aps-composed-24q.yml), check-in commit `d09e7174072a65a9d4340c31ac74e20250f65167`.
- [SUCCESSFUL GitHub Actions run 43974039411](https://github.com/isogeny-labs/crypto/actions/runs/43974039411), job `133673292302`, source-specific branch head `3602a8c0b2bbec32d167473b2310eadcc08ba8a1`: 40 library tests + 22 focused example tests PASS; 144/144 fully cold public-log recoveries, all correct.
- [Raw 24-case, six-arm artifact](https://github.com/isogeny-labs/crypto/actions/runs/43974039411/artifacts/14280322388), SHA256 of ZIP `46a58000e582e041706fec72996f9fb0459f82e99a450df834da3464e047de6a`. Every per-target chrono order, public Q, verifier-only k, complete solver output, source and S3 counters, signed relation and rho trajectory is preserved.

Curve: `p=2147483647=2^31-1`, `y²=x³+847514846*x+2087444724`, `n=2147446831` (cofactor one), `G=(625037715,2100627251)`. All arms get the same 24 distinct verifier-generated public Q; fixture k not supplied to any solver. APS partitions `[384,384,16384]`, up to six complete rounds; order rotates over six arms.

## Six-arm same-target full cold results

| Arm | Verified complete | Fully cold total, 24 Q | Compared with composed candidate |
|---|---:|---:|---:|
| Independent binary joint-Jacobian APS / sqrt lifts | 24/24 | **77.615866 ms** | Composed wins **22/24** |
| 4-bit-window Jacobian APS / sqrt lifts | 24/24 | **76.218073 ms** | Composed wins **21/24** |
| **4-bit window + public-y APS, composed** | **24/24** | **74.433895 ms** | candidate |
| Composed A/A repeat | 24/24 | 74.136036 ms | 0.4018% aggregate drift |
| Unfolded complete rho32 | 24/24 | 79.833260 ms | Composed wins **13/24** |
| Unfolded complete rho64 | 24/24 | 90.267704 ms | Composed wins **15/24** |

- **Composed/binary = 0.9590036011**, 4.0996% less whole-cold time on the same targets, but the **preregistered 5% improvement gate FAILS**. The predeclared paired-win and A/A-stability subgates PASS. Do not post-hoc lower the threshold.
- **Composed/window4 = 0.9765911426**, 2.3409% lower on this matched sample.
- **Composed/rho32 = 0.9323669734**, a 6.7633% measured aggregate advantage on this *particular* unisolated sample, but only 13/24 paired wins; **not** an established strongest-rho advantage.
- **Composed/rho64 = 0.8245905424**, a larger aggregate difference but 15/24 paired wins. Rho32 is the faster generic reference **on this sample**, not rho64.
- Four APS arms preserved every round/seed, S3 quadratic count, signed-candidate and recovered alpha/beta/log, rejected-branch and batch-inversion transcript. Across 24 Q, original two sqrt arms each made **18,432** S3 square roots; composed and repeat made **zero**.

Across the 24 Q, source setup binary/window4/composed/AA was respectively `28.620986 / 27.765684 / 27.540275 / 27.167282 ms`; S3-query time was `40.022188 / 39.606100 / 38.032218 / 38.080827 ms`. Work-counters agree, but stage-runtime deltas include host noise; do not interpret all observed source- and query-time differences as strictly causal.

## Independent arithmetic and statistical readback

I downloaded the exact 244,586-byte artifact JSON and **separately implemented short-Weierstrass affine point arithmetic with arbitrary-precision integers** to replay the public equations, without calling the Rust solver or using its point-add helper. Results:

- Curve generator on curve, `[n]G=O`.
- 24/24 caller-known `[k]G=Q`.
- 144/144 returned scalars recomputed independently as `[log]G=Q`.
- 96/96 returned APS signed congruences independently recomputed `(alpha+beta*log) mod n = 0`.
- All six cold totals, source and S3 totals, sqrt counts, exact transcript identities and paired-win counts recomputed from **individual** case records: zero mismatches.
- Reproducible 25,000-draw paired-target bootstrap (seed `0x20261116`; resamples 24 targets together, never independently resamples arms) yielded 95% percentile intervals for the *aggregate time ratio*:
  - composed/binary: **[0.9367, 0.9735]**
  - composed/window4: **[0.9644, 0.9862]**
  - composed/rho32: **[0.7023, 1.2270]**
  - composed/rho64: **[0.6306, 1.0831]**

Both rho intervals cross parity. These are **paired bootstrap diagnostics on unisolated runner observations**, not an independent-machine timing certification, and the arithmetic replay is **not a separately signed PCC report**. Results summarized in [machine-readable audit](APS_COMPOSED_WIN4_PUBLICY_24Q_AUDIT_20261116.json).

## Prior-lead assessment, avoiding duplicate research

| Lead | Evidentiary standing | Decision |
|---|---|---|
| Window4 public-jump and public-y S3 arithmetic | New whole-cold six-arm Rust CI, source and solutions matched; 4.10% win vs binary, below preregistered 5% gate | Retain exact engineering controls, do not reframe as novel cryptanalysis or strongest-rho win |
| #4391 APS 48-Q unfolded-rho baseline | Prior 109.734 ms APS vs 99.474 ms unfolded rho64, uncertainty crossed parity | Preserve as separate cohort; do not pool unlike algorithm revisions or judge performance from folded watchdog |
| #4451 zero-fibre S9 obstruction | Exact `D4`-injectivity ⇒ only net-zero `D8` kernel; source CI successful, no PCC | Use as negative original-ideal falsifier for an S9 extraction circuit, not a generic speedup |
| #4228/#4349 selected-root quotient / F4 | Certified smaller-arity roots and negative capped F4 attempts | Target-conditioned **nonenumerative f-from-R** at B4/B8, then Dickson31 B31; never enumerate the generic 4+4 support or 48,903,492 x-multisets in the candidate |
| Anchored six-summand F4 | Prior natural-target support ≤3.7407%, 0/16 natural supports and censored F4 | Do not repeat unchanged |
| Vieta/conic algebraic leads #4410 | Existing certified specialized tower and scale studies, not full prime31 solve below rho | Prefer certified bounded extraction, not another unrestricted quotient sweep |
| cryptanalysis prime-field auxiliary IC #1536 | Dependency-free Rust over p31 and p61, m2 centered-resultant and SIMD; auxiliary curve membership encoding **does not transfer DLP** | Reuse finite-field AVX2/512 SIMD arithmetic for a distinct S3-oracle hot-loop *ablation* only after matched baseline |
| External APS/PKM and tuned Wiener–Zuccherato negation rho | Published algorithms and repository optimized rho reference | Must test separately on same host and fresh blinded Q before ranking a solver as fastest |

Primary sources reviewed: Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves* (2018), DOI `10.1016/j.ffa.2018.01.009`; Kudo–Yokota–Takahashi–Yasuda, *Acceleration of index calculus for solving ECDLP over prime fields and its limitation*; PKM 2016 (`10.1007/978-3-662-49387-8_1`). Shared research-KB MCP/local wrapper unavailable in this runtime; GitHub primary sources and publisher records used. No novelty or exhaustive-prior-art claim is made.

## Prospectively proposed next experiments

1. **Strongest generic frontier, 96 fresh Q:** incorporate the repository's tuned look-ahead Wiener–Zuccherato `rho.negation` plus an optimized unfolded/batched rho into the *same native Rust 96-Q six-arm tournament*. Charge group arithmetic, all fresh cold setup, distinguished points, failed walks, and verification. Isolate CPU, record per-case distributions and independent replay. Stop calling an aggregate win a statistically established advantage while bootstrap CIs cross one.
2. **Distinct S3 oracle hot-loop ablation:** import the existing p31 AVX2/512 finite-field backend from `isogeny-labs/cryptanalysis` #1536 for the 384×384 pair arithmetic; predeclare exact same root/witness transcript, peak-memory and full-cold limits. Keep the current scalar Rust oracle as an independent matched control; SIMD speed need not carry to total solve.
3. **Genuinely non-generic S9:** reuse Rust #4228/#4349 quotient/root machinery to recover factor polynomial `f(T)` and signed orientations **from Q itself** at blinded B4/B8, cross-check with exact enumerated positives/negatives, then natural Dickson31 targets and rank32 full scalar recovery. Any candidate requiring generic B31 four-sum enumeration or uncontrolled F4 cutoff fails the gate. Compare true cold single-target cost to the tuned WZ rho and mature published prime-field IC implementations.

No paid compute, merge, leaderboard claim or external-security claim is authorized by this development result.
