# Quartic full-base line IC on transported public classes [pcc:unverified]

This is a correctness implementation of classical plane-quartic line index calculus on the actual Smith-transported inputs. It is not a new attack or an optimized soft-O(p) implementation.

Let A=Pic0(C)(F61), with G nonzero of prime order ell124459. The genus3 Hasse bound is below (ceil(sqrt61)+1)^6=531441, much smaller than ell². Thus the ell-primary component has order ell and there is a homomorphism log:A->Fell with log(G)=1. Rational point classes need not individually have order ell; their projections under this homomorphism suffice for the linear equations.

Fix the Pic module's rational base point P0. For every rational point P define L_P=log([P-P0]) and introduce the unknown L_K=log([K-4P0]). A rational line with completely rational intersection divisor sum n_i P_i yields

    sum n_i L_Pi - L_K = 0,

including all intersection multiplicities. Also L_P0=0. The solver enumerates all projective lines and retains the fully rational ones; actual full Pic class sums from every accepted line must equal the first accepted line's canonical class. This verifies the line equations before using their projections.

A public walk position R=aG+bT is represented canonically by the Pic module. For a typical class its degree3 divisor D satisfies D~R+D_inf with D_inf=K-P0. If D splits into three distinct rational points, then

    sum L_P - L_K - b*k = a,

where T=kG. Three distinct common zeroes of the canonical conics suffice: the represented effective divisor has degree3. Positions with fewer rational common zeroes are rejected, retaining the search cost. Repeated/extension support is not silently treated as smooth. For an exceptional class R=[P0-P], the equation is instead -L_P-b*k=a; this also covers zero when P=P0.

The target-last sparse elimination solves only k from these actual relations. Free factor-base directions are allowed; no point logarithm table or hidden scalar is supplied. A returned scalar is verified by [k]G=T in the quartic Jacobian, then separately on the original hyperelliptic public input in main.rs. The expected original scalar appears only in the driver, never in ic.rs or its arguments.

Line relations are shared between the two fixtures of each model as explicitly recorded setup. This is correctness reuse, not a single-target amortization claim. The all-lines enumeration is quadratic in p; relation class verification and generic Pic arithmetic are deliberately included for qualification. They will require a new prospective protocol and complete accounting before a speed comparison.

The curve's point-count/model/transport identification depends on earlier stages. The Pic arithmetic constructor checks geometric smoothness, and solver input checks include [ell]G=[ell]T=0 and G nonzero. The two models share one source kernel. Source: Smith https://arxiv.org/abs/0806.2995 section7 for the transport-to-quartic strategy; the explicit line/reference identities above are direct divisor-class deductions. Independent signed review remains open.
