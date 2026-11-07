# N15: bounded signed-lift gate

The [frozen protocol](PROTOCOL.md) replayed all 119,573 signed states in the
three 16-bit frozen subgroups and the same 1,029 states per model as N14 in
the 18 larger models: 138,095 states in 21 models. The proposed runtime
path received only `K(Q)`, `K(Q+G)`, the fixed signed anchor `G`, and model
constants. It did not receive the scalar or oracle divisor. The independent
replay constructed `Q=kG` afterward and checked every returned divisor.

| Scope | States | Fast Recover failures | Bounded fallback correct | Final null or wrong | Candidate divisors tested | Candidates with `b1=0` |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Three exhaustive 16-bit subgroups | 119,573 | 1,063 | 1,063 | 0 | 1,593 | 162 |
| Eighteen larger frozen samples | 18,522 | 62 | 62 | 0 | 84 | 1 |
| **Total** | **138,095** | **1,125** | **1,125** | **0** | **1,677** | **163** |

The successful fallback routes were direct algebraic inversion (988),
identity (21), and the first rational 2-torsion translation (116). The
direct failures split as 494 `D_zero`, 113 `Delta_zero`, 109
`b1_coefficient_zero`, and 409 `general_conversion_inapplicable`. Within
those 409 general-conversion failures, 21 were identities. The exact state
and branch are retained in [states.csv](results/states.csv). All 414,285
cached theta xADD outputs matched the Jacobian oracle, with no null or
mismatch in this replay. That xADD observation is exhaustive only at 16 bits.

## Algebraic inverse and exception conditions

For a generic degree-two divisor, normalize the recovered general Kummer
coordinate to `(1,-a1,a0,xi4)` and set `u=x²+a1*x+a0`, `v=b1*x+b0`.
The fourth coordinate gives `b1²` by the same polynomial expression used
in the native projection source. Reduce the curve polynomial `f` modulo
`u` to `rem0+rem1*x`. The congruence `v²=f mod u` yields

```
b0² - a0*b1² = rem0
2*b0*b1 - a1*b1² = rem1 .
```

If `b1` is nonzero, the second equation determines `b0`; Tonelli-Shanks
gives at most two choices of `b1`. If `b1=0`, require `rem1=0` and take at
most two square roots of `rem0` for `b0`. Thus each generic conversion
produces at most two valid divisor candidates, without a field-size scan.
The implementation verifies each candidate with the native Jacobian
validity predicate and both projective Kummer inputs. The latter fixes the
sign: if `K(Q+G)=K(-Q+G)`, then either `Q=O` or `2G=O`; the frozen generators
have odd prime order greater than two.

`general()` requires a nonsingular fixed 4-by-4 conversion matrix and a
nonzero first coordinate after inversion. Degree-two Mumford projection has
that first coordinate equal to one. Degree-zero `Q=O` is recognized from
`K(0)`. For a degree-one class `P-∞`, translation by any rational branch
point distinct from `P` gives a degree-two class; the implementation tries
the five recorded 2-torsion action matrices and translates each candidate
back. A maximum of six generic conversions, twelve modular square-root calls,
and thirteen candidate checks follows from the branch structure, conditional
on exact action matrices and projection identities. The branch proof does
not depend on the scalar replay. The existing model actions were derived
and checked on finite cases, not certified symbolically for every group
state; this is the remaining all-state model certificate gap.

The later [exact basis audit](../n17-mixed-zero-gate/REPORT.md) checked that
the matrix inverted by `general()` is `C1`, while the direct cancelled
degree-two formula uses `C2`. These are equivalent coordinate bases:
with `sigma=1+lambda+mu+nu`,
`C2(1,-a1,-a0,q3)=C1(1,-a1,a0,q3+sigma*a0)`.
Thus `general()` recovers the alternate degree-two vector with `a0` in
its third entry, as `degree_two_candidates()` expects. An interim concern
that the distinct matrices invalidated this inversion is withdrawn. This
basis audit changes none of the 138,095 replay rows or operation counters;
the source-independent geometric projection and all-state mixed-xADD
fallback remain separate gates.

For cached theta xADD, let `z=H((H(x)⊙H(y))/t)` and
`out_i=z_i²*product_{j≠i} d_j`. It returns the zero vector exactly when
every `i` has `z_i=0` or some other `d_j=0`. With all four coordinates of
the fixed difference nonzero, this reduces to `z=0`, equivalently
`H(x)_i*H(y)_i=0` for every `i`, since the Hadamard transform and selected
theta constants are invertible in odd characteristic. All 21 frozen
models have nonzero selected theta constants and nonzero coordinates in
both `K(0)` and `K(G)`. Consequently the doubling calls with `x=y` cannot
be zero; the mixed call still has a possible disjoint-support locus. The
bounded lift itself uses `K(Q)` and `K(Q+G)` and does not require mixed xADD.

## Operation boundary

Across the 138,095 safe-path calls after one-time cached theta-inverse
setup, the signed-recovery module counted **50,592,561 M + 38,978,317 A +
2,627,310 I**. The additional native Jacobian field counters were
**9,834,278 M + 414,369 S + 147,036 I + 15,791,770 field additions**,
including **139,720 Jacobian additions**. The safe path verifies every
fast-law result against the signed Kummer inputs, so one Jacobian addition
per state is included here. Modular square-root multiplications are in the
module M counter; inversion exponentiation is represented by I, not by M.
These are operation counts, not elapsed time. They omit reusable setup,
collector labels and walks, relation processing, final target verification,
and rho. The matched rho cost ratio remains **null**.

The [JSON summary](results/summary.json) binds the exact source/input
digests and CSV digest; the independent native validator recomputes the
frozen scalar sets, status split, all cost totals, and checksums. The
[graph](status-graph.svg) and [PDF](REPORT.pdf) display counts without a
ranking. The data and PDF live in the artifact store under adjacent
manifests. No 1,260-target cohort, EKS job, or Diem generator was launched.
The collector-level cost gate and all-state certificate for the model
actions/projection remain open.

Hydrate and validate from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/kummer_differential_collector_20261022/n15-bounded-lift
cargo run --offline --locked --release --manifest-path research/kummer_differential_collector_20261022/n15-bounded-lift/Cargo.toml --bin validate
```
