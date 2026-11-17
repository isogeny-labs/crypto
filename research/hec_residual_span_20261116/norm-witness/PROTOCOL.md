# Norm-witness finite algebra gate

Prospective gate; all assertions [pcc:unverified] until independent review.
Use unchanged original genus3 models p113,c3 and p421,c2 and frozen
frobenius-primes/CONTROL.tsv. No point/curve replacement or timing claim.
Predictions: all14 admitted model/prime rows have rank6 Frobenius algebra;
integer witness norm=lI; modular multiplication rank3; distinct kernel
counts 2,1,2,2,8,2,8 at l2,7,11,23,29,37,43 per model (50total).
Canonical RREF of multiplication matrices identifies their annihilators:
equal rowspaces iff equal kernels. Rank3 gives kernel dimension3.
No explicit geometric Weil pairing is computed; polarization and upper
counts are conditional deductions in ../KERNEL-NEXT-GATES.md, not outputs
of this checker. Reusing witnesses across models is deliberate; this gate
checks finite cyclotomic algebra, not independent Frobenius reconstruction.

Freeze source/protocol/derivation/screen/binary SHA256 and push Git before
one120CPU-second invocation. Rust checked-overflow build, no dependencies.
Retain stdout/stderr/exit and hashes even on failure. No scientific rerun,
coefficient substitution, S3, cloud, benchmark or speedup claim.
Success qualifies producer self-checks only. Failure rejects the stated
witness/count prediction and requires a prospective amendment.
