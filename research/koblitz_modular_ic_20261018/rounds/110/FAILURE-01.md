# Independent verifier preparation failure

The first independent verifier run reached the final `n_nonzero` comparison and failed because JSON decodes the stored pairs as lists while the verifier formed Python tuples. Its printed values matched (`[[35,2]]` versus `[(35,2)]`). The comparison was corrected to list pairs; a fresh full run passed for both branches. This was a verifier representation error, not a producer or algebraic mismatch. The initial first-run output was observed interactively; this note retains the exact failure and fix.
