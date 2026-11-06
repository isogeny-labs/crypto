#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("What the twist can contribute to this index calculus")

#image("figures/mechanism.svg", width: 100%)

== #text("Domain and evidence")

#text("This study concerns the declared prime subgroup H of E: y^2 = x^3 + b0 t^3 over Fp[t]/(t^4-d), with p congruent to 1 modulo 12 and d the least quadratic nonsquare. The exact model, subgroup, G, Q, eigenvalues and query coefficients of every instance are fixed by the SHA-256 in PROTOCOL.md (PROTOCOL.md). It is not a transfer of a logarithm between unverified subgroups of different twists.")


#text("Here Li means the coordinate line Fp t^i for the abscissa. All factors still belong to the same declared H. The earlier mixed E(Fp^2) trace study (../MIXED_REPORT.md) tested a different target and projection question; its odd-order obstruction is not transplanted to this direct quartic-twist subgroup experiment.")


#text("The mixed-component base, its active matrix, and the complete execution interval are the experiment. An endomorphism orbit's size is not a count of independent relations. The full-stream census and the stopping prefix are different populations; neither replaces the other.")


== #text("The image geometry is real and its logs are already represented")

#text("Let psi(x,y)=(omega x,y), omega^2+omega+1=0. On H the explicit map is checked against multiplication by the supplied eigenvalue lambda. For phi=1-psi, every image is independently checked as phi(P)=[1-lambda]P. The nonzero scalar 1-lambda is invertible modulo the prime order of H. Consequently the image points retain known coefficients of the original factor logarithms; they require no extra unknowns.")


#text("Writing z=x(phi(P)), the slope, curve and cube-root equations imply")


#text("    3 x^2 z = -omega^2 (x^3 + 4 B).")


#text("The exact integer polynomial identity has zero remainder and certificate IDC1h7e0104841476f369. The faithful conditional statement compiles in Lean 4.34.1; its axiom audit passes, and the false-statement control fails. See proof/identity.json (proof/identity.json), proof/ImageCoordinate.lean (proof/ImageCoordinate.lean) and proof/axiom-audit.log (proof/axiom-audit.log). The Lean theorem proves the stated conditional coordinate identity. Subgroup preservation and its connection to actual point addition are separate executable checks on every panel factor and generator.")


#text("For x=a in L0 and B=b0 t^3, division gives")


#text("    x(phi(P)) = -(omega^2/3) a - (4 omega^2 b0/(3 a^2)) t^3.")


#text("Thus phi(L0) supplies points in L0+L3 outside the old L0/L2 base, under the checked nonzero-coordinate hypotheses. The panel checks injectivity, sign encoding, action compatibility, disjointness, and every factor-log coefficient. Unsupported exceptions stop the case and retain evidence. Doubling also supplies a disjoint image of the same size with known log coefficients, checked by explicit group arithmetic. This is why it is the relevant control for attributing a benefit specifically to the twist map.")


== #text("Automorphism copies do not add folded matrix rank")

#text("Suppose a relation has folded factor coefficient vector c and query R=aG+bQ. Applying an automorphism acting as nonzero scalar u on H changes c, a and b to u c, u a and u b. After eliminating the known anchor, its entire augmented matrix row is multiplied by u. Projective normalization therefore produces the identical row. Storing this orbit repeatedly can increase raw output counts without increasing the row span.")


#text("This argument applies when the map, residual and factor coefficients are transported together. It does not identify distinct physical image-base decompositions with each other. Those are enumerated and checked separately. The psi-control reproduces the original signed sets exactly on all 21 instances. The phi and doubling arms can yield different folded rows; their actual matrix ranks and first full-rank queries must be measured.")


#text("There is also a useful bookkeeping identity for an invariant base and an invertible action h:")


#text("    R - h(P) = h(h^(-1)(R) - P).")


#text("Replacing every physical peel by one orbit representative consequently requires the corresponding transformed residuals. Without a shared solver, this just exchanges the peel-orbit loop for a residual-orbit loop. It does not remove those systems for a fixed query for free. A joint elimination could exploit the family; merely relabelling the loop does not demonstrate that saving.")


== #text("Exact first-moment comparison under uniform residuals")

#text("Let L be the number of L0 abscissae, identifying a point with its negative, and let M be the size of a disjoint peeled image, also up to sign. With three distinct L0 abscissae and one image abscissa, the number of signed physical tuples is 16 binom(L,3) M. Each tuple has one fixed sum in H. For uniform R in H, linearity of expectation gives exactly")


