# Residue and character leads on Q = kG — protocol v1 (frozen before data)

Study: `research/residue_leads_20261120`. PR #4850. Author runtime: Claude, 2026-11-20.
Machine-readable record: `protocol.json` (`experiment.protocol/v1`).
Instrument: `native/` (standalone Rust crate, no dependencies). No clocks are used anywhere. Every metric is
an exact count, or a statistic computed from exact counts.

## Question

Can a residue- or character-type invariant of a point help solve Q = kG? The question splits into four leads:

- **R1:** recover information about k directly, through a homomorphic residue;
- **R2:** recover information about k through a non-homomorphic residue predicate;
- **R3:** speed up relation finding by pre-filtering Semaev decompositions;
- **R4:** shorten rho by walking on larger equivalence classes.

## Sources checked

The research-kb wrapper is unavailable: `scripts/research-kb status` reports "no executable found". The
`research-kb` MCP server is not loaded. This is recorded as **KB unavailable**. The fallback is `rg` over the
repository (see `SOURCES.md`, written before the code runs).

## Claims and their classes

| id | class | text |
|:--|:--|:--|
| R1-thm | theorem | A homomorphism from ⟨G⟩ (prime order n) to a finite group A with gcd(n,\|A\|)=1 is trivial. |
| R1-bin | exact computation | On each binary curve in family B, Tr(x(P)) = Tr(a) for every P ∈ ⟨G⟩, and P ∈ 2E ⇔ Tr(x(P)) = Tr(a) on all of E (x ≠ 0). |
| R1-2desc | exact computation | On each prime curve in family P4 (full rational 2-torsion), the 2-descent map δ is constant on ⟨G⟩ and non-constant on E. |
| R2-spec | comparative | For each predicate f, compare the largest group-character coefficient M_f = max_{a≠0} \|Σ_k f(kG) e(ak/n)\| against the random ±1 function null. |
| R2-res | comparative | Test the association of f(kG) with k mod m (m ∈ {2,3,4,5,7,8,16}) against the χ² null. |
| R3-prime | comparative | Prime fields: does a predicate f(R) shift P(R decomposes as ±P1 ± P2, P_i ∈ F_B) beyond a random predicate of the same balance? |
| R3-bin | comparative | Binary fields, subspace factor base: is any of the 2^m linear functionals of x(R) correlated with decomposability beyond a random set of equal size? |
| R4-thm | theorem | Let α ∈ End(E) have norm d ≥ 2, and let s be the order of its eigenvalue on ⟨G⟩. Then n \| N(α^s − 1) ≤ (d^{s/2}+1)², so s ≥ 2·log(√n − 1)/log d. |
| R4-exact | exact computation | Check the R4-thm bound against exact eigenvalue orders on toy CM curves (D = −7, norm 2; and the controls j = 0 and j = 1728). |
| R4-cost | heuristic estimate | Net rho factor from s-element classes with per-element cost c ≥ c_min is √s/(1 + s·c). |

## Instances (deterministic; selection uses no measured statistic)

- **P (prime, cofactor 1).** For b ∈ {16, 18, 20}, p is the smallest prime p ≥ 2^b + 1 with p ≡ 3 (mod 4). Curves
  are y² = x³ + Ax + B for (A, B) = (1, 1), (1, 2), …, in lexicographic order with B running fastest over
  1..p−1 and A over 1..p−1. The first two curves of prime order n = #E(F_p) are taken. G is the point with the
  smallest x for which a square root exists, with the root y ≤ (p−1)/2.
