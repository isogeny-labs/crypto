# Repaired N31 columns survive the trace-zero slice

For both frozen N31 parameters, exact substitution from the 32-variable S235 code basis to two 15-variable trace-zero leaves produces the same 31 S₃ coordinate ANFs as a direct 30-variable construction. Each parameter's own `Tr(S₃/q²)` functional removes one dependent equation. The resulting **30-equation, 13,980-row** degree-four matrices have rank **13,515**.

Mapping each six-column-repaired 32-variable information set through the Boolean substitution yields a **23,286-column** monomial support in the 30-variable representation. Restricting the matrix to that support preserves rank **13,515** for both parameters.

| Frozen parameter | Full columns / rank | Mapped-support columns / rank | Full row nonzeros | Mapped row nonzeros |
| --- | ---: | ---: | ---: | ---: |
| `fresh_1`, q=1,558,806,792 | 31,931 / 13,515 | 23,286 / 13,515 | 1,567,048 | 1,269,698 |
| `fresh_15`, q=2,021,112,548 | 31,931 / 13,515 | 23,286 / 13,515 | 1,586,510 | 1,281,370 |

The candidate support is the union of monomials appearing after substitution of the repaired 32-variable columns; it is an implementable 30-variable coordinate projection, not a bijection of pivots. The [frozen protocol](PROTOCOL.md), [producer](bridge.py), and [result](result.json) retain source hashes, per-parameter trace masks, equation-drop choices, symbolic equality, row-stream hashes, exact ranks, and observed ranker resource use. The [amendment](AMENDMENT.md) records three premeasurement corrections to trace-mask and column-index handling. No Sage process was launched; the producer uses the archived pure-Python Boolean field builder and independent bit-polynomial trace arithmetic.

This bridges the repaired information set to the same **30-variable trace-zero representation** used by the completed ordinary-residual F4 study. The static degree-four rows are still different from msolve's dynamic matrices, which include Boolean field equations and later generated polynomials. The next implementation gate is to make msolve consume or maintain this projected information set while preserving its complete Gröbner basis and point-verified roots, then compare charged target-dependent work against its unprojected input on matched residuals. The local ranker wall times are exploratory stage diagnostics.
