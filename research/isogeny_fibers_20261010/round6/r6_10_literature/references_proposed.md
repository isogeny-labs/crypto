# Proposed replacement for §18 (References) of RESEARCH_ISOGENY_FIBERS_ECDLP.md

This is a proposal from R6.10 (2026-10-18). The note itself is not edited here. Each entry has a
tag with its status (see `RESULTS.md` for the evidence):

* **[c]** confirmed against a fetched source;
* **[c2]** confirmed against a fetched secondary source;
* **[b]** bibliographic data fetched (Crossref or publisher), but content not checked;
* **[u]** unverified (nothing fetched).

LNCS volume numbers appear only where a fetched page printed them. The DOIs come from Crossref
records in `raw/crossref_hits.jsonl`.

## Summation polynomials, decomposition, symmetries

* I. Semaev. Summation polynomials and the discrete logarithm problem on elliptic curves. IACR ePrint 2004/031. [c]
* P. Gaudry. Index calculus for abelian varieties of small dimension and the elliptic curve discrete logarithm problem. J. Symbolic Comput. 44(12) (2009) 1690–1702. doi:10.1016/j.jsc.2008.08.005 [b]
* C. Diem. On the discrete logarithm problem in elliptic curves. Compositio Math. 147 (2011) 75–104. doi:10.1112/S0010437X10005075 [b]
* J.-C. Faugère, P. Gaudry, L. Huot, G. Renault. Using symmetries in the index calculus for elliptic curves discrete logarithm. J. Cryptology 27 (2014) 595–635. doi:10.1007/s00145-013-9158-5 [b] *(new)*
* J.-C. Faugère, L. Huot, A. Joux, G. Renault, V. Vitse. Symmetrized summation polynomials: using small order torsion points to speed up elliptic curve index calculus. EUROCRYPT 2014, LNCS, pp. 40–57. doi:10.1007/978-3-642-55220-5_3 [c]. §3.2 contains the m^{n−1} decomposition loss for translation-invariant bases, and its compensation by easier systems. Cite it wherever the saturation law or its oracle dependence is stated.
* S. D. Galbraith, S. W. Gebregiyorgis. Summation polynomial algorithms for elliptic curves in characteristic two. INDOCRYPT 2014, LNCS, pp. 409–427. doi:10.1007/978-3-319-13039-2_24; ePrint 2014/806. [c]
* Y.-J. Huang, C. Petit, N. Shinohara, T. Takagi. Improvement of Faugère et al.'s method to solve ECDLP. IWSEC 2013, LNCS, pp. 115–132, doi:10.1007/978-3-642-41383-4_8. Journal version: Improvement of FPPR method to solve ECDLP, Pacific J. Math. Ind. 7 (2015), doi:10.1186/s40736-015-0012-6. [c]
* E. Gorla, M. Massierer. Index calculus in the trace zero variety. Adv. Math. Commun. 9(4) (2015) 515–539. doi:10.3934/amc.2015.9.515 [b]
* S. D. Galbraith, P. Gaudry. Recent progress on the elliptic curve discrete logarithm problem. Des. Codes Cryptogr. 78 (2016) 51–72. doi:10.1007/s10623-015-0146-7 [b]

## Isogenies, classes, volcanoes

* D. Jao, S. D. Miller, R. Venkatesan. Do all elliptic curves of the same order have the same difficulty of discrete log? ASIACRYPT 2005, LNCS, pp. 21–40, doi:10.1007/11593447_2. Preprint: ePrint 2004/312, titled "Ramanujan graphs and the random reducibility of discrete log on isogenous elliptic curves". Journal version: Expander graphs based on GRH with an application to elliptic curve cryptography, J. Number Theory 129 (2009) 1491–1504, doi:10.1016/j.jnt.2008.11.006. [c]
* S. D. Galbraith. Constructing isogenies between elliptic curves over finite fields. LMS J. Comput. Math. 2 (1999) 118–138. doi:10.1112/S1461157000000097 [c]
* D. Kohel. Endomorphism rings of elliptic curves over finite fields. PhD thesis, University of California, Berkeley, 1996. [c]
* J. Miret, R. Moreno, D. Sadornil, J. Tena, M. Valls. An algorithm to compute volcanoes of 2-isogenies of elliptic curves over finite fields. Appl. Math. Comput. 176 (2006) 739–750. doi:10.1016/j.amc.2005.10.020 [b]
* E. Teske. An elliptic curve trapdoor system. J. Cryptology 19 (2006) 115–133. doi:10.1007/s00145-004-0328-3 [b]
* J. Vélu. Isogénies entre courbes elliptiques. C. R. Acad. Sci. Paris Sér. A 273 (1971). [u]
* D. J. Bernstein, L. De Feo, A. Leroux, B. Smith. Faster computation of isogenies of large prime degree. ANTS XIV, Open Book Series 4 (2020) 39–55. doi:10.2140/obs.2020.4.39 [c]
* E. Kani. The number of curves of genus two with elliptic differentials. J. reine angew. Math. 485 (1997) 93–122. doi:10.1515/crll.1997.485.93 [b] *(new, §13.4)*
* W. Castryck, T. Decru. An efficient key recovery attack on SIDH. EUROCRYPT 2023, LNCS, pp. 423–447. doi:10.1007/978-3-031-30589-4_15 [b] *(new, §13.4)*

