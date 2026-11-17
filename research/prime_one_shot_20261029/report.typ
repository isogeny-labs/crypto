#set page(paper: "a4", margin: 19mm, numbering: "1")

#set text(font: "DejaVu Sans", size: 9pt)

#set par(justify: false, leading: 0.65em)

#set heading(numbering: none)

#heading(level: 1, text("One research bet for sub-exponential prime-field ECDLP"))

#image("conditional-route.svg", width: 100%)

#text("2026-10-29. Decision report and stored-evidence audit; no new ECDLP algorithm, solver run, or speedup is established.")



#text("My selection is growing-arity divisor-coefficient decomposition with shared quotient-and-remainder membership towers. Graeffe terminal membership is retained as a comparator, not the preferred encoding. This advances `LEAD-ICP-shared-coefficient-membership`, the symmetric PKM work and the P-256 cubic-divisor continuation. The five-point Graeffe version already exists as `IDEA-20261029-3d0e71`; neither that construction, Riemann-Roch decomposition, root-squaring, shared remainder membership nor PKM factor bases is newly invented here. Generalizing them to odd arity is a standard composition, not a novelty claim. The missing research contribution is a uniform, complete solver bound as arity and depth grow.")



#text("I cannot honestly deliver the requested novel sub-exponential algorithm: its essential complexity lemma remains unproved, and there is no positive measured scaling evidence for it. The choice below is a research judgment about where to spend one opportunity, not a claim that the opportunity succeeds.")



#heading(level: 2, text("Scope and coverage"))

#text("The target is classical ECDLP on nonsingular curves over prime fields, with a large prime-order subgroup. Binary curves, prime-degree extension fields, and precomputed online-only methods do not discharge this request. Special low-embedding-degree and anomalous curves have known attacks and do not supply the requested novelty. The concrete Kummer tower below covers primes with a suitable smooth divisor of p-1. Existing Dickson/norm-one torus fibres also supply a membership construction at P-256 using p+1; this is a base, not a P-256 algorithm or attack. No general-prime algorithm or secp256k1 solver is established. Auxiliary isogeny towers require another independently costed construction and analysis.")



#text("All six named default-branch trees were located and revision-pinned locally: crypto, cryptanalysis, crypto-autoresearcher, cryptanalysis-rl, ml-cryptanalysis, and ecbench. Private-repository contents and experiment records are not reproduced in this public report. The local review of those repositories does not constitute exhaustive inspection of their code or experiments.")



#text("The public lead atlas contains 150 mechanism families: 74 with recorded closures, 50 with next steps, 17 blocked, and 9 unexamined. Its coverage tool reports 2,092 of 2,093 ECDLP ledger records mapped, with H-MONO-23d347 unmapped. That record concerns extension-field counting conventions, not a prime-field solver. Mapping is not reading or independent verification. The atlas also reports 52/42/47 watched paths changed since its review in autoresearcher/crypto/cryptanalysis; the relevant current prime-field proposals and corrections were inspected, but those deltas have not all been reconciled. `LEADS.tsv` retains all 150 classifications for inspection, without promoting their dispositions to independently proved results.")



#text("The deep review concentrates on the load-bearing prime-field paths: explicit and implicit bases; PKM towers and their symmetric variant; small-root lattices; characteristic-zero and function-field lifts; isogeny/cover transfers; generic residual/representation searches; special-j symmetry; and the latest shared-coefficient proposal. This is not an exhaustive reading of all historical branches, raw archives, uncommitted worktrees, or every experiment. The pre-existing [portfolio coverage problem](../../problems/ecdlp-portfolio-reading-coverage.md) remains open. That gap is part of the requested task, not silently waived.")



#heading(level: 2, text("What was independently checked here"))

#text("Run `bash research/prime_one_shot_20261029/audit.sh /workspace /tmp/prime-one-shot-audit` with the named sibling checkouts. This reads committed evidence with jq, awk and cmp. It does not rerun the originating solvers. `audit-summary.json` and `inputs.sha256` retain the results and exact input bindings.")



