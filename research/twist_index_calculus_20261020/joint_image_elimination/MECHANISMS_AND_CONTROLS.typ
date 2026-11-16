#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Mechanisms to distinguish in twist-assisted index calculus")

#text("Search round: 2026-10-24. This inventory preserves the broader research question after the finite registered runs have reached terminal receipts. Proposed constructions below are not executed results or novelty claims.")


#image("figures/joint-mechanism.png", width: 100%)

== #text("Mechanisms tested in this round")

#text(size: 6.8pt)[#table(columns: 4, inset: 4pt, text("Mechanism"), text("What could improve"), text("Required distinguishing control"), text("Evidence boundary"), text("Transport a relation by a cheap subgroup action"), text("Number of represented copies"), text("Canonical augmented rows and independent RREF"), text("The parent study shows that jointly transported copies normalize to the same folded row"), text("Add a known-log image to the factor base"), text("Candidate domain and rank trajectory without extra image-log unknowns"), text("Equal-size, disjoint generic image, here doubling"), text("Completed 189+21 controls; more phi tuples do not produce more distinct rows or a consistent cold gain"), text("Exploit phi's coordinate equation jointly"), text("Polynomial construction and root-solving work"), text("Same joint construction for doubling; strongest pure symmetry"), text("297 constructions validated; global native failures and source-block/implicit comparisons retained separately"), text("Partition sources by U^12"), text("Bounded-degree elimination while keeping all phases"), text("Same source partition for all arms; union of complete physical sets"), text("All three regression unions match their independent signed-set/RREF oracle; local ranks pure 1/3, phi 2/3, doubling 1/3"), text("Use all ordinary-factor permutations"), text("Smaller pure baseline representation"), text("Full four-factor elementary coordinates, not a distinguished fourth source"), text("Pure finite and geometric regression solves are complete and independently audited; both rank 1/3"), text("Concentrate at R=O"), text("Reusable factor-log relations"), text("Count zero folded rows and independent factor-log rank; then charge target descent"), text("All 21 four-factor zero fibers, three full p37 populations and 21 five-factor censuses complete; five-factor point replay passes; no target recovery from zero-only rows"))]

#text("Uniform total signed-tuple mass is conserved when the candidate domain has the same size. A higher probability of any useful relation, lower variance, better independent-rank yield or lower solving cost could still matter. The finite-count Lean module now passes its transitive-axiom audit. The domain's physical injectivity and disjointness are separate executable hypotheses; the theorem does not assume that the old conditioned traces were uniform.")


== #text("What symmetry can and cannot add to a folded matrix")

#text("Suppose an endomorphism preserves the chosen cyclic prime-order subgroup and acts there by a known scalar lambda. Applying it jointly to a relation and to its residual multiplies the already folded augmented row by lambda. When lambda is nonzero, the projective row is unchanged. Such transported copies can reduce search or parameter work, but supply no extra independent row. This statement concerns joint transport of a relation; it does not equate the sets of newly found mixed relations.")


#text("The possible gains therefore need separate measurements: fewer independent factor-log columns, fewer different systems to solve, cheaper systems, or new physical relations with better rank yield. The existing fold already credits the known subgroup action. Awarding the same orbit multiplicity again as independent yield would double count it. The parent study's canonical augmented-row check is the executable control; the actual endomorphism/group/rank implication is not a new fully formalized Lean theorem here.")


== #text("Frobenius-related disjoint bases: an existing method to test fairly")

#text("Galbraith, Granger, Merz and Petit construct Frobenius-related factor bases for subfield curves, and separately study Frobenius-invariant bases. Their symmetry-breaking construction expresses image logs through one base's logs. The comparison retains the actual polynomial-system solving cost; its factorial saving is conditional on the relevant solving-cost comparison. Invariant bases also reduce the number of unknowns. These are established mechanisms, not a new observation from the current study. Primary paper (https://eprint.iacr.org/2020/1315), accepted manuscript, sections 3.1–3.2 (https://pure.royalholloway.ac.uk/files/39395430/Accepted_Manuscript.pdf).")


#text("Proposed extension to this exact twist. The implemented transported Frobenius acts on abscissae as")


#text("    tau_x(x) = t^(-(p-1)) * x^p.")


#text("The present L0 is invariant. A non-eigenvector line Fp*beta can instead have several distinct image lines. A candidate beta with several nonzero Kummer components needs explicit field-rank, disjointness, lift, subgroup, sign and eigenvalue checks on every factor. A four-line collector would need a fully specified S5 system and exact physical-set/rank controls.")


#text("There is a crucial baseline issue: an orbit of distinct lines can require more independent log columns than the already invariant L0 base, even if it adds no columns relative to its own non-invariant source base. Compare both its source-line control and the best invariant-line baseline, with their actual base sizes and matrix dimensions. An advantage against the first alone would not establish a gain against the second. This extension is not implemented or run in the present receipts.")


== #text("Zero relations and several low-degree image maps")

#text("The predeclared doubled-triangle control (SUPPLEMENT_ZERO_DOUBLING_IDENTITY.md) predicts genuine physical zero relations whose folded rows vanish. The separate post-run checker now reconstructs every predicted tuple from actual group points and checks all 21 saved complete zero sets, without repeating the oracle. Every predicted folded row vanishes. At p337, the observed 384 tuples include 360 such zero rows and 24 additional tuples with factor-log rank 1/14; on the other six curves all observed tuples are predicted zero rows. The actual audit (reports/run-20261024T0841Z-zero-identities/REPORT.md) passes every case. Earlier 75/76-test checker gates retain their own source labels; the final audit reuses the separately frozen 79-test runner.")


