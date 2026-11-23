# Third iteration: output-sensitive root isolation on the Dickson tower

Status: [pcc:unverified] mathematical conclusions about specified algorithms and representations; no new speedup, novelty claim, implementation, benchmark, or global impossibility claim. This note follows the prospective complete-component protocol and the scalar-degree obstruction.

## Setting

Let p>3, let N=2^k, and let F(X)=D_N(X)-c split squarefree over F_p into the full fibre Xset of N elements. Let R=(r,s) be a nonzero target on a nonsingular elliptic curve with s != 0. Define

q(X,Y)=S3(X,Y,r),

G(X)=Res_Y(F(Y),q(X,Y))=product over y in Xset of q(X,y).

Let Zset={x in Xset:G(x)=0} and z=|Zset|. The earlier full-fibre lift lemma shows that every such x participates in a rational signed curve relation. The immediate task is to enumerate Zset without materializing all N coefficients or scanning all N fibre elements. The output count z is the number of distinct projected x roots, not the number of signed relation rows.

## 1. Exact node predicate and an output-sensitive branch bound

For 0<=j<=k, put

Aset_j={D_(2^j)(x):x in Xset}.

It has exactly N/2^j elements. For a in Aset_j, the node (j,a) represents

Xset_(j,a)={x:D_(2^j)(x)=a},

a subset of Xset of size 2^j. Nodes form a complete binary tree. The two children of (j,a) are (j-1,b) with b^2=a+2. The split-squarefree hypothesis ensures that both children are distinct and rational. The root is (k,c), and the leaves are (0,x).

Define the exact emptiness predicate

E(j,a) := [gcd(D_(2^j)(X)-a,G(X)) has positive degree].

Equivalently E(j,a) is true precisely when Xset_(j,a) intersects Zset. If an exact oracle supplies this predicate, a depth-first search tests the root, prunes false nodes and descends only through true nodes.

At remaining depth j, the number of true nodes is exactly

s_j=|{D_(2^j)(x):x in Zset}|,

so

s_j <= min(N/2^j,z).

The total number of node tests is exactly 1+2*sum_(j=1)^k s_j when the root is tested once and each true internal node has both children tested. Therefore the number of tests is

- 1 if z=0;
- O(1+z*(1+log_2(N/z))) if z>=1.

[pcc:unverified] This is an output-sensitive combinatorial bound for **all** surviving branches; it needs no randomness or independence assumption. It is not yet an algorithmic speedup, because the cost of answering E is the unresolved part.

Generating the two rational children also has a cost: each true internal node requires a square root of a+2, followed by negation, unless an already charged representation supplies those children. Thus there are sum_(j=1)^k s_j child-generation square-root calls. Charge this work separately from the emptiness decision, or explicitly include it in the node cost; the branch bound does not make child generation free. Write T_sqrt(p) for the field-operation cost of a square root, including any charged setup. For p congruent to 3 modulo 4, including Prime31, exponentiation gives T_sqrt(p)=O(log p).

## 2. Exact norm recursion for node predicates

There is an exact polynomial sequence G_0,G_1,... with

G_0(T)=G(T),

G_j(a)=Res_X(D_(2^j)(X)-a,G(X)).

If G_j(T)=Even_j(T^2)+T*Odd_j(T^2), then

G_(j+1)(T)=Even_j(T+2)^2-(T+2)*Odd_j(T+2)^2.

Indeed the right side equals G_j(b)G_j(-b) for b^2=T+2; the resultant product over the two children gives the assertion inductively. At an admissible node a, E(j,a) is exactly the test G_j(a)=0. There are no extra algebraic-closure roots in this test: every preimage under D_(2^j)(X)=a is already a root of the fully split F.

### The norm-state degree does not shrink

If a nonzero polynomial H has degree d, its transform

Even(T+2)^2-(T+2)*Odd(T+2)^2

also has degree exactly d. For even d the first term has degree d and the second at most d-1. For odd d the second has degree d and the first at most d-1. Their leading coefficients therefore cannot cancel.

[pcc:unverified] Here G is never zero and has exact degree

d=2N if r is not in Xset,

d=2N-1 if r is in Xset.

To see this, q(X,y) has leading coefficient (y-r)^2 and hence degree two for y != r. If y=r, the factor is -4s^2(X-x(2R)), of degree one. The fibre is squarefree, so at most one factor has this drop. Thus **every G_j has degree 2N or 2N-1**, independent of the remaining node depth.

An explicit dense coefficient vector for any G_j therefore has Theta(N) slots, even when z=0 or z=1. This rules out sublinear setup for that representation. It does not lower-bound sparse support or arithmetic-circuit size.

## 3. Standard recursive evaluation still visits all leaves on misses

The alternative to expanding G_j is to evaluate its product recursively. For a node of remaining depth j, compute the two child values and multiply them. With the known short numerical evaluation circuit for G(x), one leaf costs O(k) base-field operations, before any square-root/root-enumeration cost.

For an empty node, both children are empty and neither value is zero. A standard recursive norm evaluator must therefore visit both children. Its leaf-evaluation count satisfies

L(0)=1, L(j)=2L(j-1),

