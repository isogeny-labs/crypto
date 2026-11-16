# One fixed unbound native-CAS integration check

Prospective [unit protocol](PROTOCOL.md), [receipt](receipt.json) and raw
[basis](remainder.gb) preserve the call. This local native development check
is separate from the six bound-witness importer checks. No scientific
timing, increasing-size comparison or fastest-backend claim is made.

The unchanged m3,t4 remainder system over F17 was passed to Debian
msolve0.7.5-1 with all33 variables unknown, no assignment,1thread,20s wall
cap and2GiB address-space cap. It exited0. Its243 printed grevlex polynomials
have maximum degree3. The separate native staircase checker counts28
standard monomials in their printed leading ideal, matching the28 point
models independently recorded in solver case73 before this call.

That checker certifies the printed monomial staircase only. It does not
independently verify Buchberger's criterion, equality with the original
ideal, or extract the complete CAS model set. The final printed degree3
does not measure the degrees or work of intermediate elimination. There
is no inferred scaling bound.

At least one frontend pitfall is known in advance:28>17, so no F17-valued
linear form can separate all28 rational points. A default linear-form
univariate representation is therefore an inappropriate completeness test
at this dense toy input. The raw grevlex route avoids that requirement;
it does not solve the general extraction or relation-cost problem.

Recheck the retained staircase from the repository root:

```sh
bash research/prime_one_shot_20261029/continuation/canonical/unbound-check/check.sh
```

The exact backend argv, input/binary hashes, resource caps, output and empty
stdout/stderr are retained. Package identity is in ../backend/packages.json.
Re-running the external call requires the installed native libraries;
checking the retained staircase needs only a C++17 compiler.
