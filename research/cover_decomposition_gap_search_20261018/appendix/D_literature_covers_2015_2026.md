# Appendix D: literature on cover attacks (Weil descent / GHS and descendants), 2015 to October 2026

Compiled by a web-search agent (about 70 searches, ePrint and arXiv listings, author pages).
ePrint PDFs were bot-blocked from this host; Springer/Wiley full texts were paywalled. Where only
an abstract was read, the entry says so. Tags: (a) prime fields; (b) F_{2^n}, n prime, incl.
Koblitz; (c) binary composite degree; (d) general F_{p^n}.

## 1. Chronological list

**2015**
- Iijima, Momose, Chao, "A classification of elliptic curves with respect to the GHS attack in odd characteristic", ePrint 2015/805. Complete list, odd characteristic, of E/F_{q^d} (d ∈ {2,3,5,7}) admitting a (2,…,2)-cover; d ≥ 11 fails (Diem). (d).
- Iijima, Momose, Chao, "Classification of elliptic/hyperelliptic curves with weak coverings under an isogeny condition", ePrint 2013/487 (rev. 2015). 27 classes with g(C) = d·g(C_0). (d).
- Tian, Yu, Li, Wang, "Models of curves from GHS attack in odd characteristic" and "Some elliptic subcovers of genus 3 hyperelliptic curves", ISPEC 2015. Degree-4 minimal elliptic subcovers "which can not be constructed by GHS attack". (d).
- Huang, Kosters, Yeo, "Last fall degree, HFE, and Weil descent attacks on ECDLP", CRYPTO 2015, ePrint 2015/573. Evidence against the first-fall-degree assumption. (b), negative.
- Kosters, Yeo, "Notes on summation polynomials", arXiv 1503.08001. NP-completeness of deciding a zero of S_r; FFD 2 for the S₃ descent. (b).
- Semaev, "New algorithm for the DLP on elliptic curves", ePrint 2015/310. Heuristic 2^{c√(n ln n)}; contested. (b).
- Karabina, "Point decomposition problem in binary elliptic curves", ePrint 2015/319. (b).
- Kousidis, Wiemers, "On the first fall degree of summation polynomials", ePrint 2015/1121, JMC 2019. FFD ≤ m²−m+1. (b).
- Chi-Domínguez, Oliveira, "Attacking a binary GLS elliptic curve with Magma", LATINCRYPT 2015. gGHS-vulnerable GLS curve over F_{2^62}, genus-32 Jacobian solved with Enge–Gaudry; per-curve gGHS test over the isogeny class. (c).
- Galbraith, Gaudry, "Recent progress on the ECDLP", DCC 78 (2016), ePrint 2015/1022. Binary prime-degree curves not threatened; no efficient decomposition for prime fields. (Full text not fetched.)
- Gorla, Massierer, "Index calculus in the trace zero variety", AMC 2015, ePrint 2014/318. n=3 feasible at q ≈ 30 bits; n=5 blocked; "does not affect E(F_p)". (d).

**2016**
- Petit, Kosters, Messeng, "Algebraic approaches for the ECDLP over prime fields", PKC 2016. (a), not competitive with rho.
- Oliveira, López, Cervantes-Vázquez, Rodríguez-Henríquez, "Koblitz curves over quadratic fields", ePrint 2016/603, JoC 2019. Security rests on Hankerson–Karabina–Menezes 2009: binary curves over F_{2^{2ℓ}} resist gGHS for prime ℓ ∈ [80,256] except ℓ = 127. (c).
- Sarkar, Singh, "A new method for decomposition in the Jacobian of small genus hyperelliptic curves", DCC 2017. (d).

**2017**
- Scholl, "Super-isolated elliptic curves and abelian surfaces in cryptography", ePrint 2017/383. Singleton isogeny classes. (a), defensive.
- Amadori, Pintore, Sala, ePrint 2017/609, FFA 2018. (a).
- ePrint 2017/1262 (McGuire–Mueller), Gröbner-free index calculus. (a).

