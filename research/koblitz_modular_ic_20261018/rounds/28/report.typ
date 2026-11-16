#set page(paper:"a4",margin:(x:16mm,y:15mm),numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.5em)
= Koblitz cover transfers — R28
*Adjugate norm functions select actual ordinary Hecke images* \
2026-10-28 client date / 2026-10-29 UTC publication.

The elliptic equation alone leaves three rational second-x candidates at each
of four ordinary supports. R28 uses the actual 132-branch Hecke norm to select
images, under the retained modularity, good-reduction, pole and RR bridge.

For epsilon²=0, write N(F+epsilon H)=N(F)+epsilon D_H. At a simple zero with
D_1 nonzero, the ratio D_x/D_1 gives the actual second x-coordinate. The
finite-factor algebra is proved in Lean. Its identification with the actual
modular norm and fiber remains written. The norm as multiplication determinant
is reviewed in #link("https://math.mit.edu/classes/18.782/2013fa/LectureNotes7.pdf")[Sutherland, Definition 7.1].

#image("round28.svg",width:100%)

Native Rust extends the original cyclic-121 binary-series determinant to dual
numbers, including ten middle branches and the large branch. Every dual
inverse and planted multiplicativity check passes. All underlying norms
reproduce the original M15 A/B coefficients exactly.

The four derivative functions are recovered in the original degree-792/660
base RR spaces. All 1,024 coefficients of every function are checked, using
123,904 input coefficients. Python performs only assembly and metadata.

#pagebreak()
= Selected images and precise remaining scope
#table(columns:(2.2fr,.8fr,.8fr,1.6fr),inset:5pt,stroke:.4pt+gray,
 [*Function*],[*First degree*],[*Multiplicity*],[*R28 outcome*],
 [y0·yinf+1],[436],[1],[Selected, degree 436],
 [y0·yinf+1],[12],[2],[Both first adjugates zero],
 [x0·yinf+1],[284],[1],[Selected, degree 284],
 [x0·yinf+1],[83],[1],[Selected, degree 83])

Every selected point satisfies the actual target cubic and matches exactly
one of its three rational roots. Full Frobenius orbit checks establish the
second-image degrees and residue pushforward multiplicity one. The zero
at degree 12 is kept as a failure of first-order recovery. R29 resolves it
with second-order norms; it is not divided away here.

The raw archive contains the exact A/B derivative functions and field-coordinate
point pairs. These are paired Hecke images. They are not complete scalar-C5
source fibers, exceptional fibers, arbitrary reverse divisor evaluations,
actual modular-factor actions or independent relation rows.

The first compiler attempt fails before mathematical execution because a
copied inner doc comment is placed after a definition. Its source and log
are retained. The corrected build and both native runs pass.

Lean compiles three named theorems and audits four declarations, including
one generated congruence theorem. There are zero failures and zero trusted
native results. The proofs formalize cofactor nonvanishing and weighted
finite-factor recovery; no actual modular inclusion is assumed as an axiom.

R29's complete dependency check subsequently identifies the degree-436 and
degree-12 product-plus-one pairs as half-orbit Frobenius conjugates. Their
base closed-divisor difference cancels, while the colored C5 deck offset
remains a separate construction. No IC rank or efficiency conclusion is
promoted from these image calculations.
