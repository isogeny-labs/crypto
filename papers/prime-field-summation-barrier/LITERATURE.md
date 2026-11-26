# Data-structure boundary: sources and parameter audit

Status date: 2026-11-26.  This note records primary sources checked for
successor Proposition 19.  The reduction and exponent calculations are new
producer work and remain `[pcc:unverified]`.

## Exact problem correspondence

The standard 3SUM-Indexing problem preprocesses an array and, on challenge
`y`, must return two indices whose entries sum to `y`.  A balanced split of an
elliptic `m`-sum query produces the same problem in the finite abelian group of
curve points, with each partial-sum entry retaining a tuple witness.  This is
an exact reduction to a **group** version of the indexing problem.  Integer
algorithms that rely on integer hashing or a particular universe do not apply
automatically.

## Primary sources checked

1. Itai Dinur and Alexander Golovnev, ICALP 2026 paper on time-space
   tradeoffs for 3SUM-Indexing,
   [DOI 10.4230/LIPIcs.ICALP.2026.78](https://doi.org/10.4230/LIPIcs.ICALP.2026.78).
   The paper defines 3SUM-Indexing as witness recovery, gives the asymmetric
   trade-off, and states for `k`SUM-Indexing
   `S = O~(n^(k-0.5-delta))`, `T = O~(n^delta)`.  It also reports
   `O~(n^2)` preprocessing for its symmetric 3SUM-Indexing construction.
2. Eldon Chung and Kasper Green Larsen, “Stronger 3SUM-Indexing Lower Bounds,”
   SODA 2023,
   [DOI 10.1137/1.9781611977554.ch19](https://doi.org/10.1137/1.9781611977554.ch19).
   The abstract explicitly describes strong unconditional adaptive bounds as
   out of reach beyond the logarithmic-query frontier.
3. Alexander Golovnev, Siyao Guo, Thibaut Horel, Sunoo Park, and Vinod
   Vaikuntanathan, “Data Structures Meet Cryptography: 3SUM with
   Preprocessing,” STOC 2020,
   [arXiv:1907.08355](https://arxiv.org/abs/1907.08355).
   This paper gives a Fiat--Naor-based adaptive upper bound, the
   non-adaptive `S = Omega~(N^(1+1/T))` lower bound, and explains why extending
   it to larger query time would be a major static-data-structure advance.

## Parameter translation

Dinur--Golovnev's `k`SUM-Indexing query recovers `k-1` input entries.  For
`m >= 3`, an `m`-leaf elliptic relation therefore uses `k=m+1`, giving the
favourable comparison

```text
space      S = N^(m + 1/2 - delta)
query      Q = N^delta
targets    B = p / N^(m-1)
```

The last line is the exact average relation-output requirement in the sparse
regime when `Theta(N)` rows are required.  Writing `N=p^alpha`, the storage and
all-query exponents are

```text
(m + 1/2 - delta) alpha
1 - (m - 1 - delta) alpha.
```

Their optimum is

```text
alpha = 1 / (2m - 1/2 - 2delta)
E     = 1/2 + (3/4) / (2m - 1/2 - 2delta) > 1/2.
```

This comparison is deliberately favourable to the attacker: it ignores any
preprocessing work beyond emitting the data structure and assumes the integer
construction transports to elliptic-group inputs.  Restoring those costs
cannot lower the exponent.  The calculation therefore rejects this known
upper-bound family as a sub-rho mechanism; it is not a lower bound for unknown
data structures. `[pcc:unverified]`

## Consequence for the research goal

The remaining theorem is no longer plausibly a consequence of degree or
Macaulay size alone.  It is a static witness-indexing lower bound for a highly
structured finite-group input, with preprocessing time charged.  A realistic
next step must either:

- exploit an elliptic-specific invariant to prove more than generic
  3SUM-Indexing lower-bound technology currently gives; or
- adapt a function-inversion/indexing construction to elliptic partial sums
  and audit whether its preprocessing, witness recovery, and relation output
  beat the exponent calculation above. `[pcc:unverified]`
