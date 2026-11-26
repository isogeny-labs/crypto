# Prime-field target-first summation polynomials

**Status** Anonymous research draft. Review and formalization remain open.

The manuscript studies two ways of imposing the target before eliminating a
projective summation tree. It gives model-specific work bounds for explicit
half supports, opaque affine operations, and degree-bounded coordinate tests.
For the signed four-digit base, it computes the sizes of the half supports
and bounds the specialized fibers. General shared arithmetic circuits remain
outside these models.

- [Manuscript PDF](prime-field-summation-barrier.pdf)
- [Proof map](PROOF_MAP.md)
- [Toy-curve output](evidence/replay.txt)

## Reproduction

From this directory, run:

    make check
    python3 evidence/reproduce.py
    python3 evidence/reproduce.py --field 101 --all-gcd-targets

The companion script is the supplied standard-library reference replay.
It compares signed witnesses against scalar enumeration, checks
fixed-target polynomial gcds, and checks a target-first batch norm in the
evaluation basis. All curves are synthetic finite controls. The replay
reports exact counts and uses no timing or resource-exponent estimates.

## Research and review

This is an advance in model-specific analysis with a separate correctness
replay. The related repository treatments cover prime-field resultants and
bounded norm-rootfinding. A broader novelty review and independent proof
review are pending. The proof map records the missing formalization. The
draft remains pending publication review.
[Prime-field resultant treatment](https://github.com/isogeny-labs/crypto/pull/4979) ·
[Bounded norm-rootfinding paper](../norm-rootfinding/README.md)
