# Fresh positive odd-extension controls by coordinate translation

UNEXECUTED next hypothesis, derived from recorded controls. These are exposed
training correctness fixtures, not a holdout, novelty claim or timing result.

For any field element rho, set Ctilde(X)=C(X-rho), Ptilde(X)=P(X-rho),
Gtilde(X)=G(X-rho). Then Ntilde=Ptilde²-Ctilde*Gtilde²=N(X-rho).
Translate every target/source abscissa and the finite source domain by rho,
leaving ordinates unchanged. This is an invertible curve-coordinate change.
It preserves squarefreeness, regular/common-factor guards, target disjointness,
all norm factors with multiplicities, support remainders, and every labeled
source ordering. A monic cubic remains monic; h is unchanged and ktilde=k-h*rho.

For P=X³+rX²+sX+t, the new coefficients are

    rtilde=r-3rho,
    stilde=s-2r*rho+3rho²,
    ttilde=t-s*rho+r*rho²-rho³.

For C=X³+c2X²+c1X+c0, the new coefficients are

    c2tilde=c2-3rho,
    c1tilde=c1-2c2*rho+3rho²,
    c0tilde=c0-c1*rho+c2*rho²-rho³.

GF9 uses rho=t with t²=-1, hence t³+t=0. Translation preserves C=X³+X.
The recorded GF3 positive cell z=0,yT=0,a=2,ya=1,h=2,k=0 becomes
z=t,yT=0,a=t+2,ya=1,h=2,k=t. This supplies genuinely nonprime coordinates
and retains the known target/source-domain overlap. In canonical encoding
a0+3a1, these are z=3,a=5,h=2,k=3.

GF25 uses rho=t with t²=3. Translation sends C=X³+X+1 to
Ctilde=X³+2tX²+(1+t). The recorded GF5 disjoint positive cell
z=0,yT=1,a=3,ya=4,h=0,k=2 becomes z=t,yT=1,a=t+3,ya=4,h=0,k=2,
with source domain {t+3,t+4}. The target remains outside that domain.
In encoding a0+5a1, Ctilde ascending is [6,0,10,1], z=5 and a=8.

Preregister a fresh general-cubic worker with these fixed metamorphic rules,
independent coefficient-vector arithmetic and complete tiny parameter oracles.
Compare every translated stored certificate with its original under the
displayed bijection. Keep arbitrary-source recovery and global curve coverage
separate. This changes the current lexicographic fixture assumption; it does
not retry the identical empty extension cells or claim faster support search.