#table(columns: (1fr, 1fr, 1fr), inset: 4pt, text(size: 8pt, "Evidence"), text(size: 8pt, "This audit's check"), text(size: 8pt, "What it does not establish"), text(size: 8pt, "ECBench canonical grid"), text(size: 8pt, "20 stored records equal between the two recorded runs"), text(size: 8pt, "Fresh reproduction; prime-field scaling (these are binary controls)"), text(size: 8pt, "ECBench Lattes/isogeny study"), text(size: 8pt, "90 stored records equal between recorded runs"), text(size: 8pt, "Equal-size controls or asymptotic degree improvement"), text(size: 8pt, "Symmetric PKM, arity three"), text(size: 8pt, "Six overlapping recorded rows equal after excluding elapsed milliseconds"), text(size: 8pt, "Independent solver implementation or an admitted timing ratio"), text(size: 8pt, "Original PKM msolve CSV"), text(size: 8pt, "24 rows: 16 completed `no_solution`; 8 incomplete, exits 124/139/143 in counts 4/3/1"), text(size: 8pt, "Eight completed solving degrees, or performance on natural successful decompositions"))

#text("An initial audit script used the wrong CSV verdict column. It was corrected to select columns by header, and rerun before retaining the summary. This illustrates why reporting counts without checking their interpretation is insufficient.")



#text("At m=3, t=4, N=12, the symmetric and plain-chain rows have widest matrices 1,224 versus 12,314 columns. Recorded field-operation counters are 124,297,944 versus 15,884,596,296 for target zero (target one differs slightly in the plain count). These are same-instrument stage diagnostics from frozen data, not fresh measurements or complete DLP costs. Solving degrees are 5 versus 6. At t=2,3,4,5 the symmetric degree reads 4,5,5,6; the last cell came from a separate symmetric-only run. This is evidence of a smaller recorded matrix, and evidence against assuming a degree plateau. It is not clean evidence for coefficient sharing alone: the symmetric arm excludes repeated abscissas while the plain arm permits them.")



#text("Source inspection adds a more serious scope restriction. `examples/pkm_symmetric_tower.rs` draws a,b without checking the discriminant or group order; `Instance::base()` includes every coset abscissa without checking whether it lifts to E(F_p), and measured x_R is a uniform field element rather than the abscissa of a subgroup point. Its planted checks evaluate polynomial assignments obtained from S3 roots, not signed F_p group decompositions with extraction. Therefore these rows are algebraic instrument diagnostics, not a validated natural prime-order PDP benchmark. A matched on-curve SAT/UNSAT study is a scientifically different and justified next test.")



#text("A stronger directly relevant predecessor exists in autoresearcher `research/p256_factor_base_parallel_20261025/tracks/f1/cubic-divisor/`. It records actual-field correctness examples: complete retained depth-three query unions and a separate basis replay, followed by three depth-four algebraic-adapter attempts stopped at their declared 30-second/2-GiB limits. The same-session replay is not independent scientific review; the Python/SymPy adapter is not the required final native implementation. Importantly, this predecessor distinguishes a radical divisor formulation from a nonreduced symmetric terminal formulation. It records dense expanded coefficient degree 2^k-2 at depths k=2..5. Those are reasons to preserve the unexpanded quadratic circuit and compare the right schemes, not evidence that its solving cost is sub-exponential.")



#text("The figure uses recorded matrix widths, not wall time. No leaderboard or benchmark row is changed; there is no newly measured performance result to promote.")



#image("recorded-widths.svg", width: 100%)

#heading(level: 2, text("Failures and nulls that deserve different treatments"))

#heading(level: 3, text("1. A documented busy host: rerun timing, retain the algorithmic classification"))

#text("[RESEARCH_PRIME_FAST_ECDLP.md](../../RESEARCH_PRIME_FAST_ECDLP.md) records an Apple M4 Pro shared with other compiler/experiment sessions, load average around 90, and roughly 20 GB of swap. Some inversions and batched-addition microtimings contradict neighboring rows. The author explicitly attributes them to scheduling noise. Absolute timing and speedup estimates from those rows warrant matched, isolated A/A and A/B measurement.")



#text("That is not evidence that the method's exponent might be sub-exponential. The method replaces heavy arithmetic and polynomial root loops with subtraction and table lookup. Its generic decomposition classification survives the timing defect. A clean host could change constants and timeout censoring, not turn that operation sequence into a non-generic algorithm.")



