# Native distinct-image four-summand discovery

**2026-11-24 · Development correctness · [pcc:unverified]**

The Rust driver constructs the RR5 systems from public target coordinates and uses msolve 0.7.5 for elimination. It recovers and replays the signed relations. Over \(\mathbb F_{127}\), the distinct-image method recovered the public target logarithm \(69\) after 10 oracle calls. The direct pair-index comparison recovered the same logarithm after nine calls. The larger selector over \(\mathbb F_{8191}\) timed out under the fixed three-second oracle cap.

![The distinct-image cutoff from the original study.](diagram.pdf){width=100%}

## Encodings

The distinct-image encoding inherits the regular chart of the [theory report](REPORT.md). With \(q=2^d\), the suffix is \(D_4\). The final quartic coefficients can be substituted as constants, so at \(d=4\) there are seven variables and eight equations. The direct comparison has a prebuilt signed pair index. The expanded comparison uses monic division by the RR5 quartic.

The quadratic remainder control encodes the Dickson remainder circuit with quadratic equations. Let \(Q(X)=X^4+\sum_{i=0}^{3}q_iX^i\), and let \(R_j\) be the remainder of \(D_{2^j}\) modulo \(Q\). Start from \(R_2=2-q_0-q_1 X+(-4-q_2)X^2-q_3X^3\). At each further level expand \(R_j^2=\sum_{i=0}^6 t_iX^i\) and introduce the successive monic division coefficients
\[
s_6=t_6,\qquad s_5=t_5-s_6q_3,\qquad s_4=t_4-s_6q_2-s_5q_3.
\]
Subtract \((s_6X^2+s_5X+s_4)Q\) and the Dickson constant \(2\). The high-degree coefficients vanish by construction, and the four lower coefficients define \(R_{j+1}\). Every equation is quadratic in the step variables and quartic coefficients. The monic triangular reductions make the auxiliaries unique for given RR5 coefficients. The terminal equation \(R_d=c\) is therefore equivalent to division of the full selector by \(Q\). In the degree-16 control it uses 17 variables and 18 quadratic equations. Three explicit coefficient assignments at each of depths two through four agree with the expanded remainder in the Rust unit test.

The degree-two circuit is a different control from the compact quotient-product system in [PR #4953](https://github.com/isogeny-labs/crypto/pull/4953). No comparative timing against that system is claimed here.

\newpage

## Exact development checks

The \(\mathbb F_{127}\) fixture is \(Y^2=X^3+X+7\) with group order \(109\), generator \((1,3)\), and public target \((11,98)\). The \(\mathbb F_{8191}\) fixture is \(Y^2=X^3+X+5\), order \(8053\), generator \((0,789)\). The candidate factors the univariate projection and RR5 quartic by Frobenius GCD and bounded splitting. It receives no point-sum index or coordinate scan in the recovery path. The signed ordinates are reconstructed from the function and checked by the group law.

| Fixture | Selector size | Arm | Exact / attempted | Detail |
|---|---:|---|---:|---|
| F127 | 16 | Distinct image | 4 / 4 | Relation counts 1, 0, 3, 3 |
| F127 | 16 | Expanded remainder | 4 / 4 | Relation counts 9, 10, 13, 13 |
| F127 | 16 | Quadratic remainder | 0 / 4 | Four timeouts |
| F8191 | 16 | Distinct image | 4 / 4 | Four empty controls |
| F8191 | 32 | Distinct image | 0 / 5 | Five timeouts including the planted positive |

Exact means the complete signed relation set matched the point-law reference on the specified chart. The \(q=32\) planted target was constructed from one lift in each of the four intermediate fibers. Its abscissa and ordinate were the only target information passed to msolve. The pair-index reference found one relation in the selected chart. A cap or failed extraction is classified as undetermined. [F127 panel](evidence/native_f127_panel.jsonl) · [F8191 degree-16 panel](evidence/native_f8191_q16.jsonl) · [F8191 degree-32 panel](evidence/native_f8191_q32.jsonl)

The mixed-target run uses seed 781815. Both controls select one signed relation using only the residual point and the returned rows, then eliminate over the subgroup order. The distinct-image arm accepted nine rows in 10 oracle calls and the direct arm accepted nine in nine. Both reached base rank eight and independently checked their own scalar by multiplication on the fixture. [Recovery transcript](evidence/native_f127_recovery.jsonl)

These are correctness and budget-exhaustion controls on a shared local host. A complete comparison against the production compact quotient-product, oriented S3 and direct baselines remains open. The qualified Iso Lab workflow exists, and the available dispatch routes select its registered ecbench workloads. This standalone driver has no workload integration or deployed image identity for that workflow. No EKS job was submitted.

## Reproduce

Use Rust 1.90 or compatible, Cargo, and msolve 0.7.5. Set \(\texttt{MSOLVE}\) to the executable path. The driver runs single-threaded msolve and caps each call at three seconds. All discovery modes, including the quadratic control, use the same cap.

    cd native
    cargo test --release --locked
    cargo build --release --locked
    ./target/release/dickson-four-summand 127 4 "$MSOLVE"
    ./target/release/dickson-four-summand 127 4 "$MSOLVE" recover=781815 distinct
    ./target/release/dickson-four-summand 8191 4 "$MSOLVE" distinct
    ./target/release/dickson-four-summand 8191 5 "$MSOLVE" distinct
