# Round 06 protocol — frozen before arithmetic experiments

Parent ec0d839d4200eb72eafe250aab254fa2b2f90ebb. Host date 2026-10-19 UTC
(client date 2026-10-18). Preserve all earlier records and archives.
No performance measurement or IC efficiency verdict.

**I1: the 44 degree-210 images.** Construct a verified polynomial-basis
F2^210 using the first Rabin-irreducible polynomial x^210+low, odd
low<4096. Embed the pinned F2^30 field by splitting its defining polynomial;
embed the degree15 b root and the fixed paired E[11] basis compatibly.
Use exact binary polynomial arithmetic backed by the maintained library.
Split psi7 (degree24), group its roots into eight doubling orbits, quotient
once, split again, and exclude the dual kernel. Enumerate all56 cyclic49
chains. Polynomial splitting uses at most96 deterministic trace partitions;
point searches use at most4096 abscissas. Construct all12 transported
eleven targets for each chain, with commuting-square and modular equation
checks. Normalize the eleven complements, explicitly verify Frobenius^15
on all616 labels, and trace representatives to both A5(F2) groups.
Recover44 closed orbits of degree210. Evaluate their coordinates in the
same pinned F32 basis as the previous columns and check their principal
row directly. New coordinate labels may refine the earlier abstract
companion labels; retain the explicit relabeling scope rather than assuming
a chosen abstract Tate basis was already bound to point coordinates.

**Q4: remaining determinant5 equations.** Test the three split models
B_k over F32, k=0,4,8, with product equations E^5 and descent
T_k(Y1,Y2,Y3,Y4,Z)=(-Y4,Y1-Y4,Y2-Y4,Y3-Y4,Z+pi^k Y4).
The proposed quotient is q_k(a,Q)=(a_i-a5, pi^k*a5-epsilon*Q).
Use the explicit Hermitian polarization with upper block
I4+((2^k-1)/5)*ones4, last column conjugate(pi^k), and last entry5.
Check T_k^5=1, positivity, determinant1 and the actual polarization pullback.
Match every frozen first graph modulo5 after the frozen U equivalence.
For trace+1 bind the companion torsion basis by one explicit symplectic
intertwiner, then realize that binding with actual five-torsion coordinates.
Retain every mismatch. Test complete25-point kernels over F2^24 and all
F2 rational image additions over F32. State that these are split equations
with F2 descent; expanded F2 affine/projective equations are a separate format.
Keep determinant19 equations open until an actual construction is given.

**R3: reverse divisor evaluation.** Replace [5P]-[O] by [6P]-[P] so
both fibers are ordinary nonzero order11 base points. First construct the
twelve reverse eleven-isogeny neighbors for one such base point over F32.
Freeze its full11 splitting field from the Frobenius matrix order before
splitting. Attempt a complete degree660 pullback fiber, keeping all missing
eleven-complement extensions and ramification explicit. A degree660 fiber
or an unreduced divisor is not a reduced Jacobian class. Amend this protocol
before extending to large splitting fields or additional fiber targets.

All arithmetic, search, factorization, record assembly and verification are
Rust. Outputs are write-once; retain compiler failures and partial runs.
Add relevant Lean structural and explicit matrix proofs, audit their
transitive axioms, and bind identities to source hashes. Replay prior round05
after changing shared helpers. Run required root/native tests and relevant
site/workflow checks. Leave an illustrated report, editable sources, inspected
PDF and immutable archive. Update the existing PR; respect the pending base
merge and the shared hold on problems/README.md rather than editing conflicts.

## Amendment A — after I1/I1b and Q4

All44 images vanish for both signs. I1b independently finds every relative
14-term trace zero, despite616 nonzero geometric tuples per sign. I1c will
check all8 first-seven fibers and all12 target maps: each seven-point sum
should be the fixed cusp T=(0,1), killed by2. Verify the degree7 equation
for t0=x^4/y and a local expansion at T through order12; prove the
corresponding algebraic identities and group cancellation in Lean.

For R3 choose the first nonzero order11 point of E49(F32). Reconstruct both
seven kernels with the existing exhaustive F32 helper. Determine Frobenius
order on the underlying eleven torsion, then construct its complete paired
basis over the indicated field (expected degree120, not assumed). Construct
all12 reverse neighbors and their dual11 generators. For each source curve,
compute gcd(psi11, X^(2^m)-X) and report missing complement degree.
Use exact etale quotient algebras and a Weil-pairing resolvent to represent
the five paired complement orbits of degree11 when required. Bound auxiliary
point searches by1024; no unbounded splitting-field point search. A complete
degree660 fiber in etale coordinates will be reported as an unreduced divisor,
not as a reduced Jacobian class or the whole20-fiber reverse map.

## Amendment B — a split reverse fiber instead of the predicted extension

The first reverse probe has underlying trace9. Its first source curve has
all60 eleven-torsion x roots rational over F2^120, contradicting the assumed
degree11 complement. Preserve that failed assertion. Screen both P0 and -P0
adaptively, retaining the rational-root degree for every neighbor. If full
bases split over F2^120, enumerate all55 paired bases per reverse neighbor
and all660 points of each phi0 fiber directly in that field.

If both seed fibers split, use their explicit Frobenius conjugates and
determinant-one basis changes to assemble the even reverse divisor:
Pi=Frob^i(P0), 5Pi=Frob^(i+1)(P0), 6Pi=-Frob^(i+1)(P0).
Generate all20 fibers of the translated pullback formula. Aggregate the
degree6600 positive and negative parts, check complete F2 Frobenius
invariance and evaluate its forward image. The accepted result remains an
unreduced divisor; no general genus2841 Jacobian reduction is implied.
If a seed does not split, retain that obstruction and the explicit etale
algebra attempt rather than claiming the whole reverse divisor was evaluated.

## Amendment C — reverse divisor from mixed split and etale seeds

Both seed fibers now have explicit coordinates:660 points over F2^120 for
P0 and60 irreducible degree11 point algebras over F2^120 for -P0. Use both
formats without pretending the second splits. Check each seed's Frobenius^5
descent (including Q -> -Q^32 and simultaneous basis sign). Construct all20
even reverse fibers by Frobenius conjugation and the determinant-one change
(P,Q) -> (-aQ, a^(-1)P+Q), which puts the old P line at line a.
Retain all point-algebra coordinate polynomials and multiplicities, and
check the20-term divisor's Frobenius permutation. Bound each seed component
by its declared degree1 or11. Check its forward composition using the
previously proved Gram correspondence, explicitly labeling this as a
correspondence-based check, not an independent pointwise genus2841 Jacobian
calculation. General reduction, an independent norm evaluator on all new
closed components, and the odd reverse divisor remain open.