#text("More directly relevant, [the original PKM solver study](../ic_next10_20261014/03-pkm-degree/README.md) records load 22-25 on four cores. A single-threaded m=3 job received roughly 19 percent of one core: the first degree-seven steps at N=18 and N=21 took about 2,024/2,041 seconds wall against 384/396 seconds CPU. These exact wall-censored inputs warrant a controlled retry; the timeout budget did not buy the intended CPU work. The study's protocol also planned a planted-solution solver control, but the retained results cite sixteen refutations and the exports use `--planted 0`; successful root extraction is not validated by those sixteen controls.")



#heading(level: 3, text("2. Resource-censored F4: final degree unknown"))

#text("[Native N=21 results](../pkm_lead1_m3n21_20261016/RESULTS.md) report two targets completing all degree-seven work without refuting, then stopping before productive degree-eight reduction because a dense block would require 79,330,205,871 entries. At four bytes per entry that block alone is about 317 GB decimal. The actual 124 GB host deviated from the planned 192 GB floor, but even 192 GB would not fit that block. This is a concrete matrix-memory obstruction; CPU contention is not its explanation.")



#text("[msolve N=21 results](../pkm_m3_n21_msolve_20261018/RESULTS.md) distinguish single-thread wall stops inside degree seven from a later 16-thread attempt selecting degree-eight pairs and exiting 139 at 99.6 GiB, before reporting productive output. The protocol calls these inconclusive, correctly. Neither a plateau nor a completed higher degree follows from a selected round. A larger host is a justified retry for that narrow unanswered question, provided the original instances, solver version, flags and stop semantics are preserved. It is not a scientific justification to repeat all negative experiments indiscriminately.")



#text("The incomplete CSV rows nevertheless contain real counterevidence. At m=4,N=20 both raw msolve logs complete three productive degree-eight rounds, adding 1,241, 2,507 and 3,578 basis elements, before exit 139. This supports a degree lower bound of at least eight for that solver and presentation. It must not be erased by labeling the final verdict unknown. A fresh host could complete the solve; it cannot undo the completed degree-eight work. Round selection, productive work and final completion are three separate statuses.")



#heading(level: 3, text("3. Missing raw runs: not replicated evidence"))

#text("[RESTART.md](../prime_fall_degree_20261027/RESTART.md) says the last-fall study's raw runs, table and draft results were not pushed and lived only in the owner's Mac worktree. Its quoted medians 3,4,4,4,4,5,5,5,5,6 are session-reported, not independently recoverable from that branch. It also notes that one suggested library-test command runs zero tests; the actual hand checks are in an example. Reproduction must begin with the real checks and original small cells. A lost file is not a negative experiment.")



#heading(level: 3, text("4. A strong null within a limited scope"))

#text("The latest autoresearcher evidence EV-PFDR-07d608, rather than the older atlas wording alone, records an independently reviewed log-GAP coincidence census: support is confined to a five-percent tolerance on 44 toy cells, with an invalid cell excluded explicitly and positive controls detected. The review does not establish exact random behavior, relation-bearing log-GAP behavior, or a universal absence of coordinate structure. It nevertheless gives a better-founded reason to deprioritize another identical coincidence screen than an unreviewed timeout would.")



#heading(level: 3, text("5. Null instruments with weak sensitivity"))

#text("The ECBench Lattes/isogeny study uses a degree-s top product polynomial, with s=16,24,32, and reports that solving degree is dominated by that floor. Actual base cardinalities differ across families; the report flags that confounding. Its null at m=2, p<=1031 is not a theorem about all implicit tower presentations. A retry should remove the product-polynomial floor, match usable base cardinality, retain planted positives, and measure completed cost; another run with the same insensitive design would add little information about the proposed mechanism.")



#heading(level: 2, text("Mathematical exclusions: keep their quantifiers"))

