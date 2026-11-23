# Regularity from two survivor fibers

Run 20261118T0459Z uses the [matched toy bases](../run-20261118T0159Z/RUN.md)
and follows the [conditional survivor protocol](../run-20261118T0409Z/RUN.md).
The candidate rule is that the observed degree is the largest surviving
(y)-fiber size plus one when the two nonempty fibers have the same size.
A fiber is the set of points with a given value of (y). The target and
membership equations are fixed as in the previous run.

Check the rule on every reachable mixed target and every shared pin of
the common target. The independent reference computes ranks of evaluations
of low-degree monomials. Compare all unpinned target degrees with the prior
Singular table. The separate point-set controls pair every subset of the
five (x)-values ({0,1,2,17,43}) on (y=1) with every subset on (y=2).
Skip the empty pair of subsets. Include two three-fiber controls with the
same occupancy to test where the two-fiber rule stops applying.

## Exact rule

Let (M) be the number of distinct (x)-values in the larger nonempty
fiber and (N) the size of the other fiber, with (N=0) for one fiber.
The observed degree is

```text
one fiber or two unequal fibers   M
two equal nonempty fibers        M + 1
```

The observed degree is the first degree with no standard monomials in a
graded order. Equivalently, it is one plus the least degree at which
polynomials can interpolate arbitrary values on the survivor set.
The membership polynomials split into distinct field elements. Their
quotient is a product of field copies indexed by the candidate triples.
Adding the target constraints and any coordinate pin removes the
corresponding field copies, leaving the coordinate ring of the
survivors. The linear sum lets us eliminate (z) without changing the
degree filtration. The mixed relation becomes (y(t-y)=s), giving at
most two (y)-values.

On one fiber, univariate interpolation needs degree (M-1). For two
fibers, a polynomial of total degree at most (d) can be reduced to
(A(x)+yB(x)) with (deg Ale d) and (deg Ble d-1).
Its two univariate restrictions can be any polynomials of degree at
most (d) with the same coefficient of (x^d). Conversely, two
such polynomials determine (A,B) because the two (y)-values are
distinct. At (d=M-1), the data on the larger fiber fix that
coefficient. If the other fiber has fewer points, its interpolant
can match the coefficient by adding a multiple of its vanishing
polynomial. With equal fiber sizes, independent data on the two
fibers can force different coefficients. Increasing (d) by one
then allows both interpolants to use degree at most (M-1) and
share a zero coefficient of (x^d).

## What changed the degree

Using the consecutive base with (t=3), both (s=0) and (s=2) leave
five triples. The fiber sizes are 4 and 1 for (s=0), giving observed
degree 4. They are 3 and 2 for (s=2), giving degree 3.
At the shared target ((3,2)), the centered base has sizes 4 and 3,
and the consecutive and spread bases each have sizes 3 and 2.
Those occupancies explain the observed 4 versus 3 degree split.
[pcc:unverified]

The reference rank check agreed with all 130 archived Singular
base–target rows, all nine prior pinned Singular degrees, and the
counts of all 27 tracked pins. Three pins had no survivors and no
reported degree. The rule also agreed with independent ranks on
all 1,023 nonempty two-line subset controls. The three-fiber controls
each have occupancies ((1,1,1)). The collinear control has degree 3
and the noncollinear one has degree 2, so occupancy alone cannot give
the degree at that scope. [pcc:unverified]

[Figure](fiber_regularity.svg) · [PDF](report.pdf) ·
[Recorded predictions and references](fiber_regularity.csv)

## Replay

g++ 14.2 compiled the checker on Linux x86-64. The prediction uses
only the two fiber sizes. The reference independently eliminates
evaluation columns in total-degree order. The archive comparison
uses the mixed rows of [the Singular run](../run-20261118T0159Z/base_flow.csv)
and [the tracked pins](../run-20261118T0409Z/survivor_summary.csv).

```sh
build_dir=$(mktemp -d)
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic -Werror \
    check_fiber_regularity.cpp -o "$build_dir/check_fiber_regularity"
"$build_dir/check_fiber_regularity" > "$build_dir/fiber_regularity.csv"
diff -u fiber_regularity.csv "$build_dir/fiber_regularity.csv"
python3 plot_fiber_regularity.py
typst compile report.typ
```
