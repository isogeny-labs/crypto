# Workspace-adaptive polynomial projection

2026-11-07 · T-1822 · Written derivation and native exact producer checks. [pcc:unverified]

## Construction

Preserve the previous domain: caller-certified prime scalar field F_r, r>=7 within the u64 range, squarefree monic G of degree n<r, m>=2 dividing r-1. G splits over F_r in recovery. The adaptive trace recurrence starts at mu_0=n, uses doubling prefixes L, and accepts C only if C(X^m)=0 modulo G. No scalar labels or root enumeration occur in runtime.

For any positive block width q, the shared polynomial table consists of B^0,...,B^(q-1) modulo G, with B=X^m mod G and giant step H=B^q. Projection and block-Horner certification are valid for every q; square-root width was a work choice, not an algebraic requirement. This successor chooses q subject to q*n<=W, where W is a supplied cap on short-table coefficient words. If W<n, the nonempty constructor returns Limit. If W>=n, width1 is always admissible, so a square-root-sized table is no longer an admission requirement. Empty G=1 needs no table.

## Declared work objective

At a prefix L, any shortest recurrence has degree at most d_max=min(n,L). Define a=ceil(L/q), b=ceil((d_max+1)/q). Short-power construction makes q quotient products, including H. Projection uses a-1 giant-step quotient products and a ordinary convolutions. A certificate of degree d<=d_max uses ceil((d+1)/q)-1 quotient products, at most b-1. Thus

    quotient_product_bound = q + a + b - 2
    convolution_calls      = a
    call_bound             = q + 2a + b - 2.

width_plan enumerates q=1..min(L,floor(W/n)), minimizes call_bound, and chooses the smaller q on ties. It stores the separate quotient-product and convolution quantities. The recurrence degree bound follows because the infinite image polynomial is a valid finite-prefix recurrence of degree at most n, and a finite sequence always has a recurrence of degree at most its length. No incorrect assumption deg C<=L/2 is needed.

This is an upper bound on the count of these two kinds of calls. They do not have identical field costs; convolutions have different operand lengths, quotient products include division, zero coefficients can shorten operands, and scalar pairings/recurrence work are separate. Minimizing this score does not establish a total-work or timing optimum. With M(n) bounding multiplication of O(n)-degree polynomials and corresponding Newton reductions, the score still bounds these stages up to fixed factors. Planning itself uses O(min(L,floor(W/n))) integer steps and no extra polynomial table.

## Work/storage continuum

Let K=floor(W/n)>=1 for the current candidate polynomial, held fixed through its adaptive prefixes. The feasible comparison width q0=min(K,ceil(sqrt L)) yields call_bound=O(sqrt L+L/K). The selected q is no worse in this declared objective. Summing the doubling prefixes, which stop below4t moments for t distinct images, gives O(sqrt t+t/K) such call units, plus O(nt) scalar projection and block combinations. Planning sums to O(t) integer steps.

The conservative core arithmetic bound is

    O(M(n)(log m + sqrt t + t/K) + nt + t^2 + t log r).

This includes initial B, shared trace weights, projection, certificates, recurrence synthesis and explicit Fermat discrepancy inversions. Squarefree validation, output inflation, original-parent checks and source setup/evaluation remain additional. At K=1 the quotient component returns to O(t M(n)); at K on the order of sqrt t it retains the previous O(sqrt t M(n)) bound. K changes when recovery changes G; this formula concerns one constructor invocation.

Short-table occupancy is at most W coefficient words by construction. It is also O(n sqrt t): when sqrt L is feasible the optimal score bounds q by O(sqrt L), and otherwise q<=K<sqrt L. Other arrays use O(n+t), plus backend temporaries; the cap is not a total RAM bound. Dense logical capacity q*n is charged even when individual polynomial vectors are shorter.

## Integration and checks

The public history adapter passes W=cap rather than the predecessor's2*cap^2. On the inherited public fixtures cap=64, so the declared table allowance is64 coefficient words; this is not a measured peak-RAM ratio. Current candidate degree never exceeds cap, ensuring at least one admissible row. All earlier source conditions, schedules, stopping criteria and final point verification are retained. [pcc:unverified]

Native exact checks enumerate n=1..32, prefix lengths1..2n, and every row allowance1..L to check admissibility, objective optimality and tie handling. Separate arithmetic checks over r=7,11,101 and n<=8 compare full projected vectors to direct power sums and certificate remainders to Horner across all prefix lengths and row allowances. Below-one-row caps reject. A named example over F7 with roots1..6 and m=6 previously failed its square-root table admission at W=6; the workspace path succeeds with the same two-moment recurrence Y-1. [pcc:unverified]

The inherited6,522 small adaptive cases and3 larger cases compare image polynomial, envelope, accepted moment vector and rejected-prefix count against the sequential reference twice: generous storage and exactly W=n. The63 public cases preserve scalar, route and all query fields. These are deterministic algebra/software checks; no statistical experiment, timing comparison or cloud work was performed. The original4,563-case cohort is unchanged. [pcc:unverified]

## Sources and scope

[Official FLINT modular-composition documentation](https://flintlib.org/doc/nmod_poly/compose_mod.html) describes polynomial blocking. Its documentation warns some prose is generated; the identities and work counts above are derived from the retained handwritten implementation, with exact reference checks. KB tools were unavailable. There is no novelty claim for block decomposition or a proof of optimal total recovery cost. This is unsigned producer evidence without an independent signed verdict.

Next: account for actual field arithmetic and source cost when choosing sequential, workspace-adaptive, full-image, or lcm construction. A separate source improvement is needed to lower public sign-evaluation costs; this width planner changes the recovery algebra only.
