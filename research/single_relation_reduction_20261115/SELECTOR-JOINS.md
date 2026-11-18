# Assessing an elliptic-coordinate partial join

Prospective scope b1e8ecdd26. This assesses a specific non-tensor component of
the one-relation route: fixed-slope line buckets. It gives no general lower
bound for coordinate-aware selectors. The probability/cost argument below is
a hand derivation; the actual finite curve-fibre counts are checked separately.
No native numerical case or timing is run. [pcc:unverified]

## 1. What the birthday-tree algorithm needs

Wagner2002's full author version sections2--3 describes efficiently enumerable
partial joins for XOR, scalar modular intervals and suitable group quotient
chains. Its cyclic k-sum/DLP reduction explains why a generic group join alone
cannot provide the desired improvement. Its section4 blind-signature example
solves scalar ROS arithmetic using signing interactions; it is not an ECDLP
algorithm on points. Scalar-interval joins show that the absence of a proper
subgroup chain alone does not rule out every nonhomomorphic join. [pcc:unverified]

For elliptic coordinates, a concrete candidate is: choose a set Lambda of s
slopes before drawing two lists, and for each lambda bucket the points by

    h_lambda(P) = y(P) - lambda*x(P).

Equal bucket values put P,Q on a line of that slope. Compute P+Q from its
third intersection and keep actual point/label records. This is a legitimate
field-coordinate computation with a clear join interface. The questions are
whether it produces enough candidates and whether successive output sets
have the shrinking support needed by a tree.

## 2. Exact fibre and first-merge yield

Work on a nonsingular short-Weierstrass curve over F_p, p>3, with subgroup G
of order r and independent uniform input point records as in REPORT.md.
Fix an affine P=(x0,y0) and a slope lambda. Other points Q counted by this
join have x(Q)!=x0 and lie on

    y = lambda*(x-x0) + y0.

Substitution into y^2=x^3+A*x+B gives a nonzero cubic in x. The known root
x0 accounts for one root. There are at most two other distinct x roots, and
the line fixes y uniquely. Thus for fixed P and a slope set of size s there
are at most 2s eligible Q on the entire curve, and hence at most 2s in G.
Infinity admits no such affine secant. This bound is exact as an upper bound
even when some intersections are tangent, nonrational or outside G.
[pcc:unverified]

For two independent uniform point records, the admission probability is at
most 2s/r. For J first merges, each pairing two lists of b records, let V be
the total number of admitted record pairs. Linearity of expectation and the
union bound give

    E[V] <= 2*J*b^2*s/r,
    Pr[V>0] <= 2*J*b^2*s/r.

No independence between different pair events is needed. Lambda may be
random and different for different merges, provided each set is independent
of the current base and has size at most s. If the global base-cleanliness
test has rejection probability epsilon<=1/4, then

    Pr[V>0 | clean] <= 2*J*b^2*s / (r*(1-epsilon))
                      <= 8*J*b^2*s/(3*r).

This conditions by division, without assuming that rejection preserves
independent point records. [pcc:unverified]

For W fresh retained bases and point-independent sets of at most s slopes
per first merge, another union bound gives

    Pr[any admitted first merge] <= 8*W*J*b^2*s/(3*r).

At n=ceil(log2 r), b=2^(O(sqrt(n))), J<=m=O(sqrt(n)), and W,s=2^o(n), this
is at most 2^(-n+o(n)). Any selector that requires one of these first merges
has useful answer probability no larger. Thus an explicitly small fixed
slope family together with subexponentially many base attempts cannot supply
an inverse-subexponential useful-yield bound. Materializing a larger family
also has to be charged. This rejects the stated small-slope schedule; it is
not a lower bound for every elliptic birthday-tree construction. [pcc:unverified]

When log p=Theta(log r), the exponential scale here is also in full input bit
length. No such comparison is asserted merely from n when the subgroup is
much smaller than the ambient field.

## 3. Controls and limits of this conclusion

- Empty Lambda admits no pairs. Taking all field slopes admits every affine
  pair with distinct x; the family is then of size p.
- The independence condition matters. After seeing P,Q, choosing the single
  slope (y(Q)-y(P))/(x(Q)-x(P)) always admits that pair. The bound above does
  not apply to this adaptive choice. It gives no nested support or fast
  complete selector by itself.
- Restricting sampled point coordinates to a small structured set changes
  the independent random-labelled-base distribution. Its acceptance cost and
  the later useful relation yield need a new argument.
- A large implicitly described slope set, a different coordinate map or an
  adaptive merge method remains outside this assessment. A viable successor
  needs an efficient join enumerator and controlled output distributions at
  every level, with final exact group relations and full recovery costs.

## Evidence and sources

KB unavailable: no loaded connector and wrapper127/no executable, as retained
in SCOPE.md. Read the precise repository one-shot/oracle/code contracts,
Wagner2002 full author version sections2--3 and the ROS subsection4, and
Sutherland MIT18.783 Spring2013 Lecture2 section2.1.2. Primary links and
hashes are in checks/join-primary-sources.txt. The Wagner mirror PDF timed
out; the complete author PostScript is the source, with a converted PDF for
inspection. No novelty or exhaustive literature claim is made.

Shared CryptoLean.Curve.LineIncidence reuses Mathlib's actual Weierstrass
Equation/addPolynomial semantics, Cubic degree laws and root-cardinality
bounds. Its target is the prescribed line and finite-slope-family fibre
count. It does not formalize the whole uniform-subgroup conditioning,
resampling or complexity argument above. At source867527c752375f21d47c000044d8215d07e432ba,
build003 and the canonical all-source/axiom gate audit141theorems,0failures,
0trusted-native exceptions. The seven new declarations use only propext,
Classical.choice and Quot.sound. Complete logs/source hashes and all three
builds are copied under checks/line-laws/. Build001's final type-inference
failure is preserved;002and003pass without weakening statements or limits.
Claim6186b7 remains unverified. The inheritance/statement table and explicit
formal-scope gaps are in PROOF-MAP.md. [pcc:unverified]

Shared PR14 merged asf13359f4833fa1b0a8668bfc85b151494637b032. Hosted
run43742865652 executed zero steps in both jobs; its annotation explicitly
reports the account billing lock. No hosted success is asserted.
[pcc:unverified]
