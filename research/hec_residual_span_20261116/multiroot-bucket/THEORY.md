# Fixed-pencil rank ceiling and g-known-root extraction

[pcc:unverified] Standard divisor and Vieta identities; no novelty claim.

On a smooth odd-characteristic monic model y²=f(x), deg f=2g+1, use a valid Mumford anchor (u,v), monic deg u=g, deg v<g. For a scalar t put w=v+tu and h=(f-w²)/u. The anchor condition makes h polynomial, monic of degree g+1. For u(x) !=0 the slope map (y-v(x))/u(x)=t implies h(x)=0 and y=w(x). Therefore a bucket has at most g+1 distinct x coordinates.

For fixed anchor and t the principal relation is fixed. Selecting a different pair from the same bucket simply factors the same h differently. After all residual support is expanded, the row is identical, with multiplicities and signs retained. Pair count is not independent relation rank. Partial encodings may expose different intermediate LP variables but cannot increase the rank of this one underlying principal divisor.

The x^g coefficient of h is f[2g]-t²-u[g-1]. If g distinct known bucket roots x1..xg are available, division leaves the monic linear factor X-x*, with
x* = t² + u[g-1] - f[2g] - sum(xi).
Its ordinate is w(x*). The residual may coincide with a known root; multiplicity must be retained. Extraction uses one field multiplication plus additions for any g. The condition is stronger than an ordinary pair collision for g3/4. Do not assume its coverage, useful rank or cold scan cost from an eligible-case component gain.

Application requires full anchor degree, valid Mumford input, monic odd model, distinct known x, u(xi)!=0 and actual shared slope. Singular/degree-deficient or overlapping-anchor cases require a separately justified path. Existing generic exact quotient remains a fallback. No changed group-order or DLP assumption is introduced by this polynomial identity.
