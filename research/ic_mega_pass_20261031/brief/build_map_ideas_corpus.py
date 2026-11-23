#!/usr/bin/env python3
"""Builds MAP_ideas_corpus.json: digest of the autoresearcher ideas corpus for the 2026-10-31 pass."""
import json, pathlib

ROOT = "/Volumes/SSD990-2/crypto-autoresearcher"
IDEAS = f"{ROOT}/ideas"
DEF = f"{ROOT}/ideas/deferred"
PRYM = f"{ROOT}/research/prym_leads_20261027"
THM = f"{ROOT}/research"
KO = f"{ROOT}/knowledge/open-problems"

items = []

def add(name, statement, status, scope_limits, source, **kw):
    d = dict(name=name, statement=statement, status=status, scope_limits=scope_limits, source=source)
    d.update(kw)
    items.append(d)

# ---------------------------------------------------------------------------
# A. ACTIVE hypothesis cards (ideas/*.md) -- all proposed_unapproved, no run executed
# ---------------------------------------------------------------------------
A = "proposed (proposed_unapproved; zero runs; novelty_unverified)"

add("ECDLP-IDEA-002 Split-Jacobian projected smoothness",
    "Pull E(F_p)[l] by conorm pi^* into J(C) for a bounded-degree cover pi:C->E with J(C)~E x A; reduce conorm-image classes to divisors supported on 'atoms' whose norms land in a small E-factor base; push relations back by norm (pi_* pi^* = [d]). Claim: conorm image has higher E-projected smoothness than random classes of J(C), enough for lambda<1/2. Likely fatal: reduced divisors in an elliptic isogeny factor are no smoother than random; fixed degree gives constants, growing degree raises genus (cost >= p^{2-2/g}). The P1546 producer receipt (fixed-branch counting gate) later bounds source capture per branch at O(Delta*d*B) and conserves relation work in explicit catalogues; the open escape is only an implicit/adaptive/target-dependent router.",
    A + "; P1546 receipt closes fixed-branch/explicit-catalogue arm",
    "Covers only homomorphic conorm/norm into a split Jacobian over the SAME prime field with explicit bounded-degree covers and fixed rational branches. Does not cover: non-homomorphic covers (IDEA-010), extension-field GHS/cover transfer (known), growing-degree or implicit-membership atom descriptions, Prym (non-E) components used as auxiliary constraints (prym follow-ups), genus-3 Pryms (2-dimensional).",
    f"{IDEAS}/ECDLP-IDEA-002_split_jacobian_projected_smoothness_hypothesis.md")

add("ECDLP-IDEA-003 Partial scalar-power correspondence",
    "A target-dependent multivalued algebraic correspondence plus deterministic branch rule that, on a density l^{-delta} subset of randomized instances, outputs Z=[x^D]P for a public divisor D | l-1, D~sqrt(l); then Cheon's auxiliary-input algorithm gives total exponent max(kappa+delta, alpha/2, (1-alpha)/2) < 1/2. Likely fatal: every morphism fixing O is a homomorphism hence linear in x; branch selection compatible with x^D is itself DLP-hard or enumerates l^{1/2} branches; broad version is the (excluded) self-bilinear map.",
    A + "; no correspondence specification exists (next action = write it)",
    "Covers only nonlinear scalar maps x -> x^D realised by correspondences on E itself. Does not cover: pairings (IDEA-008), multilinear maps, or any linear/affine scalar map (endomorphisms, Frobenius, isogenies are controls). Requires l-1 to have a divisor near sqrt(l): curves without it are outside applicability.",
    f"{IDEAS}/ECDLP-IDEA-003_partial_scalar_power_correspondence_hypothesis.md")

add("ECDLP-IDEA-004 Prime-to-p jet logarithm",
    "Lift E and points to a fixed-order infinitesimal / Witt-vector neighbourhood, cancel lift-dependent terms with a Frobenius/deformation cocycle, and extract an additive l-primary jet coordinate J with J([n]R)=nJ(R), J(P)!=0, for l != p. Likely fatal: formal-group kernel is pro-p and reduction is injective on prime-to-p torsion, so any finite-order deformation functional vanishes or factors through an implicit order-l module DLP; THM_JETBARRIER1 proves all jet queries in the jet-augmented GGM are simulable with zero oracle overhead (KN-FIND-002), and P1547 (artifacts/ECDLP-IDEA-004) is the prime-to-p jet coordinate gate.",
    A + "; GGM-simulability (THM_JETBARRIER1 T4) closes the in-model version",
    "Covers additive/logarithmic functionals on p-adic or infinitesimal lifts for l != p. Does not cover: non-additive coordinate/valuation functionals (IDEA-436, KN-OPEN-3417fc), ramification/field-of-norms invariants (IDEA-160), anomalous l=p (known Smart/SSSA), or models using coordinate data of composite (non-public) elements where THM_JETBARRIER1 T6 says genericity collapses.",
    f"{IDEAS}/ECDLP-IDEA-004_prime_to_p_jet_logarithm_hypothesis.md")

add("ECDLP-IDEA-005 Height-compressing global lift",
    "Deterministic global lift of E/F_p, factor base and targets to a curve over a number field with a coupled point-lift rule whose denominator ideals share controlled support, so finite-field relations become provably short (height exponent h<1/2) Mordell-Weil relations recovered by ideal factorization + lattice reduction and reduced mod p. Likely fatal: no canonical lift preserves an unknown group relation as a short relation; h-hat(P+Q)+h-hat(P-Q)=2h(P)+2h(Q) (Brief 'Heights' wall); xedni coefficient growth; number-field degree/discriminant may grow with p; selection bias of favourable lifts. Face F5 (function-field F_p(t) lifts) depends on the IDEA-435 target-coefficient audit.",
    A + "; P1543 R1 independent audit (artifacts/ECDLP-IDEA-005) strengthens global-lift controls",
    "Covers number-field and function-field global lifts with LLL/ideal recovery. Does not cover: CM canonical lifts specifically (atlas open_next_step), lifts to Q_p (face F2: IDEA-436), lifts that change the group (covers), or a dimension count over all section shapes (missing per Brief).",
    f"{IDEAS}/ECDLP-IDEA-005_height_compressing_global_lift_hypothesis.md")

add("ECDLP-IDEA-006 Elliptic-net short annihilator",
    "Build exact Hankel/Toeplitz-like blocks from a rank-2 elliptic net W(a,b) for (P,Q=[x]P), recover a short displacement annihilator of order N^rho with rho<=0.18, and read the target shift x from its root/eigenvalue data without an order-N DLP. Likely fatal: useful net period is N with linear complexity N^{1-o(1)}; Lauter-Stange equivalence; eigenvalue labelling is itself an order-N DLP; low toy rank may be normalisation degeneracy. External screening (EXTERNAL_CORPUS block-Hankel successor) found exact linear-state dimension r on every curve.",
    A + "; contract 006 exists, unapproved",
    "Covers linear/displacement-rank structure of EDS/elliptic-net sequences. Does not cover: nonlinear state models of the net (explicitly left open by the external screen), elliptic nets as membership filters, or division-polynomial character sums (Stange-Shparlinski is cited as pseudorandomness evidence).",
    f"{IDEAS}/ECDLP-IDEA-006_elliptic_net_short_annihilator_hypothesis.md")

add("ECDLP-IDEA-007 Miller S-unit descent",
    "Precompute a dictionary of Miller line functions / an S-unit-style module of rational functions with divisors supported on a replacement base S u {O}; a reducer finds, for random R, a short Miller program f_R with div(f_R)=(R)-(O)+sum e_i((F_i)-(O)), certifying R+sum e_i F_i=O. Likely fatal: on E a divisor is principal iff the group relation holds, so 'support finding' is decomposition in disguise; Miller programs evaluate functions after the relation is known; pairing specialisation becomes MOV/Frey-Rueck.",
    A + "; P1541 strengthened the S-unit control (ideas README)",
    "Covers function-field / principal-divisor representations of relations on E itself. Does not cover: Miller cocycles as non-homomorphic transfers into F_p^* with trivial pairing (Brief §3 item 10, unexamined), Miller coboundary HDX views (IDEA-164), or S-units in number fields (IDEA-005).",
    f"{IDEAS}/ECDLP-IDEA-007_miller_s_unit_descent_hypothesis.md")

add("ECDLP-IDEA-008 Partial pairing-return cycle",
    "Two scalar-compatible lifts into a pairing environment plus a partial, efficiently recognisable return map G_T -> G realise R([a]P,[b]P)=[ab]P on a cycle-level subset of density N^{-Delta}; iterate O(log N) returns to get [x^D]P, then Cheon. Likely fatal: total return = self-bilinear map (Cheon-Lee: collapses DH); ordinary curves lack distortion maps between Frobenius eigenspaces; per-gate densities multiply to Delta>=1/2; if it works on a pairing-friendly curve, MOV is the real attack.",
    A + "; P1542 strengthened pairing-return control",
    "Covers bilinear pairing + return on the same group. Does not cover: pairings used as transfers (MOV, known), pairings with trivial embedding (Brief 'Miller cocycles when the pairing is trivial', unexamined), or trilinear/multilinear constructions (Boneh-Silverberg barrier cited).",
    f"{IDEAS}/ECDLP-IDEA-008_partial_pairing_return_cycle_hypothesis.md")

add("ECDLP-IDEA-009 Nonequivariant trace-zero transfer",
    "Explicit Frobenius-NONequivariant scalar-compatible map tau: G -> T_k(E)[N] (trace-zero variety of E over F_{p^k}) whose image lies in a low-complexity locus Z where Gorla-Massierer trace-zero decomposition is cheaper. Ordinary inclusion is excluded (Tr o i = [k], so i(G) meets T_k only in O for N not dividing k). Likely fatal: every F_p-morphism commutes with Frobenius and fixes the rational line; ordinary End is commutative so no distortion between eigenspaces; image is still cyclic of order N and trace-zero summation-polynomial elimination restores 1/2.",
    A,
    "Covers transfers of the rational N-torsion line into trace-zero subvarieties of base extensions of E. Does not cover: trace-zero index calculus on curves already defined over extension fields (known, Gorla-Massierer), Weil restriction of other abelian varieties, or supersingular curves where distortion maps exist (MOV regime).",
    f"{IDEAS}/ECDLP-IDEA-009_nonequivariant_trace_zero_transfer_hypothesis.md")

add("ECDLP-IDEA-010 Torsor deck-orbit descent",
    "Public cover/torsor pi:X->E with deck group Gamma and degree d=N^alpha; lift R multivaluedly, canonicalise selected fibre orbits, build relations among precomputed deck-orbit atoms upstairs, push forward by norm. Non-homomorphic (unlike IDEA-002). Likely fatal: fixed d gives constant branch multiplicity; growing d costs N^alpha to construct/solve fibres; scalar-compatible branch selector encodes the DLP; Lang's theorem trivialises torsors without supplying a cheap branch; pushforward averages orbits to zero/known multiples.",
    A,
    "Covers non-homomorphic covers and torsors of E over the same field with deck-orbit canonicalisation. Does not cover: covers whose Jacobian is used as the relation group (IDEA-002/prym), Kani/Richelot gluing for a second N-torsion direction (closed lead LEAD-TRF-second-torsion-direction; Brief §3 item 10), or extension-field covers (GHS, known).",
    f"{IDEAS}/ECDLP-IDEA-010_torsor_deck_orbit_descent_hypothesis.md")

add("ECDLP-IDEA-011 Scalar-orbit elliptic-period descent",
    "Quotient <P>\\{O} by a multiplicative subgroup H <= (Z/l)^* and evaluate an H-invariant 'elliptic period' I_H(R)=sum_{h in H} x([h]R) (or an orbit-polynomial coefficient) in time o(|H|); refine through a chain {1}<H_1<...<(Z/l)^* to locate the coset of x. Likely fatal: (E,R) is a point of X_1(l) level structure, scalar action is by diamond operators; separating functions on large orbits have degree/evaluation cost comparable to the orbit (gonality of X_1(N), Derickx-van Hoeij, Poonen); period collisions. P1530 R1-R2 audit later isolated a 'partial elliptic-period type-2 successor' (ideas README) with Gallant types as controls.",
    A + "; P1540 strengthened source-orbit control",
    "Covers multiplicative scalar-orbit invariants of x-coordinates with subgroup chains in (Z/l)^*. Does not cover: Couveignes-Lercier elliptic periods as FIELD arithmetic/normal bases (KN-OPEN-095df5, 5e3dd0, Brief §3 item 6), additive-orbit (translation) invariants, or the type-2 partial-period successor named by P1530.",
    f"{IDEAS}/ECDLP-IDEA-011_scalar_orbit_elliptic_period_descent_hypothesis.md")

add("ECDLP-IDEA-012 Aggregate complement-divisor compression",
    "Represent an unordered decomposition D=P_1+...+P_m by one section of the degree-m line bundle whose zero divisor is D and the target condition by the Abel-Jacobi fibre; push the factor-base product tree through the aggregate section/complement so a witness query costs B^kappa with kappa<=0.25, never materialising F_p[z]/f_F(z). Likely fatal: Abel-Jacobi pushforward of a factor-base indicator is group convolution on E(F_p) whose natural index is the DLP; exact implementations touch Omega(B) data (kappa>=1) or return only counts. Must beat the 3SUM-with-preprocessing baseline.",
    A + "; contract 012 exists",
    "Covers prime-field membership-quotient avoidance for the Semaev m-sum witness oracle. Does not cover: binary/extension fields, the measured D_m-invariant 2-torsion coordinate route for S_4 (Brief §3 item 2), or x-functional filters (KN-OPEN-91d4f6).",
    f"{IDEAS}/ECDLP-IDEA-012_aggregate_complement_divisor_compression_hypothesis.md")

add("ECDLP-IDEA-050 Spinor-matchgate addition transform",
    "A public target-independent linear basis change on exact elliptic-addition signatures (all charts, denominators, signs) mapping the factor-base-to-target relation tensor to a matchgate/spinor signature certified by Grassmann-Pluecker identities, so the global contraction is a planar Pfaffian of width N^w and Pfaffian minors self-reduce to source points (Valiant holographic algorithms). Likely fatal: generic signatures violate matchgate identities in every basis; planarisation needs width N^{1/2}; source recovery needs quadratically many conditioned Pfaffians.",
    A + "; contract 050 (symbolic identity preflight) exists",
    "Covers holographic/matchgate tensor-network rewriting of the relation tensor. Does not cover: other tensor-network contractions (KN-OPEN-007 bond rank), Yang-Baxter integrable models (IDEA-102), or exterior-algebra wedge identities (IDEA-052).",
    f"{IDEAS}/ECDLP-IDEA-050_spinor_matchgate_addition_transform_hypothesis.md")

