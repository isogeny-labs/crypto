# Complete leading-count weighted degree cover

For h_j, each m=5-jfold product term contains at most min(m,k)roots of degree
d-1and all others have degree<=d-2. Its degree is bounded by m(d-2)+min(m,k);
addition cannot increase degree. Repetitions/zero roots preserve this bound.
Translation by X^(d-1)flips every leading bit, replacing kby5-k. Both raw
linear bounds therefore preserve every true prefix-supported quintic.

With fixed h4leading1, odd k=1,3,5are exhaustively distinguished by leading
flags000,110,001. Split old w0by u=0/1and retain old w1. These affine images
are disjoint, but represent necessary coefficient conditions, not a guarantee
of nonlinear prefix support. Every old q is retained through raw HC0+M*q.

Independent multiset multiplication tests classification and both degree
bounds. Arbitrary affine families exhaust all points to compare each bit
equation against direct bounds. Actual recovered points verify maps and
guards; they are not planted roots. Nominal affine counts are not speed ratios.

One core4096MiB90second outer supervision;78/84s finalization checks. No dense
enumeration, original339-variable completion or field-wide performance claim.
