# Round C: direct cubic adjugate inversion

Status: prospective deterministic code/count protocol, frozen before candidate
implementation or output inspection on 2026-11-18. Round ID:
`ghs-strong-rho-r3-direct-adjugate-inverse`.

The later execution artifacts are namespaced under `round-c-inv/` so this
inversion round remains disjoint from other Round C engineering leads.

This round asks one bounded engineering question: can the matched strong-rho
comparator replace its Frobenius/product cubic inverse by the direct adjugate
formula while preserving every field value and rho walk and removing exactly
four counted base-field multiplications per `f3_inv`? It does not run or amend
the pending paid Round A panel. It cannot establish a throughput result or
move the restricted-family frontier or leaderboard.

## Baseline, sources, and unavailable services

The baseline is `origin/main` revision
`ad2a014c0022cf000845014499a9649b6c2cef2d`, after the merged Round B
five-product cubic square. Its `strong_rho.rs` SHA-256 is
`0bb8da07393873aa2b935403b86f5b7a0c24a52fff2d598a145a9fcca103229c`;
its `main.rs` SHA-256 is
`b2244ffece12855622a88f9a2af4926b1fac6113b9519fdf11e2bc7449fa27c8`.
Round A and Round B retain their own source bindings and are not relabelled.

The frozen comparison oracle is `Kf::inv` in
`research/quartic_n4_20261019/common.rs`, which uses
`a^q a^(q^2) / Norm(a)`. The direct formula is the adjugate of multiplication
by `a` in `F_q[z]/(z^3-c)`. The background scope and bibliography remain the
ones catalogued by the main protocol and Round B: Diem (ANTS VII, 2006),
Diem--Thome (J. Cryptology 21, 2008), and Gaudry--Hess--Smart
(J. Cryptology 15, 2002).

The repository research-KB wrapper reported no executable, and the Conductor
command was not installed in this runtime. No novelty, absence, completion,
or integration claim relies on those unavailable services.

## Candidate and exact count model

For `a = a0 + a1*z + a2*z^2` with `z^3 = c`, the candidate computes

```text
A = a0^2 - c*a1*a2
B = c*a2^2 - a0*a1
C = a1^2 - a0*a2
N = a0*A + c*(a1*C + a2*B)
a^-1 = N^-1 * (A + B*z + C*z^2).
```

Both paths use one base-field inversion and three final base-field products.
With the existing `fmul_small` helper, the frozen Frobenius path uses 16
counted `fq_mul` when `c <= 16`, while the direct path must use 12. When
`c > 16`, let `k_c` be the number of the three constant-product operands that
are nonzero. The two algebraically equivalent paths have the same three
operands, so their counts must be `16 + k_c` and `12 + k_c` (at most 19 and
15). Thus the required delta is exactly four `fq_mul` even when the fallback
short-circuits on zero. Addition counts may differ and remain explicit. The
direct path must not increment `f3_mul`; this is a kernel specialization, not
a general cubic multiplication.

## Frozen deterministic controls

1. On the existing public synthetic `q=223` instance, compare 200
   deterministic nonzero field inputs against both the frozen `Kf::inv` and a
   retained Frobenius reference that exactly reproduces the baseline kernel.
2. Exercise the `c > 16` fallback on the deterministic valid cubic field
   `q = 223`, `c = 18`, `w = c^((q-1)/3) = 39`, `w^2 = 183`; `c` is a
   non-cube because `39 != 1`. Compare the direct and Frobenius paths
   value-for-value and count-for-count on 200 deterministic nonzero inputs.
3. Run the existing standalone self-test, including planted-log recovery at
   widths 1, 8, and 32.
4. Run one deterministic `q=223`, seed 1, target 0, `ic-repeat`, `strong-b8`
   code/count control from the exact baseline and candidate. Ignore timing and
   calibration fields. The target, answer, walk shape, group work, native
   counts, and phase-local `fq_inv`/`f3_inv` must be identical. For every
   phase, candidate `fq_mul` must equal baseline `fq_mul - 4*f3_inv`, and
   candidate `f3_mul` must equal baseline `f3_mul - f3_inv`. The `fq_add`
   delta must equal the per-inverse kernel delta established by the local
   control. No other count may change.

These are local correctness/count tests, not benchmark or profile evidence.
No q=4051 diagnostic, wall-time screen, profile, paid job, or complete grid is
authorized by this round. Any attempted profile or throughput experiment first
requires a successful pushed checkpoint and gate under the repository rules.

## Requirements-to-evidence matrix

| Requirement | Deterministic evidence | Pass condition |
|:--|:--|:--|
| Algebraic equality | 200 `q=223` samples against frozen and retained Frobenius inverses | every nonzero input returns the same inverse and multiplies to one |
| Fallback equality | 200 samples in a valid `c>16` cubic field | direct and Frobenius values are identical |
| Exact multiplication delta | per-call counter snapshots in both constant paths | direct `fq_mul + 4 = reference fq_mul` for every inverse |
| Same base inversion count | the same snapshots | both paths execute exactly one `fq_inv` |
| No hidden cubic multiply | the same snapshots | direct `f3_mul = 0`; reference `f3_mul = 1` |
| Exact planted-log behavior | standalone widths 1, 8, and 32 | each returns the planted scalar with `ok=true` |
| Exact walk identity | paired `q=223` JSON after timing/calibration removal | target, answer, steps, DPs, restarts, look-ahead, cycles, group and native counts are identical |
| Phase-local E2E count identity | paired setup/walk/verify field records | `fq_mul` delta is `-4*f3_inv`, `f3_mul` delta is `-f3_inv`, and `fq_add` matches the kernel control; `fq_inv`, `f3_inv`, `f3_sq`, batch inversions and group additions are unchanged |
| Evidence discipline | retained commands, outputs, hashes and failure log | no failure is discarded and no throughput/frontier claim is made |

## Stop and decision rules

Stop on a wrong inverse, zero norm for a nonzero element, count delta other
than exactly four, differing base inversion count, planted-log failure,
divergent walk identity, missing output, or checkpoint refusal for any
profile/benchmark request. Preserve the failure as evidence.

The candidate is ready to integrate only if every deterministic algebra,
fallback, planted-log, walk-identity, and exact-count gate passes. Passing
means only that this source is a code/count optimization candidate. A
throughput or frontier statement still requires the main protocol's pushed
checkpoint, standard isolation, complete same-target grid, validation, PCC,
and publication gates.