add("ECDLP-IDEA-056 Block-Krylov transition-intersection extractor",
    "Given the ledger's accepted logarithmic one-transition subgroup-norm oracle on a deck of L=N^beta x-values, represent two consecutive transitions as implicit multiplication operators M_1,M_2 on a shared module A_L of dimension L^{1+o(1)}; a common-state nullspace/minimal-polynomial identity holds iff the two-transition relation has a source witness, extracted by block Wiedemann in L^alpha, alpha<=1.25, without the dense L^2 resultant. Likely fatal: the two transition algebras need not share a small faithful module (minimal polynomial degree Theta(L^2) = dense resultant), eigenvector-to-endpoint lift enumerates pairs, emitted rows too dependent.",
    A + "; contract 056 Phase 1 (biconditional machine check) is mandatory first; successor of P1478/ECFG-MX-1478",
    "Covers Krylov/structured-linear-algebra evaluation of a two-transition norm identity. Does not cover: the common-norm gcd of IDEA-121, Kedlaya-Umans routes, or any route that materialises the L^2 pair matrix (control).",
    f"{IDEAS}/ECDLP-IDEA-056_block_krylov_transition_intersection_extractor_hypothesis.md")

add("ECDLP-IDEA-059 Cremona-shrunk toric decomposition",
    "A public Cremona (birational) transformation built from elliptic addition-law syzygies that cancels the dominant Newton faces of the symmetric point-decomposition system before elimination, so saturated mixed volume drops to N^v, v<1/2, while a rational inverse recovers source points; sparse resultants downstream. Likely fatal: birational degree conservation moves degree into exceptional divisors, coefficient heights, or inverse branches; saturation restores mixed volume (THM_BKKMV1/2 prove the sectioned family is box-saturated, MV_m=(m-1)!2^{(m-1)(m-2)} for m<=5, conjectured all m).",
    A + "; contract 059 exists; THM_BKKMV1/2 are the relevant barrier theorems",
    "Covers birational/toric re-embeddings of the Semaev system over F_p. Does not cover: Weil-descended binary systems, non-birational (e.g. cover) representations, or liftings that decrease mixed volume by adding equations (Borger et al. positive control).",
    f"{IDEAS}/ECDLP-IDEA-059_cremona_shrunk_toric_decomposition_hypothesis.md")

add("ECDLP-IDEA-158 Nonfaithful x-only WNU signed lift",
    "An x-coordinate relation language that provably does NOT primitive-positive define signed elliptic addition, carrying a non-affine weak-near-unanimity (WNU) polymorphism preserving a sparse x-factor base (Barto-Kozik bounded width), plus a public cocycle lifting every quotient solution to all exact signed source tuples. Four theorem receipts close: full S3/Kummer realisation (4 Kummer states pp-interpret an oriented element), all full affine S_m for m>=4/5 (pin to affine S3), fixed proper sign-branch subsets (not lift-invariant). Survivor: an induced-template router (implicit support/witness recurrence with setup <= B^2.25, query <= B^1.25) plus WNU plus exact source lift.",
    "proposed (theorem-preflight only; state proposed_unapproved_fixed_branch_deletion_scoped_negative_induced_template_support_router_open)",
    "Covers CSP/universal-algebra (polymorphism) treatment of x-only Semaev relations over F_p. Does not cover: Maltsev/affine structure (closed IDEA-122), faithful signatures (closed IDEA-146), SAT/CSP solver substitution (P1480 control), or non-WNU operations.",
    f"{IDEAS}/ECDLP-IDEA-158_x_only_nonfaithful_wnu_signed_lift_hypothesis.md")

add("ECDLP-IDEA-159 Non-diagonal conormal-polar source blowup",
    "A target-independent polar/Jacobian ideal J on the source-labelled five-point addition incidence, nonunit on the generic all-distinct etale stratum, whose normalised Rees blowup has exceptional valuations inverting biconditionally to signed sources. The producer theorem CLOSES the ordinary-ideal escape: at generic points of the reduced incidence the local ring is a field, so any coherent ideal stalk is 0 or unit; Cartier centres blow up trivially; proper critical centres cannot label generic tuples.",
    "proposed_unapproved_generic_stalk_rees_scoped_negative_pending_review (effectively refuted for all ordinary coherent ideals)",
    "Closure covers every ordinary coherent ideal / Rees blowup on the reduced incidence. Does not cover: non-ordinary representations (e.g. non-reduced structure, differential operators, D-modules), or a compact source-component rule supplied by other means.",
    f"{IDEAS}/ECDLP-IDEA-159_non_diagonal_conormal_polar_source_blowup_hypothesis.md")

add("ECDLP-IDEA-160 Nonlogarithmic ramification-break scalar digits",
    "A target-uniform lift of <P> into a tower of local fields whose upper/lower ramification breaks or field-of-norms data (Wintenberger, Sen) are scalar-sensitive and non-logarithmic, returning digits of x. Producer theorem CLOSES towers functorially generated by the subgroup, full torsion or full division fibres: every nonzero Q=[x]P generates the same subgroup and field as P; at good reduction the N-primary tower is unramified (Serre-Tate); p-division fibres add target-independent torsion fields. Only an explicitly oriented nonfunctorial branch selector remains.",
    "proposed_unapproved_functorial_ramification_scoped_negative_pending_review (functorial towers refuted); P1544 R1 audit filed",
    "Closure covers all functorial towers over the subgroup/torsion. Does not cover: non-functorial oriented branch selectors (none specified), p-adic coordinate valuation profiles on a fixed curve (IDEA-436), or wild ramification of covers of E (different object).",
    f"{IDEAS}/ECDLP-IDEA-160_nonlogarithmic_ramification_break_scalar_digits_hypothesis.md")

add("ECDLP-IDEA-434 Isogeny-class variation of Semaev decomposition yield",
    "Measurement: fix p, trace t, order N, m and a canonical factor-base rule (x in [0,B)); test whether mean m-th Semaev decomposition yield Y(E,B,m) varies across the F_p-isogeny class C_t by a factor growing with p (null: isogeny-class invariant). Motivated by Smith's genus-3 isogeny stratum change. Expected outcome: negative, strengthening KN-FIND-007 (yield conservation at fixed curve) into a class-invariance statement, since monodromy is C_2^{m-2} for every curve (KN-FIND-a1f3c2) and transport conservation (Brief) says isogenies change ordered m-decomposition counts by exactly |kernel|^{m-1}.",
    "proposed_unapproved_pending_review; external ISOGENY_SEMAEV_REVIEW already reports 'solving complexity behaves as an isogeny-class invariant' (three null channels)",
    "Covers mean yield at fixed canonical base across an isogeny class over F_p. Does not cover: yield VARIANCE (left free by the monodromy closure), coverage (which targets decompose at all), extension fields, curve models/symmetries at fixed curve (KN-OPEN-003, KN-FIND-007), or solving degree per relation.",
    f"{IDEAS}/ECDLP-IDEA-434_isogeny_class_decomposition_yield_invariance_hypothesis.md")

add("ECDLP-IDEA-435 Target-coefficient audit of the function-field lifting face",
    "Control: recompute, for every infinity-norm-1 Mordell-Weil relation recorded in EV-XEDN-* (face F5: global nontorsion lifts over F_p(t)), the coefficient on the TARGET section. H-B (expected): all are zero, i.e. the relations live in the target-independent part of MW ('target-blindness', the F5 analogue of Masser independence), closing the lane; H-A: a nonzero coefficient would be an anomaly contradicting h(T^)=m^2 h(S^). Also decides whether the original design even had a distinguished target section.",
    "proposed_unapproved_pending_review (control, not an algorithm)",
    "Covers only the program's own EV-XEDN evidence on fixed surface families at p<=31. Does not cover: new lifting constructions, larger p, or number-field (face F4b) lifts; outcome gates IDEA-005.",
    f"{IDEAS}/ECDLP-IDEA-435_function_field_lift_target_coefficient_audit_hypothesis.md")

add("ECDLP-IDEA-436 Coordinate-valuation profile of the canonical prime-to-p torsion lift",
    "Face F2: lift S of prime-to-p order n to the unique S^ in E(Q_p)[n]; KN-TECH-73630e shows reduction is a group isomorphism so the lift is information-theoretically empty. Hypothesis: a coordinate/valuation functional inv_{j,r}(k)=v_p(x([k]S^)-x([j]S^)) truncated at precision r, or digit statistics, is non-constant in k and efficiently invertible with precision r=N^{rho_r}, rho_r<1/2, giving a purely computational handle. Gate: GGM-simulability memo first (KN-FIND-002/b7e091 would close it without running). Fatal candidates: precision growth rho_r>=1/2; simulability.",
    "proposed_unapproved_pending_review; GGM-simulability gate not yet written",
    "Covers non-group-theoretic coordinate functionals on the canonical Q_p torsion lift of a fixed curve. Does not cover: formal-group logarithms (closed F2 classical route), ramification towers (IDEA-160), digit character sums of the torsion section (KN-OPEN-d3a6f0, which is the Fourier-mass version), or CM canonical lifts over number fields.",
    f"{IDEAS}/ECDLP-IDEA-436_local_torsion_coordinate_valuation_profile_hypothesis.md")

# ---------------------------------------------------------------------------
# B. DEFERRED cards (ideas/deferred/*.md) -- theorem-gated, no contract, no run
# ---------------------------------------------------------------------------
D = "deferred (theorem gate unmet; no contract; zero runs)"

add("ECDLP-IDEA-049 Bounded-root decomposition transducer",
    "Public integral lift of complete elliptic addition encoding addition-law carries and denominator charts so that an m-point factor-base decomposition of R becomes a certified small integer root (bounded N^eta) of a polynomial system amenable to rigorous multivariate Coppersmith small-root recovery in N^c, c<1/2, inverting to actual sources. Fatal: modular wraparound makes carries modulus-sized; eliminating carries recreates the dense membership resultant; lattice dimension or determinant fails. Gate: generic root-height and lattice-determinant inequalities with exact source inversion.",
    D + " (deferred_missing_height_theorem)",
    "Covers integer-lift + Coppersmith routes for the m-sum decomposition. Does not cover: the D4-symmetrised S_5 lattice at m=4 (Brief lever 3: never run), lattices over the x-as-integer representation without carries (Brief 'Lattice reach' wall: Jochemsz-May delta_max=1/(m(m+1)2^{m-2})), or binary fields.",
    f"{DEF}/ECDLP-IDEA-049_bounded_root_decomposition_transducer_hypothesis.md")

add("ECDLP-IDEA-052 Elliptic wedge-witness identity",
    "A curve-specific exterior-algebra identity whose antisymmetrised evaluation annihilates all non-decompositions while encoding source indices of every decomposition in N^r exterior coordinates, r<1/2, with exact coefficient extraction avoiding the B^2 pair surface. Fatal: wedge space dimension Omega(B^2); annihilates repeated-source relations; nonzero determinant certifies existence while extraction enumerates pairs.",
    D + " (deferred_missing_identity)",
    "Covers exterior-algebra / colour-coding style certificates derived from the curve law. Does not cover: generic exterior algebra, Pluecker restatements (IDEA-050), or Pfaffian/matchgate contraction.",
    f"{DEF}/ECDLP-IDEA-052_elliptic_wedge_witness_identity_hypothesis.md")

add("ECDLP-IDEA-053 Aggregate-moment large-prime decoder",
    "Implicit elliptic endpoint-moment oracle: traces/norms of a factored addition-law algebra return power sums of all successful large-prime endpoints of a batch (k=N^sigma endpoints) in N^u, u<1/2, without enumerating edges; Prony-style sparse interpolation then recovers endpoint keys, source tags and cycles. Fatal: first nontrivial moment needs the dense resultant or explicit edge stream; source tags vanish under aggregation; large-prime cycle density keeps the 2/3-style floor.",
    D + " (deferred_missing_moment_oracle)",
    "Covers large-prime variation with implicit moment/trace oracles and sparse interpolation. Does not cover: explicit large-prime graphs/IBLTs (controls), residual/partial relations with signed stitching (prym IDEA-20261027-ef2530, LEAD-ICP-residual-partial-relations), or x-functional bucket filters (KN-OPEN-91d4f6).",
    f"{DEF}/ECDLP-IDEA-053_aggregate_moment_large_prime_decoder_hypothesis.md")

add("ECDLP-IDEA-058 Quadratic-phase stabilizer witness decomposition",
    "Exact expansion of the factor-base decomposition indicator into R=N^r quadratic-phase (stabilizer/affine-symplectic + quadratic character) signatures, r<1/2, enabling batched contraction and exact conditional marginals revealing source indices. Fatal: exact phase rank inherits the balanced ordinary-rank floor; cyclotomic precision grows as N^{1/2}; conditioning multiplies rank by B.",
    D + " (deferred_missing_curve_specific_phase_identity)",
    "Covers quadratic-character / stabilizer-formalism expansions of the indicator. Does not cover: additive-character Fourier analysis of chord profiles (THM_INCBARRIER1/2), finite Weil/metaplectic transforms (rejected IDEA-101), or approximate (sampled) character tests.",
    f"{DEF}/ECDLP-IDEA-058_quadratic_phase_stabilizer_witness_decomposition_hypothesis.md")

add("ECDLP-IDEA-064 Algebraic moving-core design",
    "Explicit finite-geometry block design of Hesse/cover parameter cores with bounded pairwise overlap and a nonzero universal rank minor, realised algebraically in cover pencils so that every block exports fresh source rows (linear fresh-rank growth) before certificate materialisation. Fatal: combinatorial overlap does not control rank of elliptic coefficients; realisable cores are a tiny subfamily; enforcing the design lowers hit density by exactly the rank gain.",
    D + " (deferred_missing_realization_theorem)",
    "Covers block-design scheduling of relation generation in cover/pencil families. Does not cover: finite geometry used as the factor base itself (projective planes, ovoids: no card), or sieve/relation-matrix structure (KN-OPEN-006 AP displacement rank).",
    f"{DEF}/ECDLP-IDEA-064_algebraic_moving_core_design_hypothesis.md")

