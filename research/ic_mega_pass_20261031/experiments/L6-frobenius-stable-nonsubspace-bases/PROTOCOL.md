# L6 protocol (pre-registered 2026-11-01, before any run)

Conductor task: T-5736 (held by the swarm per BRIEF §0.6; check not re-run).
Lever 6: non-subspace Frobenius-stable factor bases at prime n with p=2n+1 prime, ord_p(2)=n.
Sources: BRIEF §3 item 6; docs/ic/RESEARCH_IC_NOVEL_DIRECTIONS_20261007.md A2, N1; KN-OPEN-5e3dd0.
The L3 clause of the computed task is not this experimenter's lane; N1..N4 dirs untouched.

## Setup
- Field F_{2^n} = F_2[z]/(f), f an irreducible degree-n factor of Phi_p over F_2; gamma = z has order p.
  Type-II ONB beta_i = gamma^{2^i} + gamma^{-2^i}, i=0..n-1.
- Koblitz curve E: y^2+xy = x^3+x^2+1 (a=1, b=1). gp: n=11: #E=2*991; n=23: #E=2*4196903. h=2.
- x=0 excluded (2-torsion). Self-tests: S3(x1,x2,x3)=(x1x2+x1x3+x2x3)^2+x1x2x3+b verified on real
  point triples; ONB is a basis; gamma^p=1.
- Known structure (cited, then checked): kernel of F_2[Z/p] -> F is a [p, p-n] code; at p=23 it is
  the Golay-type code / at p=47 the QR code. Minimum distance (measured) bounds uniqueness of
  gamma-weight-w representations. At n=11, 1+23+253+1771 = 2^11: weight<=3 tiles F (w=3 is degenerate).

## Bases (x-coordinate sets, x != 0)
- (i) CYC_w: {sum of w distinct gamma^k}. n=11: w=1,2. n=23: w=2 (w=3 at n=23: size ~16k, yields only).
- (ii) HAM_r: ONB weight 1..r. n=11: r=1,2. n=23: r=2,3. Hamming-ideal membership: all degree-(r+1)
  squarefree monomials in ONB coords vanish (C(n,r+1) generators + field eqs).
- (iii) SUB_k: random F_2-subspace (minus 0), k = round(log2 |F|) for each stable base, 5 seeds.
- Report overlaps |CYC_2 ∩ HAM_r|.

## Measurements
- |F|, #x on curve, subgroup-usable count (x with point in <P> of order r; as x-values and as points).
- Columns: for stable bases a column is a Frobenius orbit (relations come in orbits of size n);
  for SUB columns are points/±. Yields reported raw AND per column.
- 2-sum natural relations N2 = #{(P1,P2) ordered in B^2: P1+P2 = ±P3, P3 in B} (B = subgroup points
  with x in F, closed under ±). Null: |B|^3/r.
- 3-sum N3 = #{(P1,P2,P3): P1+P2+P3 in B}, by hash-join of pair sums. Null: |B|^4/r.
- Decomposition: 16 targets R (same seed for every base), R = P1+P2 with P1,P2 in B.
  Ground truth by brute force. Solvers: cryptominisat5 (--threads 1, native XOR, Tseitin ANDs,
  sequential-counter cardinality) and msolve -t 1 (Boolean, field eqs). Timeout 120 s/instance;
  after 2 consecutive timeouts on an encoding at a given n the remaining targets are recorded as
  TIMEOUT-SKIPPED (blocker). Report SAT/UNSAT times separately, msolve max degree from verbose log.
- A2 pass/fail rule (verbatim from A2): Fail if the cyclotomic system's solving degree is not lower,
  or its yield per column is lower (vs Hamming base).
- B*D^2/N with D = brute-force online cost |B| point additions per target, N = r/(2n).

## N1 falsifier (n=23, F_2[Z/47])
- Enumerate all S3(x1,x2,xR)=0 with x1,x2 in CYC_2 (x-values; also count on-curve), 256 targets,
  secondary 4096 targets.
- Lifts: X1,X2 = exponent indicator vectors. Conventions:
  L1: b -> e_0, xR -> ONB-symmetric lift.   (Lemma: augmentation of lifted S3 = 1, so count = 0 forced.)
  L2: b -> e_0 + J (J all-ones; aug 0), xR -> ONB-symmetric lift.
  L3: b -> e_0 + J, xR -> polynomial-basis lift (sum of bits_k e_k, k<n).
  Exactness by direct convolution mod z^47-1; CRT check on random elements.
## Predictions (pre-registered)
- P1: CYC/HAM yields per point within factor 2 of the random-set null; per orbit-column ~n times SUB.
- P2: SAT time: HAM/CYC slower than SUB at matched size; msolve: subspace solves at n=11 at low degree;
  HAM/CYC at n=11 solve with degree >= subspace's; n=23 msolve HAM/CYC time out.
- P3: A2 rule: FAIL (cyclotomic solving degree not lower than Hamming).
- P4: N1: 0 exact lifts under L1 (forced), L2, L3; upper bound 3/N_solutions.
- n=131 / mu_263: not executed.
