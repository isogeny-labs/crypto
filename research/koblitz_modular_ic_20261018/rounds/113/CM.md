# R113: second blowup exposes a wild x=0 base edge

The exact R51 substitution `y=1+x+x^2(1+v)` is divisible by x^6. Its quotient has 34 nonzero F2 monomials and special fiber `v^2+v^3=v^2(1+v)`. The root v=1 is the previously separated simple branch control; v=0 is the still unresolved double root. The lower hull at v=0 in `(v exponent,x exponent)` coordinates is `(0,3),(2,0),(3,0),(7,8)`. Its sole positive edge has slope `v_x(v)=3/2`, residual `1+T^2=(1+T)^2`, and coprime numerator/denominator `(3,2)=1`.

Because the edge residual is inseparable in characteristic two, the slope alone does not give a simple Hensel root or a normalized place count. The next construction should pass to `x=r^2`, set `v=r^3(1+h)`, compute higher Hasse terms and identify an Artin–Schreier or further Newton coordinate with a simple residual. The still-simple slope-5 branch from R112 should be lifted separately. The G lambda and N51 valuations remain later tasks.

The source and protocol were committed before data. Fresh replay was byte-identical; an altered donor failed the hash gate. An independent verifier composed the exact R112 transformed polynomial with `w=x(1+v)`, reproduced all 34 terms, and checked each lower supporting line and the edge support. Lean 4.34.1 checked `1+T^2=(1+T)^2`, coprimality of 3 and 2, and the special-fiber bits. There was no producer/replay/verifier failure.

Result SHA-256: `41996e050fa85958218d4d205cb5cdeec599d59e4176fa0e11db38f6c467ef26`. Raw archive SHA-256: `c4eeb180dfb476450ff566fad3bd9c3b6e493c772fd8f617138a76b653a7e009` (876 bytes). Replay: `python3 research/koblitz_modular_ic_20261018/rounds/113/blowup113.py <output-directory>`; independent check: `python3 research/koblitz_modular_ic_20261018/rounds/113/verify113.py`.

Sources: R51 donor, R105 parser, R111/R112 Newton data, [Stacks integral closure](https://stacks.math.columbia.edu/tag/00GH), [Stacks Henselian lifting](https://stacks.math.columbia.edu/tag/04GE).
