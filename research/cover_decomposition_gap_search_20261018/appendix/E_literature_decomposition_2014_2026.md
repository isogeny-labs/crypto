# Appendix E: literature on decomposition / summation-polynomial index calculus, 2014 to October 2026

Compiled by a web-search agent (about 75 searches over ePrint, arXiv, DBLP, HAL, author pages; ePrint
PDFs bot-blocked, arXiv/HAL/author copies used). Applicability: (a) F_p; (b) F_{2^n}, n prime;
(c) F_{2^n}, n composite; (d) F_{p^n}, small n.

## 0. Pre-2014 foundations

Semaev 2004 (ePrint 2004/031); Gaudry 2004/2009 (Õ(q^{2−2/n}); F_{q^3} in O(q^{10/7}) with double large primes); Nagao ANTS 2010 (ePrint 2007/112); Diem Compositio 2011 and ANT 2013 (rigorous subexponential (q^n)^{o(1)} for n→∞, n/(log q)²→0); Joux–Vitse ePrint 2010/157, JoC 2013 (130-bit oracle-assisted static DH on E(F_{p^5})); Joux–Vitse EUROCRYPT 2012 (151-bit ECDLP on E(F_{p^6})); FPPR EUROCRYPT 2012; Petit–Quisquater ASIACRYPT 2012 (heuristic 2^{O(n^{2/3} log n)}, crossover n ≈ 2000); FGHR JoC 2014; Shantz–Teske 2013 ("subexponential behaviour would manifest only far beyond the cryptographically interesting range"); Huang–Petit–Shinohara–Takagi IWSEC 2013.

## 1. Chronological list 2014 – Oct 2026

**2014.** FGHR JoC 27(4) (d). Faugère–Huot–Joux–Renault–Vitse "Symmetrized summation polynomials using small order torsion points", EUROCRYPT 2014: 8th symmetrised polynomial computed (d). Hodges–Petit–Schlather "First fall degree and Weil descent", FFA 30 (b)/(c). Gorla–Massierer trace zero (d). Galbraith–Gebregiyorgis INDOCRYPT 2014, ePrint 2014/806: binary Edwards variables; "Pollard rho remains much faster than index calculus for practical field sizes" (b). Sarkar–Singh ePrint 2014/815 (hyperelliptic). Wenger–Wolfger SAC 2014: 113-bit Koblitz rho on FPGAs (record, (b)).

**2015.** Kosters–Yeo arXiv 1503.08001: NP-completeness; FFD 2; at n=45, m=2, t=2 the degree of regularity rose to 5 (126 GB) where Semaev assumes ≤ 4 (b). Semaev ePrint 2015/310: 2^{1.69√(n ln n)} under Assumption 1; FIPS B/K-409, -571 "theoretically broken" if it holds; never peer-reviewed; contested (Galbraith on the curves list, April 2015: "concluded it did not work") (b). Karabina ePrint 2015/319, ICISC 2015: auxiliary variables; n=15: 393 s/6.4 GB → 7.1 s/256 MB (b). Huang–Petit–Shinohara–Takagi ePrint 2015/358 (generalised FFD assumptions) and PJMI 2015 (b). Huang–Kosters–Yeo CRYPTO 2015 (last fall degree) (b). Huang–Kosters–Yang–Yeo arXiv 1505.02532 (b). Kousidis–Wiemers ePrint 2015/1121, JMC 2019: FFD ≤ m²−m+1; crossover ≈ 1250 (b). Sarkar–Singh ePrint 2015/179 (hyperelliptic). Galbraith–Gaudry survey. Galbraith ECC 2015 slides. Galbraith–Wang–Zhang ePrint 2015/605 (BSGS).

**2016.** Petit–Kosters–Messeng PKC 2016: factor base from a B-smooth structure of p−1 (NIST P-224 fits: p−1 = 2^96·3·5·17·257·641·65537·…) or an auxiliary curve with smooth order; reduces to the "generalized root-finding problem"; "The complexity of our algorithms remains an open problem"; "only practical for small parameters" (a). Courtois ePrint 2016/003: 2^{n/3} splitting, unverified (b). Gebregiyorgis PhD thesis (Auckland 2016): "cryptosystems making use of the DLP for binary curves … are safe" (b). Bernstein–Engels–Lange–Niederhagen–Paar–Schwabe–Zimmermann ePrint 2016/382: FPGA rho on sect113r2; a 117.35-bit DLP over F_{2^127} in progress (record, (b)). Wallet PhD thesis (UPMC 2016).

**2017.** Amadori–Pintore–Sala ePrint 2017/609, FFA 51 (2018): one Gröbner basis, one relation; "more work is needed in order to reach a possible enhancement on Pollard's Rho" (a). McGuire–Mueller ePrint 2017/1262: Gröbner-free; preprint only (a). Faugère–Wallet DCC 2018 (hyperelliptic summation ideals, char 2). Kusaka et al. ICISC 2017: 114-bit BN-curve rho record (a). Kim–Tibouchi "invalid curve / FFT-style decomposition": no such paper exists.

**2018.** Kudo–Yokota–Takahashi–Yasuda CANS 2018: hybrid exhaustive search + Gröbner on the APS variant "and its limitation" (a).

