# Bidirectional nonsingular character columns

## Construction

Let r be an odd prime, h=(r-1)/2, and S a square-free set of at most three residues. The character feature is represented by F(Z)=product_(a in S)(Z+a)^h, reduced modulo Z^r-Z. Separate a possible zero root as Z^h; write the remaining monic polynomial as g(Z), with g(0) nonzero, degree d, and D=dh. This component changes only column generation; reflection orbits, elimination, target and reconstruction are retained.

If g^h=sum f_n Z^n, differentiating the ordinary polynomial identity gives

    n g_0 f_n = sum_(i=1..min(d,n)) ((h+1)i-n) g_i f_(n-i).

For 1 <= n < r the denominator is nonzero. The initial coefficient is f_0=g_0^h. Generate indices 0 through min(D,r-1) with this recurrence.

Define rev(g)(Z)=Z^d g(1/Z). Its constant coefficient is 1, and

    rev(g)(Z)^h = Z^D g(1/Z)^h.

Consequently its coefficient at j is f_(D-j). If D >= r, generate reciprocal coefficients j=0 through D-r; these give exactly the missing original coefficients D down to r. For d<=3, D-r <= (r-3)/2 < r. Both recurrence sweeps therefore avoid index r. The two intervals are disjoint and cover every coefficient. No Frobenius anchor or anchor convolution is computed. The prefix routine rejects a request reaching index r. [pcc:unverified]

A zero shift is restored after these ordinary-polynomial computations. Reduce exponent e=0 to 0 and positive e to 1+(e-1) mod (r-1), accumulating collisions. This preserves the value at zero as well as every nonzero field element. Differentiation is never applied in the quotient ring. [pcc:unverified]

## Bounds and implementation

For fixed cubic degree the construction remains O(r+log r) field operations after O(r) shared inverse-table setup. The new construction removes the prior singular-index convolution; it does not change the asymptotic bound. Each full coefficient is produced once: min(D+1,r) low coefficients plus max(D-r+1,0) reversed coefficients. The low vector is consumed before the high vector is allocated. At most r coefficients are held in either temporary, alongside the r-entry result and shared inverse table. These are derived array bounds, not resident-memory measurements. [pcc:unverified]

The empty label and singleton zero are handled. Caller certifies primality; r is odd, 3 <= r <= u64::MAX/2, with explicit max_rows. Labels remain sorted, distinct, within F_r and of size at most three. u128 handles products and i128 handles signed recurrence weights. Constructor q and row/column limits are unchanged.

## Exact checks recorded

The optimized overflow-checked suite reports 102 passing tests. It retains the reference tests, all 1,037 full-column comparisons in seven prime fields, 36 full/odd span comparisons, the r479/q15 inconsistency control, all 479 target identities and eight source integrations. The new packet is byte-identical to the saved reflection packet. New tests reject the singular prefix and check empty/zero-root outputs. The column diagnostic counts 109,757 reverse-generated coefficients over the fixed comparison set, not operations or time. [pcc:unverified]

Twenty-two source/data hashes are checked before and after compilation. The existing separately implemented checker is binary-hash checked and freshly accepts the new packet as the exact polynomial X and at all 479 inputs. The prior reflected-partner sign corruption artifact remains in its original component; it is not rerun here. Eight compiler warnings are retained. No comparative timing panel or cloud job ran. The test-log duration is not a speed comparison. These receipts have no signed PCC execution verdict. [pcc:unverified]

The original 4,563-case cohort is preserved. This component uses the existing eight integration fixtures and does not replace that cohort.

## Literature and next construction

Miller polynomial-power recurrence and reversal are standard tools. The FLINT module documentation and its revision notice were read: https://flintlib.org/doc/fmpz_poly/index.html . The specific current function page could not be fetched; the earlier author-hosted page is https://fredrikj.net/fdoc/html/fmpz_poly/_fmpz_poly_pow_multinomial.html . The identity above supplies the derivation; no novelty priority is asserted.

Bostan, Jeannerod, Mouilleron and Schost, *On matrices with displacement structure: generalized operators and faster algorithms*, sections 1 and 6, https://arxiv.org/html/1703.03734 , requires a supplied low-length displacement generator for fast structured system solving. A short per-column recurrence alone does not establish this property for the assembled matrix. No such generator is supplied here.

A concrete next extension follows from the interval argument: degree four gives D=4h=2r-2 and reverse prefix length D-r+1=r-1, still entirely nonsingular. Implement quartic feature synthesis and test whether the expanded odd span permits fewer source shifts. This implementation deliberately retains the cubic label API; the quartic extension is a mathematical consequence to implement next, not a completed quartic recovery. For degree five and r>=5 the two nonsingular endpoint windows leave (r-3)/2 interior coefficients uncovered, so blindly increasing the label bound is insufficient. [pcc:unverified]