add("ECDLP-IDEA-068 Pre-event elimination motif generator",
    "A bounded-degree rational SECTION of the elimination (projection) map of the universal source-incidence variety for useful support motifs, evaluated on cheap pre-event curve/source data to generate complete source forms carrying the motif before root work. Fatal: projection fibres have large monodromy so no rational section exists generically (choosing a branch is the root problem); sections on special loci have negligible density. Late P1513 receipts under this root preserve the shared bivariate common-norm identity routed to IDEA-121.",
    D + " (deferred_missing_constructive_section_identity)",
    "Covers constructive sections of elimination maps. Does not cover: discriminants used as classifiers, learned selectors, or Hilbert-irreducibility/specialisation arguments about when sections exist (no card).",
    f"{DEF}/ECDLP-IDEA-068_pre_event_elimination_motif_generator_hypothesis.md")

add("ECDLP-IDEA-069 Massey secondary-relation functional",
    "A target-independent finite collection of line bundles / defining systems on E/F_p whose triple Massey product m_3 defines a computable secondary functional mu on source-labelled divisor relations that survives quotienting by all functionals induced by known maps, obeys an A_infinity composition law, and yields a sub-sqrt-evaluable equation in the masked scalar. Fatal: additive functionals into char-p groups kill <P> for N!=p; multiplicative values move the DLP to mu_N; Massey products depend on defining systems and reduce to theta/pairing data.",
    D + " (deferred_missing_finite_field_defining_system)",
    "Covers higher (secondary) cohomological operations on E. Does not cover: primary pairings (IDEA-008), Bloch/polylog regulators (IDEA-119), theta/Heisenberg extensions (IDEA-161), or operads/A_infinity structures beyond m_3.",
    f"{DEF}/ECDLP-IDEA-069_massey_secondary_relation_functional_hypothesis.md")

add("ECDLP-IDEA-073 Algebraic discrete-Morse source contraction",
    "Partial factor-base decompositions ending at R form an implicitly evaluable chain complex C_R; a target-independent acyclic matching cancels non-witness cells leaving N^gamma critical cells, gamma<1/2, and the chain homotopy lifts critical cells to exact sources. Fatal: natural complex has Theta(B^m) cells and every tuple is a witness for some target so a universal matching must retain all; a target-aware matching is the missing oracle; homology does not preserve individual ancestry.",
    D + " (deferred_theorem_required)",
    "Covers discrete-Morse / chain-complex reduction of the decomposition search. Does not cover: persistent homology, sheaf cohomology on expanders (IDEA-164, rejected 132/236), or Koszul/syzygy-based degree-fall accounting (THM_SEMAEV_FALL1).",
    f"{DEF}/ECDLP-IDEA-073_algebraic_discrete_morse_source_contraction_hypothesis.md")

add("ECDLP-IDEA-075 Coregular genus-one orbit composition",
    "Bounded-dimensional coregular representation (Bhargava-style invariant theory) whose rational orbits encode a marked genus-one curve and point, with an explicit orbit composition realising elliptic addition; a smooth-orbit factorisation plus canonical inverse returns source points. Fatal: coregular invariants quotient out the group action and forget orientation; orbit minimisation/factorisation is an arithmetic reduction unrelated to addition; canonicalisation requires the Selmer/torsor class or an N-sized stabiliser.",
    D + " (deferred_theorem_required)",
    "Covers invariant-theoretic (coregular space) encodings of (E,P). Does not cover: Selmer-group / n-descent arithmetic over number fields as a handle (no card: only cited), GIT quotients of the incidence variety, or Cox rings (IDEA-086).",
    f"{DEF}/ECDLP-IDEA-075_coregular_genus_one_orbit_composition_hypothesis.md")

add("ECDLP-IDEA-076 Cluster-scattering broken-line coordinate",
    "A target-independent cluster/scattering-diagram atlas for a marked elliptic addition space whose canonical broken-line (tropical) charges c(R) obey a bounded-complexity piecewise-additive law under c([a]P+[b]Q) and separate the hidden scalar after sub-rho refinement. Fatal: elliptic curves are proper, not cluster tori; charges label monomials not group elements; separating N multiples needs Omega(N) chambers or a theta basis indexed by the DLP.",
    D + " (deferred_theorem_required)",
    "Covers cluster algebras / Gross-Hacking-Keel-Kontsevich scattering and theta functions on cluster varieties. Does not cover: mirror symmetry of elliptic curves proper (Fukaya category / SYZ) or log-geometry degenerations (IDEA-087).",
    f"{DEF}/ECDLP-IDEA-076_cluster_scattering_broken_line_coordinate_hypothesis.md")

add("ECDLP-IDEA-086 Cox-ring universal-torsor factorization",
    "Marked elliptic-addition variety X (fixed-output incidence of dim m-1) with finitely generated Cox ring; lifting the relation equation to the universal torsor makes it factorial, and graded atom factorisation plus the quotient map inverts to signed sources. Fatal: compactified addition/blowup varieties are unlikely Mori dream spaces (Picard-zero contribution makes Cox(X) non-finitely generated); factorisation is up to torus action on sections, not tuples; retaining enough multigrading equals the dense incidence object.",
    D + " (deferred_theorem_required)",
    "Covers Cox rings / universal torsors / Mori dream spaces of the addition incidence. Does not cover: toric homogenisation alone (IDEA-059 control), or Brauer-Manin/descent obstructions on torsors (no card).",
    f"{DEF}/ECDLP-IDEA-086_cox_ring_universal_torsor_factorization_hypothesis.md")

add("ECDLP-IDEA-087 Log-stable-map degeneration atomization",
    "Degenerate the universal m-source addition correspondence to a basic stable-log-map problem (Gross-Siebert / Abramovich-Chen) whose special fibre is stratified by tropical gluing types; for each output R the evaluation fibre product has components in bijection with signed source tuples, with a constructive component/source inverse. Fatal: degeneration formulas organise virtual classes and weighted counts, not finite-field points with ancestry; many tuples share a gluing type; expansions and gluing trees grow combinatorially; output Theta(B^m).",
    D + " (deferred_theorem_required)",
    "Covers Gromov-Witten / log-geometric degeneration of the addition correspondence. Does not cover: tropical atlases of the source set itself (IDEA-103), Berkovich/analytic skeleta beyond those, or Hurwitz-space / branched-cover degenerations.",
    f"{DEF}/ECDLP-IDEA-087_log_stable_map_degeneration_atomization_hypothesis.md")

add("ECDLP-IDEA-098 Squarefree Groebner-shelling source lift",
    "A target-independent weight order under which the source-labelled relation ideal has a squarefree (Stanley-Reisner) initial ideal whose facets encode signed source tuples; shelling/vertex decomposition enumerates only accepted facets, and a flat deformation lifts them to finite-field tuples. Fatal: flat degeneration conserves degree/multiplicity of the zero-dimensional fibre; shelling orders facets but cannot skip them; elliptic relation ideals are not Schubert determinantal ideals so no uniform squarefree initial ideal need exist.",
    D + " (deferred_theorem_required)",
    "Covers Groebner degenerations to squarefree monomial ideals and shellability. Does not cover: Hilbert-series / Betti-number structure of the Semaev ideal (EXTERNAL_CORPUS names syzygies/Betti and singular loci as untouched gaps), or semi-normality (Yokoyama) tests.",
    f"{DEF}/ECDLP-IDEA-098_squarefree_grobner_shelling_source_lift_hypothesis.md")

add("ECDLP-IDEA-102 Elliptic dynamical-R transfer descent",
    "Exact finite-field specialisation of an elliptic dynamical R-matrix / star-triangle relation (Felder, Baxter) whose spin states encode partial elliptic sums and whose local weights enforce factor-base membership; dynamical Yang-Baxter reorders the m-step addition network into commuting transfer matrices with compact spectral contraction; conditioned matrix elements invert to sources. Fatal: integrability is rigid (theta-function weights over analytic data, spectral curve is not E(F_p)); multiplying weights by the indicator of F breaks the identity; transfer traces aggregate paths.",
    D + " (deferred_finite_field_identity_required)",
    "Covers integrable-lattice-model (Yang-Baxter) encodings. Does not cover: matchgates (IDEA-050), statistical-physics sampling (rejected S03/S04/S10), or Bethe/loop-series approximations (rejected IDEA-240).",
    f"{DEF}/ECDLP-IDEA-102_elliptic_dynamical_r_transfer_hypothesis.md")

add("ECDLP-IDEA-103 Faithful tropical source atlas",
    "Finite target-independent family of toric embeddings of the universal addition family s: U_eps E^m -> E jointly separating every point of every finite witness set A_R={(eps,X_i): X_i in F}, with a bounded tropical cell/residue word inverting exactly to the signed source tuple (Payne inverse limit, faithful tropicalisation). Fatal: faithfulness preserves an analytification/skeleton, not labels of a growing finite set; distinct tuples share valuation vectors; inverting a tropical point to residue-field sources is the membership problem.",
    D + " (deferred_theorem_required)",
    "Covers tropical geometry of the addition correspondence over F_p (tropical appears in 4 cards, 15 ideas-tree files). Does not cover: tropical geometry of the CURVE E itself over a valued field (Berkovich skeleton of E), tropical linear algebra on the relation matrix, or tropical counts (Mikhalkin) as yield predictors.",
    f"{DEF}/ECDLP-IDEA-103_faithful_tropical_source_atlas_hypothesis.md")

add("ECDLP-IDEA-111 Radical Montgomery-ladder branch-ideal quotient",
    "Encode a length-t inverse Montgomery ladder for R=[s]P as a radical Boolean-selector ideal (z_i(z_i-1)=0, exact add/double, saturated denominators); claim a lossless target-uniform quotient of the 2^t branch union with normal-form rank 2^{kappa t}, kappa<1/2. Fatal: inverse transitions are distinct translations with generically disjoint saturated components doubling per bit; radicalisation does not identify scalar branches; lossless quotient keeps 2^{(1-o(1))t} idempotents (DAG edge barrier ECFG-NR-1409).",
    D + " (deferred_lossless_quotient_theorem_required)",
    "Covers Boolean/Groebner encodings of scalar-multiplication inversion (bit-by-bit). Does not cover: SAT/CDCL on the same encoding (control), or tau-adic/Frobenius ladders on Koblitz curves.",
    f"{DEF}/ECDLP-IDEA-111_radical_montgomery_ladder_branch_ideal_quotient_hypothesis.md")

add("ECDLP-IDEA-119 Elliptic Bloch-polylog regulator descent",
    "Finite public computable quotient of an elliptic Bloch-Suslin / polylogarithmic complex with marked-point map b: E(F_p)->C_E whose boundary verifies accepted identities as relations on E and whose regulator-residue word has an exact scalar-blind inverse to sparse factor-base atoms. Fatal: over finite fields regulator targets are torsion/trivial; functional equations identify classes without unique atom factorisation; any quotient separating <P> encodes the DLP.",
    D + " (deferred_regulator_source_inverse_required)",
    "Covers K_2/Bloch-group/polylogarithm machinery on E over F_p. Does not cover: Beilinson regulators over number fields with the global lift, Mahler measure / Boyd-type identities, or Massey products (IDEA-069).",
    f"{DEF}/ECDLP-IDEA-119_elliptic_bloch_polylog_regulator_descent_hypothesis.md")

add("ECDLP-IDEA-121 Shared bivariate common-norm reporter",
    "One symbolic source-marked two-transition circuit H(U,W) of size B^{2+o(1)} shared by the degree-B target selector T(U) and factor-start selector F(U); compute only the common factors of N_T(W)=Res_U(T,H) and N_F(W)=Res_U(F,H) with exact source labels in N^lambda, lambda<1/2, without expanding either norm. Receipt closes standard Kedlaya-Umans embeddings at cubic represented traffic (Theta(B^3) leaves or Theta(B^4) dense fibre-product module); only an output-sensitive nonlinear locator plus exact source inverse remains.",
    D + " (deferred_ku_circuit_common_norm_reduction_required); KU-embedding arm closed by receipt",
    "Covers resultant/gcd-of-norms formulations of the five-source prime-field relation with circuit-preserving arithmetic. Does not cover: block-Krylov module intersection (IDEA-056), fast multipoint/ECFFT evaluation tricks (IDEA-195 ECFFT map closed by P1527), or binary fields.",
    f"{DEF}/ECDLP-IDEA-121_shared_bivariate_common_norm_hypothesis.md")

add("ECDLP-IDEA-133 Target-local nonlinear apolar flat extension",
    "For the five-source relation fibre at target R, a target-local NONLINEAR apolar functional Lambda_R whose truncated Hankel/moment operator has a flat extension of sub-rho size; its multiplication algebra (joint eigendecomposition on reduced fibres, joint primary/nilpotent decomposition on non-reduced fibres) recovers every signed source tuple with multiplicity (Laurent-Mourrain flat extension, Gorenstein duality). P1514 receipts close: supplied-moment constructors, supplied-sequence border bases, frozen direct-enumeration tradeoffs (B^3/B^4/B^5/B^6), and the dense Macaulay instantiation at the sufficient cutoff k=5B-3 (Theta(B^5) coordinates). Open: adaptive early stabilisation, smaller fibre-specific cutoffs, sparse/multihomogeneous Macaulay constructions, structured target-local moment oracles, charged multi-functional non-Gorenstein inverse.",
    D + " (deferred_needs_independent_sub_rho_nonlinear_operation_theorem); P1512/P1513/P1514 receipts; verifier planned unrun",
    "Covers moment-matrix / apolarity / border-basis approaches to the zero-dimensional relation fibre. Does not cover: Waring/secant-variety rank problems per se, SOS/Lasserre relaxations (rejected IDEA-328), or Ulrich/Chow scalar-linear atomizers (closed P1512/IDEA-115).",
    f"{DEF}/ECDLP-IDEA-133_target_local_nonlinear_apolar_flat_extension_hypothesis.md")

add("ECDLP-IDEA-152 Rectangular BGG/Tate source kernel",
    "Sheafify the target-specialised five-source relation scheme in a frozen multigraded projective embedding, transport through the Bernstein-Gelfand-Gelfand correspondence to the exterior algebra, truncate the Tate resolution to a justified rectangle of homological/internal degrees; a designated differential block D_R with a public module action must have canonical atoms of ker D_R <-> signed source tuples (including primary-local data). Fatal: BGG preserves information and does not construct the resolution cheaply; Betti numbers encode the Hilbert function/scheme length; a kernel has no canonical point basis.",
    D + " (deferred_needs_rectangular_size_and_source_biconditional_theorem)",
    "Covers BGG/Tate resolutions and exterior-algebra modules of the relation scheme. Does not cover: Betti-number / syzygy growth of the Semaev ideal as a complexity invariant (named untouched gap in EXTERNAL_CORPUS; KN-FIND-006 rank deficit is a Betti count), Boij-Soederberg theory, or matrix factorisations (IDEA-163).",
    f"{DEF}/ECDLP-IDEA-152_rectangular_bgg_tate_source_kernel_hypothesis.md")

