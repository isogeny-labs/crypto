# L5-fibre protocol (frozen before any measurement) — 2026-11-13

Lead: KN-OPEN-08ba7e, structured prime-field IC with factor base F = rational two-isogeny fibres
(crates/hashed-ecdl-tensor, `tensor-structured`). Question: is there a compact (sub-generic width)
message for the full constrained summation boundary, i.e. can #{(P_1..P_m) in F^m : sum P_i = R}
be counted / a witness found in time sub-linear in B = |F|?

## Definitions (observables)
G = order-r subgroup (r odd prime) of E(F_p), #E = h*r with 2 | h. T=(t,0) rational 2-torsion,
phi: E -> E' the 2-isogeny with kernel <T>; x' = u + 3t + (3t^2+a)/u, u = x - t (crate formula).
Fibre base F_S = {P in G \ {T} : x'(P) in S}, as in `IsogenyFibres::contains` (both y signs, [r]P=O).
Ordered signed tuples with repetition (crate policy). Base is closed under negation.
- W_j (target-independent message width after j slots) = |jF| = #distinct sums of j elements of F.
  Exact: for the point-ordered DD / tensor network with R free, minimal bond at cut j is |jF|
  (rows indexed by distinct partial sums have disjoint column supports).
- w_j(R) (target-pinned minimal width) = |jF ∩ (R - (m-j)F)|, max over cuts, mean over targets.
- Spectral concentration: share of L2 energy of the Fourier transform of 1_F on Z_r carried by the
  top-k characters (k = B, k = sqrt r), a basis-free test for an approximately compact message.
Generic prediction (H0): W_j ≈ E|jA| for a random symmetric A ⊂ G with |A| = |F|, ≈ min(~B^j/(j!·2^{j-1}) , r)
scaled; spectrum flat (top-B characters carry ≈ B/r·(1+o(1)) of energy apart from the trivial one).

## Arms (same B per prime; same instrument)
A1 fibre base, S random set of x' values (size chosen so |F| ≈ B target)
A2 fibre base, S = interval {0..X'} of x' values on E'
N  negative control: uniformly random symmetric subset of G of size |F_S| (3 seeds -> A/A spread)
I  interval base on E: {P in G : x(P) < X} (lab baseline, closure rows A6)
P  positive control: AP-of-logs base {kG : 1 <= |k| <= B/2} — has additive structure, W_j = j·B/2·2 ~ jB;
   detector must report it sub-generic (ratio to N ≤ 0.2 at j=2 for B ≥ 32), else the instrument is broken.
Primes: p of 12,14,16,18,20,22,24 bits (and 26/28 for W_1,W_2 only if budget allows), curves with
#E = 2r or 4r, r prime. m in {2,3} (j = 1..m-1 for target-free; j=1..m-1 pinned).
B grid per prime: B ≈ p^{1/3} and B ≈ p^{1/4} and B ≈ p^{1/5}.

## Cross-check
Counts N_m(R) for m=2,3 at the smallest primes from the crate's `tensor-structured`
(membership circuit and fibre_message) must equal the Python exact counts for every target
(convolution in log space and direct point enumeration). Any mismatch = INSTRUMENT FAIL, stop and report.

## Rule (fixed)
Per arm A1/A2/I at each (p, B, j): ratio rho = W_j(arm)/mean W_j(N). Noise band = max(3·sd_N/mean_N, 0.02).
- SURVIVE (lead): positive control passes AND for some fibre arm at the two largest primes, rho ≤ 0.5
  for j=2 (or spectral top-B energy ≥ 5x N's), with non-increasing trend in p.
- KILL at toy scale: positive control passes AND for both fibre arms at every measured (p,B,j)
  |rho - 1| within noise band or ≤ 0.10, AND theory (task 3) shows the fibre summation is isomorphic
  to an IC instance on E' with an x'-defined base (relabel).
- INCONCLUSIVE: anything else (incl. positive-control failure -> instrument problem).
Reported against matched rho: per-relation cost implied by width W_{m-1} pinning vs p^{1/4} (m=4 need)
and total vs sqrt(r).

## Resources
One core, prlimit 3 GB, timeout ≤ 20 min/command, total ≤ ~2.5 h wall. No cloud.
