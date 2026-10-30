# Historical protocol label binding

The immutable [protocol](PROTOCOL.md) names the public synthetic curve now
registered as `icv1-f2m7-t13-616700dd`. Its original wording is retained. The
[binding](historical-label-binding.json) records the exact protocol SHA256 and
Git blob already present at commit
`a64f374747836eeac709bfade8cb851e46cf1443`; the protocol has not changed since
that commit. Its existing declaration that it was frozen before the trial is
preserved, without asserting a new approval or additional timing evidence.

The same historical commit contains the native component source at the recorded
SHA256. The retained raw archive contains both its source-hash record and the
presentation certificate. That certificate fixes characteristic two, degree
seven, coefficients zero/one, modulus `0x83`, group order 116 and the two
declared `(m,ell)` panels. These match the registered canonical model.

The curve-name checker uses its existing exact-artifact/label/model mechanism
for this one historical spelling. The full-file hash and expected canonical
replacement must both match. Changed protocol bytes, a different path, unknown
labels and a wrong replacement remain errors in both whole-tree and diff
checks. Metadata tests bind the archive, native source and certificate and
retain those negative controls. No directory-wide exemption is added.

This record changes no frozen protocol, native source, raw certificate,
reported result, acceptance criterion or experiment status.
