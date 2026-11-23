# Unit normalizer on every shifted dyadic grid

This follow-up eliminates the unspecified-unit hypothesis from the square
kernel's pointwise valuation theorem. It defines the actual punctured products
on {-L,...,L-1}, L=2^a, and derives their normalized ratio's valuation zero
for EVERY a>=0 and every valid node index. The final theorem then gives
v2(w*w/rho)=2v2(w) for every nonzero rational w. Claim records
PCC-20261106-bd8ff1 and PCC-20261106-82eac6 have standing unverified pending
trusted signed verdicts. [pcc:unverified]

## Definitions and mathematical proof

Set n=2^h-1 and define the actual finite product

    P(n,k)=product_(0<=j<=n, j!=k) (k-j).

In Lean the omitted factor is represented by1. This is not an arbitrary
function assumed to have the desired valuation. `puncture` constructs each
factor, and `shiftedPuncture` constructs the factors after a rational shift.

1. Lucas reduction modulo2 recursively splits the binary digits of n and k.
   All digits of n are1. For k<2^h, each digit factor is choose(1,0) or
   choose(1,1), hence1. Therefore choose(n,k) is odd and has valuation0.
2. The exact factorial identity choose(n,k)*k!*(n-k)!=n! implies
   v2(k!*(n-k)!)=v2(n!). All factors are explicitly proved nonzero before
   applying rational valuation multiplication.
3. The actual product satisfies [pcc:unverified]

       P(n,k)=(-1)^(n-k)*k!*(n-k)!.

   The proof first handles k0 by multiplying the negative factors. Shifting
   both the root and other node indices gives
   P(n+1,k+1)=(k+1)*P(n,k), supplying induction on k. This connects the
   factorial argument to the actual product, not an assumed product formula.
4. Rational translation cancels in every factor. For L=2^a, use height
   h=a+1, n=2L-1, shift L, and center index m=L. Since both root products
   have the same valuation, their quotient rho has valuation0. Its numerator
   and denominator are separately nonzero. This covers the original grid
   {-L,...,L-1}, including negative physical node coordinates k-L.
5. Rational valuation multiplication/division now proves
   v2(w*w/rho)=2v2(w) without an externally supplied unit hypothesis. [pcc:unverified]

These are general finite-product and rational-valuation results, not a finite
lookup certificate. They do not prove a matrix mode-count bound, a chart-
content theorem, a recursive runtime or an ECDLP existence theorem.
[pcc:unverified]

`KernelNodes.lean` additionally defines the original rational kernel itself
as an omitted-root product divided by the actual center product, and defines
its normalized square. It proves the original node values, preservation of
every source node, and the actual kernel's pointwise valuation statement.
The origin theorem is entrywise: for every real row/column slot x,y<2^a,
matrixEntry(a,x,y,0) is1 when x=y and0 otherwise. Claims7288ef,088017,e12f46
await trusted signed verdicts. [pcc:unverified]

## Sources checked

Read before the proof: existing normalized-square source/protocol, pinned
Oct25 D.4.3/D.4.5 construction, the preceding levelwise-binomial Lean source,
Mathlib.Data.Nat.Choose.Lucas and PadicVal.Basic. The identities are established
mathematics applied to this exact kernel; no priority or novelty claim is made.
KB review remains unavailable as previously recorded: no MCP tools, wrapper
status127. Exact local sources were used rather than treating no search as
evidence of absence.

## Verification and frozen scope

Lean4.35.0, CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. The actual source project
builds cleanly, with24 named theorems across CauchyNormalizer and KernelNodes.
The final actual repository gate audits26 declarations, including one generated
instance proof and one generated equation theorem, with0 failing and0
native-trusted. Accepted foundations are propext, Classical.choice and
Quot.sound. No sorry, custom axiom, native evaluator or kernel bypass is
accepted. The initial normalizer-only fresh replay completed exit0 against
the retained successful-007 source snapshot. The final import-inclusive
KernelNodes replay covers the final normalizer module and all kernel-node
theorems and has now completed exit0. See VERIFICATION.md for exact commands
and scope. [pcc:unverified]

Failed build001 (base-case tactic sequencing), build002 (nonexistent narrow
tactic import) and build005 (finite-product rewrite mismatch) remain evidence,
with exact available source snapshots. Successful build006 retained unused-
tactic warnings; build007 removes them and adds the actual shifted-grid
normalizer theorem. A diagnostic sorryAx from an unsuccessful build is not an
accepted proof. No full experimental panel was run during this proof work.

The first node-module attempts exposed two integration issues: imported
definition bodies needed explicit public exposure, and casts of natural
powers needed normalization followed by reflexivity. Those failures and an
early no-olean replay failure are retained. Only the successful nodes-build005
and final source gate support the kernel-node claims. Exposing two definitions
changes public reduction metadata, not the proved product definitions or
their mathematical hypotheses. [pcc:unverified]

The original frozen experiment protocol, instrument and binary are unchanged.
Backend approval remains pending. This proof work is independent of that
approval and uses only local formal correctness checks.

## 2026-11-08 additive product API

The subsequent local-series formalization exposes the existing baseKernel body
and adds baseKernel_formula, its exact finite-product equation. No mathematical
body or frozen native instrument changes. The24-theorem/26-declaration receipts
above remain snapshots of their recorded source commit; the expanded project
is checked in prime_common_scalar_20261108/lean/local-series-gate/. The new
actual chart theorem uses this API instead of assuming a polynomial model.
Producer evidence is unsigned pending trusted PCC standing. [pcc:unverified]
