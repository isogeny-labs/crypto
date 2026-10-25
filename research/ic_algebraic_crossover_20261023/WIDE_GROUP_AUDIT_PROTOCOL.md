# Wide quartic group and independent encoding audit

2026-10-25, frozen before output. For each m=16,17,18, in that order,
call the native wide-core find_bin_curve(m,1,16). Preserve its exact first
qualifying instance seed, or its bounded failure. Limit each rung to 900
seconds and 12 GiB address space. The first core port and arithmetic tests
are not full ecbench or IC support. These are additional correctness audits.

Independently check r with BigUint modular exponentiation at the first twelve
prime Miller--Rabin bases (deterministic below 318665857834031151167461;
assert r<2^73), r*h=#E, nonzero on-curve G and [r]G=O. Prove the group order
by its unique multiple of this r in the exact Hasse interval. The core's
order-search candidate is thereby certified independently of its random
point-search heuristic. An unproved candidate or timeout is not success.

Convert the exact tower to a polynomial basis, retain the full modulus and
coefficients, and control 64 field products against Gf2_128. Seed this
additional arithmetic control with 0xc2100000 OR m. Lift four points, force
the highest x bit in the first, and compare addition, negation and six scalar
controls with independent BigUint/BinaryCurve arithmetic. The scalars are
0,1,17,2^65+3,2^70+2^64+123,#E. Check the planted four-point S5 relation and
wide integer/coordinate serialization round trips. Encode group orders,
subgroup scalars, masks and coordinates as decimal/hex strings; never JSON
numbers which consumers might truncate. Retain ICV1 and the exact generator.

The same native executable independently reads the two earlier m6 constructor
profiles: parse raw Callgrind summary/totals, match hotspots and manifests,
rehash the actual frozen binaries, require identical constructor stdout, and
check all three field-constructor process statuses. This is one before/after
instruction attribution, including process startup and the Valgrind allocator;
the missing A/A, interleaved A/B, fastest external comparison and full-pipeline
pricing remain explicit. No wall-clock speedup or complete IC claim follows.

Pin source and binary before execution. Keep every failure, exit status,
resource diagnostic and output. Independent replay, an inspected report,
larger-rung factor-base/oracle/relation/linear-algebra integration, matched
rho, complete DLP and physical pricing are separate obligations.
