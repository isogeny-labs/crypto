# H116 quotient-digit conditional proof map

Status: written before the corresponding Lean theorem.

## Statement to formalize

For natural numbers `k` and radix `b=6`, Euclidean division supplies unique
`a,d` with

```text
k = b*a + d,   d < b.
```

For a point `P` in an additive commutative group and `Q=k • P`, this implies

```text
Q - d • P = b • (a • P).
```

If scalar multiplication by `b` is invertible on the prime-order subgroup,
the public quotient construction returns `a • P`. Repeating exact digit
recovery strictly decreases the natural scalar whenever `k>=b`; after at most
`ceil(log_b(k+1))` steps it reaches the public base case `k<b`.

## Dependency split

1. **Arithmetic core:** `k = 6*(k/6) + k%6`, `k%6<6`, and `k/6<k` for
   `k>=6`. This should use Mathlib natural-number division lemmas.
2. **Group identity:** distribute scalar multiplication over the arithmetic
   equality and rewrite subtraction. This is independent of elliptic-curve
   coordinates.
3. **Subgroup inverse:** when `gcd(6,r)=1`, choose `mu` with
   `6*mu = 1 mod r`; on an element killed by `r`, show
   `mu • (6 • X)=X`.
4. **Iteration/cost:** exact digits give a decreasing recursion of depth
   logarithmic in the scalar. This is conditional on the digit oracle and
   does not prove the moment rule exact.

## Controls and nonclaims

- The arithmetic/group theorem must also prove a deliberately wrong digit
  leaves a nonzero correction term unless subgroup wrap makes it vanish.
- No experimental result is imported into Lean.
- The theorem establishes only correctness of repeated quotienting given an
  exact digit oracle. It does not establish that any frozen moment rule is an
  oracle or that its evaluation is subexponential. [pcc:unverified]
