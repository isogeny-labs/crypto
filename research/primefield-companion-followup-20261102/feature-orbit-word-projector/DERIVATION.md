# Exact orbit-word indicator and all-mode phase

Derived statement; unsigned producer evidence. The scalar field is F_r for an
odd prime r, with 3<=m<=64 dividing h=(r-1)/2, L=h/m, and zeta of exact order m.
The elliptic curve remains over a prime field F_p with a certified order-r base
point and the inherited calibrated public character source. Primality and source
compatibility are caller-established preconditions, not certified by this kernel.
The initial public sign fixes epsilon=chi(k) in {1,-1}. Identity and observed zero
signs are checked as public endpoints before this construction.

## Zero-safe selector and shared sections

For each public nonzero radius b, retain the entire observed word
s_(b,j)=chi(k-b*zeta^j), j=0..m-1. Its entries are +/-1 after endpoint exits.
Work in the rank-m algebra F_r[Y][X]/(X^m-Y). Write
T_b=(X-b)^h=sum_e X^e A_(b,e)(Y); compute T_b by binary powering and T_b^2 once.
Since zeta^h=1, substituting X*zeta^(-j) shows that the e-section of
(X-b*zeta^j)^h is zeta^(-je) A_(b,e). The same section scaling applies to T_b^2.

For t in {0,+1,-1}, delta_s(t)=(t^2+s*t)/2 is exactly 1 at t=s and 0 otherwise.
In particular a candidate endpoint with t=0 is rejected. The simpler (1+s*t)/2
would be incorrect there. Form each offset filter from the shared section
vectors, multiply all B*m filters, and call the result J(X,Y). Evaluation at
x in F_r makes J(x,x^m) the exact indicator of matching all recorded words.
No generic constraint exponent r-1 is used. Character powers h still remain.
Repeated radii share T_b and its square. Straight section multiplication gives
an O(B*m^2*log r+B*m^3) gate upper bound; hash-consing and zero/constant folding
can reduce it. This is circuit construction work, not its polynomial evaluation
cost. Circuit nodes are capped before insertion.

## Orbit average and multiplicity from public words

Let t be the number of simultaneous rotations fixing every observed word.
These rotations form a subgroup of C_m, so t divides m. For a phase root y,
the m solutions x^m=y all lie in F_r and have x^h=epsilon. Indeed the image of
x -> x^m has order (r-1)/m=2L, and the roots of y^L=epsilon form its relevant
coset. Replacing x by x*zeta^a rotates all words together, since chi(zeta)=1.
If one root matches the transcript, precisely t roots match; otherwise none do.
Thus sum_a J(x*zeta^a,y) is t or 0. Geometric-sum orthogonality kills every
nonzero section and leaves m*J_0(y). Consequently

    I(Y) = (m/t) J_0(Y)

is the exact image-support indicator on Y^L=epsilon. The normalization depends
only on the observed words; it does not enumerate unknown roots or scalars.
The runtime finds t with O(B*m^2) sign comparisons.

## Fourier support and Bezout phase

Use the positive-exponent unnormalized convention
f_(b,e)=sum_j zeta^(je)*s_(b,j). Orthogonality gives

    f_(b,e) = m*k^e*A_(b,e)(k^m).

The transform is invertible because m is nonzero in F_r. A rotation by a fixes
all words exactly when zeta^(ae)=1 for every nonzero Fourier coefficient.
Therefore t=gcd(m, all e with f_(b,e)!=0). The implementation independently
compares this gcd with the directly counted rotational stabilizer.

At a supported image y, each nonzero f gives nonzero denominator m*A_e(y),
hence k^e. Initialize the pair (g,v)=(m,y), meaning v=k^g. For a nonzero mode e,
compute u*g+w*e=g' by extended gcd and replace v with v^u*(k^e)^w. Negative
powers use field inversion. At the end g=t. If t=1 this gives the unique scalar;
verify [k]P=Q. If t>1 this implementation returns unresolved. It does not search
roots of k^t. Unlike an individual +/-1 anchor, a combination of modes can have
gcd 1 even when the two oriented modes vanish. The included period2/period3
synthetic-word control establishes this algebraic possibility only, not a
nonempty elliptic-curve transcript with that pattern.

## Extraction, accounting and boundaries

Evaluate circuit nodes as capped sparse representatives in F_r[Y]/(Y^L-epsilon).
For I=sum c_e Y^e, trace gives count=L*c_0 and sum=L*epsilon*c_(L-1), including
L=1. Count is an integer in [0,L], since L<r. A singleton sum is the unknown image,
then the all-mode chart and independent group multiplication verify its scalar.
No supplied image enters public recovery. Multiple images remain unresolved.
All nodes, including sections used by phase, are evaluated in this implementation.
Stored-slot cap is cumulative retained coefficients, not a total RAM bound; maps,
node tables, temporaries, copies and allocator work are additional. Monomial pairs
are charged on entry; term limits can conservatively reject temporary supports.
Source construction/calibration, point shifts, source calls, DFTs, stabilizer
comparisons, inversions, gcds and final verification also contribute work.

The earlier s*d>=L indicator-support bound still applies: singleton I requires
L nonzero coefficients. This path removes generic zero-testing powers but still
materializes the resulting indicator and node representatives. It is not the
selected-terminal contraction from the predecessor. Its recorded public pair
count is greater than that predecessor's count on the same fixtures. No timing
or total-work ranking follows from these counters.

Sources checked: feature-selected-moments/circuit.rs, projector.rs, public.rs,
and DERIVATION.md; feature-implicit-oriented-circuit Fourier identity. FFTW's
https://www.fftw.org/fftw3_doc/The-1d-Discrete-Fourier-Transform-_0028DFT_0029.html
specifies unnormalized complex DFT conventions only; the finite-field identities
and orbit argument are proved above rather than attributed to that manual.
KB unavailable: no loaded MCP and scripts/research-kb reports no executable.
No novelty claim or Lean verification.

Next: evaluate selected traces through the structured word-product circuit while
charging the size of all intermediate or dual representations. Generic bilinear
rank L does not exclude special structure, but merely renaming the dense state
or omitting source costs would not resolve this dependency.