add("ECDLP-IDEA-161 Rigidified N-primary theta return",
    "An explicitly N-primary nonsplit theta/Heisenberg (Mumford theta group) extension attached to the public subgroup with a canonical scalar-blind section; its commutator and rigidified return convert endpoint data to signed atoms or typed scalar equations without a full N-torsion dictionary. IDEA-145 closes coprime-centre split/gauge-removable extensions. Fatal: N-primary theta data needs an N-dimensional Heisenberg representation or the full N-torsion field; commutators land in mu_N (another DLP); a canonical section splits the class.",
    D + " (deferred_needs_nonsplit_class_canonical_section_and_typed_return)",
    "Covers Mumford theta groups / Heisenberg central extensions of E[N]. Does not cover: classical theta-function (analytic) coordinates, theta constants / Thetanullwerte and Igusa invariants of Jacobians (no card), or Weil-representation transforms (rejected IDEA-101).",
    f"{DEF}/ECDLP-IDEA-161_rigidified_n_primary_theta_return_hypothesis.md")

add("ECDLP-IDEA-163 Nonlinear matrix-factorization source atomizer",
    "Compact genuinely nonlinear 2-periodic matrix factorisation (Eisenbud) of the target-specialised five-source relation hypersurface, constructible from public endpoint equations without dense expansion, whose canonical indecomposable summands are biconditional to signed atoms including non-reduced strata. Fatal: constructing/decomposing the factorisation is the fibre factorisation; rank tracks the Jacobian/Milnor algebra; indecomposables need not be canonical or point-labelled.",
    D + " (deferred_needs_low_rank_factorization_and_source_summand_theorem)",
    "Covers matrix factorisations / singularity categories of the relation hypersurface. Does not cover: Milnor/Tjurina algebra invariants as yield statistics, Knoerrer periodicity, or Ulrich sheaves (closed IDEA-115).",
    f"{DEF}/ECDLP-IDEA-163_nonlinear_matrix_factorization_source_atomizer_hypothesis.md")

add("ECDLP-IDEA-164 Miller-coboundary HDX source transgression",
    "A frozen family of Miller/divisor functions evaluated on a public endpoint forms a scalar-blind sheaf 1-cochain on an implicit high-dimensional expander (HDX); a proved coboundary identity equals source-bearing local views of the relation fibre, and cosystolic decoding recovers signed atoms. Fatal: Miller evaluations are public aggregates; making the cochain source-bearing needs the missing divisor; materialising face restrictions restores B^m ancestry; expansion decodes a supplied word but does not construct it.",
    D + " (deferred_needs_miller_coboundary_source_identity)",
    "Covers HDX / cosystolic-expansion decoding of Miller-function cochains (IDEA-132 sheaf decoder rejected). Does not cover: Ramanujan graph walks on isogeny graphs, LDPC/belief propagation (rejected N09/S11), or locally testable codes.",
    f"{DEF}/ECDLP-IDEA-164_miller_coboundary_hdx_source_transgression_hypothesis.md")

add("ECDLP-IDEA-165 Fixed rational pair-sum quotient unranking",
    "One target-independent bounded-degree rational map pi on oriented pair sums S_i+S_j of a sparse factor base with o(B^2) quotient states and a constant-list canonical inverse from (pi(S_i+S_j),R) to exact oriented pairs; two states plus a fifth source give five-source relations. Producer theorem CLOSES the fixed-map arm: exact state-only composition forces injectivity; states x inverse-list size = Omega(B^2); a bounded-degree rational map has quadratic image on a generic Sidon base, and small-doubling bases move the payload into source multiplicity. Open: a target-local indexed collision router.",
    D + " (deferred_bounded_degree_pair_quotient_scoped_negative_target_router_open)",
    "Closure covers all fixed bounded-degree rational pair-sum quotients. Does not cover: target-local (R-dependent) routers, growing-degree maps, or additive-combinatorial structure of the base beyond Sidon/small-doubling (e.g. Freiman/BSG structure: rejected IDEA-340; Gowers norms: rejected IDEA-118).",
    f"{DEF}/ECDLP-IDEA-165_fixed_rational_pair_sum_quotient_unranking_hypothesis.md")

add("ECDLP-IDEA-167 Nonmedial quasigroup-isotope source normalizer",
    "A public rational NONmedial quasigroup isotope of elliptic addition with a sparse factor-base subquasigroup and a target-uniform unique normal form; normalising a public endpoint unpacks to every signed source word. Fatal: medial isotopes are affine (Toyoda-Bruck); a nonmedial public isotope may be a bijective rewrite leaving decomposition unchanged; a scalar-oriented normaliser encodes the DLP or enumerates the fibre.",
    D + " (deferred_needs_nonmedial_law_and_endpoint_normal_form_theorem)",
    "Covers quasigroup/loop-theoretic (isotopy) rewrites of the group law. Does not cover: Maltsev/affine CSP structure (closed IDEA-122), term-rewriting confluence seeded by known relations (control), or Moufang/Bol loop structures on other objects.",
    f"{DEF}/ECDLP-IDEA-167_nonmedial_quasigroup_isotope_source_normalizer_hypothesis.md")

add("ECDLP-IDEA-173 Gain-graph switching-potential descent",
    "Lift partial elliptic-sum transitions target-uniformly to a compact public gain graph over a nonabelian or group-valued label system; switching to a balanced canonical representative yields vertex potentials inverting biconditionally to signed points without logs or an explicit edge deck. Fatal: useful additive gains in Z/N are scalar differences (= DLP); coordinate-derived gains lack the path-product law; restoring balance lists pair/triple transitions (B^2/B^3 deck).",
    D + " (deferred_needs_public_balanced_gain_and_source_inverse_theorem)",
    "Covers gain graphs / voltage graphs / switching classes on relation graphs. Does not cover: spectral graph theory of the relation graph (KN-OPEN-010 Markov operator), or signed-graph large-prime cycle finding (prym ef2530).",
    f"{DEF}/ECDLP-IDEA-173_gain_graph_switching_potential_descent_hypothesis.md")

add("ECDLP-IDEA-195 Non-Cartesian S3 intertwiner source router",
    "A target-independent rational map psi outside the canonical ECFFT family (x+c+1/x) and a non-Cartesian recursive support Sigma_F on which both coefficients of the complete Kummer S3 branch polynomial descend through psi with bounded exact inverse fibres; iterating this support-changing intertwiner yields five-source rows. P1526 closes target-isogeny/auxiliary-tree routing; P1527 closes the canonical map on y^2=x^3+1 except deck-fixed/bounded residual loci; P1528 proves same-field rational isogeny kernels on near-prime-order targets are N^{o(1)} and give duplicate factor-log columns; P1549 closes explicit D^4 path expansion, whole-edge scans and B^3 provenance; P1550 closes globally rational O(D) realisations (each branch captures <= B targets; explicit finite-domain branches need degree >= B^{11/4}); P1551 closes the frozen compact finite-field grammar at pointwise-vs-aggregation. Survivor: genuinely non-group/non-Cartesian map or charged extension-field return.",
    D + " (deferred_needs_noncanonical_intertwiner_and_exact_source_inverse_theorem); scoped-inconclusive finite-state closure audit",
    "Closures cover ECFFT-style isogeny/rational maps on x-coordinates over the same field and explicit branch catalogues. Does not cover: extension-field returns with charged cost, non-Cartesian supports arising at later recursion stages, or maps that are not rational functions of x (e.g. using y or Frobenius-twisted coordinates).",
    f"{DEF}/ECDLP-IDEA-195_noncartesian_s3_intertwiner_source_router_hypothesis.md")

# ---------------------------------------------------------------------------
# C. Prym follow-ups 2026-10-27 (research/prym_leads_20261027)
# ---------------------------------------------------------------------------
P = "proposed (adaptation; unexecuted; no new hypothesis, zero runs)"
PR = f"{PRYM}/README.md and {PRYM}/ideas/candidates.md"

add("PRYM context: RUN-ECDLP-967315 retained-matrix diagnosis",
    "Genus-2 double cover C: y^2=x^6+2x^2+B over F_4099 of E: Y^2=U^3+2U+B (B=13: r=4021; B=16: r=4111), 16-atom base with one public anchor; 4,587,520 attempts gave 97/67 native witnesses but only 13/9 distinct affine equations, rank 13/15 and 9/15, nullity 2/6, zero identifiable individual logs. Loss of diversity is exactly explained: each repeated group has one source scalar, one interpolation cubic and one complete six-point zero divisor; multiplicity = number of admissible triples from the five remaining zeros (4/7/10 and 2/4/7/10). Exactly one projective identifiable pair per matrix (z_0+310 z_12=1599 mod 4021; z_1+2 z_5=60 mod 4111). Anti-invariant (Prym) projector kills the conorm image; Tate/Zarhin: coprime Frobenius polynomials imply Hom(E,A)=0.",
    "open (finite post-data derivation; not a constructor-wide bound)",
    "Diagnosis covers only the frozen chart (x=0 excluded, regular degree-six pole stratum, V3 function space) of one cover family at p=4099. Does not cover: genus-3 covers (2-dimensional Prym), extension fields, or an asymptotic obstruction.",
    PR)

add("IDEA-20261027-d434c8 Retained kernel features and natural target-row certificates",
    "Reconstruct the two sealed signed/public-anchor projections, verify ranks and full right-kernel bases, and test a short public feature dictionary (zero columns, equal/opposite columns, connected row-support component indicators) against the kernels; leave-one-source-block-out span checks. For later target descent the exact interface is c(Q+tP)=sum v_j F_j + hP with v in rowspace(M): v.z is determined iff v.K=0 and a certificate lambda.M=v gives k=c^{-1}(lambda.b+h)-t. The load-bearing missing operation is constructing (v,c,h,t) from public Q by a fixed algorithm, not choosing v after seeing the rowspace.",
    P + "; priority high; implementation low, compute low",
    "A finite diagnostic on existing matrices; not a descent algorithm. Does not supply the natural-target constructor, does not test new curves, and says nothing about rank at scale.",
    PR)

add("IDEA-20261027-0bbc32 Translated quadratic covers of the same E as a row-space control",
    "Family C_t: y^2=f(x^2+t), pi_t(x,y)=(x^2+t,y) onto the SAME E with complementary quotient E'_t: V^2=W^3+(3t^2+A)W^2+3tcW+c^2 (c=f(t)!=0; W=c/x^2, V=cy/x^3); smooth when c!=0, distinct branch supports over E for distinct t (Diem, degree-2 covers). Compare rowspaces of t=0 and t=-1 only on an exact common signed atom base liftable to BOTH covers, selected before acceptance outcomes; measure new syndrome/rank after paying the second cover.",
    P + "; priority medium",
    "Measures finite relation-space diversity, not an exponent. Does not cover stacking different E's, Frey-Kani/Richelot gluing as a second r-torsion direction (closed lead), or cover automorphism accounting.",
    PR)

add("IDEA-20261027-d217c2 Pole-complete Riemann-Roch filtration for the native constructor",
    "The old constructor rejected 417/451 cases where v3(x)^2-f6(x) lost its degree-six term (leading cubic coefficient +-1 cancels at one infinity); complete these with exact unequal infinity pole orders (E2 pole contribution (m_+ - m_-)(0,B); E1 both infinities map to O), then compare regular V3=L(3H)=<1,x,x^2,x^3,y> vs completed V3 vs V4=L(4H) (dim 7; ~p^2 projective freedom; residual A4^2-(b0+b1x)^2 f6 of degree <=8), charging parameter enumeration and residual factorisation; keep x=0 excluded to avoid confounding.",
    P + "; priority high",
    "Bounded-degree constructor inside P1546/KN-OPEN-020 scope. Does not cover higher function spaces beyond V4, non-hyperelliptic covers, or growing-degree implicit routers.",
    PR)

add("IDEA-20261027-ef2530 Signed partial cycles with a nonzero Q coefficient",
    "Two-large-prime successor of PO-transfer-003: keep exact native signed places and polar markers ([2T-H] atoms; hyperelliptic conjugate is inverse integrally since T+bar T-H is a vertical divisor; E1 norms carry factor 2, r odd); certify cancellation by signed integer incidence dependencies plus vertical/polar witnesses; require nonzero Q coefficient mod r, reject integer dependencies that vanish mod r (the r*e_i example), count independent equations after aliases, charge zero-Q cycles, sorting I/O and coefficient growth.",
    P + "; priority medium; existing lead LEAD-ICP-residual-partial-relations",
    "Covers signed 2LP stitching in the native divisor representation. Does not cover: implicit moment oracles (IDEA-053), hashed/x-functional filters (KN-OPEN-91d4f6), or a proof that joint vanishing of both elliptic residual checks certifies native cancellation (explicitly stated insufficient).",
    PR)

add("IDEA-20261027-d5c1ee Isolated x=0 completion of the native source/interpolation chart",
    "pi2((0,y)) already extends regularly to the E2 identity (local parameter x/y vanishes; B!=0 so y!=0) and residual zero roots are already accepted (16/6 witnesses), but lift_source rejects x=0 sources and descriptor rejects interpolation abscissa 0, dropping public source a=1 and C(15,2)*4^3=6,720 branches per block (D counts 194,880/221,760 match). Change only those guards, keep schedule/poles/base, score distinct projected affine signatures and new kernel syndromes; repeated abscissae stay excluded; deck choices coalesce at the ramification point (stabiliser).",
    P + "; priority high; implementation low, compute medium",
    "A chart-regularity control; does not imply useful-row probability, rank gain or cheaper recovery. Does not cover degree-drop (d217c2) or V4 changes.",
    PR)

add("PRYM router boundary (not filed): native source-coupled residual circuit",
    "Considered and NOT filed: impose v3(x_R)=y_R, define monic residual h5(x)=(v3^2-f6)/(lc*(x-x_R)) on the regular stratum, and for an implicit growing-degree base A_d(x)=D_{2^d}(x^2+t)-c (Dickson recursion D_{2n}=D_n^2-2) require h5 | A_d, evaluated by short remainder circuits with squarefreeness, coprimality and h5 | x^p-x checks. Verdict: a predicate/encoding only; it does not extract v3, h5 or a successful branch cheaply; duplicates the merged P-256 F1 obligation and IDEA-20260723-001's compact-support constructor obligation. P1546 explicitly leaves compact adaptive/implicit routing unclassified.",
    "rejected as new algorithm (duplicate of F1 obligation); the implicit/high-degree/target-dependent router remains the single open escape from P1546",
    "Any successor must name a public extraction/branch-selection operation distinct from F1, obtain the source witness without a scalar oracle, and charge quotient dimension, branches, coefficient/field growth, retries, root extraction, implicit LA, natural target attachment, memory and return. Unexplored adjacent space: Dickson/Chebyshev-type implicit bases over F_p, growing-degree membership described by recursion rather than a polynomial of degree B.",
    PR)