and hence L(j)=2^j.

In particular an empty root requires exactly N leaf evaluations, giving O(Nk) work through this circuit, plus enumerating the fibre. A zero-aware multiplication can stop early after discovering a zero when answering a single yes/no query, but it cannot certify an empty root this way without visiting all leaves. Since z=0 is allowed, this method has no O(polylog N+z*polylog N) bound.

[pcc:unverified] This is a precise obstruction for the ordinary norm-product descent, not a lower bound on other symbolic algorithms for these same circuits.

## 4. Quotient-vector states do not rescue standard descent by themselves

At node (j,a), one can instead evaluate the inner Dickson/S3 norm in the quotient F_p[X]/(D_(2^j)(X)-a). Its dimension is M=2^j. This is a useful exact auxiliary vector state, and its multiplication can be accelerated, but an explicit coefficient state still has M slots once saturated.

[pcc:unverified] Suppose a partition-based search charges at least c*M work to certify an empty node containing M leaves. If the root is empty, the terminal certified-empty nodes form a disjoint cover of all N leaves, regardless of how adaptively the tree was partitioned. The sum of their sizes is N, so the total charge is at least c*N. More generally the empty terminal nodes cover N-z leaves. This **cover-charge lemma** applies even if all successful branches are found cheaply.

Consequently changing a fixed block size or keeping a small number of explicit quotient vectors cannot yield an output-sensitive sublinear search if every certified block still costs proportional to its dimension. It can improve constants or memory use. It becomes a new mechanism only if the representation proves sublinear emptiness certification for growing blocks, or if shared work across blocks/targets has a charged bound that avoids paying this linear cost repeatedly.

For comparison, evaluating every block of size M with the current schoolbook norm state costs O(N*M*(1+log(N/M))) kernel operations. This reaches the leafwise O(N log N) regime at M=1; changing M alone does not supply a sublinear exponent. This is a bound for the stated exhaustive block algorithm, not an optimum over all representations.

## 5. The smallest unmet lemma

The missing lemma can now be stated without conflating branch counts and arithmetic work:

**Target subfibre emptiness lemma.** For a specified growing family of admissible (p,N,c,E,R), construct an exact algorithm for E(j,a), using a specified auxiliary state, such that target-dependent setup plus the total cost of all node queries made by the search satisfies

U_R + sum_(queried nodes) Cost_R(j,a)

  <= O(N^(1-epsilon)*polylog(p)+z*polylog(p,N))

for a fixed epsilon>0, under explicitly stated worst-case or distributional assumptions. Cost_R must include both the emptiness decision and generation of the rational child values when that node is expanded. Include failures, precomputation amortization, state size, and zero-target/diagonal handling. No false negatives are allowed for complete relation enumeration. If probabilistic fingerprints are used, specify their one-sided behavior and the cost of exact verification.

The smallest decisive case is **empty-root certification E(k,c)=false** on unconditioned targets. A proposal that still enumerates N leaves or writes a Theta(N)-coefficient vector merely restates a linear or worse method, even if its surviving-branch count is one.

A small evaluation circuit for G is not this lemma. Nor is a bounded-rank description of an intermediate tensor enough unless contraction, the emptiness decision and the complete extraction cost are all bounded. The scalar-degree theorem permits compact circuits and vector states, so this remains a genuine open avenue within the stated scope.

## 6. Witness extraction is not the missing bottleneck once x roots are isolated

[pcc:unverified] For each returned x, q(x,Y) has degree at most two. Recover its finite-field roots, test membership by k Dickson squarings, and use the exact oriented-line formula from the preceding note. For arbitrary p>3, the per-output bound is O(z*(T_sqrt(p)+log p+log N)) field operations, allowing exponentiation-based inversions, plus charged target setup, output bookkeeping and source-curve verification. For p congruent to 3 modulo 4, including Prime31 and the intended growing 2-adic torus family, exponentiation-based square roots give the specialized bound O(z*(log p+log N)). Handle x=r by its linear factor. Every x contributes at most two candidate y coordinates; normalized relation rows can be deduplicated deterministically.

Thus the next proposed representation should focus on E(j,a), not promise an improvement by omitting sign recovery or membership tests. Factor-base identifiers and any reusable tables must still be charged under the protocol; implicitly recording leaf paths is a possible bookkeeping choice, not a free precomputed table.

## Concrete next step before another benchmark

Specify one exact auxiliary-state representation for the empty-root predicate and prove its update, composition and zero-decision recurrences. Give a recurrence for total arithmetic and memory as a function of j,k, including every branch that survives or must be certified empty. Require a bound that is sublinear in N on z=0 targets after charged setup. If the recurrence reduces to the norm-product recurrence above or to a cover of explicit dimension-M quotient vectors, record that obstruction immediately rather than performing another dense timing run.

[pcc:unverified] No such sublinear predicate has been constructed in this iteration. The mathematical conclusions are the exact output-sensitive branch bound, the persistent degree of scalar norm states, and the linear-work obstruction for standard leaf recursion and explicit block certification. These narrow conclusions identify the missing mathematical operation without claiming that all alternative representations are impossible.
