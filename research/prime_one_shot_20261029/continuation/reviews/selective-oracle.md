# Independent review: selective positive-relation oracle reduction

2026-10-30 UTC. Abstract probability and cost review only; no relation solver was constructed or run.

The proposed correction is valid: fast complete UNSAT refutation is a sufficient oracle contract, not a necessary one. A time-bounded, label-free oracle with sufficiently frequent verified positive answers is enough for the direct mixed-target reduction. False negatives and timeouts are harmless to correctness.

## Precise assumptions

Fix an input group of prime order r, P generating it, and Q=kP. A sampled base has B known points in this group. At least a constant q>0 fraction of independently sampled bases are good. On each good base, a fresh uniform R in the group and an independent oracle seed produce a verified relation with probability p_base >= theta. Here theta>0 is a known, computable uniform lower bound; merely knowing that some unknown lower bound is exp(-o(n)) does not specify the block length below. Every call, including verification and bounded output handling, has a computable uniform cap T_total. Calls use independent randomness, and the oracle is memoryless conditional on the fixed base. The base sampler, oracle, acceptance rule and relation selection do not inspect the mixed-target labels.

Require B<=Bmax on every executed block, or reject oversized bases within the charged setup bound. A property of good bases alone does not bound storage and linear algebra on bad bases. In the earlier structured bases, B<=2d is a global bound and supplies this condition directly. A skipped identity target counts as a miss in the stated success probability for uniform R; conditioning on nonidentity targets instead requires adjusting that probability.

## Collection bound and label conditioning

Set K=Bmax+1 and N=ceil(4K/theta). For a fixed good base, imagine all N independent oracle trials even if the actual algorithm stops earlier. Their successful-answer count X is binomial with mean mu=N p_base >=4K. The multiplicative Chernoff bound gives

    Pr[X<K] <= exp(-mu(1-K/mu)^2/2) <= exp(-9K/8).

The last inequality follows because mu(1-K/mu)^2 increases for mu>=K and its value at 4K is 9K/4. Thus the ceiling does not weaken the constant. Collecting only B+1<=K answers can fail no more often than collecting K. No full-rank assumption is needed.

For each trial choose a,b independently uniform in F_r and form R=aP+bQ. The invertible change of variables (a,b) -> (u=a+kb,b) shows that b is uniform independently of R. Conditional on the entire label-free transcript, including failed attempts, successful relation choices, seeds and the stopping index, the accepted b labels remain mutually independent and uniform. This remains true for a selective oracle: its selection is a function of R and its independent randomness, not of a or b.

Let C be the (B+1)-by-B accepted relation matrix over F_r. Select any nonzero left-kernel vector w using C only. Then beta=w dot b is uniform conditional on that transcript, so Pr[beta!=0]=1-1/r. The verified relations imply alpha+k beta=0, where alpha=w dot a. Thus -alpha/beta is the desired logarithm whenever beta!=0; a final scalar check may additionally verify it. The base points must be annihilated by r, as in the whole-prime-order setting, for matrix cancellation modulo r to imply group cancellation.

## Restarts and cost

A fresh independent base/block succeeds with probability at least

    q (1-exp(-9K/8)) (1-1/r).

No independence between base goodness and its success rate is asserted or needed; the argument conditions on each good base. Restarting after a shortage or beta=0 preserves the bound when the next base, labels and seeds are fresh. For r>=2 and K>=1 this is bounded below by a positive constant when q is constant. Bad bases may have zero yield, but their entire blocks must still obey the same finite cost cap.

With dense elimination on at most B+1 rows, the proposed expected bound is therefore correct up to the constant inverse success probability:

    O(base_setup + N(T_total + poly(n)) + Bmax^3 poly(n)).

If verification/output processing is not polynomial in n (for example an unrestricted dense B-entry relation), include that cost inside T_total or charge it explicitly. To infer exp(o(n)) total work, Bmax, charged setup, T_total and theta^{-1} must all have effective exp(o(n)) bounds. This reduction does not establish any of those oracle bounds by itself.

## Positive-only mean-time variant

Let H be the set of covered targets, with Pr_R[H]=rho. Define tau to be the time until a verified relation is returned, taking tau=infinity on a seed that never produces one. If, for each good base,

    E[tau | R in H] <= A,

where the expectation includes all independent oracle seeds, then Markov gives Pr[tau<=2A | H]>=1/2 for A>0. A uniform cap 2A on every target consequently has verified-answer probability at least rho/2. The behavior on targets outside H can be unbounded before imposing that cap. Known computable upper/lower bounds for A and rho suffice to choose the cap and block length.

The quantifier on seeds is essential. A mean computed only over seeds that eventually succeed does not imply this yield: an oracle that succeeds immediately with probability 2^-n and loops otherwise has mean successful-run time 1 but capped yield only rho*2^-n. Such a statement needs the seed-success probability as an additional factor. Likewise, a mean averaged over bases does not automatically give the stated per-good-base contract without another argument extracting a sufficiently large good set.

Accordingly, the existing complete fixed-base oracle condition should be described as one sufficient contract. The remaining algorithmic obligation can also be met by a verified positive-only oracle with effective subexponential capped cost and inverse yield. Neither this reduction nor the coverage theorem currently supplies that algorithm.
