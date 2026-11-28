# Weighted tuple iterator invariant

This note isolates the state-machine proof for `next_tuple`. It is a
producer-side argument tied to the Rust implementation, not an independently
checked or kernel-checked refinement. `[pcc:unverified]`

## Statement

Let the modes be indexed by `j=0,...,m-1`. Mode `j` has a positive integer
weight `w_j` and a target group `g_j`. For group bounds `B_g` and precision
`p>=1`, define the admissible set

```text
A = { beta in N^m :
      sum_j beta_j*w_j < p
      and, for every g, sum_(j:g_j=g) beta_j <= B_g }.
```

Starting from the zero vector, the loop in `evaluate_at` visits the current
vector and then calls `next_tuple`. The claim is that this visits every member
of `A` exactly once and then returns `false`. `[pcc:unverified]`

## Cached-state invariant

At entry to `next_tuple`, the cached values satisfy

```text
weight = sum_j beta_j*w_j < p,
group_usage[g] = sum_(j:g_j=g) beta_j <= B_g.
```

When the scan reaches index `i`, indices below `i` have been reset to zero,
and both caches have had exactly their old contributions subtracted. The
increment condition

```text
w_i < p-weight  and  group_usage[g_i] < B_(g_i)
```

is therefore equivalent to saying that incrementing `beta_i` by one remains
in `A`. On success, all three cached equalities are restored. On failure at an
index, subtracting `beta_i*w_i` and `beta_i` from the appropriate caches is
exact before `beta_i` is cleared. Thus the invariant holds at every return and
at every scan boundary. `[pcc:unverified]`

The same invariant rules out arithmetic wraparound on an admitted state:

- `p-weight` is positive;
- a successful weight addition is strictly below `p`;
- a successful usage addition is at most its `usize` bound; and
- each reset product `beta_i*w_i` is at most the cached total weight.

Production modes have `w_i=b+1`, so the required positivity is structural.

## Completeness, uniqueness, and termination

Order exponent vectors with index zero least significant: compare

```text
(beta_(m-1), ..., beta_1, beta_0)
```

lexicographically. `next_tuple` is the restricted odometer successor in this
order. It first tries the least-significant increment. If that increment is
inadmissible, positivity of weights and the group cap imply that no larger
value at that digit is admissible while the more-significant suffix is fixed.
The digit is reset and the same argument proceeds at the next index. The first
successful increment is consequently the least admissible vector greater
than the current vector; every lower digit is already zero. If no increment
succeeds, no greater admissible vector exists. `[pcc:unverified]`

Every successful call strictly increases the displayed lexicographic vector.
Every digit is bounded by its target-group bound, so `A` is finite. The calls
therefore terminate, cannot repeat a vector, and cannot skip an admissible
vector. The zero vector is admissible and is evaluated before the first call,
which completes the enumeration argument. `[pcc:unverified]`

## Native finite gate

The native control compares the production iterator with a separately written
recursive Cartesian oracle. Its exhaustive small-state sweep covers zero
through four modes, every assignment of two target groups and weights 1/2/3,
both group bounds from zero through two, and precisions one through six:

```text
(1+6+6^2+6^3+6^4) * 3^2 * 6 = 83,970 configurations.
```

Two additional fixtures exercise distinct packets and four physical group
slots. These checks make the cached-state and carry behavior load-bearing but
do not replace an independent refinement of the general proof above.
`[pcc:unverified]`
