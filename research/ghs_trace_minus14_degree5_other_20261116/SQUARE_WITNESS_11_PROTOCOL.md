# Pre-run mask-11 orbit-representative square replay

The original mask-11, carrier-9735 degree-five screen retained two
numerators, and the new mask-11, carrier-17371 screen retained one.
These representatives cover the two recorded six-model carrier orbits
only after the exact `CARRIER_ORBITS.json` coordinate assertions pass.

Freeze `square_witness_11.sage` before execution. It independently
reconstructs each retained quotient `P=B(A³+A²B+bB³)/f` and verifies
exact divisibility. It then saves either a nonsquare leading coefficient
or a nonzero lower-coefficient remainder after forcing a degree-four
square root. This verifies the last square-test rejections without
calling `P.is_square()`; it does not independently enumerate numerators.
