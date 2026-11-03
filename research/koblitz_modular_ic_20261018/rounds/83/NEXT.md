# Repair source-polynomial transport

Before changing rings, serialize the denominator and 19 numerator
polynomials from `module50` as strings. After loading the candidate
ring, parse each into the literal `x,y,p` coordinates and check the
same 19 denominator-cleared identities. Freeze the corrected source
and protocol under a fresh round; do not reuse R83's rejected ID.
