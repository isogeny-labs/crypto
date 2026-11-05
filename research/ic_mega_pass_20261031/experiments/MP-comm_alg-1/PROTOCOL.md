# MP-comm_alg-1 protocol (pre-registered 2026-10-31, written before any run)

Class: theory/measurement. No B*D^2 change. novelty_unverified.
Known (not claimed): bilinear normal form = falldegree.tex Thm reform; k+2 bound = Thm upper (FSS 2011 (1,1) bound);
Frobenius collapse = Kosters-Yeo / THM_SEMAEV_FALL1 Lemma 1.

System. K=F_{2^n} (irreducible low-weight modulus), V=<v_1..v_k> random F_2-subspace, k=ceil(n/2) (plus off-diagonal k cells),
z random nonzero, a6 random nonzero. Descended S_3 (m=2):
 S = sum_{j,l} a_j b_l * L(v_j v_l) + sum_j a_j z^2 v_j^2 + sum_l b_l z^2 v_l^2 + a6,  L(x)=x^2+zx (F_2-linear).
n Boolean equations = coordinates of S. Bilinear tensor T in F_2^{k x k x n}; affine part + constant kept fixed across arms.
Code check: equations compared to ffd_semaev_core.build_system at n=7,9,10.

Arms (same affine part and constant per draw; only T changes):
 A   T = L o mu_V (true Semaev).
 A'  A after random invertible L1 (a-vars), L2 (b-vars), L3 (equations). Code-validity control: must equal A exactly.
 B   dense uniformly random T (null / generic reference).
 C1  L o mu_W, W another random k-subspace (Galois-stable, K-bar-rank n).
 C2  Lr o mu(W1 x W2), W1!=W2 random, Lr random invertible F_2-map of K (twisted multiplication tensor).
 D_R random F_2-rank-R tensor, R in {n,2n,4n} (independent comparison only; not a model of mu_V).

Observables (counted, exact). Boolean closure W_d (treatise def): span of F, closed under multiplication by variables
whenever the product has degree <= d, iterated to a fixed point. Report dim W_d, low_d = dim(W_d ∩ B_{<=d-1}),
1 in W_d. d=3 for all n in {9,11,...,45}; d=4 for n<=TBD by cost (n<=19 target).
Excess e_d(n) = low_d(arm) - mean low_d(B).
Seeds: draw s at size n uses random.Random(1000*n+s), s=0..19 (fewer at large n only if runtime forces it; reported).

Decision rule. Arms X,Y "differ" at n if |mean_X - mean_Y| > 5 * sqrt(var_X/r + var_Y/r) (and any nonzero
difference when both variances are 0).
prediction_if_true (tensor-structure hypothesis): A differs from B (A shows extra low-degree falls, e.g. low_3 = 3n
vs smaller for B); C1,C2 match A while differing from B (Galois-stable K-bar-rank-n structure is the cause);
D_R does not match A (F_2-rank is not the controlling quantity).
prediction_if_false: A ~ B at every n (tensor structure irrelevant to the closure profile), or C ~ B while A != B
(cause is something specific to mu_V/curve, pointing to MP-comm_alg-3), or A' != A (code bug -> not_run).
Only n=131-relevant output: linear fit in n of e_d(n) with 95% CI. d_F itself is NOT measured beyond d<=4.
Budget: < 20 CPU-min total.