#text("1. Generic bounds. Shoup-type lower bounds constrain group-operation/equality algorithms. They do not prove that an arbitrary coordinate-reading algorithm cannot work. The coefficient system below explicitly reads the field representation and curve equation; escaping the generic model grants permission to investigate, not an advantage. 2. Counting. Every ordered tuple has one sum, so mean representation count is fixed by tuple count and group size. Mean count is not coverage probability. Clustering can leave coverage low despite a large mean. No empirical constant such as 1.582 is used here as a universal coverage theorem. 3. Univariate membership. A nonzero polynomial vanishing on B distinct abscissas has degree at least B. This does not imply that every auxiliary-variable circuit for that set has B equations or solving degree B. PKM's entire construction exploits this distinction. 4. Presentation matters. [DERIVED.md D5](../prime_fall_degree_20261027/DERIVED.md) gives a naive degree bound at least 2^t, whereas tower presentations of the same set reach much smaller degrees at measured sizes. The ECBench survey's broad claims that degree is intrinsic or invariant under reparametrization must not be extended to arbitrary presentations, auxiliary variables, term orders or solving algorithms. Its own engine-dependent examples already demand care. 5. Naive prime IC lower bounds. KR-IC-401ebf corrects a superseded row: the Yokoyama et al. result relies on statistical assumptions and concerns the naive construction. The atlas records that its underlying reading was abstract-only. It is not an unconditional exclusion of every algebraic prime-field attack. Full primary verification remains missing here. 6. Lattices. The repository's [small-root bound](../smallroot_optimal_reach_20261026/README.md) states its scope: one block, convex monomial sets, equal bounds, polynomials in (p,f)^t; unequal bounds are numerical. Joint systems and different constructions are not ruled out. I am not selecting that direction because the measured/theoretical deficit in the tested construction is large and a missing constructive advantage remains, not because every lattice approach has been disproved.")



#heading(level: 2, text("The selected construction"))

#text("This is a proposed generalization with a correctness proof sketch, not a machine-certified new theorem or measured solver. Known ingredients are credited below. Let E: y^2=f(x)=x^3+ax+b over F_p, p>3. Initially assume the whole curve group has prime order r (a cofactor version needs explicit membership and transport). Let d=2^t divide p-1, choose g nonzero, set c=g^d, and use the signed point base whose abscissas satisfy x^d=c. Its actual usable point count B must be measured; d is not automatically B.")



#text("For odd arity m=2h-1, target R=(r_x,r_y), search for")



#raw("A(X) monic of degree h; deg C <= h-2;\nU_0(X) monic of degree m;\nA(r_x)-r_y C(r_x)=0;\nA(X)^2-f(X)C(X)^2=(X-r_x)U_0(X).", block: true)

#text("The function A(x)+C(x)y vanishes at -R. On the chart where U_0 has distinct roots, U_0(r_x) is nonzero, and C has no common root with U_0, residual points are (alpha,-A(alpha)/C(alpha)). The principal-divisor group relation gives their sum R. Conversely a distinct-abscissa decomposition avoiding the excluded cases has a function with this divisor by the elliptic Abel-Jacobi criterion. Its pole order is m+1=2h, so it has this A+C y shape with monic A after scaling. These statements require a fully checked chart correspondence before experimental admission; no claim is made about coverage of excluded charts.")



#text("The primary membership formulation enforces U_0 | X^d-c. Retain remainder polynomials V_j of degree below m and quotient polynomials Q_j of degree at most m-2, rather than expanding the remainders:")



#raw("V_0=X;\nV_j(X)^2=Q_j(X)U_0(X)+V_{j+1}(X),  j=0,...,t-1;\nV_t=c.", block: true)

#text("Coefficient comparison gives quadratic equations with O(mt) variables; the quotient coefficients are essential to that quadratic claim. Eliminating them gives X^(2^t)=c modulo U_0. Divisors of the separable polynomial X^d-c give a reduced finite membership scheme. The existing cubic-divisor predecessor already supplies this insight at arity three; this is an extension of it, not a newly discovered solver. For Dickson membership replace the squaring identity's left side by V_j^2-2. The actual P-256 construction uses suitable fully split norm-one torus fibres and v_2(p+1)=96. Producing a base there does not bound decomposition work.")



#text("The Graeffe comparator instead writes")



#raw("U_j(X)=E_j(X^2)+X O_j(X^2),\nU_{j+1}(T)=T O_j(T)^2-E_j(T)^2,\nU_t(T)=(T-c)^m.", block: true)

