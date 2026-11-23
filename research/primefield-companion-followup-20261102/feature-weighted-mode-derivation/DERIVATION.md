# Compensated orbit modes and phase extraction

2026-11-07, T-1811. Derived identities and native producer software checks; no signed independent verdict, timing comparison, or full formalization.

## Domain and prior work

Scalar field F_r, r>=7 prime, m>=2 divides r-1, zeta has exact order m; h=(r-1)/2, d=(r-1)/m. Caller certifies primality; implementation verifies root order and caps before allocating. All values below are in F_r. Base-curve field F_p is also prime. No extension-field transport is involved.

Read the preceding triple-orbit NEXT.md and CONTINUATION spectral/source sections. The shared knowledge-base MCP tools were unavailable. Primary sources checked: [FLINT transform conventions](https://flintlib.org/doc/gr_dft/index.html), especially its exact-order-root requirement and sum convention; [Bostan, Gaudry, Schost 2007](https://specfun.inria.fr/bostan/publications/BoGaSc07.pdf), introduction and selected polynomial-coefficient recurrence construction. Fourier filtering and recurrence blocking are established tools, not novelty claims. The implementation uses the existing native product/remainder-tree engine; it does not call FLINT. The identities below are derived directly rather than assumed from the library documentation.

## Identity, including odd d

Let y=k+c, b!=0, x=y/b, v=x^m, and s_j=chi_r(y-b*zeta^j), with chi(0)=0. Define

    A=m*chi(b)
    F_e=sum_(j=0..m-1) s_j*zeta^((e-h)j)
    g_n=(-1)^(h-n)*binom(h,n)
    H_e(V)=sum_(t>=0,e+tm<=h) g_(e+tm)*V^t, 0<=e<m.

Expanding (x-zeta^j)^h gives the term g_n*x^n*zeta^((h-n)j). The extra weight leaves zeta^((e-n)j). Its sum is m precisely when n=e mod m and zero otherwise. Therefore

    F_e = A*x^e*H_e(v).

This also holds for x=0 and for source zero endpoints. It does not require chi(zeta)=1. Omitting the compensating weight is generally incorrect, as the retained controls demonstrate.

For m>=3, cancellation of the unknown x^2 gives

    F_1^2*H_2(V) - A*F_2*H_1(V)^2 = 0 at V=v.

No observed mode is inverted. The polynomial may vanish identically when modes vanish; gcd with a zero constraint leaves the previous image state intact.

## Image degree and direct phase

H0 has degree floor(h/m)=floor(d/2). Its leading coefficient is nonzero because every factorial index is below r. For d>=2, A*H0(V)-F0 is a nonzero polynomial of that degree, giving an unconditional upper bound floor(d/2) on distinct image candidates after a complete orbit. For d=1 H0 is constant, so this bound is inapplicable. The second constraint has degree at most d. Convert to common image Z=(k+c)^m by substituting V=Z/b^m, then intersect with Z*(Z^d-1). The origin is preserved.

When image v is known and H1(v)!=0,

    x = F1 / (A*H1(v));  k=b*x-c.

The implementation checks x^m=v and the public-point equation [k]P=Q. It never chooses or enumerates a fiber root. When H1(v)=0 it returns unresolved or tries the next prescribed radius. Knowing an image is essential: algebraic reference checks supplying v are distinguished from end-to-end public recovery.

## Scalar coefficient compiler

The recurrence is

    g_(n+m)/g_n = product_(i=0..m-1)(n-(h-i)) / product_(i=0..m-1)(n+i+1).

The two factors (-1)^m cancel. Build the numerator and denominator polynomials using balanced product trees. Evaluate them at n=e+jm only when n+m<=h, through a shared remainder tree for that e. Batch-invert the evaluated denominators, then form coefficient prefixes. All denominator factors are between1 and h, so no characteristic crossing occurs. Initial g0,g1,g2 need constant work plus inversions. Empty prefixes are handled without constructing an empty tree.

For a fixed number of selected modes the conservative field-operation bound is O(M(m+d) log(m+d)+log r), with O(m log(m+1)+d log(d+1)) stored tree coefficients and O(d) persistent coefficients. Existing Karatsuba multiplication is used; no FFT multiplication bound or measured speedup is asserted. The explicit batch inverse uses exponentiation; polynomial division uses the existing Euclidean-inversion backend. Selected mode accumulation is O(m) per mode; no complete transform is computed. The source still costs m point queries per orbit, plus setup, target validation and verification. A favorable divisor m and cheap source remain material dependencies.

## Native checks and public adapter

`validate.py` builds modes.rs with optimized Rust and overflow checks, runs deterministic assertions, and hashes compiler inputs. `receipt.json`, check.stdout and check.stderr are final validation-03 evidence. Passing validation-01 and02 source/output snapshots are preserved before subsequent additions.

The independent small reference repeatedly multiplies by X-1 using schoolbook arithmetic, rather than using the stride recurrence. All x in each selected scalar field and radii1,2,3 are checked. Larger r65537 configurations m64,128,256,512 compare to a streamed scalar recurrence; these references share the recurrence identity. See exact counts in check.stdout. Zero observed modes and H1-zero cases remain in the counts. An uncompensated zeroth-mode control is checked. Allocation and root-order rejection controls are included.

The public adapter takes points only, validates the target through the existing Source before group operations, uses fixed radii1,2,3,4 and center0, and intersects only polynomial image constraints. Known scalars are confined to fixture construction/assertions. It stops at a verified endpoint or singleton-image phase recovery and returns unresolved on exhaustion. Source setup and final verification are outside the reported source-call counter; the counter includes one target-validation call and every queried sign, including endpoints. There is no timing claim.

Fixtures: E/F7: y^2=x^3+3, P=(1,2), r13, m3 and6, all13 scalars each; E/F509: y^2=x^3+2x+10, P=(3,68), r479,m2, original frozen20 file; E/F701: y^2=x^3+2x+8, P=(2,106), r683,m11,17 prescribed scalars. The public source retains its existing domain: r!=p, gcd(r,p-1)=1, v2(p-1)<=2, prime orders and geometry caps. The original4563 cohort is unchanged. No cloud work or empirical spectral study was run.

## Next construction

Integrate these constraints and direct phase extraction with retained-sign fiber replay, preserving the explicit unresolved cases. Study a bounded additional-mode or shifted-center strategy for H1-zero/image ambiguity. For d=1 the selected coefficients are constants and F1 already gives x directly, but m=r-1 source calls must remain charged. Derive any full-cost benefit before choosing a timing protocol. No claim that a few modes form a complete invariant follows from these checks.
