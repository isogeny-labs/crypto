# Coordinate-input cubic CM19 transfer continuation

Draft PR #5302 continues the explicit F7 model from crypto #5114 and actual divisor arithmetic from cryptanalysis #1964. The current work is a planned construction with no new qualified result or PCC claim. Conductor is waived by the user for this continuation; all other repository requirements apply.

The pre-interruption candidate is **unqualified**:

```
D: Y^2=6W^6+2W^4+1; E: v^2=u^3+2u+6.
F=W^3+W^2+3W+1; G=6W^3+W^2+4W+1.
N=W^3+5W^2+W+3; R=2W^3+W+2.
H3(W,Y)=(N/G, Y*R/G^2).
```

Construction checks will establish the full curve identity, degree/separability and proper extensions before interpreting the induced Jacobian map. Direct effective cubic fibres and balanced Cantor reduction will implement the coordinate-only `eval_point(E,G,Q)` interface. Complete finite controls will use every nonzero target on the fixed order-eleven curve; the oracle scalar stays in the driver.

The decisive control replaces geometric Frobenius by the identity: its agreement on rational points must be retained alongside any failure of the full function-field identity. Exact coefficient mutations and original outputs remain evidence.

Published source inputs remain at their original commits. Lost RAM-only executables and their separate immutable qualifiers remain missing originals. A later compilation is a new execution with distinct provenance. The recovery checkpoint is https://github.com/isogeny-labs/crypto/pull/5129#issuecomment-7224020581.

Read `QUEUE.md` before taking the next step. No randomized or timed experiment is admitted by this draft or by the Conductor waiver. Rust/Lean implement mathematical work; Python is restricted to file/Git/hash/JSON custody.
