# R47exact data encoding
Degree13tuple coordinates use binary coefficient words modulo0x201b, same as
R45/FORMAT.md; retain physical coefficients nu+b*gamma,lambda+(1+c)*gamma,
gamma²+gamma+1=0. JET47rows: block0coords(u,v,b,c,lambda,nu),1units,2inverses,
3descended68sections,4quadratic196sections,5cubic324sections. Each row has
block,index,17coefficient-pairs(constant,gamma). New source jets useu-u(P).
All13coefficient Frobenius conjugates are checked without dropping gamma.

The global representation uses154coefficients overF32mod0x25, packed as
bits5*k+j in13u64words, matching retained R35/R40. Coupling columns store
23Qcoefficients (115bits) then all13bits of Pi's constant value, total128.
Small/large basis rows have index plus154F32words. Coordinates of all input
columns/kernel combinations are in coupling-certificates.txt.

Class operations retain every full-generator quotient step, residuals,
kernel coordinates, candidate spaces, final memberships and independent
products in signed/isolation-certificates.txt. Class bases have196rows of
index plus154words. Equality witness dimension0meansnonprincipal/unequal,
1meansprincipal/equal under retained proper/RR/Picard bridge. Read exact
labels: a positive zero comparison flag is not a nonzero target image.