- **P4 (prime, full 2-torsion, #E = 4n with n prime).** Use the same p values. Curves are
  y² = (x−e1)(x−e2)(x−e3) with e1 = 0, e2 = 1, e3 = 2, 3, …. Take the first curve with #E = 4n, n prime and
  odd, and n > 4. G = 4·P0, where P0 is the first point by x.
- **B (binary).** Use Koblitz curves y² + xy = x³ + a x² + 1 over GF(2^m) for m ∈ {11, 13, 17, 19}. For each m,
  keep a = 0 when #E/4 is prime and a = 1 when #E/2 is prime. Also use random-b curves
  y² + xy = x³ + x² + b, with b the smallest b ≥ 2 (as an integer encoding) giving #E = 2n, n prime. The field
  is GF(2)[t]/(f), where f is the lexicographically smallest irreducible trinomial of degree m, or the smallest
  pentanomial if no trinomial exists. G is h·P0, where P0 is the first point by x.
- **R4 CM family.** For D = −7: primes p ≡ 1, 2, 4 (mod 7) in [2^16, 2^20]. j = −3375. Take the first 20 twists
  with prime order. Controls: j = 0 (p ≡ 1 mod 3) and j = 1728 (p ≡ 1 mod 4), 20 each.

## Predicates

- **Prime.** χ(x), χ(y), χ(x+1), lsb(x), lsb(y), and bit ⌊b/2⌋ of x.
- **Binary.** Tr(t·x), Tr(y), Tr(y/x) for x ≠ 0, x0 (the constant coefficient), y0, and the WHT family in R3.

Each predicate is mapped to ±1, with f = +1 at the point at infinity.

## Controls

- **Positive controls**, run through the same code path:
  - R1: the full group E. Expected: the halving and 2-descent maps are non-constant.
  - R2: a planted predicate f_pc(kG) = sign(cos(2π·a0·k/n)) with each value flipped with probability 0.5 − ε,
    where ε is chosen so that E\|S(a0)\| ≈ 8·√(n ln n). Expected: p < 1e−12. A second planted predicate,
    [k mod 3 = 0] with the same flip rule, gets an ε that gives an expected χ² of 100.
  - R3: f_pc = 1[R∈D] flipped with probability 0.4. Expected lift > 3. In the binary case, a planted set
    {R : x(R) ∈ W} for a random hyperplane-pair W, with expected max-WHT z > 30.
- **Negative (null) controls:**
  - R2: analytic. For a random ±1 function on n points, S(a) for a ≠ 0 is approximately complex Gaussian with
    E\|S\|² = n, giving P(M ≤ t) = (1 − e^{−t²/n})^{(n−1)/2}. k mod m uses χ²_{m−1}.
  - R3: a two-proportion z-test under the hypergeometric null, and Gaussian-max for the WHT over 2^m − 2
    functionals.
- **Analytic-null validation (A/A).** K = 64 simulated random ±1 functions and random sets per curve, run
  through the same code (SplitMix64 seeded from the protocol hash). The analytic p-values of the simulated
  replicates must be roughly uniform. A KS statistic above 0.25 across the 64 flags the analytic null as invalid
  for that instance, and its p-values are then not used.

## Decision rules

The tests form one family per lead, with Bonferroni correction at α = 0.01 over the number of tests in that lead.

- **R2.** A predicate is "*correlated with k*" if p < α/m_R2. It is "*more uniform than random*" if
  P(M ≤ M_f) < α/m_R2 (lower tail). Preregistered prediction: algebraic characters χ(x), χ(y), χ(x+1) and
  Tr(t·x) are more uniform than random for n ≥ 2^16, because the Weil-type bound O(√q) lies below
  √(n ln n). Bit predicates (lsb, bit b/2, x0, y0) are within the null. None is correlated with k.
- **R3.** A predicate is a "*useful pre-filter*" if p < α/m_R3 **and** lift ≥ 1.10. Prediction: none is.
- **R1 and R4 exact.** A single counterexample falsifies the claim.

## What does not count

Changing an instance list, a predicate, a threshold or a null after seeing data. Any such change is filed as a
labelled amendment in `AMENDMENTS.md`, stating whether it preceded the data it affects.

## Execution

Local run in this container (4 vCPU). The default isolab-eks route is unavailable because this session has no
AWS identity (`aws` is not installed). These are exact counts with no timings, so host noise cannot move a
metric. The S3 resume checkpoint is attempted and its outcome logged. There is no benchmark record because
there is no timing.
