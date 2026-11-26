# Symmetric complete-multigraph source family

The schema-1 symmetric-source-family certificate gives an explicit symbolic
root construction compatible with the source parameter guards. It allocates
no graph: the standalone verifier checks only exact power-of-two exponents.
This is a root packing construction, not recursive closure or an external
contraction proof. `[pcc:unverified]`

## Construction

At depth zero let

```text
M = 2^lambda,
u = floor(lambda/2^17),
a = 2u,
L = 2^a,
q = 64^a = 2^(6a),
N = 2^((lambda-a)/2).
```

Use `N` vertices on each side of a bipartite graph, put exactly `L` parallel
edges between every vertex pair, and label vertices cyclically modulo the `q`
labels. Then `M=N^2 L` and every source degree is `NL`.

For this uniform complete multigraph, the conditional-expectation objective is
symmetric in label buckets. At each row step, candidate cost is an affine
function of the number of earlier vertices in the candidate bucket, so
first-minimum tie breaking cycles through `0,...,q-1`. Once rows are balanced,
the same argument applies to columns. Since `q` divides `N`, every label-pair
class contains exactly

```text
h = (N/q)^2 L
```

edges and `h` is divisible by `L`. `[pcc:unverified]`

## Exact packing and shrink

For an unordered digit pattern with `j` unequal coordinates, packing emits
`h/2^(a-j)` packets. Summing the orientation multiplicity gives

```text
K = (h/L) * sum_patterns 2^j
  = (h/L) * q^2
  = N^2
  = M/L.
```

Thus `KL=M`, stronger than the source `KL<=3M` guard. At every coordinate,
each radix group owns exactly `N/64` vertices and therefore `M/64` endpoints
on each side. A group child has exactly

```text
M_child = 2M/64 = M/32,
```

stronger than the required eightfold shrink. Packet vertices still have degree
at most `2L`. `[pcc:unverified]`

## Source parameter guards

For `M=2^lambda`, the setup degree cap lies between exponents
`floor(9lambda/10)` and `ceil(9lambda/10)`. The certificate checks the
sufficient exponent inequalities

```text
(lambda+a)/2 <= floor(9lambda/10),
1+a <= floor(9lambda/10),
a+12a <= lambda,
1+24a+ceil(9lambda/10) <= lambda.
```

They respectively establish source degree, `2L<=delta`, `M>=LD`, and
`M>=2D^2 delta` for `D=64^(2a)`. At the schema-6 threshold `lambda=2^26`, the
checked transcript is

```text
u                         512
a                       1,024
log2 N             33,553,920
log2 q                  6,144
log2 D                 12,288
log2 h             67,096,576
log2 K             67,107,840
log2 degree        33,554,944
delta exponent  [60,397,977, 60,397,978]
log2 M_child       67,108,859
```

The certificate round-trips deterministically. Altered exponents and a root
size with incompatible vertex parity fail closed. `[pcc:unverified]`

## Remaining closure problem

This construction supplies an infinite parity-compatible sequence of strict
root packing witnesses and makes symmetry and parallel-edge splitting
explicit. It does not show that the packet/old-vertex child graph is another
member of the same family or that the next conditional-label call remains
balanced. Proving such closure, or finding a different invariant family using
packet vertices as auxiliary variables, is the next decisive lead. No
practical Curve25519 attack follows. `[pcc:unverified]`

The finite production probe in `SYMMETRIC_CHILD_CLOSURE.md` supplies new
positive data without closing this gap: all four two-group paths at the first
radix-two `LD` boundary accept the next packing scale, while the smallest
under-scale analogue fails exactly at padding. `CONDITIONAL_LABEL_GRAM.md`
reduces the exact greedy objective to degree-bucket and sparse incidence-Gram
scores, exposing packet orbits as the proof variables. The target is now an
orbit-wise class cap `max_class<=2M/D` plus the existing one-side group-load
and degree caps, uniformly over all source-admitted children.
`[pcc:unverified]`