**2018**
- Tian, Li, Wang, Yu, "Cover attacks for elliptic curves with cofactor two", DCC 86 (2018). GHS over cubic extensions needs 4 | #E; for #E = 2·prime they build genus-3 hyperelliptic covers via a degree-3 cover map; two explicit vulnerable curves (149-bit and 256-bit primes). (d).
- Huang, Kosters, Petit, Yeo, Yun, "Quasi-subfield polynomials and the ECDLP", JMC 2020; Euler, Petit, FFA 2021 (no speedup). (d)/(b).
- Kudo, Yokota, Takahashi, Yasuda, "Acceleration of index calculus for solving ECDLP over prime fields and its limitation", CANS 2018. (a).
- Costello, "Computing supersingular isogenies on Kummer surfaces", ASIACRYPT 2018. Explicit Scholten genus-2 C/F_p with Jac(C) ~ Res_{F_{p²}/F_p}(E). Constructive. (d).
- Frey, Shaska, "Curves, Jacobians, and cryptography", arXiv 1807.05270. §8 "Isogenies via correspondences"; open problem on low-degree correspondences between Jacobians.

**2019**
- Fan, Tian, Li, Li, "Constructing hyperelliptic covers for elliptic curves over quadratic extension fields", ACISP 2019. Genus-2 covers of E/F_{q²}; no asymptotic gain alone (genus-2 IC Õ(q) = rho). (d).
- Trimoska, Ionica, Dequen, "A SAT-based approach for index calculus on binary elliptic curves", ePrint 2019/313, AFRICACRYPT 2020. Up to 300× faster than Magma F4; still not sub-generic for prime n. (b).
- Di Tullio, Pal, "A new method for geometric interpretation of ECDLP", arXiv 1909.08901. Speculative. (a).

**2020**
- Galbraith, Granger, Merz, Petit, "On index calculus algorithms for subfield curves", SAC 2020, ePrint 2020/1315. Frobenius-invariant factor bases: 1/n systems, 1/n² linear algebra; rho remains fastest. (b).
- Tian, "Cover attacks for elliptic curves with prime order", arXiv 2012.07173; published as "Cover attacks for elliptic curves over cubic extension fields", J. Cryptology 36(4) (2023). For E/F_{q³}, W = Res E ≅ E×E^σ×E^{σ²}; choose an odd prime ℓ and an F_q-rational maximal isotropic Λ ⊂ W[ℓ] not containing subgroups of the factors; W/Λ is (when 36 of 64 theta-nulls are nonzero) the Jacobian of a non-hyperelliptic genus-3 curve C/F_q, and Diem's Õ(q) index calculus applies against rho's q^{3/2}. Worked example q = 523, ℓ = 3. No density estimate, no timings, no large example. (d).
- Tian, "Translating the DLP on Jacobians of genus 3 hyperelliptic curves with (ℓ,ℓ,ℓ)-isogenies", J. Cryptology 34 (2021). Hyperelliptic → non-hyperelliptic genus 3, q^{4/3} → Õ(q). (d).
- Yokoyama, Yasuda, Takahashi, Kogure, "Complexity bounds on Semaev's naive index calculus method for ECDLP", JMC 2020. Naive small-x index calculus cannot beat rho. (a)/(d).

**2021**
- Chi-Domínguez, Rodríguez-Henríquez, Smith, "Extending the GLS endomorphism to speed up GHS Weil descent using Magma", ePrint 2021/676, FFA 2021. Record: 115-bit DLP on a GLS curve over F_{2^{5·31}}, about 1,035 CPU-days in Magma. Suggests combining GGMP Frobenius factor bases with GLS endomorphisms (not done). (c).
- Tsakou, Ionica, "Index calculus attacks on hyperelliptic Jacobians with effective endomorphisms", ePrint 2021/721. (d).

**2022**
- Fan, Fan, Song, Wang, "Hyperelliptic covers of different degree for elliptic curves", Math. Probl. Eng. 2022. Genus-2 covers of prime-order curves over quadratic extensions; low-reputation venue; genus-2 IC cost equals rho. (d), low confidence.
- Galbraith, "Kani for beginners" (note, 2022). Robert, ePrint 2022/1704: no DLP application.

**2023**
- Oyono, Thériault, "Index calculus algorithm for non-planar curves", FFA 90 (2023). Canonical models of non-hyperelliptic genus-g curves; Diem's exponent with constant-factor gains. (d).
- Filippone, arXiv 2304.14150 (local fields, unreviewed); Abdullah, Mahalanobis, arXiv 2310.04132 (minors, unreviewed). Not cover.
- "Intractability of learning the discrete logarithm with gradient-based methods", ACML 2023. Negative.

