# Signed two-summand coverage of the registered collector

This is a source-specific mathematical derivation, with finite native checks as sanity checks. It is not an experimental recovery rate, a timing estimate, or signed independent evidence. [pcc:unverified]

## Sources and assumptions

Read `src/cryptanalysis/ecbench/curve25519_pair.rs`, `exact_sampling.rs`, and `src/cryptanalysis/curve25519_wide.rs`. The factor base is independent of Q, contains B nonidentity points distinct modulo sign, and the subgroup has odd prime order r. The collector probes Q first, then aG+bQ with a sampled in [0,r) and b in [1,r). Its only initially admitted logarithm is log_G(G)=1. [pcc:unverified]

The probability statements assume Q uniform over the nonidentity subgroup and the ideal independent-digest model already stated in exact_sampling.rs. They do not apply to deliberately chosen correctness fixtures such as Q=±2G. Deterministic SHA256 executions are not asserted to be literally independent random draws. Hash exhaustion can stop a run; the bound below remains an upper bound on hit probability. [pcc:unverified]

Literature context: Andrew Sutherland, [18.783 Lecture 10, Fall 2023](https://math.mit.edu/classes/18.783/2023/LectureNotes10.pdf), pages 1–3, distinguishes relation discovery from determining the target coordinate by linear algebra. Its finite-field smoothness estimates are not transferred to this elliptic-curve signed-pair collector. The Gaudry 2004/073 primary PDF was inaccessible through the browsing tool and the CiteSeer mirror returned403; no theorem from those inaccessible texts is used here.

## Counting proof

Let S={±P_1,...,±P_B}. There are 2B distinct signed points and hence (2B)(2B+1)/2 = 2B²+B unordered pairs with repetition. The B opposite pairs all sum to zero. Collapsing those B representations to one yields

    |S+S| <= 2B²+1,
    |(S+S) without zero| <= 2B².

Other collisions only reduce these cardinalities. The first Q probe therefore has hit probability at most 2B²/(r-1). For any fixed Q and b, addition by bQ is a permutation of the subgroup, so aG+bQ is uniform when a is uniform; each later successful draw has hit probability at most (2B²+1)/r. Independence between hit events is not needed for the union bound. Fresh uniform a draws are the modeling assumption that supplies each marginal bound. [pcc:unverified]

For at most T>=1 probes, the probability of at least one decomposition is bounded by

    2B²/(r-1) + (T-1)(2B²+1)/r
    <= T(2B²+1)/(r-1).

The initial G relation has zero target coefficient, so it cannot determine the target column. Recovery in this collector therefore requires at least one subsequent hit; requiring enough independent information can only lower the recovery probability. [pcc:unverified]

At the registered maximum B=128 and T=1,000,000:

    T(2B²+1) = 32,769,000,000 < 2^35,
    r-1 = 2^252 + 27742317777372353535851937790883648492 > 2^252,
    probability of any hit < 2^-217.

This upper bound is derived under the stated model and scope. It is not a measured rate. [pcc:unverified]

## Native checks

`check.rs` exhaustively checks297 signed bases of size at most3 in cyclic groups of orders5,7,11,13,17,19. It checks the sumset bounds, all translations a+bQ for nonzero b,Q in those groups, and the exact u128 numerator inequality. The terminal execution returned0. Finite checks do not replace the general counting proof or establish the digest model. [pcc:unverified]

Replay: `rustc --edition=2021 -O check.rs -o /tmp/pair-coverage-check && /tmp/pair-coverage-check` from this directory. No external crate or network is required.

## Research consequence

Keep this method as a checked bounded baseline and its deliberate recovery fixtures as correctness controls. Increasing the trial cap within its existing envelope cannot make a full-size recovery panel informative under the stated model. The next construction needs substantially broader decomposable-point coverage per affordable probe, with no hidden logarithm input and a retained exact relation check. A broader sum representation or a structured decomposition oracle must be evaluated for its actual coverage and decomposition cost before cloud scheduling. This conclusion does not authorize a new experiment, change the caps, or close the overall goal. [pcc:unverified]