# ---------------------------------------------------------------------------
# D. Theory notes research/THM_*.md
# ---------------------------------------------------------------------------
add("THM_BKKMV1 Sectioned box-saturation and the mixed-volume law for Semaev S_m",
    "PROVED all m>=3, char!=2: deg_{x_i} S_m = 2^{m-2}, total degree (m-1)2^{m-2}, lead_{x_m} S_m = S_{m-1}^2, coefficient tower, and the universal diagonal corner (coefficient exactly 1), so every generic target-section has the full box as Newton polytope support projection. PROVED: if sectioned box-saturation holds then MV_m=(m-1)!2^{(m-1)(m-2)} = the multigraded (P^1)^{m-1} Bezout number; ratio to total-degree Bezout is (m-1)!/(m-1)^{m-1}. PROVED saturation for m<=5 (13/439/54777 cells; MV 8/384/98304). Even-m bottom fibre degeneration X | u_m (t=0 is an exceptional section). CONJECTURE C1: saturation for all m>=6, reduced to an interior-fibre non-cancellation lemma (Lemma-G).",
    "proved (m<=5) / conjecture (all m)",
    "Covers the target-sectioned Semaev family over F_p in Weierstrass x-coordinates. Does NOT cover: other coordinate systems (Edwards/Montgomery/Kummer-line) or Cremona-transformed systems (IDEA-059), Weil-descended binary systems, or whether support-aware elimination beats dense when MV = box-Bezout (it says sparse == dense in this class, KN-OPEN-004).",
    f"{THM}/THM_BKKMV1.md")

add("THM_BKKMV2 (a,b)-grading route to interior-fibre non-cancellation; Theorem B; bottom-fibre law",
    "PROVED: two-root (Poisson) expansion of S_m=Res_X(f,g) as sum over (k,l) of f_k f_l g_2^{d-l} g_0^k U_{l-k}; weight grading wt(x)=2, wt(a)=4, wt(b)=6 makes S_m homogeneous of weight (m-1)2^{m-1}, each x-monomial coefficient lies on a single line 2A+3B=q(E) (q=1 fibres are exact holes; q<=5 single-grade cone). Theorem B (b-extremal non-cancellation) proved conditional on hypothesis (T_m), verified m<=5 and tropical tables through m=8: Lambda_m = Psi*Phi never cancels. Exact bottom-fibre factorisation law u_4,u_5,u_6 with torsion law c_m = +-prod psi_{k-1}psi_{k+1}(0;-a,-b)^{N(m,k)} (ERRATUM to BKKMV1 §4: its u_5 claims were a script typo). m=4: every one of 125 box fibres is covered by an explicit certificate family (0 uncovered). Residual gaps G1 (T_m)+(NZ_j) Krawtchouk zeros, G2 covering, G3 grade migration.",
    "proved (conditional theorem + m<=6 symbolic) / conjecture (C-B1, C-B2, C-B3, C-u)",
    "Same scope as BKKMV1. Does not cover: the solving degree or Groebner behaviour (only support/mixed volume), and makes no statement about root counts over F_p (Bernstein bound is over the torus of an algebraically closed field).",
    f"{THM}/THM_BKKMV2.md")

add("THM_COMMUTATOR_KERNEL1 Commutator generation of the evaluation kernel for the prime-field translation/negation quiver",
    "PROVED: for the single-vertex quiver with loops t_i (translation by P_i) and nu (negation) acting on G=<P> of odd prime order n in E(F_p), every word acts as an affine map X -> eps X + beta P (Aff(G) = Z/n x| Z/2), and the evaluation kernel is the normal closure of commutators [t_i,t_j], dihedral torsion nu^2=1, (nu t_i)^2=1, and the commutative subset-sum lattice L=ker(Z^B -> Z/n). Every noncommutative word relation descends to an ordinary signed subset-sum relation of no larger l1 weight. Reproduces the measured NC 350/439 vs commutative 437/439 value-set inclusion. Closes candidate C3 (noncommutative Groebner word search, KN-OPEN-008) by theorem for this family. Conjecture: collapse persists for Frobenius-augmented skew-group-ring quivers over F_q. Open gaps: multi-vertex isogeny-graph quivers (groupoid evaluation), cost-level simulation theorem, non-affine arrows.",
    "proved (single-vertex translation/negation quiver, odd n) / conjecture (Frobenius skew quivers) / open (multi-vertex isogeny quivers)",
    "Covers only arrows that are endomorphisms/affine maps of one cyclic group. Does not cover: quivers whose vertices are different curves in an isogeny graph (groupoid), correspondences that are not maps, or Hecke-type operators acting on functions on the isogeny graph (no card).",
    f"{THM}/THM_COMMUTATOR_KERNEL1.md")

add("THM_FALLDEG_INVARIANTS1 Fall-degree invariants read from Caminata-Gorla / Caminata-Ceria-Gorla",
    "Records the primary definitions (first fall degree d_ff via Syz(F^top)/Triv in B=F_q[x]/(x^q-x); degree of regularity; last fall degree d_F via the V_{F,i} filtration = Macaulay row space for degree-compatible orders) and theorems: sd_sigma(F)=max{d_F, max GB degree} (CG Thm 3.1); Semaev-Tenti sd <= 2 d_reg - 2 when d_reg >= max(q,deg f_i); CCG exact formula d_reg(Weil(F)) = n(d_reg(F)-1)+1 and sd(Weil(F) u field eqs) <= n reg(F^h) - n + 1 (LINEAR in n). Consequence: KN-OPEN-7f0511's four inconsistent 'first fall degree' definitions are disambiguated; the program's measured D is a solving degree, and the open question (Brief lever 1) is whether the V-restricted descended system has sd bounded by f(m) independent of n or grows linearly.",
    "proved (literature statements; no new proof) / open (V-restricted bound: KN-OPEN-8f3d27)",
    "Covers definitions and the unrestricted Weil-restriction bound. Does not cover: the factor-base-restricted linear section (the actual IC system), lower bounds on solving degree, or prime-field digit presentations (Brief lever 9).",
    f"{THM}/THM_FALLDEG_INVARIANTS1.md")

add("THM_INCBARRIER1 EC-chord richness ceiling in the uniform-x model",
    "PROVED: for an x-saturated lift set of a factor base F (|F|=B) on E/F_p, Bezout collapses the chord-arrangement richness profile to one integer T_3 (number of triples P+Q+R=O): c_r=0 for r>=4, c_2=C(M,2)-3T_3; T_3(F)=2X(F)-Y(F); full-group count T_3^full=(n^2-6n+5+3z+2t)/6; exact mean and variance in the uniform-x model giving the phase diagram B=p^alpha: alpha<1/3 -> T_3=0 whp, alpha=1/3 Poisson, 1/3<alpha<1/2 concentrated at the generic mean (I_EC=I_generic(1+o(1))). Worst case T_3 <= 2B(B-1)/3 attained up to 3/4 by the group-AP family (excess trivialises by rank defect). Character-sum (Weil) certification of the ceiling for a FIXED F is impossible below B=Omega(p) (optimistically p^{2/3}); harvesting needs p^{1/5..2/5}. Conjecture C2: ceiling holds for structured F (x-intervals).",
    "proved (uniform-x random model, toy-verified) / open gap G1 (fixed/adversarial F)",
    "Covers m=3 (chord) incidences over F_p in the x-saturated uniform model. Does not cover: m>=4 incidences, structured/deterministic factor bases (conjecture only), binary fields, or additive-energy structure of specific F (THM_INCBARRIER2).",
    f"{THM}/THM_INCBARRIER1.md")

add("THM_INCBARRIER2 Chord-profile identity, energy certificate, norm ladder to alpha=1/2, cross-cancellation barrier",
    "PROVED (black-box free): chord profile r(x)=#{(P,Q): x(P+Q)=x}; S=sum_{x in F} r(x)=6T_3+2d; exact identity T_3(F)=mu~+(1/6)sum_{x in F} delta(x) with mu~=BT/(6L)-d/3; N_2=2E_1 - sum_{2-torsion} A'(g)^2 (restricted additive energy, 2-torsion correction found by a verification failure); deterministic per-instance energy certificate |T_3-mu~| <= (1/6)sqrt(B N~_2) never worse than Weil and better by Theta(sqrt p) when the profile is flat. Norm ladder: under profile-moment hypotheses H(k), certification holds for alpha > (k+1)/(2k+1) -> 1/2; Theorem C: the entire norm/moment family cannot cross alpha=1/2 (needs cross-x sign cancellation of delta on F). Unconditional character-sum lower bounds; theta-quality scale: a bilinear bound with theta in [2/5,7/10] would certify harvesting range. Group-AP family saturates the certificate.",
    "proved (identities, certificate, ladder, barrier) / conjecture C1 (uniform-x flatness) / open (alpha<=1/2 cancellation statement F)",
    "Covers fixed-F m=3 incidence certification over F_p via additive energy of the lift set. Does not cover: sum-product / Bourgain-Garaev type estimates on E (not invoked), higher Gowers norms (rejected IDEA-118), or a proof for alpha<=1/2.",
    f"{THM}/THM_INCBARRIER2.md")

add("THM_JETBARRIER1 Jet-augmented generic group model: simulation theorem and exact boundary",
    "PROVED: in the formalised jet-augmented GGM J(E,FB,r) every order-r jet/dual-number query on a public-equation variety is decided by zeroth-order data; the dual-number addition law is Lie-linear in the invariant-differential scalar; all jet queries are simulable with ZERO group-oracle overhead so Shoup's Omega(sqrt l) bound lifts verbatim (closes A1 tangent-screen channels; KN-FIND-002). The x_1=x_R fibre degree drop is the doubling fibre (zeroth-order). T6: boundary is sharp - adding coordinate data of composite (non-public) elements collapses genericity; no intermediate well-defined non-simulable jet model exists in the encoding-independent class. Open: maximality of the simulable class (C2) and Weil-restriction/descent scope (G2).",
    "proved (in-model) / open (C2, G2)",
    "Covers jets/dual numbers/infinitesimal data of public elements. Does not cover: representation-dependent oracles using field coordinates of composite elements (the single door named in the Brief), p-adic lifts of higher precision as computational handles (IDEA-436), or Weil-descent models.",
    f"{THM}/THM_JETBARRIER1.md")

add("THM_QUOTIENT_DESCENT1 Only 2-torsion translations descend through a degree-2 quotient of E",
    "PROVED (elementary, Lean-formalised as translation_commutes_iff_two_torsion): for iota_S(P)=-(P+S), any S in E(k), translation tau_T descends through the degree-2 quotient E/<iota_S> -> P^1 iff 2T=O. Corollary: the translation group acting on solutions of any x-type summation polynomial is E[2](k)^m cap {sum T_i=O}, so the full symmetry group from translations and permutations is a subgroup of E[2]^{m-1} x| S_m of order <= 4^{m-1} m!. Closes Galbraith's 'larger group actions?' question for every x-type coordinate (corrected 2026-10-16 to all S, not only 2-torsion S).",
    "proved",
    "Covers degree-2 (x-type) quotients of E. Does not cover: higher-degree quotient maps E -> P^1 (e.g. degree-3 or degree-N quotients where larger translation groups could descend, Kummer-line-of-degree-d), quotients to other curves, or symmetries that are not translations (Frobenius, CM automorphisms).",
    f"{THM}/THM_QUOTIENT_DESCENT1.md")

add("THM_SEMAEV_FALL1 Decidable attribution of degree falls in Weil-descended Semaev systems",
    "PROVED Lemma 1 (Frobenius collapse): over K=F_{q^n} with field equations imposed, q-th powering equals coefficientwise Frobenius, so Frobenius conjugates are redundant modulo q-th powers and field equations, giving degree falls at a degree not growing with n (the structural reason descended systems are not semi-regular). Lemma 2: symmetrisation to elementary symmetric coordinates preserves weighted degree and does not increase total degree (FGHR gain NOT proved). Lemma 3: the decomposition-certificate predicate is decidable. NOT PROVABLE by this route: that a fall is unexplained (attribution is sound, never complete). CORRECTION: KN-LIT-7605 (Huang 2021) does NOT prove last fall degree bounded in n for summation polynomials; no corpus record proves that.",
    "proved (lemmas) / out of scope (growth law)",
    "Covers attribution of individual falls to Frobenius/symmetry/field-equation syzygies. Does not cover: the growth law of d_F or sd in (m,n',n,q) (Brief lever 1), prime-field digit presentations, or lower bounds.",
    f"{THM}/THM_SEMAEV_FALL1.md")

