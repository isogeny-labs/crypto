# R118: exact unit-part precision at the four x=0 places

Writing each local x series as `s^e U` removes the artificial `e*denominator_exponent` precision loss in R117. The unit `U` is known through `s^(640-e)` on the wild branch and equals 1 exactly on the tame monomial branches. A term with numerator order `n` and denominator `x^d` is therefore known through exponent `n-ed+min(640-n,640-e)`, omitting the second bound when `d=0` or `U=1`. The 29 pinned G51 terms were recombined by lambda exponent under that bound.

At lambda exponents `[0,1,4,8,16]`, the exact coefficient orders are:

| base place | exact orders | lower Newton edge | lambda order on edge |
|---|---|---|---|
| y0/e1 | `[-160,-127,-97,-80,0]` | `(0,-160)` to `(16,0)` | `-10` |
| y0/e3 | `[-112,-82,-61,-56,0]` | `(0,-112)` to `(16,0)` | `-7` |
| y1/e1 | `[0,23,23,0,0]` | `(0,0)` to `(16,0)` | `0` |
| y1/e2 | `[0,23,23,0,0]` | `(0,0)` to `(16,0)` | `0` |

Each edge has on-edge powers `0,8,16` and initial polynomial `1+T^8+T^16=(1+T+T^2)^8` over F2. All four lower hulls are now exact at the recorded precision. The initial polynomial has repeated roots, so the next local task is a lambda lift that resolves normalized branches and their ramification. These coefficient results do not by themselves determine the N51 valuation.

The initial source run exposed a missing `d=0` precision exception in the first implementation: its monotonicity assertion rejected an otherwise known R117 cutoff. The exception was added before accepting data, and the protocol now states it. No accepted result used that failed run. The proposed matplotlib plot failed because matplotlib is unavailable; a dependency-free PDF renderer replaced it.

Checks: all previously exact R117 rows and supports below their old cutoffs agree; every new cutoff is at least the old cutoff. Fresh replay is byte-identical. A changed R116 byte is rejected by the hash gate. An independent schoolbook multiplication and unit inversion reproduces every coefficient support and cutoff. Lean 4.34.1 checks the four finite lower-edge inequalities, on-edge index sets, and repeated polynomial identity, with generated constants bound to the result JSON. Lean does not formalize the complete G51 substitution or the Newton polygon theorem.

Result SHA-256: `3840eb0fae9e2242abda0f33b411b39f2cfba983a88fd621ef16bda8536c9150`. Replay: `python3 research/koblitz_modular_ic_20261018/rounds/118/unit118.py <output-directory>`. Independent check: `python3 research/koblitz_modular_ic_20261018/rounds/118/verify118.py`. No cloud work or spend.