## Weil descent and GHS

* P. Gaudry, F. Hess, N. P. Smart. Constructive and destructive facets of Weil descent on elliptic curves. J. Cryptology 15 (2002) 19–46. doi:10.1007/s00145-001-0011-x [c] *(new in §18; cited in §15.4 and §17.7)*
* S. D. Galbraith, F. Hess, N. P. Smart. Extending the GHS Weil descent attack. EUROCRYPT 2002, LNCS 2332, pp. 29–44. doi:10.1007/3-540-46035-7_3; ePrint 2001/054. [c]
* A. Menezes, M. Qu. Analysis of the Weil descent attack of Gaudry, Hess and Smart. CT-RSA 2001, LNCS 2020, pp. 308–318. doi:10.1007/3-540-45353-9_23 [b] *(new)*
* M. Maurer, A. Menezes, E. Teske. Analysis of the GHS Weil descent attack on the ECDLP over characteristic two finite fields of composite degree. LMS J. Comput. Math. 5 (2002) 127–174. doi:10.1112/S1461157000000723 [c] *(new)*
* A. Menezes, E. Teske, A. Weng. Weak fields for ECC. CT-RSA 2004, LNCS, pp. 366–386. doi:10.1007/978-3-540-24660-2_28; ePrint 2003/128. [c] *(new)*
* C. Diem. The GHS attack in odd characteristic. J. Ramanujan Math. Soc. 18(1) (2003) 1–32. [b] *(new)*

## Hyperelliptic and genus-2 index calculus

* P. Gaudry. An algorithm for solving the discrete log problem on hyperelliptic curves. EUROCRYPT 2000, LNCS, pp. 19–34. doi:10.1007/3-540-45539-6_2 [c] *(new; Õ(q²) for fixed g)*
* N. Thériault. Index calculus attack for hyperelliptic curves of small genus. ASIACRYPT 2003, LNCS, pp. 75–92. doi:10.1007/978-3-540-40061-5_5 [b] *(new)*
* P. Gaudry, E. Thomé, N. Thériault, C. Diem. A double large prime variation for small genus hyperelliptic index calculus. Math. Comp. 76 (2007) 475–492. doi:10.1090/S0025-5718-06-01900-4 [c2] *(new; Õ(q^{2−2/g}), the source of "Õ(q) in genus 2")*
* A. Enge, P. Gaudry. A general framework for subexponential discrete logarithm algorithms. Acta Arith. 102 (2002) 83–103. doi:10.4064/aa102-1-6 [c2] *(new; L_{q^g}[1/2, √2] per secondary sources)*
* K. Nagao. Decomposition attack for the Jacobian of a hyperelliptic curve over an extension field. ANTS IX (2010), LNCS, pp. 285–300. doi:10.1007/978-3-642-14518-6_23 [b] *(new)*

## Generic and algebraic models

* V. I. Nechaev. Complexity of a determinate algorithm for the discrete logarithm. Math. Notes 55 (1994) 165–172. doi:10.1007/BF02113297 [b] *(new)*
* V. Shoup. Lower bounds for discrete logarithms and related problems. EUROCRYPT 1997, LNCS, pp. 256–266. doi:10.1007/3-540-69053-0_18 [b] *(new)*
* G. Fuchsbauer, E. Kiltz, J. Loss. The algebraic group model and its applications. CRYPTO 2018, LNCS, pp. 33–62. doi:10.1007/978-3-319-96881-0_2; ePrint 2017/620. [c] *(new)*
* J. Duman, D. Hartmann, E. Kiltz, S. Kunzweiler, J. Lehmann, D. Riepel. Generic models for group actions. PKC 2023, LNCS, pp. 406–435. doi:10.1007/978-3-031-31368-4_15 [c] *(new; related work)*

## Generic attacks, multi-target, automorphisms

* I. Duursma, P. Gaudry, F. Morain. Speeding up the discrete log computation on curves with automorphisms. ASIACRYPT 1999, LNCS, pp. 103–121. doi:10.1007/978-3-540-48000-6_10 [b]
* F. Kuhn, R. Struik. Random walks revisited: extensions of Pollard's rho algorithm for computing multiple discrete logarithms. SAC 2001, LNCS, pp. 212–229. doi:10.1007/3-540-45537-X_17 [c2] *(new; √(2rL) for L < r^{1/4})*
* D. J. Bernstein, T. Lange. Computing small discrete logarithms faster. INDOCRYPT 2012, LNCS, pp. 317–338. doi:10.1007/978-3-642-34931-7_19 [c] *(new, §13.6)*

