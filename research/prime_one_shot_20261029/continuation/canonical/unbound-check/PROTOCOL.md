# Fixed unbound CAS library unit check

Frozen 2026-10-30 before execution. This is a bounded native development
integration check of an existing fixed F17 system, not a scientific timing,
scaling study, new experimental campaign or admitted performance comparison.

Input: existing continuation/canonical/outputs/m3_t4_remainder.ms, unmodified,
all33 variables unknown. No assignment, generating summands or scalar passed.
Expected selected-chart relation model count:28, from independent fixed point
oracle case73, target(0,6). m3 has no mixed-twist obstruction under the stated
odd-order/chart conditions. The reduced norm/remainder scheme has one point
per model. Thus its geometric degree should be28. This expectation was
specified before invoking the library.

Backend: checksum-verified Debian msolve0.7.5-1 with extracted FLINT dependency.
Arguments -t1 -g2 -f INPUT -o OUTPUT. Single fixed test,20s wall cap,2GiB address
space cap. Keep stdout,stderr,GB,exit,input/output hashes,version. No timing is
used as a result. A timeout or failure is retained, not treated as a degree
or correctness verdict. A printed non-unit GB alone does not establish complete
point extraction. If successful, compare degree via independent native leading
monomial staircase count and retain any limitations.

Do not request a default F17 linear-form RUR as a completeness test:28>17
prevents any F17-valued linear form from separating all28 rational solutions.
No random seed tuning, amended inputs or retrospective parameter selection.
