# Trace +22 degree-eight linear-square screen

The source frozen at `00cc0f077328` completed all four exact carrier/source representative cells for five denominator shapes: `L(x-c)^2`, `L(x-c)^4`, `L(x-c)^6`, `LQ_i(x-c)^2`, and `LQ_i(x-c)^4`, where `L=x`, `c∈K×`, `K=F3[a]/(a^5+2a+1)`, and the five `Q_i` are the quadratic branch factors of the selected carrier. The source equations have b masks 68 and 84, both with order 222; the carrier masks are 32280 and 35349.

| carrier mask | source mask | logical candidates | direct point controls | point survivors | exact identities | CPU s |
|---:|---:|---:|---:|---:|---:|---:|
| 32280 | 68 | 23,894,838 | 1,952,100 | 36,189 | 0 | 124.836 |
| 32280 | 84 | 23,894,838 | 1,952,100 | 35,911 | 0 | 125.010 |
| 35349 | 68 | 23,894,838 | 1,952,100 | 42,873 | 0 | 131.245 |
| 35349 | 84 | 23,894,838 | 1,952,100 | 42,904 | 0 | 131.485 |

The `LH` arm solves the exact `x^9` numerator-coefficient equation in the nonbranch linear pole parameter, with a full-field fallback for a zero polynomial. The `LQiH` arm tests all nonzero `c` and square leading coefficients with necessary point evaluations, then checks every survivor against the full polynomial identity. The four JSON receipts are bound to the one source SHA256; [`audit_degree8_linear_square.py`](audit_degree8_linear_square.py) checked source hashes, exact model masks, target order, expected assignment and logical candidate counts, direct controls and zero-hit fields. Its output is [`DEGREE8_LINEAR_SQUARE_AUDIT.json`](DEGREE8_LINEAR_SQUARE_AUDIT.json). This audit is structural, not an independent implementation of elimination. [pcc:unverified]

Together with the earlier sparse and two-pole screens, this closes these named denominator shapes for the four exact cells. General quadratic or cubic square parts, `LQ_iQ_j(x-c)^2` with arbitrary `c`, and three-quadratic-pole shapes remain open. This is a partial finite exclusion, not a degree-eight theorem or a demonstrated new vulnerable family.