## Transfer and lifting attacks (§13.1–13.2, §15.3)

* A. Menezes, T. Okamoto, S. Vanstone. Reducing elliptic curve logarithms to logarithms in a finite field. IEEE Trans. Inf. Theory 39 (1993) 1639–1646. doi:10.1109/18.259647 [b] *(new)*
* G. Frey, H.-G. Rück. A remark concerning m-divisibility and the discrete logarithm in the divisor class group of curves. Math. Comp. 62 (1994) 865–874. doi:10.1090/S0025-5718-1994-1218343-6 [b] *(new)*
* N. P. Smart. The discrete logarithm problem on elliptic curves of trace one. J. Cryptology 12 (1999) 193–196. doi:10.1007/s001459900052 [b] *(new)*
* M. J. Jacobson, N. Koblitz, J. H. Silverman, A. Stein, E. Teske. Analysis of the xedni calculus attack. Des. Codes Cryptogr. 20 (2000) 41–64. doi:10.1023/A:1008312401197 [c] *(new; the obstruction is a bound on dependency coefficients)*

## Point halving, validation and subgroup membership (§16.2, §17.2)

* E. W. Knudsen. Elliptic scalar multiplication using point halving. ASIACRYPT 1999, LNCS, pp. 135–149. doi:10.1007/978-3-540-48000-6_12 [c2] *(new)*
* K. Fong, D. Hankerson, J. López, A. Menezes. Field inversion and point halving revisited. IEEE Trans. Computers 53(8) (2004) 1047–1059. doi:10.1109/TC.2004.43 [c] *(new)*
* R. Schroeppel. Elliptic curve point ambiguity resolution apparatus and method. PCT/US00/31014, WO 01/35573 A1 (2001). [b; reference as given in Fong et al.] *(new)*
* D. R. L. Brown, A. Yamada (Certicom). Method and apparatus for performing validation of elliptic curve public keys. US 7,257,709 B2 (granted 2007-08-14; priority 2002-09-03); published as US 2004/0114760 A1. [c] *(new; its h = 4 test is Tr(x) = 0, x ≠ 0, Tr(x·H(b/x²)) = 0)*
* B. King. A point compression method for elliptic curves defined over GF(2^n). PKC 2004, LNCS, pp. 333–345. doi:10.1007/978-3-540-24632-9_24 [b] *(new; §5 not read)*
* P. N. J. Eagle, S. D. Galbraith, J. B. Ong. Point compression for Koblitz elliptic curves. Adv. Math. Commun. 5(1) (2011) 1–10. doi:10.3934/amc.2011.5.1; ePrint 2009/086. [b] *(new)*
* T. Pornin. Point-halving and subgroup membership in twisted Edwards curves. IACR ePrint 2022/1164. [c] *(new)*
* D. Koshelev. Subgroup membership testing on elliptic curves via the Tate pairing. J. Cryptogr. Eng. 13(1) (2023) 125–128, doi:10.1007/s13389-022-00296-9; correction in J. Cryptogr. Eng. 14 (2024) 127–128, doi:10.1007/s13389-023-00331-3. [c] *(new in §18; cited in §15.3)*
* Y. Dai, Y. El Housni, D. Koshelev, K. Reijnders. Divide-and-Pair: faster subgroup membership testing for elliptic curves. IACR ePrint 2026/749. [c] *(new; a method paper, not a survey)*
* D. V. Bailey et al. Breaking ECC2K-130. IACR ePrint 2009/541 (source for the ECC2K-130 subgroup order). [c] *(new)*

## This repository

* RESEARCH_PDP_SPEEDUP_AND_ISOGENY.md (binary isogeny-class Macaulay measurements); RESEARCH_KOBLITZ_INDEX_CALCULUS.md; ml-cryptanalysis `csd/volcano.py`, `csd/isogeny.py`.

## Changes from the current §18

* **Corrected:**
  * √élu now carries its venue (Open Book Series 4).
  * Huang et al. now give both titles.
  * JMV now lists the ePrint title and the journal version.
  * Gaudry 2009 gets its issue number.
* **Added:** 33 references cited in §13–§17, or needed for the corrected attributions (marked
  *new*).
* **Kept unchanged:** Teske, Kohel, Miret et al., Semaev, Diem 2011, Gorla–Massierer,
  Duursma–Gaudry–Morain and Galbraith 1999 (bibliographic data confirmed).
* **Removed:** none.
