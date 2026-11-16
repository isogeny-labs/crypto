# Retained orbit signs and a single-image fiber transition

T-1807, 2026-11-07. Derived algorithm and unsigned producer checks. [pcc:unverified]

## Mechanism and scope

The preceding power-fold adapter queried m individual signs and used only their product to update an image polynomial. It then queried a second image chart and intersected fibers. This successor preserves the individual signs in a bounded transcript, stops image queries as soon as one image is a singleton, and applies the retained signs to its degree-m scalar fiber. The second configured center is never queried. This is a change in information retained and extraction scheduling, not a new fast source norm.

The existing Fold and PublicRecovery production prefix is copied byte-for-byte by generate.py; generation.json pins the original and generated files. single.rs adds the new path without changing the historical construction. The existing half-GCD RootSet and source arithmetic are reused. Prime r, exact-order zeta, m dividing r-1, all field/cap/source assumptions remain as in feature-native-power-fold/DERIVATION.md. The existing public Source is still restricted to its certified prime-field domain; this addition does not broaden that domain. There is no scalar label in the public recovery API.

Sources checked (KB): no MCP tool loaded and scripts/research-kb status reports no executable. Read local power-fold source, standalone-source Ward covariance, earlier joint-window batching, and quotient-indicator/lifted-sign notes. DLMF27.9 (https://dlmf.nist.gov/27.9) fixes character conventions. Cheon-Kim2016 section1.2/2 (https://doi.org/10.1112/S1461157015000303) motivates inspecting residual polynomial fibers but requires auxiliary powers/collision lists that are not used here. The inspected Ward formulas do not themselves supply a cheaper orbit-product evaluator; no impossibility or novelty conclusion follows.

## Exact transition invariant (derived)

Let H=((a_i,s_i)) be all retained observations, s_i=chi_r(k+a_i). Let the current power-image root set reduce to the single value z=(k+c)^m. Define

    F(X)=(X+c)^m-z, if z!=0;
    F(X)=X+c,       if z=0.

The special zero case represents one distinct root rather than multiplicity m. A nonzero valid image satisfies z^d=1, d=(r-1)/m; lift checks this condition. F then splits into m distinct roots over F_r, since m<r and its roots are nonzero after translation. Let D_(a,s) denote the polynomial sign constraint already implemented by RootSet.observe. Replaying H produces

    R = monic_gcd(F, D_(a_1,s_1), ..., D_(a_L,s_L)).

Its roots are precisely the scalars t with (t+c)^m=z and chi_r(t+a_i)=s_i for every retained observation. Proof: F represents exactly the image fiber; intersection with each D retains exactly the corresponding sign class. Induction proves the conjunction invariant. The actual k remains unless an observation/image is inconsistent. A degree-one state returns a candidate only after [k]P=Q. An empty state is a verification error; a larger state remains unresolved or receives explicitly supplied extra offsets.

All observations from an orbit imply its folded product constraint. Therefore, once z is fixed, replaying the full signs preserves every prior image constraint and also restores information the product discarded. No source evaluation occurs in Fold::lift: it accepts only scalar-field data and constructs/intersects polynomials. Source values must already be bound to the same unknown input; the primitive does not authenticate arbitrary caller histories. The public adapter creates that binding from its own validated source calls.

## Quadratic orientation fact (derived)

If m=2, c=0, r=3 mod4 and k!=0, the image fiber is {k,-k}. The initial input-validation sign chi_r(k) is retained. Because chi_r(-1)=-1, that single saved observation distinguishes the two roots. Hence, conditional on obtaining the image singleton, the adapter needs no further source query for orientation. It still pays for the image phase. This explains the tested r479,m2 route; it is not an image-separation guarantee. For c!=0 one would need the corresponding centered sign, which is not assumed free.

## Residual separation bound (derived, not a randomized run)

For two distinct scalars k,t and uniform a in F_r, the number of shifts with chi(k+a)=chi(t+a) is exactly (r-3)/2. At a=-k or -t the signs differ. Away from -k, the map a -> (t+a)/(k+a) is a bijection onto F_r minus {1}; excluding a=-t removes zero. Equality of the two nonzero signs is equivalent to the ratio being a square. There are (r-1)/2 nonzero squares, and excluding1 leaves (r-3)/2.

For a fixed residual set of n candidates containing k, after q fresh independent uniform offsets (independent of the preceding history), the union bound on false survivors is

    Pr[any false candidate survives] <= (n-1)*((r-3)/(2r))^q.

Equivalently the expected false-survivor count is that expression. Here n<=m after lifting. Zero-source answers terminate immediately. This is a conditional mathematical bound on the residual phase; it says nothing about how soon the restricted folded shifts b^m identify z. The code takes an explicit canonical extra-offset list. It does not generate uniform randomness, and the deterministic test list is not covered by the probabilistic premise. No timing or statistical measurement is claimed.

## Resource accounting

Before lifting, one image polynomial has degree at most d+1 and L retained (u64,i8) observations. Vec tuple layout includes padding; use size_of::<(u64,i8)>() rather than assuming9 bytes. L is bounded by max_records and source queries by max_calls. The record cap is checked before a new source query. Each completed folded update still uses m separate source evaluations, m offset scalar multiplications by double-and-add, and the scalar-field modular-power/gcd work of the preceding construction. Source calibration is paid once; target validation is a charged query and its sign is retained.

On transition the fiber degree is at most m. During transition both image and fiber may coexist; afterward the image and transcript are dropped before extra queries. Replaying L signs costs L ordinary root updates on degree at most m: a conservative O(L*(M(m)*log r + M(m)*log m + m*log r)) modular-product bound, retaining the backend's exponentiation-based inversions rather than treating them as one modular product. Binary fiber construction has the existing safe O(M(m)*log m) bound. Source work during replay is zero. Polynomial workspace is conservatively O(d log(d+1)+m log(m+1)); add O(L) transcript slots, O(m) offsets, the supplied extra-offset list and source workspace. These are bounds, not RSS measurements.

Post-lift ordinary queries pay one source evaluation each plus shift and degree-m root-update work. The source query cap applies across both phases. The source counters describe outer calls and group work, not total internal field cost. No universal gain over the preceding algorithm is asserted: the first chart may exhaust its query schedule even if another center would separate, replay can cost more destination arithmetic, and a suitable divisor/source batch remains necessary for a favorable scaling tradeoff.

## Verification and formal map

Validation-01 passing source was preserved before extracting the post-lift refinement helper and adding branch coverage. Final validation-02 adds direct remaining-fiber and exhaustion controls; the prior source snapshot and receipt are retained. The full original4,563-case cohort was not rerun. Native overflow checks and normal-build dependency exclusions remain enabled. Tests use scalar labels only for known-answer references, never as production inputs.

Replay.lean, FoldFiber.replay_exact, proves that recursively intersecting arbitrary predicate constraints yields exactly the initial predicate conjoined with all observations. This is the representation-transition logical invariant; it does not formalize the finite-field polynomial-root interpretation, image factorization, quadratic character theorem or probability bound. Lean4.34.1; #print axioms reports only standard propext. No sorry, custom axiom, native_decide or bypass is used. The previous formal project still lacks built Mathlib; the full arithmetic formalization is open. No lean4checker executable is available. The report/receipt pin source and toolchain.

Next: exploit the retained signs earlier or accelerate the source orbit product; investigate image separation separately from residual orientation. The remaining-image bottleneck is explicit and is not replaced by the residual probability bound.

## Recorded final native output

```text
running 4 tests
test tests::exact_lift_and_retained_prefixes ... lift_cases=897 prefix_sets=5382 final_singletons=897 zero_images=23
ok
test tests::exact_pair_separation_count ... pair_separation_counts=1420
ok
test tests::public_single_chart_and_boundaries ... public_cases=46 recovered=46 unresolved=0 retained_routes=28 extra_routes=0 endpoint_identity=18 source_calls=462
ok
test tests::remaining_fiber_queries_and_exhaustion ... conditional_fiber_cases=12 ordinary_routes=9 zero_routes=3 exhausted_controls=12
ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 157 filtered out; finished in 0.42s
```

The twelve conditional-fiber cases call the residual helper with a reference-derived image and no history. They validate that helper, not end-to-end image acquisition. The46 public cases include the earlier frozen20 and all13 small-curve targets with both m3 and m6. Their28 retained-sign routes used zero extra source queries;18 exited on identity or source zero. All31 source/input hashes and the unchanged-prefix generation hashes match. These are producer checks, not signed independent verification.
