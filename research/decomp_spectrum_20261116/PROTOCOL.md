# EXP-DSPEC-001: is the decomposability indicator flat in Fourier space?

Frozen before any data. Owner authorisation: user, 2026-11-16 ("Yes chase this").
Standing of every number this study produces: `[pcc:unverified]` until a pcc claim is filed.

## Question

Index calculus over F_p^* gets a cheap decomposition oracle (sieving) because
"divisible by ell" is periodic along candidates. The elliptic analogue would be
a factor base FB whose decomposable set D = {R : R = P1 + P2, P1, P2 in FB}
has a non-flat indicator along some structured family of targets. Every
arithmetic progression R0 + kG is a family, and all of them are covered by the
Fourier transform of 1_D on Z/N (in discrete-log coordinates). A large
coefficient |hat 1_D(a)| is exactly a progression along which decomposability
is biased.

H0 (predicted): for every x-coordinate-defined factor base, 1_D is spectrally
indistinguishable from the 1_D of a random factor base of the same size.

## Instances

- Fields F_{2^n}, n in {17, 19, 23}; moduli z^17+z^3+1, z^19+z^5+z^2+z+1,
  z^23+z^5+1 (irreducibility checked by the tool: z^(2^n) = z mod f, n prime).
- Curves y^2 + xy = x^3 + a x^2 + b:
  - Koblitz K0 (a=0,b=1) and K1 (a=1,b=1), kept if #E = h*N, h in {2,4}, N prime;
  - random R1..R3: a = 1, b = splitmix64(n*1000 + i) mod 2^n (i = 0,1,...),
    the first three nonzero b with #E = 2N, N prime.
- G: h * (first point found scanning x = 1,2,...), required order N.
- Points are folded under negation: index k in [1, (N-1)/2] stands for +-kG.

## Arms (factor bases, all as sets of folded indices)

- `poly`: x(kG) in span_F2{1, z, ..., z^(d-1)}, d = ceil(n/2) (Diem / Petit-Quisquater / Courtois).
- `frob`: x(kG) in a Frobenius-stable subspace ker((s+1) g(s)), s = squaring,
  g the lexicographically first irreducible factor of (z^n - 1)/(z - 1) over F_2.
  Exists with a mid dimension for n = 17 (dim 9) and n = 23 (dim 12); N/A for n = 19.
- `rsub`: x(kG) in a uniformly random F_2-subspace of dimension d; seeds 1..5.
- `null`: uniformly random set of folded indices, of the same size as the arm it is
  matched to; 20 seeds per (curve, arm).
- Positive control `pint`: FB = {1, ..., B}, B = |FB_poly| (a DL interval).
- Sensitivity control `pmix`: 10% of FB from {1..B/10}... precisely: floor(B/10) interval
  indices plus B - floor(B/10) random indices (seed 1).

## Statistic

D = fold({k1 + k2, k1 - k2 : k1, k2 in FB}) minus {0}; f = 1_D extended to Z/N by f(k) = f(-k).
Z = max_{a in [1, N-1]} |hat f(a)| / sqrt(|D| (1 - |D|/N')), N' = (N-1)/2.
Secondary: yield ratio |D| / mean(|D| over the matched nulls).

## Decision rule

An arm is **non-flat** on a curve if Z_arm > max(Z_null over its 20 seeds) and
Z_arm > 1.5 * median(Z_null). (`rsub`: each seed separately.)

- Detector valid iff `pint` is non-flat on every curve. If not, the run is void.
- Outcome FLAT: no `poly`, `frob` or `rsub` cell is non-flat. Reading: no
  progression-sieve structure for m = 2 at n <= 23; the "number-theoretic oracle"
  thread closes at this scope.
- Outcome SIGNAL: any cell non-flat. Then a fresh-seed replication and n = 29 are
  required before any statement; no claim from this run alone.

## Exact controls

- Koblitz curves with `frob`: FB must be invariant under k -> lambda*k (lambda = the
  Frobenius eigenvalue, read off as the index of x(G)^2). Any violation voids the cell.
- The tool checks [N]G = O and [h]G != O... precisely: ord(G) = N.

## Execution

Default route is isolab-eks. This container has no AWS CLI or identity (checked
2026-11-16), so the run is local, single host, 4 vCPU. The statistic is exact
combinatorics, not timing, so the host does not affect values. Rust (native/), no Python
in computation; Python only plots the recorded JSON.

## Out of scope

m >= 3 decompositions, n > 23, prime fields, cost or rho comparison (this is a
structure test, not a speed claim).