#text("For odd m this is the monic polynomial whose roots are the squares of U_j's roots, with multiplicities. Iteration forces every alpha^d=c. Since X^d-c splits with distinct roots in F_p under the stated hypotheses, the recovered abscissas are rational. The sign is recovered from -A/C when A,C are over F_p, rather than by enumerating 2^m sign assignments. Root extraction, coefficient rationality, denominator handling, exceptions, and group verification are all charged.")



#text("Set equality is insufficient for a Groebner comparison. Graeffe terminal membership can be nonreduced even when U_0 is squarefree, because the terminal has m equal roots. This issue was already derived in the P-256 F1 work. An odd-arity check over F_17 uses epsilon^2=0 and roots 1+epsilon,-1+epsilon,4: their fourth powers are 1+4epsilon,1-4epsilon,1, so the terminal polynomial is (T-1)^3. The initial polynomial deforms by epsilon(-2X^2+8X), remains squarefree, but does not divide X^4-1 over the dual numbers. Thus discriminant saturation does not equate the two schemes. The prototype retains this as a negative fixture; no inference of reducedness is made from matching ordinary point sets.")



#text("Field of definition is another substantive condition. Rational abscissas do not force all geometric coefficient solutions of the norm system to be rational. On E/F_17: y^2=x^3+2x+2, take z^2=3 in F_(17^2). The three anti-Frobenius points (1,8z),(12,16z),(15,12z) sum to O, while (5,1)+(6,3)=(10,6). Together they give a five-point residual geometric model with all x in mu_16 and distinct, yet with nonrational y. A finite-field relation solver must reject it. One can append coefficient field equations via repeated-squaring circuits or enforce square-y membership by shared Euler-criterion remainders, but the extra variables and solving cost must be included. A Groebner solve over the algebraic closure is not automatically the required F_p extraction algorithm.")



#text("The coefficients of the norm identity and each squaring layer are quadratic in unknown coefficients. There are O(mt+m) variables and equations. Squarefreeness and coprimality can be represented by polynomial Bezout witnesses, retaining O(m) extra coefficient variables and quadratic coefficient equations, instead of an uncharged discriminant oracle. Counts here concern a proposed presentation; small input degree alone gives no fast solving guarantee. A dense coefficient layer has O(m^2) convolution terms, also charged.")



#text("The reason to test growing arity is that eliminating directly to S_{m+1} exposes exponential-in-m polynomial degree. Keeping the function and divisor coefficients avoids that explicit expansion. It can still hide exponential work in the closure or number of surviving branches; that is the central question.")



#text("In particular, backward root-squaring enumeration is not the proposed breakthrough. Before the norm constraint, choosing m distinct roots from d possibilities gives binomial(d,m) coefficient states. At the balanced parameters below, log of that quantity is of order n, so writing all states is already exponential. A generic full membership-algebra basis has the same problem. The solver has to impose the elliptic norm constraint early enough to avoid materializing those states. Moreover x -> x^2 on the abscissa is not generically an elliptic-curve homomorphism: a point-sum relation does not automatically propagate to another point-sum relation after squaring every abscissa. There is no free constant-size group-law recurrence through this tower. Neither a low-width dependency graph nor a quadratic input system fixes that issue by itself.")



#heading(level: 2, text("What would actually imply sub-exponential complexity"))

#text("Let n=log_2 r and take odd m of order sqrt(n). Suppose a suitable tower yields a usable base")



#raw("log_2 B = n/m + O(log m),", block: true)

#text("and construction of that base costs exp(o(n)). The factorial correction is essential: under a random-like distribution the mean number of unordered distinct-point decompositions is approximately B^m/(m! r), not B^m/r. Enlarging B by a factor of order m can compensate; this still leaves B=exp(O(sqrt(n))). Signs and chart losses require consistent counting, not an extra free 2^m gain.")