# ---------------------------------------------------------------------------
# E. Open problems knowledge/open-problems/KN-OPEN-*.md (title + statement)
# ---------------------------------------------------------------------------
KOI = [
 ("001","Does index calculus beat Pollard rho for prime-field ECDLP?","Is there an algorithm with expected cost asymptotically below ~sqrt(p) for E/F_p? (program-level root question)","open","Umbrella question; all other prime-field items refine it."),
 ("002","Growth of Groebner solving degree for prime-field summation-polynomial systems","How do solving degree and first-fall degree of point-decomposition systems grow with field bit size and m over PRIME fields?","open","Prime fields only; binary growth is KN-OPEN-8f3d27/d218ec."),
 ("003","Do curve representations/symmetries reduce decomposition cost over prime fields?","Does the curve model (Weierstrass/Edwards/twisted) or exploitable symmetry materially lower decomposition/Groebner cost over F_p as in binary settings? KN-FIND-007 (yield conservation) and external GLV/CM folding (3-6x flat) answer 'constants only' so far.","open","Fixed curve; isogeny-class axis is IDEA-434."),
 ("004","Does support-aware (BKK/mixed-volume) elimination undercut dense resultants for prime-field Semaev systems?","Are target-sectioned Newton polytopes proper subpolytopes of the degree box (MV<Bezout) or Newton-saturated? THM_BKKMV1/2: saturated for m<=5, conjectured all m -> sparse == dense.","open (effectively answered negative for m<=5)","Weierstrass x-coordinates only."),
 ("005","Is a non-generic representation (jets, elliptic nets, incidence reporting) GGM-simulable?","For each augmented oracle, is it simulable with O(1) overhead (closed at exponent 1/2) or does it give k-dependent relations below birthday? THM_JETBARRIER1 answers yes-simulable for jets; nets/incidence by KN-FIND-b7e091/194294.","open (jets answered)","Does not address representation-dependent oracles on composite elements."),
 ("006","Can AP relation supports give a low-displacement-rank matrix whose structured solve beats sparse LA?","Supports {x,x+d,...}: is the relation matrix Toeplitz/Hankel-like with O(1) displacement rank enabling superfast solves, net of the relation-probability penalty? THM_INCBARRIER1 T5: the group-AP family has excess T_3 that trivialises by rank defect.","open","Linear-algebra stage only."),
 ("007","Do bond tensors of the recursive Semaev resultant tree admit low-rank factorisations over F_p?","Reading S_m=Res_y(S_k,S_{m-k+2}) as a tree tensor network, does the recall-0.99 bond rank grow with exponent <1 in d=2^{m-2}, beating ~1.979 dense exponent?","open","Tensor-network contraction; matchgate transform is IDEA-050."),
 ("008","Does a noncommutative path-algebra (quiver) word search find relations the commutative subset-sum quotient misses?","Translations/negation/small correspondences as quiver arrows with noncommutative Groebner search. THM_COMMUTATOR_KERNEL1 proves NO for single-vertex translation/negation quivers of odd prime order.","open (single-vertex case closed by theorem)","Multi-vertex isogeny quivers remain."),
 ("009","Is the geometric monodromy of the m-th Semaev summation cover full symmetric/wreath for generic ordinary E/F_p?","Or is there an exceptional (e.g. CM) locus with smaller monodromy giving curve-specific relation rates or block systems? Program finding: monodromy is (Z/2)^{m-2} for EVERY curve, no exceptional locus (KN-FIND-a1f3c2); relation-rate half NOT closed (KN-OPEN-62c870/8a7432).","open (monodromy half answered)","Curve selection by monodromy is closed; variance/coverage free."),
 ("010","Can the leading spectrum of a coarse-grained transfer operator of the translation-by-P walk localise k below sqrt(n)?","Coarse-grain translation into a CxC Markov operator, estimate spectrum from o(sqrt n) trajectory samples; does localisation factor L grow or is it O(1) by character orthogonality?","open","Spectral/dynamical route; external spectral decoder screen passed 0/7 curves."),
 ("011","Do lattice-reduction/HNP techniques give any advantage for the PLAIN ECDLP?","Without nonce leakage, biased nonces or side channels, can HNP/lattice machinery recover a log, or is the advantage provably confined to the partial-information model?","open","Leakage-free setting."),
 ("012","Do ideal/module lattices admit structure-exploiting attacks beyond BKZ?","'Index calculus for lattices' exploiting ring/ideal structure; does ECDLP structure-exploitation experience transfer? (Lattice-crypto side question.)","open","Not an ECDLP route; cross-program."),
 ("013","How hard is the supersingular endomorphism-ring/isogeny-path problem post-SIDH?","Is O~(p^{1/4}) the true complexity or does orientation/small endomorphisms/special curves admit faster attacks?","open","Isogeny-crypto; not ECDLP."),
 ("014","Concrete quantum security of CSIDH under the Kuperberg sieve","What is the true concrete quantum cost and required prime size?","open","Out of scope for classical ECDLP."),
 ("015","When does publishing auxiliary structure (torsion images, degrees, orientations) collapse an isogeny/DL assumption?","General lesson from the SIDH break: which auxiliary information admits polynomial/subexponential attacks?","open","Framework question; ECDLP analogue = auxiliary-input (Cheon) and KN-OPEN-ed922d."),
 ("016","Does the improved dual attack on LWE actually beat the primal attack after heuristic repair?","Quantitative LWE security question.","open","Lattice crypto, out of ECDLP scope."),
 ("017","Where does the enumeration/sieving crossover move once sieving memory is fully charged?","Full-cost (Wiener) accounting for SVP solvers at cryptographic dimension.","open","Lattice crypto, out of scope."),
 ("018","Does lattice machinery bear on the plain ECDLP at all, outside the leakage model?","Coppersmith appears only in IC descent/relation search and HNP; is confinement to auxiliary roles structural?","open","Same family as 011; Brief 'Lattice reach' wall quantifies the Jochemsz-May gap."),
 ("019","What object does each ECDLP attack family track, and is that enumeration closed?","Symmetric-key cryptanalysis has a taxonomy indexed by tracked object (differences, parities, sums, degree); ECDLP has none; can one build it and argue closure over a class?","open","Meta-taxonomy; no card attempts it."),
 ("020","Universal algebraic-factor-base no-go for prime-field index calculus","Prove that EVERY algebraically described factor base over a generic prime-field subgroup fails to yield subexponential IC with all costs charged. P1546 gives a fixed-branch counting gate; implicit/adaptive routers unclassified.","open","Bounded-degree explicit descriptions partially covered."),
 ("02200b","Is H-TOP true at m>=5 (top form of digit-substituted summation polynomial is a single monomial)?","Base-2 digit presentation: is the top-degree part of S_{m+1} in the m unknowns the single monomial prod x_k^{2^{m-1}} with coefficient 1? Load-bearing for generator degree delta=m 2^{m-1} and the first-fall closed form.","open","Prime-field digit presentation (Brief lever 9)."),
 ("024","Does the rank-1 quaternion-order principal ideal problem inherit tractability shown for M_g(O), g>=2?","Castryck et al. 2026 solve PIP in M_g(O) heuristically in polynomial time for g>=2; Mendelsohn-Nelson assume hardness at g=1.","open","Quaternion/isogeny crypto; not ECDLP."),
 ("025","For which automorphism orders k can Galois symmetry accelerate TNFS linear algebra?","Al Aswad-Pierrot-Thome give k=6,12 with ~k^2 LA speedups; generality and end-to-end gain open.","open","Finite-field DLP; relevant to pairing security only."),
 ("026","How large is the concrete security discount from module/ring structure in lattices?","2026 coefficient-isometry amortisation results at deployed parameters.","open","Lattice crypto."),
 ("027","Is HAWK's module-LIP assumption reachable through nrd-PIP, and do the four heuristics hold?","Nelson et al. claimed classical poly-time HAWK key recovery under heuristics; Heuristic 4 acknowledged insufficient.","open","Lattice crypto."),
 ("028","Corrected cost of the nrd-PIP route against HAWK","Q1 closed: density of easy principal ideals -> 0, super-polynomial.","closed_all_questions_answered","Lattice crypto."),
 ("046735","Does the native Caminata-Gorla first-fall definition handle zero top forms in B?","Example 4.4 includes w^q among top forms although zero in B; grading of that generator unresolved.","open","Definition-level interface for d_ff measurement."),
 ("095df5","Is there a commutative algebraic group over F_2 with a rational point of order 131 admitting an explicit Galois-invariant factor-base description?","At n=131, ord_131(2)=130 leaves no Frobenius-stable subspace; both Couveignes-Lercier constructions are excluded by their own conditions; no abelian variety of dim<=4 has 131 | #points; only their conjectural route survives.","open","Binary Koblitz Frobenius-stable bases (Brief lever 6)."),
 ("2c095b","Is the multiplicative-order deficit of a curve-derived lambda a fact about CM arithmetic or about small integers?","For Z[pi]-minimal non-scalar alpha=u+v pi, lambda=-u v^{-1} mod deg alpha has order smaller than a random unit at small p (deficit 0.07-0.12).","open","Endomorphism-derived scalar statistics."),
 ("2e7514","Does the fall_dim row-collapse exception occur at a strict-early-fall cell?","The fall-dimension identity fall_dim(d_ff)=m[C(s,a_0)-C(s,a_0+e)] is false universally (counterexamples p=13,19); mechanism fall_dim(D)=dim ker(m_{S_top}) - dim Rel_D.","open","Prime-field digit-descent first-fall accounting."),
 ("2f5e66","H-CREP-001 z_R construction-cost ceiling broadens to recursive meet-in-the-middle; Wagner closed as sub-avenue","Six decompositions derived; still over the declared exponent caps.","open","H-CREP-001 (complete-chart intersection) lane."),
 ("3417fc","Is there a computable non-group-theoretic invariant on the canonical prime-to-p torsion lift E(Q_p)[n] separating [k]S at sub-rho amortised cost?","Reduction is a group isomorphism so group invariants are constant; coordinates/valuations v_p(x([k]S^)-x([j]S^)) are extra symbols with no extra information; computational handle only. Made falsifiable by IDEA-436.","open","Face F2."),
 ("3c8f51","What produces the degree-4 Macaulay refutation of most unsatisfiable m=2 S_3 descents on Semaev's diagonal, and how far in n does it persist?","Unsatisfiability certificates at degree 4 for binary descents (CERTBIN).","open","Binary m=2."),
 ("3f7a21","Can the literature corpus support its own citation_verified markings?","7457 of 7666 entries claim 'read' against a downloads tree that never existed; repair options.","open","Corpus hygiene, not mathematics."),
 ("57907f","Does omega <= 9/4 (matrix multiplication, Oct 2026) hold over GF(2^k)?","Transfer of the omega(C)<=9/4 proof to characteristic 2; conditions the hashed-factor-base tensor ECDL lead (Brief lever 13).","open","Char 2 only."),
 ("5b3a08","Do the harness/semaev.py s4_expr bug or whole-curve factor-base sampling affect other experiments?","Two confirmed harness properties; materiality open.","open","Infrastructure."),
 ("5e3dd0","A Frobenius-stable orbit-union factor base on a binary Koblitz curve DOES have a bounded-degree algebraic description (Hamming ball in a normal basis)","In a normal basis {alpha^{2^i}} the Hamming-weight ball is Frobenius-stable with a bounded-degree Hamming-ideal membership; what descended degree does it reach?","open","Binary Koblitz (Brief lever 6)."),
 ("62c870","Which factor-base statistic, if any, carries curve-specific relation-rate information for Semaev decompositions, and does any transfer to extension fields?","After the scoped cycle-type no-go (generic-fibre Frobenius census closed as a relation-rate probe), the relation-rate half of KN-OPEN-009 is NOT closed; D-4 factor-base-point count not condemned.","open","Counter-statements CS-R7/CS-D2 bind."),
 ("701adf","H-CREP-001's r_R has a candidate definition under two readings but neither meets its exponent caps","Two-pairs Theta(B^4) vs one-pair Theta(B^2) reading; forward biconditional verified, RT103-C1 not discharged.","open","H-CREP-001 lane."),
 ("73f0f4","Does r_R have real degree Theta(B^2) from multi-way elimination or from non-deduplicated pair-product accounting?","Decks 2-3 combination rule constructed (G01_local, G23_local, Adm4) but cannot route through the frozen engine.","open","H-CREP-001 lane."),
 ("7f0511","Nagao names the first-fall-degree definition under which his assumption is stated; four inequivalent 'first fall degrees' circulate","D_ff (Petit), D_ff^max over recombinations, and two others; disambiguated by THM_FALLDEG_INVARIANTS1.","open","Definitional."),
 ("7f0d85","Can a learned cryptanalytic feature search be made long-round-aware?","Neural mask candidates for symmetric cryptanalysis; re-ranking vs generative defect.","open","Symmetric-key ML; not ECDLP."),
 ("7f424d","Does an interleaved modulus-reduced combination rule exist for a second deck pair in H-CREP-001?","Only decks 0-1 have a compact pair-product generator; build_chart_index bundles the rest as a whole set.","open","H-CREP-001 lane."),
 ("86e7e1","Semaev 2015's asymptotic claim reports time only, compared against memoryless rho","No memory figure propagates into the 'n>310 broken' claim.","open","Binary IC accounting (Brief rule 5)."),
 ("875d43","Does weakness beat description length as a hypothesis-selection proxy under a non-uniform task distribution?","Bennett's enactive-cognition result assumes uniform tasks; program's distribution is structured.","open","Meta-methodology."),
 ("8a5965","Does Simon's claimed poly-time DCP algorithm survive scrutiny, and does it reach ML-KEM/ML-DSA?","Quantum dihedral coset problem claim (ePrint 2026/1591).","open","Quantum lattice."),
 ("8a7432","After the cycle-type no-go: which statistic carries relation-rate information, and is m>=4 monodromy full or exceptional uniformly?","Carries counter-statements CS-N1, CS-D2; root-multiplicity structure of the fibre is non-constant on the locus.","open","Companion of 62c870."),
 ("8f3d27","Does the linear-in-n solving-degree bound for the unrestricted Weil restriction hold for the V-restricted linear section, and is sd bounded by f(m) independent of n?","Transfer and boundedness questions for factor-base-restricted descent of S_{m+1} with x_i in V, dim V=ceil(n/m).","open","Brief lever 1 (most leveraged binary question)."),
 ("91d4f6","Characteristic-2 approximate x-filters on binary curves: does the single-bucket gain G(M)=M max pi_c(d) grow?","For poly(n)-computable h_j from x(Q) (e.g. F_2-linear trace functionals), the merge-rate gain a Wagner level pays; does it exceed the generic rate?","open","Brief lever 8 (x-functional filters)."),
 ("94f456","Is the chain length t a free cost parameter? Semaev declines the t<m trade-off","Closed scoped: the trade-off is a constant-level choice.","closed_scoped","Binary IC."),
 ("9b4a2b","Does an F_p-rational 2-torsion quotient of the n=m=5 summation system on EcGFp5 follow D=2^{(m-1)^2}?","Double-odd curve over F_{p^5} with T=(0,0), x -> b/x; ideal degree of the (Z/2)^{m-1} x| S_m quotient.","open","Specific k=5 extension-field instance."),
 ("9b71bb","Hard-to-factor composite group order as a partial ECDLP trapdoor","Holder pays sqrt(max q_i) via Pohlig-Hellman while outsider pays sqrt(N); is any (log N, r) regime feasible for holder and infeasible for outsider including ECM?","open","Trapdoor/backdoor, not generic attack."),
 ("9c41af","Exact minimum depth-reducing vertex-deletion sets on doubling-distance DAGs at n~64","Combinatorial/MILP question from a pebbling/depth-robust side task.","open","Not ECDLP."),
 ("9d47e1","Does any retrievable source supply the two missing prerequisites for H-CRYPTO-fde1f1's transfer target?","Audit found the cited source insufficient; successor is an independent derivation.","open","Source audit."),
 ("ac409f","Is there a LOWER bound on the eliminant-solving exponent kappa for quasi-subfield chain systems?","Quasi-subfield factor base beats generic iff kappa < 1/(2 beta); only upper bounds exist in the literature.","open","Quasi-subfield polynomials (Brief says closed as a construction, but the lower bound is open)."),
 ("b9907c","Partially anomalous ring curves over Z/NZ as a trapdoor","E mod p anomalous (Smart lift) and E mod q ordinary; can the q-part be made easy for the holder only?","open","Trapdoor."),
 ("c2e86a","Frobenius beyond orbit bookkeeping on binary Koblitz curves: a coordinate-reading canonicaliser for H-orbits?","For H <= (Z/l)^* containing <+-lambda_pi> with index k>=2, is there a non-generic canonicaliser with cost o(sqrt k) (rho) or o(k) (IC on H-saturated bases)?","open","Binary Koblitz symmetry."),
 ("cbbd97","Endomorphism-ring-gated isogeny walks across a large-conductor barrier","Can secret knowledge of End(E) (where computing it requires factoring the conductor) gate a DLP advantage the public cannot reach?","open","Trapdoor/isogeny."),
 ("cc1988","Does any prime-field ECDLP weakness vary within an isogeny class?","Existence of a non-isogeny-invariant property W enabling cost n^c, c<1/2, rare, hard to decide publicly (Teske-style hiding).","open","Prime-field trapdoor; Brief: group order is an isogeny invariant (Tate)."),
 ("d218ec","Where in (n,m,t,k) does Semaev's Assumption 1 (d_F4<=4) fail, and is the d_F4 it names a solving degree?","Supported only at n<=21, m<=6 plus one n=40 cell; FIPS conclusion needs n=409,571. Program: d_ff=2 always, d_F>=5 certified at n=33-45.","open","Binary descent (Brief lever 1)."),
 ("d3a6f0","How fast does the maximal Fourier mass of p-adic digit characters of the torsion section s_tor decay with n?","Phi_i(n)=max over additive characters of digit a_i(c(s_tor([k]P))) correlation with e(-jk/n); does any digit beat the n^{-1/4} Fourier-learning breakeven?","open","Face F2 digit statistics (Fourier version of IDEA-436)."),
 ("d6ad3f","Does KN-FIND-006's 8*dim(V) Macaulay rank deficit survive to a prime field in a descended presentation?","Binary deficit law deficit(4)=8k-1 for full systems; the prime-field digit twin measured deficit 0 for structural reasons.","open","Prime-field digit presentation."),
 ("de67f0","H-CREP-001's complete-chart intersection resultant r_R has no self-contained definition (RT103-C1 never discharged)","The mechanism sentence z_R=gcd(g_I,r_R) is undefined.","open","H-CREP-001 lane."),
 ("ed922d","Can a prime-field ECDLP advantage remain hidden behind a subgroup-preserving correspondence?","Separate: does a transfer offer computational advantage, and can it stay hidden?","open","Trapdoor framing of transfers."),
 ("f008ae","H-CREP-001's z_R has an improved but still over-cap MITM construction (Theta(B^3)/Theta(B^2))","Resultant root-product identity plus MITM on S_3 degree 2.","open","H-CREP-001 lane."),
 ("f2f6dc","KKM crater/floor asymmetry across a large prime conductor gap","Can ECDLP be strictly easier on the volcano floor than on the crater, and can a floor-only weakness be exploited against a crater curve faster than sqrt(q)? Known: floor->crater O~(q^{1/4}); crater->floor min(N^2,q^{1/2}); random self-reducibility within floor O~(q^{2/5}).","open","Isogeny volcano structure."),
]
for kid, title, stmt, st, scope in KOI:
    add(f"KN-OPEN-{kid} {title}", stmt, st, scope, f"{KO}/KN-OPEN-{kid}.md")