#text("The subsequent exact pure-zero component search (reports/run-20261024T0303Z-pure-zero/REPORT.md) finds a certificate expressing 2048 times the quartic discriminant in the ideal of the four H4 component coefficients. Integer replay and +1 mutation rejection pass. Both new Lean component/field theorems pass; independent IDC1h09d838b7dca9895f also passes, with 32 extra points and both constant and multiplier mutations rejected. The readable argument excludes a signed zero sum of four distinct Fp-abscissa points when B has degree four and characteristic is not 2 or 3. It does not cover mixed image coordinates, nonzero residuals or larger relation arities. In particular, precomputation using random known multiples of an anchor has a general residual, so this zero-only obstruction does not rule out ordinary index-calculus precomputation.")


#text("The stronger square-component check (reports/run-20261024T0445Z-square/REPORT.md) finds 64*S³ in the same ideal, S=64*s4-(4*s2-s1²)². Its combined 10-theorem Lean gate passes and excludes three distinct roots, allowing the fourth abscissa to repeat. Independent IDC1h65c11f01d0b1e67a passes exact replay, 32 extra points and both mutation controls. The finite-Fp residual audit retains infinity and now passes every frozen full-tape physical-presentation, unique-row, RREF and recovery check on all 21 cases. It proposes one phi and two doubling rejections in the full stream, and zero before either arm reaches full rank. No filter is deployed and this predicate supplies no through-rank system saving on the tape.")


#text("Five or more distinct ordinary factors remain an explicit open precomputation avenue. A useful test must count independent factor-log rank after folding, rather than assume that additional zero sums carry information. The current certificate concerns S4 and does not exclude that avenue. No higher-arity polynomial collector is implemented here. The frozen five-factor diagnostic (SUPPLEMENT_FIVE_FACTOR_ZERO.md) specifies all 21 cases, pure/phi/doubling domains, exact partition multiplicities and anchored factor-log rank. Its provenance amendment reconstructs factor logs from the saved verified matrix, because the historical recovered field stores only the target scalar. The native census now completes all 21 cases. Both mixed arms give 504 physical tuples and 15 projective rows in aggregate, with different ranks: phi/doubling are 2/2 versus 1/2 at p37, 2/8 versus 1/8 at p157, and 1/14 versus 3/14 at p337. Pure has no relation. These are oracle results, not a polynomial price. Independent public-point completeness replay is implemented and owned separately; all 21 cases and 63 arm comparisons now pass. The public field/curve arithmetic is shared, while enumeration, point keys, partition and matrix elimination differ. The point-replay report (reports/run-20261024T0908Z-five-point/REPORT.md) states the exact remaining collection and cost obligations.")


#text("Proposed next collector control. At the zero residual, test an orbit quotient of the image/ordinary peel pairs against every saved five-factor set and rank. Charge map construction, all failed S4 systems, matrix work and subsequent target descent. Apply the same action quotient to doubling. This could test whether fixed-zero symmetry saves relation-finding work; transported copies must still count as one projective row. No polynomial implementation or cost result for this proposed quotient is claimed here.")


#text("Another untested construction uses several distinct known-log images in one relation, rather than the present three-plus-one pattern. Low-degree CM maps could make that representation cheaper than scalar-image controls. Every proposed image needs an explicit coordinate map, nonzero subgroup scalar, injectivity/disjointness checks and complete costs. Enlarging the candidate domain is also available with generic scalar images; its contribution must be separated from a twist-specific algebraic saving. No such several-image collector has been measured here.")


== #text("Stronger algorithmic baselines remain separate work")

#text("The pinned msolve 0.11.0 help also documents -S, a specialized saturation algorithm over the supported small prime fields. This is an untested representation control: components saturated by their discriminant can be supplied without the explicit inverse-discriminant variable. It would need a fresh frozen input, checked projection equivalence and the same physical-set oracle; it is not one of the completed 21 literal GB inputs. The exact installed help is retained in results/msolve-0.11.0-help.txt. Likewise a complete GB and a complete Fp physical decomposition are different computations; the current external checker preserves that distinction.")


#text("Trace-zero index calculus already has a substantial literature. Gorla and Massierer analyze relation generation, sparse linear algebra and a double large-prime variation under stated geometric and solving heuristics; their prototype also illustrates the gap between asymptotic formulas and actual polynomial-system costs. Their trace-zero subgroup is a precisely defined group, not a property automatically supplied by placing points on a twist. Primary paper, sections 3–4 and 6 (https://arxiv.org/html/1405.1059).")


#text("The present collector has neither a new large-prime graph algorithm nor a measured sparse black-box linear algebra implementation. Those baselines, strong matched rho, scaling and complete prices remain required before a general competitiveness claim. The current rung can establish bounded construction, completeness or pipeline improvements within its declared scope; it does not establish a new asymptotic attack.")


== #text("Graph coverage")

#text("This round adds the construction profiles and explicit pipeline diagram in the study. The old admitted cold-cost plots retain their frozen values. No newly admitted performance point, curve transfer or proved subgroup edge moves the canonical IC leaderboard, performance-gain graphs or curve registry. Those graphs are left unchanged for that reason. Failed and incomplete runs retain their own descriptive reports.")
