# Six-image bound for conics tangent at the exceptional point

Frozen before implementation or execution. The completed degree-six tests
at 2rho and kappa retain closed rank six. [pcc:unverified]

Let H be the actual reference line section x=56, and R original point 0,
whose verified exceptional-map image is O. Check H is disjoint from R,
R is smooth, and its conic value/tangent constraints have rank two in
the six-dimensional conic coefficient space. Conics N satisfying both
constraints vanish to order at least two at R. For the reference line
equation L_H, div(N/L_H^2)=Z+2R-2H with Z effective of degree six,
including extra copies of R if contact is higher. Thus f_*(Z)=2kappa.
The RR divisor D=2H-2R has degree six and l(D)=4; it is not an effective
pole divisor because its coefficient at R is negative. [pcc:unverified]

Reuse the exact native shifted enumeration/dual joins from the completed
degree-six tests. Use rho=kappa, target=2kappa, and reference coefficient
word twice that of H. Re-evaluate H/R, global map identity and named/GMP
target arithmetic. Check all 15,390,826 unordered triples of the original
451-symbol signed alphabet and both exact join/witness sets. Unlike the
previous degree-six divisors, this family need not contain a formal zero
word of length six. Do not require a zero candidate or invent one.

The earlier 2kappa test had four symbols. This six-symbol condition bounds
all conics tangent at R whose remaining individual images are in the fixed
original signed rational domain, including multiplicities and closed source
points with those rational images. Other conics and other image domains
remain outside the bound. It is a necessary image test, not a construction
of all conics. For any independent positive row, construct the actual conic
or RR/KM class and perform separate elliptic/elimination replay.

Keep the original six rows and 108x429 tangent matrix fixed. Verify their
row sums/ranks in both group implementations and compare old-column rank
with full minus auxiliary rank after independent candidate insertion.
Reuse the public nonzero G/2G fixture with exhaustive small six-word
comparison and row/triple/orientation corruptions. No fixture enters actual
data. Add an actual reference-sign mutation check before enumeration.

Limits unchanged: one CPU, 1 GiB virtual address space, 300-second strict
build, 600 seconds per worker, dense helpers <=256 rows and <=1024 columns,
<=4096 retained rows. Native C++ performs all mathematics; Python only file,
JSON, hash or rendering work. Freeze execution subjects and receipts,
preserve failures, and do not repeat completed scientific tests. Run locally;
Conductor ignored by explicit user instruction.

Sources: frozen `canonical-six-v1` and `shifted-six-v2` sources/results;
`CANONICAL_SIX_PROTOCOL.md`, existing conic/contact protocols and synthesis;
parent projective/exceptional-map reports and native smooth local lift.
The existing report links the primary Diem and double-large-prime papers.
This is an application of known divisor/pushforward identities, not a
novelty claim. KB unavailable in this runtime as recorded in the journal.