#text("The decisive unproved hypothesis is a solver that returns one valid decomposition or a sound refutation at cost exp(O(sqrt(n) log n)) for this family, with success probability at least exp(-o(n)), including chart loss, extraction, and rational-solution selection. A sufficient stronger route would be a certified root/refutation procedure with polynomially many variables (O(n) before rationality circuits), effective degree D=O(m+t)=O(sqrt(n)), and work polynomial in binomial(N+D,D) and n. For N polynomial in n, the usual monomial-count estimate gives exp(O(sqrt(n) log n)) field work. An upper bound on refutation degree alone is not this procedure: SAT extraction and algebraic extensions must also be controlled.")



#text("With enough independent relations, collecting O(B poly(n)) of them costs exp(o(n)), even with sub-exponentially small success probability. Dense linear algebra B^3 is already exp(o(n)); sparse B^2 is helpful but unnecessary for this qualitative implication. Every target-dependent trial uses R=aP+bQ, retains a,b, and all costs start from scratch for one previously unseen Q. A rank-producing relation supply, nonzero Q coefficient in the recovered dependency, and bounded failure probability remain hypotheses. No free precomputation is assumed.")



#text("Under these hypotheses the conditional total is exp(o(n)), sub-exponential in bit length. The sharper exp(O(sqrt(n) log n)) total requires all overheads, including inverse success probability, chart loss, construction and rank production, to satisfy that sharper bound; merely calling them exp(o(n)) does not suffice. These are conditional implications with unproved premises, not an established algorithm. If instead effective degree is proportional to mt=Theta(n), the monomial bound is exponential and this argument fails. Even D=O(t) at fixed m would not settle behavior as m grows. A fixed-m p^0.4 algorithm would still be exponential in n and would not meet the user's requirement.")



#text("There are also field-family restrictions. For an arbitrary prime, v_2(p-1) may be tiny, so d near the desired scale may not exist. P-224 supports the Kummer shape, while P-256 has the different Dickson/norm-one torus base described above. Neither fixed field is an asymptotic family, and neither construction establishes an attack. A general-prime claim needs an independently costed suitable auxiliary tower, usable rational fibres and the same solver bound. None is supplied here.")



#heading(level: 2, text("Why this bet, and why the evidence is still weak"))

#text("Compared with digit encodings, lifts, isogeny navigation, or coincidence harvesting, this route combines an explicit non-generic algebraic interface, a compact growing-arity presentation, and existing algebraic correctness predecessors. It also avoids requiring rational two-torsion on the attacked prime-order curve. Those are reasons to allocate research effort here. The recorded matrix shrinkage is a confounded diagnostic, not the decisive premise.")



#text("The counterevidence matters just as much: symmetric degree still rises; all 16 completed rows in the audited original CSV are refutations; the five-point Graeffe proposal has no unknown-coefficient solving measurement; no measured arity-growth law exists; base coverage and relation rank are unproved; and symmetry alone removes only the known m! labeling redundancy. At growing m that redundancy must be charged consistently, but quotienting duplicate solutions is not an independent complexity breakthrough. There is no quantitative basis for assigning a probability of success to this bet.")



#heading(level: 2, text("Concrete native development result"))

#text("[prototype/norm_tower.cpp](prototype/norm_tower.cpp) now constructs both selected-chart encodings and known-answer assignments for m=3,5,7. The fixed curve is E/F_17: y^2=x^3+2x+2; exact tiny enumeration verifies its nonsingularity and prime order 19, different from the field characteristic. The membership base x^16=1 is deliberately dense; these are development fixtures, not a balanced scaling experiment.")



#text("The strict C++17 build passes 162 bounded correctness checks, including norm identities, recovered signs, explicit Bezout chart witnesses, malformed inputs, even/odd nonreduced Graeffe examples and the full F_(17^2) nonrational coefficient witness. Six quadratic systems are retained with exact assignments:")



#table(columns: (1fr, 1fr, 1fr, 1fr), inset: 4pt, text(size: 8pt, "Arity"), text(size: 8pt, "Graeffe variables / equations"), text(size: 8pt, "Remainder variables / equations"), text(size: 8pt, "Input degree"), text(size: 8pt, "3"), text(size: 8pt, "25 / 27"), text(size: 8pt, "33 / 35"), text(size: 8pt, "2"), text(size: 8pt, "5"), text(size: 8pt, "42 / 44"), text(size: 8pt, "58 / 60"), text(size: 8pt, "2"), text(size: 8pt, "7"), text(size: 8pt, "59 / 61"), text(size: 8pt, "83 / 85"), text(size: 8pt, "2"))

