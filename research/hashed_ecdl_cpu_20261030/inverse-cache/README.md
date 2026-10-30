# Bounded field-inverse cache candidate

## What was done

Added a per-Curve field inverse table for validated primes p <= 4096. Larger
fields retain extended Euclid. Affine additions, Jacobian normalization and
CPU residual batch inversion use the table. Hash and scalar inversions retain
the original routine. Full independent certificate verification remains.

Reference source: 4c03472f6bdda531a93e8f8df5a6ac4f29695db6. Its complete
release binary was built and copied before editing; validation.json records
its hash for the next comparison. All 34 standalone release tests pass,
including exact archived certificates and exhaustive nonzero residue checks
at p=5,7,17,211,4093 against both Euclid and Fermat exponentiation. Larger
prime tests exercise the fallback. Root crypto/CUDA was not built locally.

## Decisions and references

Reviewed the existing inverse kernel and integration records and
[HAC chapter 2, Algorithms 2.107 and 2.142](https://cacr.uwaterloo.ca/hac/about/chap2.pdf).
The bounded recurrence follows directly from p=q*a+r: a^-1=-q*r^-1 mod p.
For prime p and 1<a<p, 0<r<a, so its table entry is already available.
Products are bounded by 4096 squared. This is an arithmetic engineering
candidate, with no novelty or measured performance claim.

The cutoff is fixed before measurement, independent of fixture identities.
Each Curve owns and constructs its table inside the complete setup interval;
there is no process-global cache or reuse across separately constructed solves.
Auxiliary memory is 8*p bytes plus Vec overhead (at most 32 KiB of entries),
and is not included in the existing retained sum-table byte statistic. That
statistic is not total memory or RSS. Large-field allocation remains empty.

## Open threads

Freeze and run a fresh five-arm EKS comparison with original baseline
2bd4588fba603ba1787e1d8ad82f084ef461d7cf, integrated reference 4c03472,
and this candidate, preserving all three fixtures and complete accounting.
No timing has been collected for this candidate. A regression is possible
because construction and lookup overhead remain charged. The further 10x
goal is unmet; this change targets the smallest fixture's remaining costs.

## Errors and ownership

T-929 reserved both source files and this handoff directory before edits.
The daily session log path is held by T-828; this scoped handoff records the
work without modifying that task's file. No release-test failure occurred. Strict all-target Clippy, formatting and
diff whitespace checks pass.
