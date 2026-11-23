# Round71: explicit sublinear construction of the marked response

Derived construction and local exact checks, 2026-11-09, T-2076. Independent signed scientific review remains pending. [pcc:unverified]

## What changed

The giant numerator can be multiplied before baby interpolation in the signed Kummer-surface coordinate ring. Write each factor as A+WB+eps(C+WD), with W^2=f(T)f(V),eps^2=0. A product of k factors has four bivariate coefficient polynomials, each of individual degree at most2k, hence at most4(2k+1)^2 dense coefficient slots. Evaluation uses the original public signed baby pairs, not a beta/gamma interpolant oracle.

A balanced product tree with fast bivariate multiplication and four applications of finite-field multipoint evaluation gives an explicit response-construction algorithm. The accompanying surface-product-proof.md supplies every factor, product relation, denominator and clipped-tail rule. It includes a deterministic bit-complexity bound for the full two-coefficient response: for every fixed epsilon>0, O(r^(2/3+epsilon) polylog(p)), using b approximately r^(2/3) babies and n approximately r^(1/3) giants. This is a written reduction to published fast arithmetic, not an extrapolation of the prototype. [pcc:unverified]

The balancing expression is b+n^2, with n=ceil(r/(2b+1)). Form a single giant product of degree at most2n in each variable and evaluate at b baby points. Public point generation, product-tree denominators and their fast univariate evaluation, centers, clipped tail and signed jet recombination contribute O~(b+n) work. The fast evaluation theorem has m=2 fixed variables and works over K=Fp2. Four ordinary polynomials are evaluated; differentiation of the nonalgebraic MME backend is not assumed.

No numerator inversion is used: singular responses, multiple zeros and nilpotent jets remain in the same exact algebra. The calibrated B-only extractor can use a fixed number of these responses, followed by fixed-degree root finding and point verification. A further written extraction uses the zero index directly: take U=G, identify j* from one zero baby evaluation plus a giant scan, return k=1-(j*)^(-1) modulo r and verify. It needs no subsequent action/orbit loop; see the proof appendix. The present native prototype does not yet emit those indices, so this extraction remains a written construction pending native qualification. The class-number/L-value observations do not supply an additional saving in this reduction; an actual CM compression or transport improvement must be accounted for explicitly.

## Exact native qualification

The new Rust contraction uses classical sparse polynomial multiplication and direct sparse evaluation. It does not implement Kedlaya–Umans, fast bivariate multiplication, fast denominator multipoint evaluation, or the full P192-width layout. Its outputs qualify the algebraic interface used by the theorem.

All nine surface controls pass: empty,unit,simple zero,same-root twice,distinct roots,zero,zero-then-unit,odd-W square and all four channels. They compare evaluated surface products with independently assembled scalar jet products at retained points satisfying W^2=f(T)f(V). Incorrect first-coefficient multiplication is rejected on the recorded applicable controls.

The complete small decoder output is byte-identical to Round69:381rows and2694response queries over the inherited r7,r19,r199 panels. The new backend also matches all24prescribed P19227-factor response coefficients. Every query separately asserts equality with explicit signed pair products. These retained inputs are deterministic exact controls, not a random sample or a performance experiment. No full order-r P192 response was evaluated.

The P192 prototype records36087quadratic multiplications,213quadratic inversions and168source point additions across the24selected queries;34344multiplications and zero inversions are charged to the surface contraction itself. The r199 panel reaches individual degree6 with148nonzero coefficient slots, within its196slot bound; P192 chunks reach degree4 and68slots, within100. These are output-polynomial slots, not total peak process memory. Balanced-tree temporaries, point arrays and retained reference data are additional storage.

The inherited qualification harness still constructs legacy baby interpolants/caches even though the surface contraction does not use them. Their setup counts remain recorded and are not free. The written algorithm avoids these unused constructions; the prototype is not assigned that asymptotic setup cost until those backends are implemented.

## Publication, replay and actual rough edges

Freeze19d570d6f2716775c635aa152f71b915a844f077 and exact source134ea4f2065e73394b47dd11e392c405e72c6691 preceded native data. The first build and all three native runs passed. Compiler unused-variable warnings are retained. An inherited P192 mode/scope string incorrectly described the predecessor norm backend despite correct per-row surface records. A metadata-only source correction was published, initial-P192.json retained, and a qualification run confirmed that only mode/scope changed; every scientific row/count remained identical. This was not an algorithm change or an unchanged retry.

A separate source copy built offline with one build job and reproduced all three final scientific outputs and all three diagnostic logs byte for byte. A changed small B coefficient and a changed P192response coefficient each caused exit101. Each command retained the120second bound. Cloud jobs, generators and unchanged retries were zero. The earlier Round70small-panel failure was not rerun or altered; these are the distinct frozen Round71 algebra/response controls.

Raw records, binary/source hashes, commands and failures are retained. Scientific standing is unverified until an independent signed verdict. Earlier Rounds63–70 remain immutable; PR3774 merged at2026-11-09T19:59:43Z.

## Literature and continuation

Sources checked(KB): tools and migrated wrapper unavailable. The bounded review covered Round25relation/orbit, Round62marked transport, Round68coefficient-cache-proof and Round69one-norm-proof. Primary theorem read: Kedlaya and Umans, Fast polynomial factorization and modular composition, June2011 version, section4.3/Corollary4.5, https://users.cms.caltech.edu/~umans/papers/KU08-final.pdf . It gives a finite-field/extension-ring multipoint evaluation bit algorithm with arbitrary fixed exponent slack; its hypotheses are checked in the proof. No novelty is claimed for MME, product trees or standard polynomial multiplication. Novelty of this particular cryptanalytic application has not been established by an independent prior-art review.

The next implementation task is to retain and qualify the zero indices and signed scalar extraction, then implement the actual fast multipoint backend with a clean front end that omits unused interpolant caches. The next theoretical task is to exploit the explicit horizontal relation to reduce the response representation or the number of action calls, while preserving the signed point action. These are concrete dependencies of using the new response constructor in the requested ECDLP algorithm.
