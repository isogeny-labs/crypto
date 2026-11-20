# Default genericity coefficient audit

The archived c=2 preflight announces powers k^344, then prints the linear
form coefficient vector [1,−2147483648,1]. For k=2 this disagrees with the
announced coefficient even after reduction to F5. Lean proves
2^344=1 and −2147483648=2 in F5, with these values unequal; it separately
proves 2^344 exceeds the maximum signed 32-bit integer. Claim
PCC-20261109-c7c353 remains unsigned. The proof is arithmetic, not a formal
semantics for the C process or a claim of incorrect recovered solutions.
[pcc:unverified]

Source is pinned msolve0.6.5 commit
5e72b9d99eaea55fe87eb8ac945ec9e914a69327. In msolve.c:609–619 the routine
increments linear_form_base_coef, announces k^(bcf−1), then computes
`((int32_t)(pow(k, bcf - 1)) % gens->field_char)`.
The real-to-integer cast precedes the field reduction. Printing repeats
the cast at:165 without reducing. The c=2 raw stdout reaches exponent344
at line16952; its output explicitly contains the negative coefficient.
Source and bounded raw excerpts are retained under evidence/.
[pcc:unverified]

Primary C11 committee draft N1570 §6.3.1.4 paragraph1 says:
“If the value of the integral part cannot be represented by the integer
type, the behavior is undefined.” The advertised integer is far outside
int32_t's range. This is a concrete nonportable conversion boundary in the
source, with an observed coefficient discrepancy. Lean does not prove
libm rounding, IEEE execution, the compiled C trace, or a sanitizer result;
no sanitizer run is claimed. The source-to-machine bridge remains explicit.
[pcc:unverified]

Sources checked (KB): KB unavailable; primary/source fallback:
[N1570](https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf),
SHA256208969fec021e4f4ce87a1873765298c73e7bbb5209b1c5868967c7adfc8492e;
[pinned msolve.c](https://github.com/algebraic-solving/msolve/blob/5e72b9d99eaea55fe87eb8ac945ec9e914a69327/src/msolve/msolve.c#L609).
The successful preflight had already been read during calibration. This is
an after-data source audit of that retained output, not a newly preregistered
experiment or an alteration to the frozen protocol/native analysis.
[pcc:unverified]

The16registered calls check bases, S-polynomials, input reductions,
staircase/support, printed qdim and eliminant degree. They do not evaluate
every parameter coordinate map, validate all genericity coefficients, or
certify every internal F4 row. Counts agreeing on all fixtures therefore
coexist with this coefficient-generation finding. Neither observation
proves a general C-to-F4Run implementation theorem. No canonical performance
or corpus status is promoted. [pcc:unverified]

A proposed repair is to generate finite-field powers in the field before
converting to bounded integers, use exact integer arithmetic for the
characteristic-zero path, print the coefficients actually used, and bound
genericity retries. No upstream solver source or binary is changed here.
This proposal still needs implementation, targeted tests and a new protocol
before evaluating a repaired solver. [pcc:unverified]

Artifact migration: the repository bot archived preflight param.stdout and
param.stderr under evidence/preflight/.artifacts.json. Their local hydrated
bytes are retained; both manifest SHA256/size pairs match. A fresh checkout
must run the repository artifact pull before checking export file hashes:
[pcc:unverified]

```sh
python3 tools/artifacts/artifacts.py pull \
  research/solver_multiplicity_audit_20261109/evidence/preflight
```

The migration exactly matches remote955b4e749cf336fef2fbde375e93ab3c5fefd1ad.
It does not rewrite the frozen protocol, raw evidence, immutable export,
or historical ledger. [pcc:unverified]