#text("[A separate native textual evaluator](independent-check/check_export.cpp), which does not reuse the exporter's polynomial engine, evaluates all 312 exported equations at those rational assignments with zero residuals. It rejects a changed norm coefficient (four equations fail), an incomplete assignment and an empty system. This is independent implementation within this session, not an external scientific replay. Exact commands, outputs and hashes are retained in the two subdirectories.")



#text("Crucially, these systems have not been solved for unknown coefficients. The exports omit coefficient field equations and remain geometric systems; the supplied witnesses are rational, but a future solver must enforce that domain. No complete model-set census, missing-chart coverage test, degree-growth measurement, useful-rank measurement, runtime advantage or sub-exponential result follows from these checks. The prototype makes the research object concrete and exposes two correctness traps before expensive measurement.")



#heading(level: 2, text("Discriminating execution protocol"))

#text("This protocol is a proposal, not a submitted job or an assertion that work continues after this turn.")



#text("1. Correctness first. Extend the present witness checks to per-root PKM, existing radical symmetric membership, norm/remainder and norm/Graeffe over the same root sets. Exact tiny finite-field model-set equality must include zero-y points, repeated abscissas, opposite pairs, target overlaps, poles, and failed charts. Test geometric and rational models separately, retaining the mixed-twist regression. Use m=3,5,7 where enough usable points exist. A deliberately too-small base is an empty distinct-chart negative control. Independent group sums verify every recovered relation. 2. Expose both axes. Use m=3,5,7,9 and t=3,4,5,6 subject to actual B>=m, plus diagonal growing-(m,t) cells. At least three independent primes per shape and generic-j curves; hold out primes and seeds. Separately label balanced B^m/(m!r) near one, low-density UNSAT, and planted SAT panels. Include uniform natural targets. Do not learn a SAT cost from only refutations or plant easy target-dependent coefficients. 3. References. Exact census at the smallest cells; pinned msolve, Singular, and Magma where available; existing symmetric tower as the main algebraic comparator. Generic enumeration is a correctness/cost reference, not the proposed new algorithm. Compare the same selected chart, then separately charge total chart coverage. Include a matched non-elliptic quadratic norm null and a known easy positive control. Source and output agreement precede performance claims. 4. Resources. Run native computational experiments through the repository's EKS route. Preserve exact source, binary and image hashes, CPU/memory/thread placement, operation counts, every failure and stop point. Bound each candidate cell before launch; do not silently discard a cell exceeding resources. Use dedicated L2 and randomized paired A/A/A/B ordering for timing claims, with at least five measured rounds after warmup; counts and degree findings remain separate from timing admission. Record CPU pressure, throttling, swap and neighboring jobs. A wall timeout is censoring, not a completed degree or security result. 5. Retain failures. A missing natural relation, incomplete root extraction, field-extension point, chart omission, or rank-deficient relation matrix is not a success. Keep all failed attempts in complete cold cost per independent verified relation. Do not compare a selected successful candidate to average baseline attempts. 6. Decision. First require a reproducible stage improvement beyond multiplicity differences. Then ask whether the diagonal degree/work data distinguish O(m+t) from O(mt); small samples cannot prove either asymptotic law. If only t-axis constants improve, record that gain but do not call it a sub-exponential mechanism. Attempt a uniform proof or lower-bound certificate on the resulting systems, rather than extrapolating a fitted exponent into a theorem.")



#heading(level: 2, text("Sources and execution limits"))