**2019–2020.** Trimoska–Ionica–Dequen ePrint 2019/313, AFRICACRYPT 2020; CP 2020 (arXiv 2001.11229): S₃ at n=41, l=20: 4.2 s vs 16.8 s F4; S₄: 0.6 s vs 229 s (b). Di Tullio–Pal arXiv 1909.08901 (a, speculative). Yokoyama–Yasuda–Takahashi–Kogure JMC 2020: lower bound, naive small-x IC ≥ rho (a). Huang–Kosters–Petit–Yeo–Yun JMC 2020 (quasi-subfield); Euler–Petit FFA 2021 (no speedup). GGMP SAC 2020 (b)/(c). Abdullah–Mahalanobis–Mallick arXiv 2005.05039 (minors, ≤ 2^50).

**2021–2023.** Trimoska thesis 2021. Euler–Petit 2021. Jindal–Jatain–Bajaj 2023 survey (no new result). No 2022–2023 primary research on summation-polynomial ECDLP found.

**2024.** Hu ePrint 2024/1923: "pseudo-index calculus"; concludes no subexponential method for F_p; unrefereed (a).

**2025 – Oct 2026.** Gupta–Gupta–Singh–Kumar, Jñānābha 55 (2025): internally confused, not credible. Johari (IIT Kanpur report, 2025): survey. Dallaire-Demers–Doyle–Foo arXiv 2508.14011: classical records ladder (112-bit prime 2009; 113-bit Koblitz 2014; "117-bit binary"; secp256k1 interval puzzles 114 → 129 bits by 2025, kangaroo on GPUs). Bhati SPACE 2026 (hyperelliptic). All 2026 ECDLP items found are quantum resource estimates.

## 2. Current best practical records

| Setting | Best by decomposition / index calculus | Best generic |
|---|---|---|
| (a) F_p | None beyond toy sizes; Yokoyama 2020 lower bound | 114-bit BN curve (Kusaka 2017); 112-bit secp112r1 (Bos 2009); secp256k1 interval 129-bit (2025, not full ECDLP) |
| (b) F_{2^n}, n prime | None; largest PDP experiments n ≤ 41 | 113-bit Koblitz (2014); sect113r2 (2016); 117.35-bit over F_{2^127} (in progress 2016, cited as achieved); ECC2K-130 open |
| (c) composite n | GLS over F_{2^{5·31}}, 115-bit (Chi-Domínguez et al. 2021, cover+IC) | — |
| (d) F_{p^n} small n | 151-bit on E(F_{p^6}) (JV 2012); 130-bit static DH on E(F_{p^5}) (JV 2013); Gorla–Massierer T_3 at 36 bits | — |
| precomputation rho | — | Bernstein–Lange ePrint 2012/458; "Two grumpy giants" 2013 |

## 3. Explicit open problems quoted

- PKM 2016: "The complexity of our algorithms remains an open problem"; Problem 1 (generalized root finding: f(X_1..X_m) = 0 with L(X_i) = 0).
- APS 2018: "more work is needed in order to reach a possible enhancement on Pollard's Rho".
- Kousidis–Wiemers 2019: is d_reg = m²−m+1+o(1)?
- Huang–Kosters–Yang–Yeo: "find a method which does not use a monomial order"; systems from a single multivariate polynomial.
- Kosters–Yeo: the trace morphism "can speed up relation generation" (unexploited).
- GGMP 2020: "combined effects of larger degrees, smaller block sizes and the homogeneous structure … for future work".
- Gorla–Massierer: specialised Gröbner techniques needed; crossover for n > 3 unknown.
- Gebregiyorgis: degree of regularity of structured systems open.
- Trimoska et al.: exponential in l; vertex-cover bound k' is the lever.
- HPST 2015/358: cases where FFD "is unlikely to hold, with very unexpected consequences".

## 4. Sub-rho claims and status

| Claim | Status |
|---|---|
| Gaudry 2009 / Diem 2011, 2013 | proven / accepted; (d) and (c) only |
| Petit–Quisquater 2012 | heuristic; doubted (Shantz–Teske, Kosters–Yeo, HKY 2015, HPST); crossover ≈ 1250–2000 |
| Semaev 2015 | preprint; contested; effectively discredited |
| Courtois 2016 | preprint; unverified |
| Yokoyama et al. 2020 | peer-reviewed negative (naive IC ≥ rho) |
| quasi-subfield 2020 | conditional; Euler–Petit: no speedup |
| APS / McGuire–Mueller / Kudo | no sub-rho claim |

No claimed sub-rho result for (a) or (b) is proven or independently validated.

## 5. 2023–2026 claims on deployed curves via decomposition

None found. Hu 2024 concludes the opposite; Gupta 2025 not credible; all 2025–2026 "256-bit" items are Shor resource estimates.

## 6. Queries that returned nothing

Kim–Tibouchi decomposition; McGuire–Mueller "factor base of 2 elements"; Hong–Kim; Zhu/Wang/Ding symmetrisation; Tang–Galbraith; "half-size" factor base; Semaev post-2015; division-polynomial / elliptic-net factor bases; lattice-based decomposition; ML-guided relation search; tensor / quantum-inspired classical; GPU rootfinding for summation polynomials; crossbred for ECDLP; 2022–2025 degree-of-regularity bounds for summation systems; Kudo/Ikematsu/Yasuda 2021–2024; composite-n binary decomposition records. Not run (budget): rho records 2019–2024; Pintore/Sala follow-ups; Galbraith 2023–2025 talks.
