# LEAD-ICE-semaev-yield-model-coset-typing

## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-24). Family: ic-engine. Cost: cheap-medium (conservation test).
- Title: Semaev yield model (eq. 11, the m! factor) and coset-typed / trace-parity factor bases.
- Next-step excerpt (truncated in leads.json): "Run the in-regime coset-typed solver-degree conservation test (IDEA-20260918-c05e71): matched (n..."
- Why it might beat sqrt p: m! factor via typed domain; constant factor only.
- Survey summary: coset-typed bases trade m! for a typed domain; eq. (11) overstates decomposition probability; trace-parity confinement explains the low-degree-window yield deficit (EV-SEMBIN-0c8bf4, preliminary); c=1.6986 carried by m! "weakened" (EV-SEMBIN-4614e7). Binary-flavoured.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 39-45, 104-110, 224-230); /home/user/crypto/docs/session-logs/2026-11-07.md (grep of c05e71 only). The IDEA-20260918-c05e71 text and the EV-SEMBIN records were not found or read.

## 2. Hypothesis
Over a prime-field toy (cyclic group of order N ~ 2^12..2^18, m = 3), typing a natural x-coordinate-prefix factor base by a coset-like label (Legendre symbol of x) and restricting to typed tuples (A,A,B) changes only the number of tuples T (the m! trade), not the yield per tuple: distinct covered targets equal the Poisson/conservation prediction N(1-exp(-T/N)) within the null band, for the typed and untyped arms alike. Falsified if either x-prefix arm deviates from a random-base null by more than the band, i.e. coordinate typing on a prime field creates real non-conserved structure.

## 3. Preregistered prediction and falsification criterion
- Statistic: R = (distinct targets covered) / (N(1-exp(-T/N))), with T chosen as the smallest prefix size giving T >= N. Averaged over curves and sizes per arm.
- Null (negative control): random base elements of the same group, random type labels (null_untyped, null_typed pooled) give null mean nm and sd ns. Band = max(3*ns, 0.02). The null mean sits below 1 (finite-size effects, about 0.9); the band is centred on nm, not on 1.
- Positive control: arithmetic-progression base {k*G}; sums collide, so R is far below nm. It must satisfy nm - R_pos > band, else the detector is blind and the verdict is inconclusive.
- A/A arm: an independent replicate of the untyped null (aa_untyped), same instrument; |mean(aa) - mean(null_untyped)| <= band required, else inconclusive.
- Prediction: |R_x_untyped - nm| <= band and |R_x_typed - nm| <= band.
- Outcomes: both inside, controls valid: consistent_with_prediction (typing is a pure constant-factor domain trade at toy scale). Either outside, controls valid: falsified (typed x-structure changes coverage; points at a real yield effect). Controls invalid: inconclusive.

## 4. Method
- For bits in {12,14,16,18}: p = first prime >= 2^bits+1 with p = 3 mod 4. For rep in 0..9, seed 1000*bits+rep (random.Random), draw random short Weierstrass curve, a point G of order N >= p/2 by walking multiples (affine addition). Table x(kG).
- Representatives k in 1..floor(N/2) (one per +-pair). Base orderings: x_* arms sorted by x(kG); null/aa arms random shuffles; pos_ap arm k=1,2,3,...
- Untyped: all 3-subsets, T = C(B,3). Typed: label = Legendre(x) (random label for null_typed); tuples are 2 from class A plus 1 from class B, T = C(a,2)*b. Sum mod N marks a bytearray; count marks.
- Cost: about 4 sizes x 10 reps x 6 arms, roughly N triples each, tens of seconds, a few MB RAM, one core, stdlib only.

## 5. Classification
runnable_local. A stdlib toy decides the conservation statement directly.

## 6. What a run cannot establish
- It tests m=3 coverage on prime-field toys only, not the binary trace-parity confinement behind EV-SEMBIN-0c8bf4, not a solver-degree (Groebner) conservation test, and not the actual IDEA-20260918-c05e71 design, whose text was not read.
- Legendre(x) is a stand-in for a coset type; prime-order groups have no index-2 coset, so this is a labelling, not a subgroup typing.
- Coverage of random targets is a proxy for relation yield; no linear algebra, rho matching, or GAE pricing (AGENTS.md 5a) is involved, so no cost or speed claim follows.
- Toy sizes only; asymptotics are not addressed. Result is [pcc:unverified].
