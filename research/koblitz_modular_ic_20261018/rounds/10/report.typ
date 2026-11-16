#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.40em)
#set heading(numbering: none)
#show link: set text(fill: rgb("245c91"))
#heading(level: 1)[#text("Round10 — F2 equations, denominator saturation and coupled divisors")]

#text("2026-10-19 UTC. Parentf75e3ddaf54148ffe874a5f927020aae107bc92e. All four determinant19 quotients now have explicit quadratic F2 presentations and closed affine embeddings of specified descended dense opens. The presentations include inverse witnesses,so denominator saturation is built into those affine models. A closed projective embedding of the whole proper quotient remains open.")

#text("The divisor of x0*xinf is complete:22 closed cusp points,with zero and pole degrees2640 each. All eight pole divisors and cuspidal zeros are computed. Seven noncuspidal zero divisors remain unresolved. The diagonal component has400 identified supersingular moduli labels but460 total multiplicity still unaccounted for. No relation rank or IC cost is promoted.")

#image("routes.svg", width: 100%)

#heading(level: 2)[#text("Four actual polynomial presentations over F2")]

#text("The construction expands each K=F2^360 coordinate in the fixed power basis. All coefficients are0 or1. Every extension-coordinate operation has360 scalar polynomial equations. Its addition and coefficient-Frobenius blocks are linear; multiplication and inverse blocks are quadratic. The five elliptic equations,two Kummer equations and twelve descent coordinates are imposed through the retained rational Frobenius circuits.")