# ---------------------------------------------------------------------------
# F. Corpus-level pointers
# ---------------------------------------------------------------------------
add("ideas/README generation history (IDs 001-436; 17 active, 27 deferred, ~370 rejected)",
    "Every 12-ID generation pass since 2026-07-17 produced zero promoted survivors; the active registry has stayed at 15-18 cards; the rejected tree (ideas/rejected/, 369 files) contains scoped-negative or merged cards spanning: Hecke operators (042, 038), geometric Satake (095), Bost-Connes (295), AG/Goppa codes and list decoding (014, 047, 130, 210, 336), Anderson t-motives/Drinfeld (031, 407), prismatic/perfectoid (116, 270), Ax-Katz/Chevalley-Warning (379), combinatorial Nullstellensatz (156), Ihara zeta (083), Kolyvagin/Euler systems (283), Iwasawa (294), Coleman-Gross p-adic heights (264), Coleman elliptic units (211), unipotent Albanese (021), Noether-Lefschetz (039), Raynaud-Gruson (408), Dwork Frobenius (197), Cartier operator (020), Witt-Burnside (179), Rota-Baxter (313), Hopf-Galois (100), motivic DT (287), Garside braids (189), Picard-Lefschetz (126), Freudenthal-Jordan (214), SOS/Lasserre (328), Lovasz theta (335), flag algebras (366), determinantal processes (282), Bethe loop series (240), random currents (238), HDX sheaf decoder (132), sumcheck (138), Gelfand-Levitan-Marchenko (278), Balog-Szemeredi-Gowers (340), nilspaces (118), metaplectic/Weil representation (101), polynomial partitioning (326), ray-class Artin descent (018), tame symbols (040), shtukas (272), Arthur trace (293), Hrushovski group configuration (301), Lattes/arboreal (113), Serre-Tate jets (109), arithmetic differential characters (263), Saito higher residues (265), semismall maps (249), Chow-Lam forms (255), cyclotomic trace (271), cellular sheaf Laplacian (236), derived Tor (088), holonomic telescopers (105), Gorenstein liaison (204), Ulrich/Chow atomizer (115), Mal'tsev cosets (122), formal-logarithm torsion (129/140/150), and pre-ID duplicates on KZ associators, Drinfeld-Sokolov, Cartan-Dieudonne, WDVV, Quillen A, Lang-Weil, Brandt matrices, Goemans-Williamson, Swendsen-Wang/Wang-Landau/umbrella sampling, survey propagation, Szemeredi regularity, FKS hashing.",
    "corpus state (not a hypothesis)",
    "Generators must grep ideas/rejected/ by keyword before proposing; the recurring kill is the same: an 'exact source inverse' (endpoint -> labelled factor-base tuple) is never supplied, and any object faithful enough to carry source labels has B^m size or dense-resultant cost.",
    f"{IDEAS}/README.md")

add("ideas/EXTERNAL_CORPUS pointer index and recorded external verdicts",
    "Non-canonical dedup input from /Volumes/Volume/research: ~111-mechanism catalogue (unscreened), 23-paper review naming three untouched gaps (syzygies/Betti numbers of the Semaev ideal, singular loci of the Semaev variety, Yokoyama semi-normality as top F_p test), isogeny x Semaev sweep (solving complexity is an isogeny-class invariant; alpha-stable FHJRV bases enrich yield 7-16x without lowering solving degree; GLV/CM folding 3-6x flat), rho frontier (0 live/9 capped/17 dead), external coordinate/orbit/p-adic-cocycle/spectral/quadratic-character-tree screens (all 0/7 curves against matched random controls; everything collapses to signed/shifted BSGS time-memory variants), P-256 structure certificates (End=O_K, k=(n-1)/3), SAT-vs-GB, ML triage signal real at n=8 gone by n=10, EC_SIEVE Walsh predicates flat ~1e-3.",
    "corpus state (external screening priors; no approved state)",
    "Names explicit unexplored items: Betti/syzygy structure, singular loci, semi-normality; everything else listed is screened negative at toy scale.",
    f"{IDEAS}/EXTERNAL_CORPUS.md")

# ---------------------------------------------------------------------------
# G. GAPS -- areas appearing in NO card (47 active+deferred cards, prym memos, THM notes, KN-OPEN),
#    with grep evidence over the whole ideas/ tree (incl. 369 rejected cards) and knowledge/ (non-literature vs literature).
#    Counts: cards = files among the 124 digested card/memo/open-problem files; ideas_tree = all files under ideas/;
#    know_nonlit = knowledge/{findings,techniques,frontiers,leads,gathers,open-problems,...}; know_lit = knowledge/literature (8132 seed entries).
# ---------------------------------------------------------------------------
G = "gap (no card; grep-verified 2026-10-31, word-bounded, case-insensitive)"
def gap(name, statement, scope, counts):
    add("GAP: " + name, statement, G, scope, "grep over /Volumes/SSD990-2/crypto-autoresearcher/{ideas,knowledge}; counts(cards/ideas_tree/know_nonlit/know_lit)=" + counts)

gap("Arakelov theory / arithmetic intersection theory",
    "No card uses Arakelov intersection pairings, arithmetic Chow groups or Faltings-height style bounds on the global lift. Relevance: the Brief's 'Heights' wall and IDEA-005/435 use only the naive/Neron-Tate canonical height; an Arakelov-theoretic treatment of the lifted surface (archimedean + finite contributions, Zhang's admissible pairing on the reduction graph) could give the missing 'dimension count over all section shapes' or a sharper lower bound killing all height-compressing lifts.",
    "Nearest cards: IDEA-005 (height compression), IDEA-435 (MW target coefficient), THM none. Knowledge mentions only in sources index.",
    "0/0/2/4")
gap("Shimura varieties and Shimura curves (quaternionic uniformisation)",
    "Zero occurrences in any idea card or rejected card. Relevance: supersingular/CM loci and Hecke correspondences on modular/Shimura curves give a geometric home for isogeny graphs; a Shimura-curve (quaternionic) parametrisation of E (non-modular over Q, RM over totally real fields) is unexamined as a representation of the pair (E,P).",
    "Hecke operators themselves ARE in rejected cards (IDEA-042, 038, 095); Shimura varieties are not.",
    "0/0/4/25")
gap("Sato-Tate / Katz-Sarnak equidistribution and random-matrix statistics of Frobenius",
    "No card uses Sato-Tate groups, Katz-Sarnak monodromy of families, or random-matrix (GUE/orthogonal) statistics of normalised Frobenius traces. Relevance: the monodromy closure (KN-FIND-a1f3c2) concerns the Semaev cover; equidistribution in FAMILIES of covers (C_t same-E family of prym 0bbc32, or the isogeny class) could predict the variance of yield that the mean-yield closure leaves free.",
    "Random matrix theory appears only incidentally (rejected IDEA-141, KN-TECH-021). Chebotarev/Frobenius cycle census IS used (KN-OPEN-009/62c870).",
    "0/0/4/17 (Sato-Tate); 0/3/3/4 (random matrix)")
gap("Lang-Trotter and prime-order/trace distribution heuristics",
    "No card uses Lang-Trotter conjectures or the distribution of traces/primes of prescribed Frobenius trace. Relevance: density of curves with weak extra structure in an isogeny class (KN-OPEN-cc1988) and of favourable divisor structures of l-1 (IDEA-003/011 applicability) are exactly Lang-Trotter-type counts; nothing quantifies the 'applicability density' zeta those cards charge.",
    "Nearest: IDEA-003/011 charge N^zeta without computing it.",
    "0/0/2/11")
gap("Cohen-Lenstra heuristics / class-group statistics",
    "Never mentioned outside two literature seeds. Relevance: Brief says isogenies buy nothing at h(O_K)=1 and KN-OPEN-f2f6dc/cbbd97 concern conductor gaps; Cohen-Lenstra statistics for Pic(O) of the endomorphism order would quantify how often a crater has a usable class-group action or a smooth class number.",
    "Class groups/numbers appear in 4 cards only as 'class number 1' facts.",
    "0/0/0/2")
gap("Mazur torsion theorem, Ribet level-lowering, Eisenstein ideal (modular arithmetic of torsion)",
    "Zero idea-tree hits. Relevance: KN-TECH-06bb4e/73630e lifting faces and IDEA-436 lift torsion to Q_p; the modular-curve X_1(N) appears only via gonality (IDEA-011). Rational-torsion and Eisenstein-ideal congruences between cusp forms and Eisenstein series are a structured source of relations among torsion points over number fields that no card has examined as a global-lift target.",
    "Nearest: IDEA-011 (X_1(l) gonality), IDEA-005.",
    "0/0/6/57")
gap("BSD / Tate conjecture / L-function special values as computational handles",
    "No card attempts to use L(E,s), its derivatives, Heegner-point formulas (Gross-Zagier appears only in rejected IDEA-283/096) or analytic rank as a route to Mordell-Weil generators of the lifted curve; Mestre-type high-rank constructions (choosing the lift to have large rank with known generators) appear only in two lead YAMLs (LEAD-TRF-global-lift-small-height, LEAD-ICP-char0-lifting-xedni-smoothness) and no card.",
    "Nearest: IDEA-005 global lift (needs MW generators but computes them by lattice search); rejected IDEA-283 Kolyvagin.",
    "0/0/5/50")
gap("Complex-analytic uniformisation: period lattice, Weierstrass p-function, elliptic integrals, AGM",
    "Zero ideas-tree hits for period lattice / elliptic integrals / Weierstrass p (one KN-OPEN mention). Relevance: on a CM canonical lift (face F3) the elliptic logarithm z=p^{-1}(x) is a complex number; Masser-style linear forms in elliptic logarithms give the explicit height/degree obstruction and could prove the missing 'section-shape dimension count' for lifts; AGM/Mestre point counting appears only in literature.",
    "Nearest: IDEA-005 (heights), KN-TECH-06bb4e taxonomy. Transcendence theory also absent (0/0/2/6).",
    "1/0/2/4")
gap("Towers of function fields (Garcia-Stichtenoth, Drinfeld-Vladut-Ihara bound) and asymptotically good AG codes",
    "Only rejected IDEA-083 (Ihara zeta, graph version). Relevance: the Brief's Drinfeld-module question (why DLP on Drinfeld modules is IC-easy) sits next to explicit towers; a factor base indexed by places of a tower over F_{2^131} would be a non-subspace, Frobenius-stable algebraic description (Brief lever 6) never considered.",
    "AG codes as decoders are in rejected cards (014, 047, 130, 210, 336) but towers/asymptotic bounds are not.",
    "0/5/0/4")
