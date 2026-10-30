# Fast-Kummer theta-constant base-field gate

Frozen on 2026-10-22 UTC before candidate execution. Input is the 21
independently factored Scholten quintics in
`research/four_followups_20261021/results/kummer-model-gate.jsonl` and the
checked affine Rosenhain setup from
`research/kummer_walk_20261022/ROSENHAIN_GATE.md`.

Following [Chung, Costello and Smith, §7.1](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf),
search for base-field nonzero constants `a,b,c,d,e,f` with
`A=a+b+c+d`, `B=a+b-c-d`, `C=a-b+c-d`, `D=a-b-c+d`, all nonzero,
and

```
lambda = ac/(bd),  mu = ce/(df),  nu = ae/(bf),
e/f = (1+alpha)/(1-alpha),  alpha^2 = CD/(AB).
```

For each model, enumerate all 20 ordered choices of two finite roots sent
to `0,1` while infinity stays infinity. Keep only choices for which the
transformed leading coefficient is a square over `Fp`; for each, enumerate
all six orders of the other three roots as `(lambda,mu,nu)`. Set `b=f=1`,
write `u=a/b`, `v=c/d`, `t=e/f`, solve `u^2=lambda*nu/mu`, `v=lambda/u`,
`t=nu/u`, `alpha=(t-1)/(t+1)`, and solve the resulting quadratic equation
for `d^2`. Check both square-root signs and every original equation exactly,
including all nonzero denominators in the fast model. Record all attempted
normalizations, qualifying triples, solutions and failures per model.

This finite search is **only** for affine normalizations keeping infinity
fixed and for constants in `Fp`. Failure is not a theorem for all Kummer
models, extension fields, or general projective root permutations. A found
theta tuple is a setup prerequisite, not an implemented projection or
xDBL/xADD. No candidate/rho cost ratio is formed. Save raw rows, native
validator output, a PDF report, and SVG/PNG graph with exact scope.
