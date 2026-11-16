# R81 explicit candidate F4 quotient maps, frozen before data

On each R80 component of the rejected R50 `normalP` candidate, rerun
the complete radical/minimal-prime census and check two degree-2 prime
quotients. For each prime choose the first ring variable satisfying
`g²+g+1=0` modulo that prime. Express every one of the 22 ring
variables uniquely as `0,1,g,g+1`; print its code and verify the
resulting 23-generator map ideal has dimension 2 and equals the prime
ideal by reductions in both directions. Check the two maps on each
component are distinct. This yields explicit candidate quotient
homomorphisms, including the `x,y,p` source-coordinate images.

Run locally in Singular, one Singular/FLINT thread and one math job at
a time, seed 81, 1200 seconds and 4 GiB virtual per component. Copy
source/protocol/driver bytes from committed Git and verify all donor
hashes. Accept only exit 0, exact marker, zero native whitespace-`?`
errors and all assertions. Retain failures and timeouts; freeze any
repair before a fresh ID. No timing comparison. These maps belong to
the rejected candidate algebra; source-field and local normality
certification are separate obligations before branch promotion.
