# Trace-projected power images and stable envelopes

2026-11-07, T-1819. Derived algebraic construction, native implementation and deterministic producer checks. No measured total-speed claim or independent signed verdict.

## Prior work and scope

Read the previous stable-envelope and orbit-replay components, fast-moment-roots converter, and CONTINUATION trace/power-projection notes. KB tools unavailable. Primary sources: [FLINT power sums and conversion](https://flintlib.org/doc/nmod_poly/index.html) around lines1074–1105, especially invertibility of1..n and the reciprocal/exponential formulas; [Brent and Kung1978](https://maths-people.anu.edu.au/brent/pd/rpb045.pdf), introduction and section2.1 grouping of polynomial powers. One individual FLINT page failed to open; its index section was read. Polynomial trace reconstruction and algebraic power blocking are established methods. No new generic logarithm search or collision table is introduced. This implementation uses the existing native polynomial backend and fast moment converter, not FLINT.

Let r>=7 be a caller-certified prime in the existing native u64 range, m>=2 divide r-1, and G be monic squarefree of degree n<r. Recovery supplies a split divisor of its existing candidate polynomial. All coefficients and arithmetic stay in F_r; the quotient algebra is not a change to an extension-field ECDLP. Input-degree and output-degree caps are separate. The standalone construction rejects repeated roots and n>=r, and handles G=1 separately. Recovery's initial nonzero sign refinement has at most(r-1)/2 candidates, so n<r holds on its intended path.

## Quotient traces determine the image multiset

Let A=F_r[X]/G and B=X^m mod G. For roots alpha of G, multiplication by B acts diagonally with eigenvalues alpha^m after splitting. Hence

    mu_j=Trace_A(B^j)=sum_(G(alpha)=0) alpha^(mj), j=0..n.

The monic degree-n polynomial C(Y)=product_alpha(Y-alpha^m) follows from these moments through the existing fast converter. Different scalar roots can share the same image, so C generally has repeated factors. Since n<r, all multiplicities are less than the characteristic and

    D(Y)=C(Y)/gcd(C,C')

is the squarefree polynomial of distinct images. Pull back D using the preceding zero-corrected inflation to obtain the minimal stable envelope. This is the same envelope as lcm rotation closure. The candidate set is not enlarged: the envelope is only a modulus for subsequent residue computation. No root enumeration or factorization is needed.

## Trace functional and sequential reference

Let Q(T)=T^n G(1/T), with Q(0)=1. Its logarithmic derivative gives

    -T Q'(T)/Q(T)=sum_(i>=1) s_i T^i,
    s_i=sum_alpha alpha^i, s_0=n.

For a degree-below-n residue f, Trace(f)=sum_i f_i*s_i. The sequential implementation computes B^j modulo G one at a time and applies this functional. It remains in the component as a control. Its projection cost is O(n M(n)+n^2), plus initial B and conversion work.

## Blocked projection

Choose q=ceil(sqrt(n+1)). Store B^v mod G for0<=v<q, prepare B^q, then advance giant residues f_u=B^(uq) mod G. For each giant residue define

    ell_u(i)=Trace(f_u X^i)=sum_(t=0..n-1) f_(u,t) s_(i+t).

Precompute s0..s_(2n-2) by the same logarithmic derivative. All n values ell_u(i) are coefficients n-1+i of the convolution of reversed, n-padded f_u with the power-sum vector. Then

    mu_(uq+v)=sum_i coefficient_i(B^v mod G)*ell_u(i).

This covers mu0..mu_n, truncating the final block. There are O(sqrt n) modular products and ordinary convolutions, and O(n^2) scalar dot-product work. No fast matrix multiplication is claimed. A checked coefficient-word cap bounds the stored short-power table before allocation. Runtime passes cap^2 for this table; this bound is not a bound on every polynomial/backend workspace.

## Costs and interface limits

For multiplication cost M(n), blocked trace projection costs O(sqrt(n) M(n)+n^2), with O(n^(3/2)) short-table storage and O(n) additional coefficient arrays apart from backend workspace. Computing B costs O(M(n)log m); power-sum preparation and fast conversion cost O(M(n)) under the installed multiplication regularity; squarefree gcd/division cost O(M(n)log(n+1)), with scalar inversion mechanisms separately charged. The final inflation costs O(N) output words and assignments, N=m*degree(D)-(m-1)*[D(0)=0]. Thus the smaller candidate degree controls projection work, but the envelope still has to be materialized.

The public adapter retains original-parent validation and final divisibility checks of the reconstructed envelope; their costs depend on the original parent degree and remain additional. These checks cannot be hidden inside an n-only bound. Source setup, query and final-point verification costs are unchanged. Lcm closure, sequential traces and blocked traces remain available. No comparison of total field counts or elapsed time was run.

## Exact evidence

Validation01 passed for sequential projection and is preserved with its exact trace.rs before adding the blocked implementation. Validation02 compares both constructions on all6,522 subsets of the small scalar fields7 and11 with n<r and all eligible orbit widths. It checks image moments, the entire characteristic polynomial with multiplicities, distinct-image polynomial, and equality to lcm envelopes. The independent reference multiplies factors using schoolbook arithmetic.5,842 subsets have colliding images;3,258 contain zero. Six full-degree-r cases are explicitly rejected, rather than treated as zero-count input. Repeated-root, input/output degree and blocked-workspace controls reject.

Three larger cases use r65537 with(n,m)=(8,128),(16,64),(32,32); coefficient/moment equality and G-divisibility of the output are checked. The63 inherited public targets match lcm-envelope recovery in scalar, route and all query/transition fields, with511 source calls. Every recovered scalar passes its public-point equation. Prior controls remain active in the same executable. The original4563 cohort and source domain are unchanged. No cloud, new timing study, full release rerun or full formalization is claimed.

## Next

Use the three construction options to derive a degree/workspace-aware selector and freeze complete operation accounting before measuring a preferred path. Candidate-count n, envelope size N, orbit width m and retained work all matter. A faster modular-composition/power-projection backend could reduce the scalar dot-product term; it is not implemented here. Source work remains separately charged.
