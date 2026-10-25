# Appendix C: wide-net literature sweep, adjacent and unconventional routes (October 2026)

Compiled by a web-search agent (about 45 searches and 20 fetches) for the gap search in the
parent directory. IACR ePrint PDFs and HAL were blocked from this host (HTTP 403), so for
Galbraith–Gaudry §10 and Howe–Nart–Ritzenthaler's exact criterion the agent relied on abstracts
and flagged recollection. Everything below is *proposed* literature context, not a repository
measurement.

## 0. The structural obstruction

For E/F_q with q = p prime or q = 2^n with n prime, F_q has no proper subfield. Every transfer of
the DLP (cover C→E, isogeny/gluing E×E'→J(C), Weil restriction, trace-zero/torus) lands in an
abelian variety A/F_q of dimension g with #A ≈ q^g and the target subgroup of order ≈ q still
inside it. Known index calculus on dimension-g varieties over F_q costs Ω̃(q^{2−2/g})
(Gaudry/Thériault/GTTD; Diem's plane-quartic Õ(q) for g=3; Diem–Kochinke pencils
Õ(q^{2−2/(g−2)}) for g ≥ 5; Enge–Gaudry L(1/2) only when g ≫ log q). Since rho costs q^{1/2},
any same-field transfer loses unless the field shrinks, and it cannot (Galbraith's notes:
"curves over prime fields avoid Weil descent entirely", https://www.math.auckland.ac.nz/~sgal018/dec.pdf).
The open question (Diem Compositio 2011; Petit–Kosters–Messeng PKC 2016; Amadori–Pintore–Sala
ePrint 2017/609) is whether summation-polynomial index calculus with a non-subfield factor base
can beat q^{1/2} over prime fields; all attempts are ≥ Õ(p). Delaplace–May's representation
technique for F_{p²} reaches p^{1.314}, then p, and p^{4/5} only with p^{6/5} precomputation
(https://eprint.iacr.org/2019/800).

Cover genus bound for extension fields: if C/F_q covers E/F_{q^n} compatibly with the Weil
restriction W (dim n), W is an isogeny quotient of J(C), so g(C) ≥ n (Iijima–Momose–Chao,
https://eprint.iacr.org/2013/487, https://eprint.iacr.org/2015/805). Menezes–Qu: GHS fails for all
E/F_{2^n}, n prime in [160,600] (https://eprint.iacr.org/2001/084). Maurer–Menezes–Teske: GHS
provably fails on Koblitz curves for every n because the descent to F_2 only yields the DL modulo
#E(F_2) (J. Cryptology 19 (2006) 115–133).

## 1. Higher-dimensional isogenies / Kani / gluing into Jacobians

- Robert, "Some applications of higher dimensional isogenies to elliptic curves" (ePrint 2022/1704):
  End(E) in polynomial time, canonical lifts, point counting Õ(n²·polylog p). No DLP application;
  the author notes lifts "do not seem to help attack (commutative) isogeny based cryptography".
  No one (Robert, Castryck–Decru, MMPPW, Oudompheng, Dartois, Leroux, Duparc "Superglue"
  ePrint 2025/736) has connected Kani to ECDLP transfer.
- Gluing theory: Frey–Kani; Hanselman–Schiavone–Sijsling (arXiv 2005.03587); Saengrungkongka–Walsh;
  Robert "Reducible gluing" note; Kani "Curves of genus 2 on abelian surfaces"; Richelot atlas
  (arXiv 2101.00917).
- Which isogeny classes contain Jacobians: Howe–Nart–Ritzenthaler (Ann. Inst. Fourier 2009,
  arXiv math/0607515, complete in dimension 2); Howe–Maisner–Nart–Ritzenthaler (arXiv math/0602650);
  Serre's obstruction in dimension 3 (Ritzenthaler et al., arXiv 0901.2920).
- Assessment: the only live version is "isogeny-based Weil descent" for E/F_{q^n}, n = 3 or 4:
  build an (N,N,N)-isogeny from W = Res E to a principally polarised threefold, test whether it is
  a plane-quartic Jacobian, then run Diem's Õ(q). This is Momose–Chao / Diem–Thomé territory done
  with 2022–26 theta machinery. Irrelevant to secp256k1/P-256/K-163 by §0.

## 2. Transfers to smaller fields, tori, trace-zero, p-adic/Drinfeld analogues

- Trace-zero: Gorla–Massierer (ePrint 2014/318) feasible for n = 3 with q ≈ 30 bits; "no dramatic
  speed-up". Granger–Vercauteren tori apply to F_{q^n}^* only. For E/F_p the trace-zero subvariety of
  E(F_{p^k}) does not contain E(F_p).
- MOV/Frey–Rück: only small embedding degree; no new work.
- Drinfeld analogues: Scanlon 2001, Joux–Narayanan (ePrint 2019/1329): Drinfeld-module DLP is easy
  because the action is F_q-linear; Wesolowski (ePrint 2022/438). The analogy breaks exactly where
  E's group law is non-linear.

## 3. Lifting

- Xedni (Silverman) killed by Jacobson–Koblitz–Silverman–Stein–Teske (DCC 2000): success ≤ C₀/p.
  Huang–Kueh–Tan (ANTS 2000); Cheng–Huang (Algorithmica 2006) reduce ECDLP to partial lifting
  under rank conjectures. No 2015–2026 revival using BKZ/Coppersmith found. Gadiyar–Padma
  arXiv 1702.07107 is a speculative note.
- Smart/anomalous generalisations (Belding; Abdullah–Mahalanobis–Mallick arXiv 2010.15543) all
  require ℓ = p. Structural reason: E(Z/p²) is an extension of E(F_p) by Ê(pZ_p) ≅ F_p^+, so
  ℓ-torsion with ℓ ≠ p lifts isomorphically and the only homomorphic leak is for ℓ = p.

## 4. Sieve analogues, elliptic nets, pairings, modular curves

- Lauter–Stange (ePrint 2008/444): ECDLP to F_q^* DLP only when #E = q−1. Stange (arXiv 0803.0728):
  EDS equivalences, no factor base.
- Huang–Raskind "Global duality, signature calculus and the DLP" (LMS JCM 2009, arXiv 0710.2363):
  lifts E to a number field, uses Tate–Poitou/Brauer-group reciprocity; "computing signatures" is
  randomly-poly-time equivalent to ECDLP (heuristically). Frey "On bilinear structures on divisor
  class groups" (AMBP 2009) is the companion. Never implemented.
- Zhang–Liu "Solving ECDLP via list decoding" (ePrint 2018/795): elliptic AG codes; no complexity
  claim.
- Modular curves / modular polynomials as factor base: nothing exists. Jao–Miller–Venkatesan random
  self-reducibility; "Climbing and descending tall isogeny volcanoes" (2024) gives Õ(q^{1/4})
  isogenies within an ordinary class.

## 5. Symmetry exploitation on Koblitz curves

- Galbraith–Gebregiyorgis (INDOCRYPT 2014); Galbraith–Granger–Merz–Petit (SAC 2020, ePrint
  2020/1315): Frobenius-invariant factor bases, 1/n on solving, n² on linear algebra; generalised to
  any efficient endomorphism (Math. Cryptology; ePrint 2021/721 for hyperelliptic Jacobians).
- FGHR (JoC 2014, ePrint 2012/199); FHJRV "Symmetrized summation polynomials using 2-torsion"
  (EUROCRYPT 2014).
- Upward base extension (E/F_{2^n} → F_{2^{kn}} then GHS down to F_{2^n}): not analysed in print
  because dead by counting; for a Koblitz curve the magic number is 1. Hankerson–Karabina–Menezes:
  binary GLS curves over F_{2^{2ℓ}} resist generalised GHS for prime ℓ ∈ [80,256]∖{127}.
- Tafazolian 2026, "Nakajima-extremal Artin–Schreier covers of ordinary elliptic curves in char 2"
  (arXiv 2609.14830): bielliptic covers of genus 2^h+1 with large automorphism groups; pure
  geometry, same field.

## 6. Algebraic-geometric alternatives

- Diem: plane curves of small degree (genus-3 non-hyperelliptic Õ(q)); Diem–Kochinke pencils
  (genus ≥ 5, conjectured Õ(q^{2−2/(g−2)}), fails for g = 4).
- Hess (EUROCRYPT 2003): generalised GHS; genus always one of two computable values; further GHS
  variations judged unlikely to help. Karabina–Menezes–Pomerance–Shparlinski asymptotics.
- Belyi/dessins, Hurwitz spaces, Chow groups, trigonal/tetragonal covers for DLP: zero hits.

## 7. Computational alternatives (2020–2026)

- WDSat family (Trimoska–Ionica–Dequen 2020–2021); nothing new on SAT-for-PDP after 2021.
- ML/neural Gröbner: Kera et al. (NeurIPS 2024); HATSolver (Malhou–Perret–Lauter, ePrint 2025/2219).
  No application to summation-polynomial systems yet.
- XL/crossbred/tensor/annealing on PDP systems: no publications. Kosters–Yeo (arXiv 1503.08001):
  FFD of Weil-descended S₃ is 2; degree of regularity appears to grow with n; checking S_r = 0 is
  NP-complete for large r. Semaev 2015/310 unrefuted but unsupported.
- GPU/FPGA: only rho. No GPU PDP solver exists.

## 8. Maps of what is open; challenge status

- Galbraith–Gaudry (DCC 78, 2016; ePrint 2015/1022): §10 open problems (from recollection, flagged):
  any index calculus for prime fields; exploiting subfield/Koblitz structure (partly answered by
  GGMP 2020); Gröbner behaviour of Weil-descent systems; asymptotics for binary prime-degree
  fields; better factor bases via symmetries.
- Bernstein–Lange "Non-uniform cracks in the concrete" (ASIACRYPT 2013): P-256 in 2^85 with free
  precomputation. ECC 2025 was a retrospective; no DLP-algorithm talk.
- ECC2K-130: never completed; Certicom 131-bit challenges open. Records: ECC2-113 (Wenger–Wolfger),
  112-bit prime (Bos et al. 2009), 117.35-bit binary FPGA (BELNPSZ, unverified here).
- "Brace for impact: ECDLP challenges for quantum cryptanalysis" (arXiv 2508.14011): secp256k1-shaped
  ladder from 256 down to 6 bits, usable as a shared toy-instance set.

## Crank-vs-credible ledger

- Academia.edu "Complete break of ECDLP on secp256k1 … 11 s": crank.
- Mahalanobis(–Mallick–Abdullah) Las Vegas / initial minors / guess-and-determine (ePrint 2018/134;
  arXiv 2310.04132; arXiv 2607.09814): conjecture-driven, never shown to beat rho; the 2026 paper
  says "no immediate cryptanalytic threat".
- Semaev 2015/310 and Petit–Quisquater 2012: credible authors, heuristic, contradicted experimentally.
- Jnanabha vol. 55 (2025) prime-field paper: low-tier venue, assume no.
- Hu ePrint 2024/1923: low impact, no sizes.
- Zhang–Liu list decoding: credible, no complexity, unimplemented.
- Delaplace–May: credible and honest (worse than rho).
- Quantum estimates (Google 2025 ≤ 1200 logical qubits; ePrint 2026/106): credible, not classical.

## Ranked untested ideas proposed by the sweep (before cross-checking against the repository)

1. Isogeny-based Weil descent for E/F_{q³} via (2,2,2)-gluing and theta reconstruction, then Diem Õ(q).
2. Crossbred / libFES-GPU exhaustive search on Weil-descended PDP Boolean systems, n = 23…41, m = 3.
3. Transformer-Gröbner (Kera / HATSolver) on PDP systems.
4. Full stack on K-163-shaped toys: GGMP + FHJRV + WDSat symmetry breaking + negation.
5. Huang–Raskind signature calculus at toy size.
6. Prime-field summation-polynomial bake-off (Amadori–Pintore–Sala, Kudo–Momonari, PKM, Delaplace–May).
7. Elliptic-net / EDS smoothness experiment.
8. Xedni with BKZ-2.0/flatter at p < 2^16.
9. Upward base extension cost model, measured.
10. Reproduce the Mahalanobis "initial minors" 2^50 claim.