#text("- Primary source read directly: [Petit-Kosters-Messeng, PKC 2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf), especially sections 3.1-3.4 and 5. The authors explicitly leave solver complexity open and suggest dedicated generalized root-finding. Their factor bases and analysis are prior art, not a new discovery here. The retrieved PDF hash is in the local source manifest. - Primary source read from frozen extracted text: autoresearcher `inputs/NAGAO-2013-548/paper_fulltext.md`, sections 2-4; Riemann-Roch coefficient decomposition is prior art. The source records a retracted proposition and projective/affine gaps; no sub-exponential theorem is imported from it. - Internal predecessors read: IDEA-20260918-5a9a51, IDEA-20261029-3d0e71 and the four other current prime-field solver proposals; LEAD-ICP-shared-coefficient-membership; P-256 F1 and cubic-divisor reports/derivations; symmetric PKM source and raw overlapping rows; PKM tower and fall-degree derivations; current prime IC decisions; original CSV/raw traces and subsequent N=21 reports; ECBench report and rank-six raw records. See `inputs.sha256` for the recounted files. Independent theory, novelty and empirical reviews within this session corrected the draft; their public findings are retained under `reviews/`. - The current naive-IC literature row is heuristic and abstract-read. Direct publisher retrieval returned an empty stream; direct ePrint retrieval of additional papers returned HTTP 403. These sources are not represented as fully verified. This is not a complete current-literature novelty search. The native paper-author PDF above was accessible. - GitHub access worked. No AWS identity or EKS MCP is configured in this environment's runtime status; `aws` and the harness tools are absent. GitHub workflow enumeration is paginated (583 workflows); the active `isolab-experiment.yml` was located on page three and its dispatch schema was read. It takes an ecbench spec, not the arbitrary candidate solver proposed here. No compute job was submitted. There is also no implemented candidate solver yet, so even compute access alone would not produce the proposed measurement. - `conductor check` was attempted and returned command-not-found. The repository's Conductor documentation explicitly describes its missing-binary hooks as warning-and-continue; the report and standalone fixture tools occupy a new study directory. No shared solver source is changed. - The standalone native checks and audit pass. The prescribed full-library command `cargo test --release --lib` was attempted but cannot start because Cargo/Rust are not installed in this runtime; it is not a passing library check. The study does not modify the Rust library. This remains a CI/environment verification gap, separate from the passing C++ fixture checks. - Existing scoreboard graphs remain unchanged because no admitted complete-cost result or new curve/map is established. The new explanatory figure and recorded-width plot accompany this report. `REPORT.pdf` is a readable rendering of this report and both visuals; Markdown is the canonical text.")



#heading(level: 2, text("Requirement status"))

#table(columns: (1fr, 1fr), inset: 4pt, text(size: 8pt, "Requested requirement"), text(size: 8pt, "Status"), text(size: 8pt, "Locate all six named repositories"), text(size: 8pt, "Verified; default trees pinned locally"), text(size: 8pt, "Exhaust all prior code, history and experiments"), text(size: 8pt, "Partial; atlas census and focused source/raw review completed, full corpus/history review still open"), text(size: 8pt, "Reassess bad experiments rather than accepting prior verdicts"), text(size: 8pt, "Completed for the named cases above, not the entire corpus"), text(size: 8pt, "Empirical evidence"), text(size: 8pt, "Stored-record recount and native development checks completed; fresh solver and isolated performance replication not run"), text(size: 8pt, "Pick one opportunity"), text(size: 8pt, "Completed: growing-arity shared-coefficient decomposition"), text(size: 8pt, "Concrete implementation"), text(size: 8pt, "Known-answer encoding/export prototype implemented and checked; unknown-coefficient solver not implemented"), text(size: 8pt, "New algorithm rather than relabeled existing work"), text(size: 8pt, "Not established; ingredients and five-point precursor known, growing-arity delta proposed"), text(size: 8pt, "Sub-exponential prime-field ECDLP"), text(size: 8pt, "Not established; uniform solver bound, usable field family, coverage and rank supply remain open"))

#text("Recording this report does not resolve those open requirements. No deployed-curve break, universal impossibility, independent proof certification, or fresh experiment is claimed.")



#text("The existing [symmetric global-solver obstruction](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/problems/p256-symmetric-radical-global-solver.md) records the nearest implementation/proof gap for a different concrete base. This proposal additionally needs a uniform bound in both arity and depth, exact chart coverage, a usable field family, and useful relation rank. Exit criteria are the explicit procedure and cost bound above, followed by independent correctness/complexity review; a faster small refutation alone does not resolve the obstruction. The [mixed-fibre complete-cost problem](../../problems/mixed-fiber-pkm-full-solver.md) remains a separate cost-accounting gap, not evidence against all coefficient solvers.")