**2024**
- Galbraith, "Climbing and descending tall isogeny volcanos", ePrint 2024/924, ANTS 2024. Transfer between ordinary curves with the same order: heuristic Õ(q^{0.4}) worst case; rigorous Õ(q^{1/4}) for random and pairing-friendly classes. This is the "walk to a weak curve" subroutine of Galbraith–Hess–Smart 2002. (a),(b),(d).
- Hu, ePrint 2024/1923. (a), weak.
- Dzierzkowski, arXiv 2410.08725 (quantum annealing). Not cover.
- Kudo, Yokoyama, arXiv 2410.23211 (semi-regular sequences). Tangential.

**2025**
- Galbraith, Gilchrist, Robert, "Improved algorithms for ascending isogeny volcanoes, and applications", ePrint 2025/1243, LATINCRYPT 2025. Removes the heuristic; worst-case class navigation via Kani's lemma. (a),(b),(d).
- Bhati, Singh, "A new perspective on the decomposition in the Jacobian of small genus hyperelliptic curves", SPACE 2025. Weak hyperelliptic curves over extension fields where decomposition goes through the cyclic group of the extension field. (d).

**2026 (to October)**
- Mahalanobis, "A guess and determine attack on the ECDLP", arXiv 2607.09814 (unreviewed). Not cover.
- Lin et al., D-Wave Advantage2, 16-bit instance. Not cover.
- Numerous quantum-resource papers. Not cover.
- No 2025–2026 paper on GHS / Weil descent / covers proper was found; the most recent true cover paper is Tian (JoC 2023).

Pre-2015 backbone: Frey ECC'98; Galbraith–Smart 1999; GHS JoC 2002; Menezes–Qu CT-RSA 2001; Maurer–Menezes–Teske INDOCRYPT 2001; Galbraith–Hess–Smart EUROCRYPT 2002; Hess EUROCRYPT 2003, LMS JCM 2004, "Weil descent attacks" 2005; Diem thesis 2001 and JRMS 2003; Thériault 2003 (Kummer extensions); Menezes–Teske 2004/235; Arita–Matsuo–Nagao–Shimura 2004/240; Momose–Chao 2005/277, 2006/347, 2009/236; Hashizume–Momose–Chao 2008/215; Nagao 2007/112; Hankerson–Karabina–Menezes 2009; Karabina–Menezes–Pomerance–Shparlinski JMC 2010; Diem 2011, 2013; Joux–Vitse EUROCRYPT 2012; Petit–Quisquater 2012.

## 2. Classification results

1. Menezes–Qu (CT-RSA 2001): for E over F_{2^n} the GHS cover has genus 2^{m−1} or 2^{m−1}−1 with m the magic number; for prime n the irreducible factors of Φ_n over F_2 have degree ord_n(2), so m ∈ {1} ∪ {1 + k·ord_n(2)}; for every prime n ∈ [160, 600] the attack is infeasible for all curves. Exception n = 127 (ord_127(2) = 7, m = 8). (Reconstructed from paraphrases; original PDF not retrieved.)
2. Koblitz curves (a, b ∈ F_2) are immune for every n: m = 1 and the descent yields the logarithm only modulo #E(F_2) (Maurer–Menezes–Teske). Smart's page says "Frey and his students extended GHS to Koblitz curves"; no paper found substantiating a non-trivial GHS-type attack on Koblitz curves; treat as unverified.
3. Maurer–Menezes–Teske (ePrint 2001/084): per composite N ∈ [100, 600] the most favourable parameters; X9.62 curves over F_{2^176}, …, F_{2^368} not GHS-weak.
4. Galbraith–Hess–Smart (ePrint 2001/054): isogeny walk to a weak curve; F_{q^7} weaker; more curves over F_{2^155} weak than thought.
5. Hess (2003, 2004): arbitrary Artin–Schreier extensions; "no threat for prime extension degrees in small characteristic or prime fields in large characteristic".
6. Menezes–Teske (2004/235): under gGHS, F_{q^3} partially weak; F_{q^5} weak; F_{q^6}, F_{q^7}, F_{q^8} potentially (partially) weak; F_{2^N} not weak when N not divisible by 3, 5, 6, 7, 8.
7. Hankerson–Karabina–Menezes (2009): binary GLS over F_{2^{2ℓ}}, ℓ prime ∈ [80,256] ∖ {127}, resist gGHS.
8. Karabina–Menezes–Pomerance–Shparlinski (JMC 2010): Weil descent asymptotically ineffective for almost all curves.
9. Odd characteristic (Diem; Iijima–Momose–Chao): only d ∈ {2,3,5,7}; cubic case ≥ 1/2 of Legendre-form curves weak; cofactor 2 (Tian 2018) and prime order (Tian 2020/2023) handled.
10. Isogeny-class defence/offence: Scholl 2017; Galbraith 2024; Galbraith–Gilchrist–Robert 2025.
11. NIST SP 800-186 (2023) deprecates binary curves for usage reasons, not cryptanalytic ones.

