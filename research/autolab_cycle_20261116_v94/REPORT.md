# C94: certified degree blocks and a separate odd-field backend

Three immutable experiments passed under one-core, 4 GiB per-process limits. The frozen release gate remains unchanged and unpassed; the user's 50x stretch goal remains active. All previous attempts, including C93's native F4 timeout, remain preserved.

## The important correction to the apparent hard case

The hypothesis was that original Boolean equations conceal pure linear source relations. We implemented general elimination of nonlinear monomial columns, with explicit row-combination witnesses, followed by exact affine substitution into EVERY equation. An independent checker uses different pivots and independently composes the substitution. Rank plus an injective affine map proves a bijection between all residual roots and all original ordered roots. An inconsistent checked linear combination proves emptiness. A remaining nonlinear system is sent to unchanged genuine native F4/F5; the pipeline truthfully records when no native engine was required.

All 120 literal S3 cases over GF(4/8/16/32) matched 188 complete root pairs. Eighty additional random Boolean systems matched independent exhaustive enumeration. Three altered certificates were rejected. The actual GF(2^83)/D18 profile covered every one of the nine exposed C93 cases: four unsupported cases reduce from 36 source bits to 18, then contradiction; the two supported cases reduce 36 -> 18 -> 1 and give both ordered roots; u=2 is immediately inconsistent. The u=0 and u=1 residuals retain 36 and 24 bits respectively. Their existing separately proved shortcuts remain available, but this general preprocessor does not claim to have solved those wide residuals and does not rerun their native search.

The changed cold diagnostic used three actual cases (first unsupported, first supported, u=2) plus a fixed tiny nonlinear residual, three repetitions and alternating arm order: 36 complete child processes with identical original roots. The six actual F4/F5 invocations occurred only on the tiny residual. All three actual-field cases were solved by preprocessing alone. Calling these arms completed genuine F4/F5 hard solves would be false.

| Conditional case | Symmetric solver median | Preprocessor/F4 pipeline median | Preprocessor/F5 pipeline median |
| --- | ---: | ---: | ---: |
| GF83 unsupported | 0.03252 s | 0.23973 s | 0.23288 s |
| GF83 supported | 0.03197 s | 0.23407 s | 0.23169 s |
| GF83 u=2 | 0.03582 s | 0.03803 s | 0.03582 s |
| Tiny nonlinear control | 0.03258 s | 0.03577 s | 0.03450 s |

These are exposed conditional diagnostics, subsecond and without qualified isolation. They fail the hardness floor. The useful scientific result is that the former native timeout did not establish inherent hardness: general linear preprocessing resolves the generic cases. The specialized symmetric representation remains cheaper than this expanded Python certificate pipeline, but this is not a qualified F4/F5/F6 speed ratio. Native streams and failed-child observers are now saved before assertions; this retention boundary also applies to future failures.

## Odd prime and extension fields

A different backend uses the odd-characteristic discriminant identity b^2-4ac=16 f(x)f(y), f(X)=X^3+AX+B. It caches each source's square class and a normalized square root, retains equal nonsquare/twist branches, handles repeated sources and two-torsion explicitly, and batches the nonzero quadratic denominators. Generic finite-field Tonelli-Shanks supports odd extensions; prime arithmetic uses exact modular exponentiation. Smoothness and characteristic-three exceptions are checked. This exact finite-domain pair-to-third-coordinate contract is narrower than the full five-source rootfinding target.

Ten exhaustive full-domain curve cases over GF(3/5/7/9/27) matched all 1,758 literal S3 roots, with repeated sources, zeros, twist classes and torsion. Each NIST field was then tested with D={0,...,15}, unrestricted third coordinate, three internal repetitions and complete output equality. These source domains are small calibration fixtures, not hard intervals.

| Field | Complete triples | Direct quadratic median | Cached/batched median | Internal warm component ratio |
| --- | ---: | ---: | ---: | ---: |
| P-192 | 244 | 0.04244 s | 0.00787 s | 5.39x |
| P-224 | 256 | 0.16718 s | 0.02464 s | 6.78x |
| P-256 | 244 | 0.06735 s | 0.01058 s | 6.37x |
| P-384 | 240 | 0.16646 s | 0.02600 s | 6.40x |
| P-521 | 256 | 0.36331 s | 0.04490 s | 8.09x |

The cached method uses 16 source square-root calls rather than 240 pair discriminant square-root calls, and one batched denominator inverse rather than 128–136 individual denominator inverses. The frozen `inverse_calls` counter counts quadratic-denominator inversions only: BOTH arms also perform one setup inverse of delta, and exponentiation costs are charged. It is not a total arithmetic inversion counter. These internal measurements include solving, original membership checks and serialization, but reuse a warm process; they are not cold all-roots Gröbner comparisons. A stronger direct comparator should also batch denominators and cache field constants before attributing gains to square-root reuse.

## Memory, provenance and decision

Cold child wait4 RSS ranged 12,288–12,544 KiB. Outer worker high-water measurements were 84,536 KiB (oracles), 89,868 KiB (cold controller), and 85,680 KiB (odd profile). The odd per-method readings are cumulative process high-water marks, not isolated per-method memory; inherited/process accounting can differ from fresh child measurements. Limits are per-process address space, not aggregate concurrent RSS. All exact commands, root sets, certificates, stream files, observers, source/input hashes and attempts are archived.

The primary repository already supplies the S3 formulas; symmetric variables, degree-block elimination, Tonelli-Shanks and batched inversion are established. This checkpoint claims exact implementations and measured scoped evidence, not novelty clearance. Main 339-variable completeness, a new search mechanism for its outer parameters, matched full F6, 20 preregistered hard completed instances, all-family hard coverage, cold repetitions/isolation/A/A and 50x end-to-end gain remain open.

Next: integrate certified block elimination as a strong preprocessing baseline and forecast which conditional cells really remain nonlinear; separately freeze cold odd-backend runs against a direct comparator that already batches inverses. An adaptive selector must use preregistered structural features, retain exceptional branches and charge every outer parameter. Do not extend the same native timeout or present these easy pair cases as hard instances.

Replay: `PYTHONDONTWRITEBYTECODE=1 python research/autolab_cycle_20261116_v94/REPLAY.py`.
