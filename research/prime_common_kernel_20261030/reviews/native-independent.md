# Independent exact-control review

2026-10-30 UTC. This is a review of bounded native development correctness
controls, not a timing experiment, fitted complexity result, or ECDLP run.
No root source files were edited by this reviewer.

## Inputs and executions

Read the prospective `PROTOCOL.md`, complete `exact_controls.cpp`, and
`check.sh` in
`/workspace/crypto/research/prime_common_kernel_20261030/native/` before
compiling. The C++ source SHA256 is

    2e003144089f906e82f4b9709c3971aaf307c07e1b2ae56a3d3fe86110869b93

Compiler: g++ (Debian 14.2.0-19) 14.2.0. GMP and GMPXX: 6.3.0.
Compilation used C++20, -O2, -Wall, -Wextra, -Wpedantic, -Werror and exact
GMP rational arithmetic. Both independent strict compilations and both
executions completed with exit zero. The source's assertions remained
enabled. Successful compiler output was empty.

First replay:
`/workspace/audit/common-kernel-independent-native/`.
At that time `expected.tsv` was absent and the helper conditionally skipped
its diff. Therefore this first execution was checked against the written
prospective protocol, not automatically against a frozen expected file.
The first helper SHA256 was
`ac6938e0ebd4c9ef310178ee0305262d582a8ac158684a7e6a387b89782c9042`.

The parent then added `expected.tsv` and made the helper diff unconditional.
The final replay is preserved in
`/workspace/audit/common-kernel-independent-native/final-replay/`, with the
complete output and exit status in its parent directory. Compilation,
executable, and mandatory expected-output diff all passed. The final helper
SHA256 is
`fa91506eb3e7ab6b657c631dbf8bc6b26a3691abf0443a583cd0266d229ccfd9`.
The C++ and protocol hashes were unchanged between the two replays.

Both raw output files and the final frozen expected file have SHA256

    40aafe57e9a46510703c4375a7bc553a6291b54237850e82c68090740f44996f

The binary SHA256 of the first replay is
`9d891f614ffff7d3846abd51f935b7b869837f1b06bba16e30361649c5a8b923`.
Source pin files and compiler version are retained with the replay.

An auxiliary shell read after the successful final replay returned exit one
because a separate agent's forthcoming review file did not yet exist. This
did not affect the compiled control or expected-output comparison; the
separately recorded final replay exit status is zero. No failed native
compilation or assertion occurred in this independent review.

## Mathematical conventions checked directly

`boolean_kernel` implements exactly

    K(W)[Y,X]=product_i l_(Y_i)((W X)_i), l_0(z)=1-z,l_1(z)=z.

All row and column assignments are enumerated, with the same least-significant
bit convention throughout. Thus this is the source's Boolean multilinear
extension and not an exterior or quantum operation. The displayed two-by-two
matrices give composition entry four and direct product entry six at (11,11).
Their difference is nonzero modulo four and zero modulo two. The native
test checks the exact integers; the modular statements follow from them.

`exterior_entry` selects row and column indices in increasing mode order,
returns zero for unequal cardinalities, and uses the signed determinant for
equal cardinalities. Empty minors evaluate to one. Exact elimination
accounts for row swaps. It need not overwrite entries below an eliminated
pivot because only the trailing submatrix is subsequently used. Rational
division is valid here; this is not a purported division algorithm in a
truncated 2-adic ring.

Composition is tested in the order Gamma(A)Gamma(B)=Gamma(AB). The monomial
Boolean positive control has exactly one nonzero entry per row and per
column. Its left Boolean kernel is multiaffine in the intermediate variables,
which explains why the positive-control composition law is appropriate even
with the displayed weights two and minus one.

For the passive-axis witness, `cauchy_chart` multiplies the actual seven
linear factors of the length-four chart after omitting the designated
factor, and divides by C'(0). It gives

    [z] W_2(1+8z)=40/3,    W_1(1)=-5/3,

so their product is -200/9. The row slot (0,0) and column slot (2,1) satisfy
the stated global-even-parity support. This is a nonzero one-axis jet
witness. Its valuation is three, not one; neither protocol nor output
claims otherwise. One scalar witness does not certify a general crossing
rank or refute every code.

The swap matrix exchanges each adjacent pair in the fixed order
`a1,b1,...,an,bn`. Inputs and outputs are constructed from a,b explicitly.
The signed minor equals (-1)^popcount(a&b). The constructed Walsh matrix
is symmetric, so the exact `product(h,h)` is H H^T. The identity
H H^T=2^n I certifies rational full rank. It makes no uniform rank claim
over Z/(2^p), and the adjacent swaps are not asserted to be source Cauchy
translations.

The intermediate-hole control squares the full two-mode swap and observes
the one-particle entry one. Separate restriction to the first mode has
one-by-one matrix zero (visible directly in the displayed swap), hence
composition zero. The restricted zero matrix is spelled out as a fixed
known answer rather than dynamically extracted; the source inspection
confirms it is the intended restriction.

Rank-one updates use the fixed vectors u=(1,2,3,4) and v=(-2,-1,0,1).
The equal-cardinality minors are checked at z=0,1,2,3 for affine dependence.
These are finite-instance controls for the written determinant
multilinearity argument, not a standalone proof for all matrices or a
complexity improvement.

## Counts recomputed from loop ranges

* Exterior composition: 12*(4+16+64+256)=4080 entries.
* Monomial Boolean positive control: the same 4080 entries.
* Adjacent swaps: 4+16+64+256+1024=1364 selected entries for n=1,...,5.
  Each Walsh orthogonality certificate also checks all its 4^n entries.
* Equal-cardinality minors of a four-by-four matrix:
  sum_k binom(4,k)^2=binom(8,4)=70; eight fixed bases give 560 minors.
  Two independent sample-difference assertions are made for each minor.

Every reported count and exact value matches the prospective protocol.
The sample matrices and update directions are deterministic, finite fixtures;
the test does not exhaust all matrices of those dimensions. No measured
runtime or scaling assertion follows from the pass.

## Verdict

PASS for the stated finite development controls and their exact conventions.
The root mathematical review remains responsible for general identities,
modular precision, actual coded-kernel density and recursive costs. The
native pass supplies no new ECDLP algorithm and formalizes no theorem.
