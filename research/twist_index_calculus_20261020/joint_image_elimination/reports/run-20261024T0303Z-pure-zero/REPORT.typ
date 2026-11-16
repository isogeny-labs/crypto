#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Exact component certificate for the pure zero fiber")

#text("The 34-equation, 31-unknown rational search in pure-zero-certificate-v1 finds an exact identity. Independent integer multiplication gives zero remainder and rejects a constant +1 mutation. The actual coefficient certificate (../../results/pure-zero-obstruction/certificate.json) has SHA-256 3d0cfc9cd47e1b0633a0cc4e0fca05256ad6a55e574e7be3831994052479f73f. The separate native replay now passes: exact integer remainder zero, IDC1h09d838b7dca9895f accepted with 32 extra points, and both constant and multiplier mutations rejected. Its unchanged receipt is runs/pure-zero-verify-v1/verified/summary.json. The Lean gate now passes both the component identity and its field cancellation consequence, with standard foundations only and no trusted native declaration.")


#image("../../figures/pure-zero-obstruction.png", width: 100%)

== #text("Statement and readable argument")

#text("Let F have characteristic other than 2 or 3, let K contain F, and let B have degree four over F. On the nonsingular curve E: y^2=x^3+B, four finite points whose abscissae are pairwise distinct elements of F cannot have a signed sum equal to O. Signs do not change abscissae.")


#text("Write s1,s2,s3,s4 for their elementary symmetric coordinates. The already integer-verified, fully symmetric S4 polynomial is")


#text("    H4 = C0 + C1*B + C2*B^2 + C3*B^3.")


#text("A signed zero relation makes H4 vanish: the finite point obtained by adding the first two signed points supplies a common root of the two S3 quadratics defining the resultant. Pairwise distinct abscissae exclude an infinite intermediate sum. This is the standard summation-polynomial necessity argument; its actual curve/group formalization is not provided by the new coefficient proof. The finite-field definition and group-sum property are stated in Gorla–Massierer, Definition 2.4 and Theorem 2.5 (https://arxiv.org/html/1405.1059), citing Semaev's original construction (https://eprint.iacr.org/2004/031). The new ideal-membership identity and the degree-four coefficient argument are the present derivation; no novelty claim is made for the result.")


#text("Since s1,...,s4 lie in F and 1,B,B^2,B^3 are independent over F, all four Ci vanish. With a=s1,b=s2,c=s3,d=s4, the exact search certificate is")


#text("    2048*D = A0*C0 + A1*C1 + A2*C2 + A3*C3,     A0 = -8192,     A1 = -64*c + 96*a*b - 24*a^3,     A2 = 112*c^2 - 512*b*d + 8*a*b*c + 48*a^2*d,     A3 = 137*c^3 - 588*b*c*d + 2*b^3*c          + 792*a*d^2 + 3*a*b*c^2.")


#text("Here D is the standard quartic discriminant, not an arbitrary target polynomial. Thus D=0 in odd characteristic. But its exact factorization is the product of the six squared pair differences, which is nonzero in the field F when all abscissae are distinct. This is the contradiction. The independent quartic factorization Lean gate (../run-20261024T0252Z-quartic-factored/REPORT.md) already passes; the new component and field-cancellation statements also pass their separate two-theorem gate. No full Lean curve-level theorem is claimed.")


#text("For the actual models B=b0*t^3, t^4=d0 irreducible and b0*d0 nonzero, t=B^3/(b0^3*d0^2), so B generates the same degree-four extension. This supplies the stated field-basis hypothesis for those models. Checking the finite model data and embedding, and formalizing this implication, remain distinct from verifying the integer certificate.")


== #text("Consequences and exclusions")

#text("Under the stated hypotheses, zero-fiber four-distinct-factor precomputation in the pure Fp-abscissa base cannot supply relations. Mixed images, nonzero residuals and higher arities remain open. In particular, the 72 doubled zero relations in the first diagnostic do not contradict it; their separate control checks that their folded rows vanish.")


#text("The prediction for every pure-L0 zero set in the running 21-case census is an empty signed set. This cross-check is recorded before inspecting the remaining census contents. Any mismatch requires investigation of the exact public model, field-basis, S4 or signed-set implementation; it must not be silently discarded. Repeated factors, infinite factors, other base components, three-factor CM identities, five or more factors and target descent are outside the statement.")


#text("The search is native Rust under one thread, the busy reservation, 8 GiB and a 60 s watchdog, with separate compilation. It makes no runtime, asymptotic or novelty claim. Prior S4/symmetry methods remain the baseline.")
