Prefilter (bitset on raw x3) experiment rows, 2026-11-14. [pcc:unverified] wall clock, one host, no isolation.
- pf32_on*/off*: interleaved A/B, same curves/seeds, identical target counts.
- sw30_*, sw30b: B sweep with prefilter on, tuning curves CSKIP=100 (30 bits), best B about 180.
- ho28/ho30/ho32: holdout, fresh curves CSKIP=200, frozen rule B = 0.176*2^(bits/3) (116/184/292); plain3 = ic_solve 3-summand c=0.5.