#text("    E[number of these relations at R] = 16 binom(L,3) M / |H|.")


#text("This counts tuples, not independent rows, and requires no independence between tuple sums. Collisions between sums do not change the first moment. The disjointness and distinct-abscissa conditions matter. Both phi(L0) and [2]L0 have M=L in this panel, so their first moments agree. Their higher moments, cross-peel correlations and rank trajectories need not agree. An image's existence alone is not a larger first moment.")


#text("For four distinct L0 abscissae the corresponding count is 16 binom(L,4)/|H|. All four possible peeled presentations of the same tuple must be deduplicated. The image-to-pure tuple-count ratio for M=L is 4L/(L-3). This combinatorial expansion is available to doubling too. It is not, by itself, a twist-specific relation-density advantage.")


#text("These are elementary counting arguments, not new asymptotic algorithms. The identity certificate above certifies the coordinate identity, not a probability model or the counting arguments in this section. No Lean formalization of the finite-uniform counting statements is claimed. The older pure-four-factor falling-factorial expansion has certificate IDC1hf0237ddd316cd9d8 (../quartic_twist_relation_count_identity.json); that algebraic certificate likewise does not assert uniform sampling of these historical traces.")


== #text("Why the peeled solver does not see a lower-degree image equation")

#text("For any fixed P in H, R -> R-P permutes H. Under a uniform residual, R-P has exactly the same distribution for every P. In this implementation the finite S4 solve takes x(R-P) and three L0 unknowns. The image's coordinate equation is not inserted as an additional polynomial unknown.")


#text("Consequently any deterministic, memoryless work function of that remaining point has the same expected value for each fixed peel. Infinity handling can be included in the work function. Summing over equal-size signed peel sets preserves this equality. Degree three for phi versus degree four for doubling does not itself lower the degree of the S4 systems actually run.")


#text("The argument does not cover adaptive stopping, a nonuniform query stream, joint solvers that share work across peels, stateful caches, rank correlations, or different setup costs. Those are possible sources of an advantage and remain separate questions. The complete measured setup and online phases in this experiment include construction, all failed peels, solving, group checks, matrix work, recovery and verification.")


== #text("What the completed relation and rank controls establish")

#text("Across the same 9,658 frozen residuals, phi(L0) produces 1,865 unique signed tuples and 1,698 distinct projective augmented rows. Doubling produces 1,804 and 1,705 respectively. Both active arms reach full rank on every one of the 21 instances. Distinct projective rows are not all linearly independent; the independently recomputed rank is the relevant quantity.")


#text("Through each arm's first full-rank query, phi consumes 275,784 finite signed polynomial systems, doubling 260,496. These are aggregate diagnostics over separate targets, not an amortized DLP cost. They show why the additional physical tuples alone do not establish less work. Every target's complete cold interval is compared separately.")


#text("Keeping inactive L2 columns in the phi(L0)-only arm makes all 21 systems rank deficient at the stream cap. Restricting to the justified active L0 columns solves all 21. The active projection explicitly checks that no discarded column contains a relation term. This distinguishes a matrix encoding defect from a failure of the physical image base.")


== #text("Limits on inference and the next discriminating question")

#text("Every target is public and synthetic, with the same known scalar 271. Seeds select query trajectories and physical encodings; they are not 21 independently sampled secret scalars. The historical streams were stopped by an earlier oracle. Uniform-residual formulas are unconditional model predictions, not fitted explanations or unbiased density estimates for these conditioned traces. Repeat timings estimate implementation noise on fixed inputs, not uncertainty over independent curves or targets.")


#text("The current controls discriminate physical tuples, folded rows, full rank, work to rank and complete cold elapsed cost. They do not establish a scaling exponent, a deployed-curve attack, an advantage over strong rho, or a complete GAE operation price. Missing native-operation emissions and the required 0/0.1/1 GAE band remain explicit obligations for such rankings.")


#text("The untested algebraic opportunity is a joint relation solver that retains the image equation and exploits correlations across peels, rather than solving each translated S4 system separately. An unpeeled mixed S5 formulation, shared elimination across the image orbit, or a proved rank-yield bias would test that opportunity. Each needs an explicit algorithm, same-size generic-image control, unique-row/rank checks and a new complete-cost protocol. No result for those algorithms is claimed here.")
