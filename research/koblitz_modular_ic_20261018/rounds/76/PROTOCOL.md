# R76 quadratic-coordinate fiber classification, frozen before execution

Use the integral R50 source lift of `V`, the complete R52 multiplication
table, R61 unit, and R72 primitive idempotents. Rebind all four primitive
factors above `z+1` to the table, verify the full source equation
`V^2+z^2 V+z+z^3=0` in each of the rational fibers, and classify each
`z+1` factor by whether its projected `V` is zero or its primitive unit.
Require two factors of each kind and the exact saved R72 rank/index
profile. Test that `V` is nonzero and square-zero on each `z=0` factor.

The deterministic exact algebra runs once in local Singular with one
Singular/FLINT thread, seed 76, timeout 1200 s and 4 GiB virtual memory.
The source/protocol/driver copies must match the frozen Git revision and
donor hashes must match. Accept only exit 0, exact marker and zero
whitespace-question-mark native errors. Retain every failed attempt and
freeze a repair before using a new attempt ID. No comparative timing or
statistical claim is made. The result classifies finite rational fibers;
it does not by itself complete global pole divisors or the infinity chart.