gap("Drinfeld modules / Carlitz modules / Anderson t-motives as the comparison object",
    "Only rejected IDEA-031 (Anderson t-motive extension coordinate) and IDEA-407 (Drinfeld center, a different notion). The Brief's lever 14 (DLP on Drinfeld modules is broken by index calculus: what structural difference makes E(F_q) resist?) has NO card and no knowledge/findings entry.",
    "Nearest: IDEA-031 (rejected), knowledge literature seeds (14).",
    "0/7/5/14")
gap("Hilbert modular / RM (real multiplication) surfaces, Humbert surfaces",
    "Zero ideas-tree hits (word-bounded). Relevance: genus-2 Jacobians with RM are the natural destination of Richelot/Kani gluing and of the Prym family; the moduli (Humbert surfaces) decide when a genus-2 cover has extra endomorphisms that could supply the 'second N-torsion direction' the closed lead lacks.",
    "Nearest: prym README (Enolski-Fedorov (1,2)-polarised Pryms), closed LEAD-TRF-second-torsion-direction.",
    "0/0/4/22")
gap("Siegel modular forms, Igusa invariants, theta constants (Thetanullwerte), Rosenhain/Thomae formulae",
    "One rescreen slice only. Relevance: the Prym and genus-2 cover work (prym_leads) uses explicit curve equations; Igusa/theta-constant coordinates give isogeny-equivariant coordinates on the genus-2 moduli where (2,2)-isogenies (Richelot) are rational maps, i.e. a 'Kummer surface' analogue of the x-only summation polynomials for the cover.",
    "Nearest: IDEA-161 (theta groups, algebraic), prym MULTICOVER branch.",
    "0/1/2/15")
gap("Ekedahl-Oort / Newton-polygon stratification, p-rank and a-number of abelian varieties",
    "Zero hits everywhere for Ekedahl-Oort/Newton stratification; Hasse-Witt/Cartier-Manin appear in one rejected card (IDEA-020). Relevance: for the Prym/genus-2 cover route over small characteristic (binary curves), the p-rank of J(C) determines whether 2-torsion/Cartier-type structure (FGHR 2-torsion translation symmetry, Brief symmetry inventory) extends to the cover.",
    "Nearest: rejected IDEA-020 (Cartier differential syndrome), THM_QUOTIENT_DESCENT1 (2-torsion).",
    "0/1/3/3")
gap("Honda-Tate theory / Weil polynomials classification of isogeny classes",
    "Only one card mention (prym README: Tate/Zarhin Hom(E,A)=0 from coprime Frobenius polynomials). Relevance: the TORSION/PRYMMODEL branches of the prym memo need, for a given E, the classification of abelian varieties A over F_q whose Weil polynomial shares the r-factor; Honda-Tate gives the complete list and could settle 'second N-torsion direction' existence by theorem.",
    "Nearest: prym TORSION branch, KN-OPEN-f2f6dc.",
    "1/1/4/5")
gap("Hasse invariant / Deuring-Eichler supersingular polynomial / Deuring lifting theorem",
    "Zero ideas-tree hits. Relevance: Deuring lifting (every (E,alpha) over F_p lifts to a CM curve over a number field) is the theorem behind face F3 CM canonical lifts (Brief lever 11, open_next_step) and no card states or uses it.",
    "Nearest: IDEA-005, LEAD (atlas) height-compressing lifts; Deuring correspondence (quaternion side) appears in 2 cards.",
    "0/0/0/2")
gap("Deligne-Lusztig characters as x-functional filters (THIN, not empty: rejected IDEA-153 anisotropic-torus character tomography and IDEA-187 Soergel light leaves exist)",
    "No active/deferred card and no KN-OPEN uses Deligne-Lusztig characters; the two rejected cards used them as scalar-tomography/source-basis devices, not as Wagner-level bucket filters. Relevance: the x-functional filter question (KN-OPEN-91d4f6) asks for non-group filters correlated with sums; Deligne-Lusztig characters of E viewed as an algebraic group (or of its Weil restriction) are the canonical non-abelian-looking functions on E(F_q) with controlled sum behaviour.",
    "Nearest: KN-OPEN-91d4f6, THM_INCBARRIER (additive characters only), rejected IDEA-101 (metaplectic).",
    "0/4/2/1")
gap("Eisenstein series, Kronecker limit formula, Chowla-Selberg, Hecke Groessencharacters / CM L-functions",
    "Zero ideas-tree hits. Relevance: for CM curves (Koblitz K_0 has End=Z[tau], class number 1) the Hecke character of the CM field determines Frobenius exactly; CM L-function values and Kronecker-limit-type identities relate periods to class invariants and are the only analytic handle on the canonical lift (face F3) that no card uses.",
    "Nearest: KN-OPEN-2c095b (lambda order deficit), Koblitz program.",
    "0/0/3/6")
gap("Large sieve / Bombieri-type inequalities, Burgess and Polya-Vinogradov short character sums",
    "No card beyond THM_INCBARRIER's per-character Weil bound. Relevance: THM_INCBARRIER2 Theorem F says certification at harvesting scale needs a BILINEAR bound of quality theta in [2/5,7/10]; large-sieve and Burgess-type bilinear/short-interval estimates (or Bourgain-Garaev sum-product on E) are exactly that class and were not attempted.",
    "Nearest: THM_INCBARRIER1/2 (named gap G1/Theorem F).",
    "1(THM mention only)/0/5/7")
gap("Explicit class field theory / Hilbert's twelfth problem, Stark/elliptic/Siegel/modular units",
    "Ray-class descent (rejected IDEA-018), Coleman elliptic units (rejected IDEA-211) and modular-unit diamond eigenspaces (rejected IDEA-112) exist; Hilbert's 12th problem / Stark units / Siegel units as explicit generators of ray class fields of the CM field do not appear in any card. Relevance: units of the CM field are the S-units of IDEA-007's module in the global-lift setting.",
    "Nearest: rejected IDEA-018, 112, 211; IDEA-007.",
    "0/4/2/2")
gap("Brauer-Siegel / Siegel zeros / Dedekind zeta and class number formula",
    "Zero everywhere (Brauer-Siegel) or sources-only. Relevance: regulator-class-number products bound the size of MW/unit lattices used in IDEA-005/007 lifts.",
    "Nearest: IDEA-005.",
    "0/0/0/0")
gap("del Pezzo / cubic surfaces / 27 lines (surfaces containing E as a hyperplane section)",
    "Zero ideas-tree hits. Relevance: E embeds as a plane cubic = hyperplane section of a cubic surface; the 27 lines and Cremona group of the surface act on the configuration of points of E; unexamined as a source of correspondences on E beyond IDEA-059's Cremona maps on the decomposition variety.",
    "Nearest: IDEA-059 (Cremona on the symmetric product), IDEA-050.",
    "0/0/2/5")
gap("Jacobi forms / theta functions with characteristics as functions of (E, point)",
    "Jacobi forms: zero hits. Relevance: the pair (E,P) is a point on the universal elliptic curve; Jacobi forms are exactly the modular objects on that space, and their Fourier-Jacobi coefficients organise division polynomials and elliptic nets (IDEA-006) in a way not used.",
    "Nearest: IDEA-006 (elliptic nets), IDEA-161 (theta groups).",
    "0/0/0/2")
gap("Sphere packings / Leech lattice / lattice codes (Construction A, Barnes-Wall)",
    "Zero word-bounded hits. Relevance: the Coppersmith/Jochemsz-May lattices of the Brief's 'Lattice reach' wall are generic; structured lattices with known dense packings could change the determinant/root tradeoff only if the Semaev lattice can be mapped onto them; never examined.",
    "Nearest: IDEA-049, Brief lever 3 (D4-symmetrised S_5 lattice).",
    "0/0/0/3")
gap("Finite geometry proper: projective planes, ovoids, hyperovals, spreads as factor-base indexing",
    "Only block designs (IDEA-064) appear. Relevance: a factor base indexed by a hyperoval/ovoid over F_{2^131} has bounded-degree algebraic description and large automorphism group (Brief lever 6 non-subspace Frobenius-stable bases); unexamined.",
    "Nearest: IDEA-064 (design scheduling), KN-OPEN-5e3dd0 (Hamming ball).",
    "0/3/1/1")
gap("Igusa local zeta functions of the descended/digit systems (precision recursion)",
    "Only rejected IDEA-193 (Denef-Loeser motivic arc source transform) touches motivic integration; no card uses the Igusa LOCAL zeta function Z(s)=sum_k #{solutions mod p^k} p^{-ks}. Relevance: counting solutions of the Weil-descended/digit systems modulo p^k is exactly an Igusa local zeta function; its rationality (Igusa/Denef) gives exact recursion in precision, relevant to IDEA-436's precision growth question and to the PKM plateau (Brief lever 4).",
    "Nearest: IDEA-436, KN-OPEN-3417fc, rejected IDEA-193 (motivic arcs, closed).",
    "0/1/0/0")
gap("Mahler measure, difference equations and Mahler functions",
    "One rejected card (IDEA-070 p-kernel automaton). Relevance: Boyd's Mahler-measure/L-value identities are the number-theoretic face of elliptic regulators (IDEA-119, deferred) and automatic sequences model the digit presentation (Brief lever 9).",
    "Nearest: IDEA-119, rejected IDEA-070.",
    "0/1/2/4")
gap("Fermat quotients / Wieferich-type conditions",
    "Two incidental hits. Relevance: Satoh-Araki's anomalous attack is literally a Fermat-quotient computation; higher Fermat quotients q_k and their distribution are the natural 'digit' invariants for KN-OPEN-d3a6f0 and IDEA-436 but are not named.",
    "Nearest: IDEA-004 (cites Satoh-Araki), IDEA-436, KN-OPEN-b9907c.",
    "1/2/3/4")
gap("Semi-normality (Yokoyama), singular loci and Betti/syzygy tables of the Semaev ideal",
    "Named as untouched by EXTERNAL_CORPUS; semi-normal appears in 5 non-literature knowledge files and 2 ideas-tree files, no card. Relevance: KN-FIND-006's 8k rank deficit IS a Betti count; a syzygy-table law for S_m would decide the solving-degree growth (Brief lever 1) directly.",
    "Nearest: THM_SEMAEV_FALL1, KN-OPEN-d6ad3f, deferred IDEA-098/152; Yokoyama semi-normality is named in frontier rows KR-IC-401ebf/5931ee/0d2021 and EXTERNAL_CORPUS but no card tests it.",
    "0/2/5/4")
gap("Hilbert irreducibility / specialisation theorems for sections of elimination maps",
    "Word 'specialization' appears generically; no card uses Hilbert irreducibility or Noether-type specialisation bounds. Relevance: IDEA-068's fatal obstruction ('no rational section generically because of monodromy') is a Hilbert-irreducibility statement; making it quantitative over F_p (Lang-Weil + irreducibility of fibres) would turn several deferred cards into theorems.",
    "Nearest: deferred IDEA-068, KN-OPEN-009. Counts confounded by the generic word 'specialization' (12 cards use it in other senses); 'Hilbert irreducibility' itself: 0 cards.",
    "0(HIT)/87(confounded)/8/12")
gap("Model theory of finite fields (pseudofinite fields, Chatzidakis-van den Dries-Macintyre, Ax-Kochen), definable sets",
    "One card mention; rejected IDEA-301 (Hrushovski group configuration). Relevance: definability/complexity-of-definable-sets bounds (CDM: definable sets have point counts mu q^d + O(q^{d-1/2}) with finitely many mu) give uniform-in-p statements about any 'algebraically described factor base' (KN-OPEN-020 universal no-go) without case analysis.",
    "Nearest: KN-OPEN-020, P1546.",
    "1/4/0/6")
gap("Transcendence theory (Schanuel, linear forms in elliptic logarithms, period conjecture)",
    "Zero ideas-tree hits. Relevance: the only tool that PROVES height/degree lower bounds for global lifts (Masser independence quoted in IDEA-435) is Baker/Masser-Wuestholz theory; no card uses it to prove the F4/F5 obstruction rather than measure it.",
    "Nearest: IDEA-005, IDEA-435.",
    "0/0/2/6")
gap("Locally testable/decodable codes, PCPs and interactive-proof arithmetisation as relation-verification compression",
    "Rejected IDEA-138 (sumcheck) and IDEA-132 (HDX sheaf); no active card. Relevance: 'certified decomposition' appears in every card as a cost; batch verification of B relations via IOP/sumcheck is an engineering (constant) device by Brief rule 2, but output-sensitive sumcheck over the Semaev ideal is the one algebraic-proof technique not screened.",
    "Nearest: rejected IDEA-138; Brief rule 2 (engineering only).",
    "0/1-3/3-7/17-78")
gap("Mumford-representation / Cantor arithmetic and arbitrary-class reduction in the Prym/genus-2 destination",
    "Zero word-bounded hits for Mumford representation or Cantor's algorithm anywhere; the prym memo lists 'arbitrary-class arithmetic and verified complete cost' and 'destination relation algorithm' as open obligations (PRYMMODEL/TORSION branches) but no card specifies the Jacobian arithmetic (Cantor/Mumford, or Kummer-surface arithmetic) through which a conorm image would be reduced.",
    "Nearest: IDEA-002, prym README PRYMMODEL/TORSION branches.",
    "0/0/0/2")
gap("Derived/infinity-categorical and homotopy-theoretic frameworks",
    "Zero hits; listed for completeness. Low relevance (no computational content distinct from the classical objects already closed).",
    "n.a.",
    "0/0/0/0-1")

out = pathlib.Path("/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/brief/MAP_ideas_corpus.json")
doc = {
    "generated": "2026-10-31",
    "task": "digest of crypto-autoresearcher IDEAS corpus (active + deferred cards, Prym follow-ups 2026-10-27, THM_* theory notes, KN-OPEN open problems) plus grep-verified GAP list",
    "sources_read": {
        "active_cards": 20, "deferred_cards": 27, "prym_followups": 5, "theory_notes": 9, "open_problems": 66,
        "ideas_rejected_tree_files_grepped": 369, "knowledge_literature_files_grepped": 8132
    },
    "status_vocabulary": ["proposed", "deferred", "proved", "refuted", "open", "corpus state", "gap"],
    "recurring_obstruction": "Every representation-changing card dies at the same point: no public 'exact source inverse' from an endpoint/aggregate to a labelled factor-base tuple exists without materialising a B^m (or dense-resultant) object; every scalar-digit card dies at functoriality (all generators of <P> give the same functorial data) or GGM-simulability; every transfer card dies at Frobenius equivariance / Tate's theorem; every yield card dies at (Z/2)^{m-2} monodromy and transport conservation.",
    "items": items,
}
out.write_text(json.dumps(doc, indent=1, ensure_ascii=False))
print(out, len(items))
