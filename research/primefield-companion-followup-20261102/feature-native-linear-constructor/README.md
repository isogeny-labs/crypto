# Universal affine-linear character constructor

Construct coefficients from the public odd prime alone, using factorials.
DERIVATION.md proves the h=(r-1)/2-shift identity and the matching lower
bound for fixed affine-linear shifted-character decoders. lib.rs contains
the native constructor and exact checks. Caller certifies primality and
supplies max_terms. No label-dependent interpolation is used.

Build/run commands and bounded timeouts are preserved in receipt.json.
At r=479 this construction requests 239 source signs, versus 16 for the
previous cubic packet. It is an exact constructor and structural control;
no timing improvement is asserted.
