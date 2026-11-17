# F631 inverse solver gate

Use the retained generated-D1/positive.ms, generated-D1/target.ms and generated-B/target.ms, in that order. Preserve the original equations. The private official msolve 0.11 installation is reused with hashed wrapper, binary and libraries. Run two solver threads, verbosity one, rational parametrization option c0, with a 300-second wall cap per case. Stop and retain unsupported, timeout or failure; do not change the separating form or rerun.

The independently ported verifier works over F631 and substitutes every rational coordinate into all twelve original polynomials modulo the elimination polynomial. Require dimension zero, degree 6 or 12 (the inherited fixed fiber gate), squarefree eliminant, invertible common denominator, exactly one valid numerator convention, and positive-point recovery for the first case. An unexpected degree is retained rather than silently admitted. No numerical or sampled-only substitution.

Freeze generated input hashes, verifier/collector source, dependency lock and binaries, plus solver dependencies; commit and S3 checkpoint/gate before once-only execution. Record all child stdout/stderr/status. These results establish algebraic inverse systems only; rational target-class reconstruction, adjacent sign checks and global correspondence remain separate. No timing or tenfold claim.
