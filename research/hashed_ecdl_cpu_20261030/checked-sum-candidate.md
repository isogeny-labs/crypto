# Checked projective oracle witness sum

The CPU oracle independently checks each found tuple before returning it. Replace
affine addition after every tuple point with a checked Jacobian accumulator and
one final affine conversion. Every point still passes Curve::contains; identity,
doubling and inverse pairs are handled explicitly. The resulting affine sum is
still compared exactly with the requested target. Neither solver-side relation
checks nor either full certificate replay is removed or changed.

Use the previously reviewed [EFD Jacobian mixed-add/doubling formulas](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html).
The helper is private to the standalone CPU crate because the shared reduction
file remains reserved by T-906. It uses the existing tested local reciprocal
kernel and explicit field bounds. No claim of arithmetic novelty is made.

All31 release tests and strict Clippy pass. New coverage compares checked sums
with the independent affine Curve::add for every pair in the order199 group,
with identity and inverse points embedded in the sequence, plus pair sequences
on the40-bit fixture. Noncanonical and off-curve points must return errors.
Existing all-target first-witness, wide-field and full archived-certificate tests
pass. These tests do not establish a performance gain or larger-scale recovery.

Freeze a five-arm complete comparison against baseline2bd4588fba60 and previous
candidate d0ad5a617f59; retain A/A and batch1 controls and every timed phase.
