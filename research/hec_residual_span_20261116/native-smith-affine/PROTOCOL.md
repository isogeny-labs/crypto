# Smith affine construction gate [pcc:unverified]

Custody amendment: the owner authorizes Git custody while S3 is unavailable. Commit and push this protocol, source and INPUT.sha256 before execution; retain raw output, exit status and output hashes in Git afterward. No S3 checkpoint is required for this bounded run. No S3 polling, cloud execution or merge is authorized.

Fixed inputs: all six maps in native-trigonal-map/CONTROL.txt on the original genus-three fields 61, 211 and 401. Normalize N/D to Smith's monic cubic/quadratic form. Compute F modulo G=N-tD over Fp[t], its determinant norm s(t), and test whether s is a scalar times a square. Retain both square classes; do not replace the source by a twist.

For every field element t independently compare the norm with the native resultant and delta4 with the native cubic discriminant. On the restricted open delta0*delta4*s != 0, test split distinct fibres with common ordinate square class. Check all four Veronese points, six minors, three linear equations and Smith's affine quartic identity. When the norm scalar is square, check rho reconstruction and b squared equals F modulo G. Also verify that the fixed nonsquare twist flips the norm square class.

One invocation, 600 CPU seconds, overflow checks enabled; preserve failure and do not rerun unchanged. Zero witness coverage must be reported. These exact algebra checks do not establish smooth normalization, genus, nonhyperellipticity, Jacobian action, target transport or any speedup. Mathematical source: Smith, https://arxiv.org/abs/0806.2995, sections 6-7. Native checks are not independent signed review.
