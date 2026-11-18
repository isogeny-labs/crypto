# Imagequartic recovery and exact composition

Prospective producer assertions [pcc:unverified]. Read four fixed quintic
vectors from ../quintic-map/CONTROL.txt; do not refit/change the map.
Use unchanged auxiliary twist and original-side Kummer projection. Fit
35quartic coefficients on image samples from first128catalogue places
plus eachfirst4catalogue places. Deduplicate normalized imagepoints;
require at least70distinct samples, one-dimensional quartic nullspace.
Write normalized35coefficients in monomials(4) order.

Substitute four quintic polynomials into recovered quartic, degree20,
and divide exactly by the sourcequartic overF601. Lex order with p first
makes p⁴ the leading term; allremaining sourcequartic terms have smaller
p-degree. Require identically zero remainder, not just sampled vanishing.
Require originimage nonzero, lies on recoveredquartic, allfour first
partials zero. This detects a singular originimage but does not prove
ordinarynode, smooth genus2reconstruction, degree25, globalbasepoint
freeness, independent verification or a speedup. Prior finite map tests
remain separate; interpolation alone would not certify the identity.

Same primary paper section5.1 and prior generic biquadratic provenance as
quintic-map protocol. No special Fast model/rational2torsion premise.
Copy reusable Rust routines, freeze all imports/native/map/protocol/binary
hashes and push Git before one120CPU-second invocation. Retain failures,
no retries/targetsubstitutions/S3/cloud. General-model normalization and
originaltarget transport remain next obligations.
