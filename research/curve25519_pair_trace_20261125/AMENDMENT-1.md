# Amendment 1: retain the trace-doubling cross product

**Filed before any D/R/P solver row.** The first release unit test rejected
the initial pair-trace implementation because the written fast-doubling
identity

```text
U_(2k-1) = U_(k-1) * (2*U_k - c*U_(k-1))
```

is cubic if expanded directly in the retained variables. To implement the
protocol's explicit requirement that every P input equation be quadratic,
each nontrivial doubling now retains one local cross product

```text
t_k = c*U_(k-1),
U_(2k-1) = U_(k-1)*(2*U_k-t_k).
```

This adds one variable and one equation at `d=9`, and two variables and two
equations at `d=19`; `d=7` is unchanged because its first odd output is the
existing variable `c`. P therefore has 18, 20, and 28 variables at
`d=7,9,19`. No arm, seed, target, threshold, cap, equation zero set, or
measurement order changes.