#table(columns: 5, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Graph")],
[#text("Operation blocks")],
[#text("F2 variables")],
[#text("F2 equations")],
[#text("Maximum degree")],
),
[#text("0")],
[#text("1171")],
[#text("421560")],
[#text("424080")],
[#text("2")],
[#text("1")],
[#text("1171")],
[#text("421560")],
[#text("424080")],
[#text("2")],
[#text("2")],
[#text("1148")],
[#text("413280")],
[#text("415800")],
[#text("2")],
[#text("3")],
[#text("1148")],
[#text("413280")],
[#text("415800")],
[#text("2")],
)

#text("The total is1,679,760 equations. This is an explicit auxiliary-coordinate presentation,not a smaller relation ideal in the previous trace generators. The JSON stores every indexed operation and residual,with a shared table of719 reduced basis powers. This lossless block format specifies every scalar polynomial. Seventy-two representative scalar equations are also expanded to literal sparse monomials,and4608 local gate controls pass. There is no unspecified correction function or symbolic arithmetic oracle.")

#text("Coefficient Frobenius acts linearly on the360 coefficient variables. Squaring those indeterminates would change the scheme:over dual numbers, coefficient Frobenius fixes the nilpotent parameter while absolute squaring kills it. No Boolean equations are imposed. Native controls check all129600 basis products,360 basis-Frobenius identities and64 dual-number identities. For each graph,two K-valued fixtures recover all twelve input coefficient blocks through the trace-dual inverse. All360 split components are checked. The generator accounts for3359520 equation-component checks and144000 unit-component checks;the parser repeats the checks and recovers 34560 input coefficient values across the eight fixtures.")

#heading(level: 2)[#text("What saturation and embedding have been established")]

#text("For every inverse gate,the polynomial equations state d*u=1 in K tensor the base algebra. Taking determinants of multiplication matrices shows Norm(d)*Norm(u)=1. Consequently the graph ideal is saturated by the product of these denominator norms. The inverse witnesses are unique, so the polynomial graph represents the rational construction on its specified open exactly. The Lean proofs establish inverse cancellation, ideal cancellation,power saturation and the determinant implication.")

#text("After base extension back to K,the restriction of scalars splits into360 conjugate components. The equalizer equations identify adjacent components by the actual rational lift. The retained360-fold cocycle closes the cycle. Projection to one component and its inverse recover the intersection of all360 translated domains. This gives the stated closed affine model of a descended dense open;the scheme-descent bridge is written in PROOF_​MAP.md and uses ")#link("https://stacks.math.columbia.edu/tag/0244")[#text("Stacks affine descent")]#text(". It is not a full Lean formalization or a consequence of samples alone.")

#text("An eliminated saturated generating ideal in just the4320 original coefficient variables has not been computed. Keeping the inverse and operation variables provides the effective affine presentation. Nor does this construction recover every boundary of the proper quotient,the full six-chart projective gluing,or its polarization on a closed projective model. Jacobian identification and individual descent remain separate gaps.")

#text("The projective control makes this distinction concrete. Homogenizing xy=z,xu=1 gives xy-zh=xu-h²=0. The projective point [x:z:y:u:h]=[0:1:0:1:0] satisfies these equations but is excluded by zu-yh=0. Its membership certificate is")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("h*(zu-yh)=y*(xu-h²)-u*(xy-zh).", block: true)]

#text("Adding zu-yh gives the complete saturated prototype ideal. All three Buchberger S-pairs reduce to zero;the leading monomials xy,xu,zu omit h, which proves h-regularity and saturation. This repairs the prototype. The homogenizing-variable saturation of the full quotient ideal remains uncomputed. The false point refutes the general inference from naive homogenization to a correct projective closure.")

#heading(level: 2)[#text("Complete cusp strata for all eight functions")]

#text("The modular carrier has eight base cusps and60 full-eleven-level cusp labels above each,480 geometric cusps total. A fixed line is canonical at five of the60 labels. Its quotient map has ramification121 there and1 at the remaining55,so each map has degree660. The six middle base cusps have explicit F8 coordinates;both normalized Tate-level transport actions and all local curve jets are checked. Distinct eleventh-root leading coefficients verify cancellation orders when both maps have a pole. The moduli/Tate interpretation is a written bridge,not an inference from the degree counts. The base eta-coordinate divisor is independently supported by ")#link("https://arxiv.org/pdf/1110.6801")[#text("Kilford–McMurdy,section2")]#text(".")

#table(columns: 4, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Function")],
[#text("Pole degree")],
[#text("Cuspidal zero degree")],
[#text("Residual noncuspidal zero degree")],
),
[#text("x0+xinf")],
[#text("2520")],
[#text("420")],
[#text("2100")],
[#text("x0+xinf+1")],
[#text("2520")],
[#text("0")],
[#text("2520")],
[#text("y0+yinf")],
[#text("3780")],
[#text("360")],
[#text("3420")],
[#text("y0+yinf+1")],
[#text("3780")],
[#text("0")],
[#text("3780")],
[#text("x0*xinf")],
[#text("2640")],
[#text("2640")],
[#text("0")],
[#text("x0*xinf+1")],
[#text("2640")],
[#text("0")],
[#text("2640")],
[#text("x0*yinf+xinf*y0")],
[#text("3900")],
[#text("450")],
[#text("3450")],
[#text("x0*yinf+xinf*y0+1")],
[#text("3900")],
[#text("0")],
[#text("3900")],
)

#text("The cross-coordinate function has30 extra zeros where the two middle cusp images are distinct points with y=x. Exact elliptic jets give multiplicity1 there. The residual column includes possible supersingular points;it is not labeled ordinary without resolving that stratum. All bihomogeneous ambient zero sections and fixed components are retained.")

#text("For x0*xinf,the complete formula is")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("div(x0*xinf)=2 phi0^*(T)+2 phiinf^*(T)\n  -2 phi0^*(O)-2 phiinf^*(O), T=(0,1).", block: true)]

#text("Only cusps occur. At each of the two rational base cusps,the sixty labels have multiplicity244 on the ten labels where one tested line is canonical, and4 on the other fifty. The normalized Frobenius sends(b,d) to(b,d/2) modulo simultaneous sign. This partitions each sixty-label fiber into five degree1 points,one degree5 point and five degree10 points. Thus the whole divisor has22 closed points,with signed multiplicities retained. Every projected even/odd image is zero. This supplies a complete coupled divisor but no new nonzero-image relation rank. The previous rank11 remains.")

#heading(level: 2)[#text("Equal-map support: a precise unresolved multiplicity")]

#text("For x0+xinf,the complete Cartier identity separates two components:")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("div(x0+xinf)=(phi0-phiinf)^*(O)+(phi0+phiinf)^*(O)\n  -2 phi0^*(O)-2 phiinf^*(O).", block: true)]

#text("The two map degrees are1280 and1360,using the retained Gram entry20. The diagonal component has cuspidal degree420,leaving860. Its ordinary source-j support is screened by the exact collision resultant")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("Res_Y(Phi11(X,Y),(1+X^4)Y^4+X^3Y^3+X^4)\n  =X^88*(X+1)^8.", block: true)]

#text("Exact native rational-polynomial arithmetic computes this degree96 resultant;2046 finite-field gcd controls agree. Lean checks the derivative and factorization identities. The only ordinary candidate is j=1. The two horizontal norm11 endomorphisms have no equal transport on any of the56 cyclic49 lines;both charts of that finite exclusion are kernel checked in Lean. This is a transported-level test beyond j alone.")

#text("For j=0,the actual24 automorphisms of y²+y=x³ are bound to Hurwitz units using225 curve points and a degree≤36 identity argument. Actual Frobenius is identified as -J+IJ. The chosen torsion-module representations then enumerate288 norm11 elements,12 kernels,56 cyclic49 lines and660 normalized full-eleven bases. Quotienting by the12 units modulo sign leaves3080 modular classes. Exactly400 match the two transported49 subgroups,in40 Frobenius orbits of degree10. These are explicit moduli labels;coordinates of all49/11 torsion points have not been generated. The Hurwitz framework is described in ")#link("https://abel.math.harvard.edu/~elkies/M223.24/index.html")[#text("Elkies's elliptic-curve notes")]#text(".")

#text("The available target-stabilizer lower bounds sum to400,not860. The degree-exhaustion test therefore fails,leaving460 multiplicity unaccounted for. No exact supersingular diagonal divisor is claimed. The antidiagonal component and the other noncuspidal supports remain open. The next discriminating calculation is the local deformation/intersection multiplicity at these40 closed labels,with a verified local model or a faithfully applicable CM deformation theorem. Labels and total degrees cannot replace that calculation.")

#heading(level: 2)[#text("Proofs,evidence and remaining work")]

#text("Twenty-three new named Lean proofs cover inverse ideals and saturation, coefficient algebra,homogenization,finite49 transport,collision identities and divisor arithmetic. Full field semantics,Galois descent,Tate cusp geometry and Hurwitz-module identification remain written bridges. The proof map states each scope. Two source-bound universal identities receive64 issuer and128 extra checks each. All thirteen new JSON records replay exactly,and round09 also replays unchanged. Root/native/Lean/site checks and the archive receipts are in VALIDATION.md.")

#text("Failures are retained:Rust API/type errors,a special-case gcd call with untrimmed coefficients,the first Lean cancellation rewrite,and an imprecise coefficient-counter label corrected without changing arithmetic. The false projective point and the diagonal degree deficit remain negative results. Earlier evidence is unchanged. Same-agent parser/replay checks share declared primitives;there is no external audit.")

#text("Remaining acceptance obligations are a verified global projective model with its saturated boundary ideal and polarization,and complete noncuspidal zeros with multiplicities and pole accounting for the seven remaining functions. No new complete evaluated relation matrix,solver run or timing is supplied. The user assesses IC efficiency.")

#heading(level: 1)[#text("Formal proof map and written arguments")]

#heading(level: 2)[#text("Lean scope")]

#text("KoblitzRound10.lean adds23 named results to the pinned project. The actual build and transitive audit pass;the full gate checks146 project theorems with zero native trusts. Only the standard propext,Classical.choice and Quot.sound foundations are admitted; no custom axiom,sorryAx or trusted native decision is used.")

#table(columns: 2, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Named results")],
[#text("Exact scope")],
),
[#text("inverse_​witness_​unique,inverse_​witness_​cancel")],
[#text("Ring inverse uniqueness and cancellation")],
[#text("inverse_​ideal_​cancel,saturation_​powers")],
[#text("Ideal cancellation with a unit witness;all powers under an explicit regularity hypothesis")],
[#text("determinant_​inverse_​witness")],
[#text("Matrix inverse implies determinant-unit identity")],
[#text("coefficient_​bilinear_​expansion")],
[#text("Finite bilinear expansion over any commutative base ring")],
[#text("dual_​number_​square,square_​sum_​residual")],
[#text("Characteristic2 nilpotent square and universal integer residual")],
[#text("component_​sequence_​unique")],
[#text("A recursively linked component sequence is determined by its first component")],
[#text("homogeneous_​saturation_​certificate,naive_​projective_​false_​point,saturated_​relation_​on_​affine_​chart")],
[#text("Exact membership certificate,false projective point and h≠0 implication")],
[#text("product_​cusp_​degree,coupled_​pole_​degrees,sum_​difference_​map_​degrees")],
[#text("Integer arithmetic for the recorded degrees")],
[#text("radial_​line_​derivative,product_​divisor_​sum")],
[#text("Conditional characteristic2 derivative and formal divisor addition")],
[#text("collision_​derivative_​residual,collision_​derivative_​factorization,collision_​resultant_​factorization")],
[#text("Universal residual,char2 derivative factorization and degree96 factorization")],
[#text("ordinary_​transport_​determinant,ordinary_​transport_​finite_​chart,ordinary_​transport_​other_​chart")],
[#text("Universal determinant and both explicit ZMod49 projective charts")],
)

#text("The homogeneous certificate and the collision derivative residual receive source-hash-bound IDC certificates,each64 issuer and128 extra evaluations. Those are identities,not certificates for the whole descended scheme or for the Euclidean resultant implementation. Native matrix/field semantics and global geometry are not imported as axioms into Lean.")

#heading(level: 2)[#text("F2 polynomial systems and scheme interpretation")]

#text("For any F2-algebra A,let K_​A=A[z]/(z^360+z^5+z^3+z^2+1). It is free on1,z,...,z^359. The multiplication tensor is explicitly specified by the719 reduced powers. Comparing coefficients makes each operation block a list of360 scalar polynomials overF2. Constants and addition are linear. A multiplication output satisfies")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("out_i+sum_(j,k with z^(j+k) coefficient_i=1) left_j*right_k=0.", block: true)]

#text("An inverse output uses the same tensor with right=out and constant coefficient1. Coefficient Frobenius is the A-linear map sending z^j to z^(2j);it fixes A. Its equations are linear in the input variables. It is not absolute squaring in K_​A. The64 dual-number controls test this ring distinction. The native field/bit reductions and23 structural Lean lemmas are separate evidence layers.")

#text("The graph equations uniquely determine all operation coordinates from the twelve input blocks whenever each inverse exists. Conversely,any solution of the graph equations supplies precisely those inverses and operations. Hence the graph is an effective affine presentation of the rational formulas on the domain they specify,over arbitrary A,not only over finite fields. The original seven curve/Kummer residuals and twelve equalizer residuals are all explicit indexed outputs in the program.")

#text("For a denominator d and witness u,the multiplication matrices satisfy M_​d*M_​u=1. Their determinants show Norm(d)*Norm(u)=1. Thus the full graph ideal is saturated by D,the product of the norms of all denominator inputs. The norm is the constant coefficient of the product of all360 coefficient-Frobenius conjugates,so D is an explicit polynomial circuit using the retained tensor and inverse-node indices. The ideal-cancellation and determinant statements are Lean checked;their interpretation as these actual free-algebra multiplication matrices is a written bridge.")

#text("This does not provide an eliminated generating set for I:D^infinity in the original4320-variable ring. It supplies its localized graph through inverse witnesses. Eliminating the witness/operation variables would give an affine closure ideal;one must retain the open condition to identify the localized model. No elimination or small invariant presentation is claimed to have run.")

#text("Let U be the original Kummer chart with the further displayed generic line denominators inverted. Apply restriction of scalars to its affine inverse-witness presentation. The equalizer is X=theta(sigma_​coeff X), where theta is the actual round09 rational lift on point coordinates. After base extension to K,the algebra K tensor_​F2 K splits into360 copies through its embeddings. The equalizer becomes a cyclic chain of rational graphs between these conjugate copies. Because all inverse witnesses are required to exist in K_​A,every component lies in the necessary domain. The360-fold cocycle from round09 closes the chain. Projection to one component therefore identifies it with the intersection of all360 translated opens. The inverse is the uniquely specified orbit of graph maps. These substitutions hold over all base algebras and give the scheme isomorphism;the native fixtures merely check its application.")

#text("The intersection is a nonempty stable affine open in the separated proper quotient (a finite intersection of affine opens of a separated scheme is affine). Affine Galois descent is effective,as in ")#link("https://stacks.math.columbia.edu/tag/0244")[#text("Stacks0244")]#text(". The emitted quotient ring gives a closed embedding of this descended open into the stated F2 affine space. It is smooth of dimension5 under the retained connected Kummer/isogeny identification. A full Lean construction of these schemes and that descent equivalence has not been supplied.")

#heading(level: 2)[#text("Projective saturation control")]

#text("The repaired prototype ideal is generated by xy-zh,xu-h²,zu-yh. Under lex order x>z>y>u>h,the leading monomials are xy,xu,zu. All three S-pairs reduce to zero in native F2 polynomial arithmetic. Buchberger's criterion proves this is a Groebner basis. Its initial monomials omit h:if h*f belongs to the ideal,the normal form of h*f is h times the normal form of f,so the latter is zero. Therefore h is regular modulo the ideal and it is h-saturated. Localization at h identifies this ideal with the original two generators by the explicit membership certificate. It is exactly their h-saturation. Lean checks the certificate, the false point and the conditional regularity-to-power implication; the concrete Buchberger/normal-form bridge remains native/written.")

#text("The full quotient has an explicit homogenized block system,but its h-saturation has not been computed. No argument equates its projective closure with the proper polarized abelian quotient. The prototype shows why a general claim from naive homogenization would be invalid;it is not a counterexample exhibited on the full quotient's own homogenized ideal.")

#heading(level: 2)[#text("Cusps,ramification and complete product divisor")]

#text("Use the eta coordinate x of X0(49),which is also the retained minimal elliptic x coordinate. Its divisor is2(T)-2(O). The six other cusps have x a nontrivial7th root and y=(x+1)/(x²+x+1) overF8. This follows both from the frozen eta equation and its cusp divisor;see ")#link("https://arxiv.org/pdf/1110.6801")[#text("Kilford–McMurdy,section2")]#text(".")

#text("On a Tate chart with q=s^11,full-eleven bases have exponent vectors (b,d) in F11² minus zero,modulo sign. There are60. A line is the canonical multiplicative subgroup precisely when its exponent vanishes. Quotient by that line sends q to q^11;quotient by a different line sends q to zeta^k*q^(1/11). Hence its ramification on C is121 or1. Five labels are canonical for each line. Base-cusp widths are1 or49. Both are coprime to11 and do not change these relative exponents. The counts give 5*121+55=660 for each map. This is a Tate-moduli calculation,not a ramification conclusion inferred just from equality of degrees.")

#text("At a middle cusp,a cyclic49 generator can be written using a primitive 49th root and q^(1/7). The multiplicative quotient multiplies its cyclotomic label by11;the other quotients multiply it by11^(-1) modulo7. On the chosen cusp x coordinates these are x^4 and x^2. All actual F8 points and corresponding function values are checked. At equal ramification,the distinct zeta phases yield distinct leading powers for the exponents1,2,3,5 used here. The440 finite leading-coefficient checks cover every pair of distinct phases. The general Tate/integral-moduli framework is described by ")#link("https://math.stanford.edu/~conrad/papers/J5final.pdf")[#text("Cais–Conrad,section2")]#text(".")

#text("At O,the poles of x,y have orders2,3. If one tested line is canonical, the two ramification exponents are121,1;otherwise they are1,1. This gives pole orders242/2,363/3,244/4 and365/5 for x-sum,y-sum, x-product and cross-coordinate functions respectively. At T,x has order2 and y+1 has order1. At middle cusps,x is a uniformizer and the exact implicit y series is checked through order8. These calculate every cuspidal valuation. Three middle cusps also satisfy y=x,producing the additional30 cross-function zeros of order1. All other residual degrees remain noncuspidal without an ordinary/supersingular split.")

#text("For x0*xinf,pulling back2(T)-2(O) gives the complete displayed divisor. Every preimage of either cusp is a cusp,with the sixty labels above. Frobenius on the determinant-normalized level is(P,Q)->(P²,Q²/2); on the Tate exponents this gives(b,d)->(b,d/2). Both O and T are rational. The enumerated cycles give22 closed divisor points. At O every target is O,and at T every target is the two-torsion point T;the even and odd paired target differences both vanish. Thus the projected images are zero already geometrically. No new nonzero-image rank follows.")

#heading(level: 2)[#text("Collision screen and supersingular diagonal gap")]

#text("Differentiating the frozen sparse mod2 Phi11 gives")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("dPhi/dY=X^3*Y²*((1+X^4)Y^4+X^3Y^3+X^4)².", block: true)]

#text("For X nonzero,Phi(X,0)=X^12 excludes Y=0. A repeated eleven-neighbor j value therefore requires the displayed quartic and Phi to have a common root. Native Euclidean resultant arithmetic overF2(X) computes X^96+X^88;all denominators cancel. Trial division exhausts it as X^88*(X+1)^8. Separate small-field gcd controls cover2046 X values. Lean proves the characteristic2 derivative and factorization identities; it does not formalize this resultant algorithm. The only ordinary source candidate is j=1,whose maximal CM order has discriminant-7.")

#text("Its two horizontal norm11 maps are alpha=1+2w and conjugate(alpha)=3-2w, w²-w+2=0. In the recorded basis their determinant on a vector(x,y) is -8*(x²+xy+2y²). A primitive vector modulo49 has one of the two enumerated projective forms;Lean excludes zero in both. Thus those two quotients cannot have the same transported cyclic49 subgroup. The correspondence between maximal-order torsion and this split matrix representation is written,not inferred from determinant matching alone.")

#text("For the supersingular curve,E_​ss:y²+y=x³,the24 automorphisms have the explicit u,r,t formulas recorded in the raw artifact. The selected I,J satisfy I²=J²=-1 and IJ=-JI. Each of the24 Hurwitz norm1 units is bound to an actual automorphism by checking2*unit against its integer quaternion numerator on225 points. The difference is a sum of at most six degree1 automorphisms,with degree≤36;225 distinct zeros force it to vanish. Canceling[2] in the endomorphism domain proves the identities. Actual Frobenius is -J+IJ. The maximal Hurwitz order interpretation is described in ")#link("https://abel.math.harvard.edu/~elkies/M223.24/index.html")[#text("Elkies's notes")]#text(". The finite checks and this global degree bridge are explicitly separate.")

#text("At11 and49,the chosen matrices give the split two-dimensional torsion modules of that maximal order. Their identification with actual torsion requires the standard freeness/uniqueness theorem over each split local matrix algebra;no coordinates for full torsion bases are emitted. The288 norm11 elements give12 kernels. Two source-level lines in an SL2(F11) basis select the corresponding isogenies;applying their49 matrices and comparing modulo the actual units tests equality of the transported modular targets. The660 bases modulo sign have55 free unit orbits,leaving55*56=3080 modular point labels. Four hundred match. Normalized Frobenius using the actual -J+IJ puts them into40 cycles of length10. These are complete set-theoretic moduli labels under the stated bridge,but not explicit torsion-point coordinates or a divisor with known multiplicities.")

#text("Target stabilizers give only multiplicity lower bounds. Here their sum is400,whereas1280 minus the420 cuspidal contribution is860. The condition for degree exhaustion fails. A total460 intersection multiplicity remains to be computed at these labels. The antidiagonal part is separate and uncomputed. Neither a complete x0+xinf row nor a rank increase is claimed.")

#text("The abstract11-module basis fixes some primitive Weil-pairing value. If it differs from the earlier chosen value by a power r,rescale the first basis vector by r^(-1). This diagonal right action preserves both lines0 and infinity and commutes with the normalized Frobenius right action diag(1,1/2). It therefore gives an F2 isomorphism between the two determinant components preserving the two functions tested here. No uncomputed symplectic identification is used to compare other line maps or the earlier projected column images. An explicit torsion-coordinate adapter would still have to determine r and verify the pairing itself.")