## 3. Proposed-but-unworked ideas and explicit open problems

- Frey–Shaska: "It is an open and difficult problem to find interesting correspondences of low degree between Jacobian varieties."
- Tian 2023: which curves admit a suitable F_q-rational maximal isotropic Λ ⊂ W[ℓ]; no density estimate; d = 5, 7; even characteristic.
- Huang–Kosters–Yeo: rigorous last-fall-degree bounds for summation-polynomial Weil descent "open and challenging".
- Joux–Vitse 2012: decomposition on non-hyperelliptic covers "much more complicated"; not computed with available Gröbner implementations.
- Gorla–Massierer: trace-zero n = 5 blocked by Gröbner cost; n ≥ 7 untouched.
- Chi-Domínguez–Rodríguez-Henríquez–Smith: combine GGMP Frobenius-invariant bases with GLS endomorphisms on GHS Jacobians (not done).
- GGMP 2020: whether Frobenius symmetry can give more than polynomial factors for Koblitz curves is open.
- "Diem's families of small-dimension covers" and "Frey's tandem": no papers under those names found.
- Prime-field no-go: any cover via base change lands in dimension ≥ k over F_p where index calculus costs ≥ Õ(p) > p^{1/2}.

## 4. 2023–2026 work combining covers with other tools

- Isogenies / Kani: Tian JoC 2023 is an (ℓ,ℓ,ℓ)-gluing of the Weil restriction into a genus-3 Jacobian; Galbraith 2024 and GGR 2025 use Kani for class navigation. No paper uses (2,2)-gluing of Res_{F_{p²}/F_p}(E) for ECDLP beyond Scholten/Costello-type constructions.
- Lattices / Coppersmith: nothing. Gröbner / XL: Kudo–Yokoyama 2024; Bhati–Singh 2025. ML: only a negative result. SAT: WDSat; nothing newer. GPU: nothing for cover/Jacobian IC.

## 5. Claims of progress on deployed curves via covers, 2020–2026

None credible. Items seen: keyhunters.ru MOV claim (false); LinkedIn "Trying to attack secp256k1" (informal); Mahalanobis 2023/2026 and Filippone 2023 (unreviewed, not cover); Hu 2024/1923 (weak); Semaev 2015 (binary, disputed). ePrint 2026/1916 "256-bit ECDLP in 26 days" is a quantum resource estimate. The only post-2020 binary record is on a composite-degree GLS curve (F_{2^{5·31}}, 2021).

## 6. Queries that returned nothing useful

"Weil descent / GHS / cover attack" + 2022–2026; Castryck–Decru–Smith + Weil restriction; Kani + DLP; Diem families / Diem–Kochinke; Frey tandem; Nagao 2016–2019; Momose/Chao/Iijima 2016–2020; Hess 2015–2025; Vitse 2022–2024; trace zero 2017–2025; F_{p^4}/F_{p^5} covers 2016–2021; ML/transformer ECDLP; lattice/Coppersmith ECDLP IC; GPU hyperelliptic IC 2020–2024; Weil-descent theses 2018–2024; deployed-curve descent claims. Not completed (budget): Japanese SCIS proceedings; FourQ's own Weil-descent discussion; dblp cross-check; Galbraith–Gaudry full text; Menezes–Qu and HKM 2009 full texts.
