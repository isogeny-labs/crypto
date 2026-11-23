# Identity evidence addendum

This addendum binds section 3 of the frozen MATHEMATICAL_REVIEW.md to the
repository's native identity-certificate machinery without rewriting the
earlier source-review artifact.

**Identity:** S3(-X,-Y,t)-S3(X,Y,t) =
4(X+Y)(t(XY+a)+2b).

**Record:** [IDC1h095fa9aa1df01499](identity/identity.certificate.json).
The expression tree constructs both S3 substitutions separately, with all
five variables X,Y,t,a,b and declared syntactic degree 4. Both expression
sides have syntactic degree 4. [pcc:unverified]

The [issuer](evidence/identity-01/issuer/stdout.txt) evaluated 32 points using
seed 20261123. A [separate checker process](evidence/identity-01/checker/stdout.txt)
replayed the recorded points and evaluated 32 extra points using seed
20261124. The wrong RHS sign, underdeclared degree 3 and a mutated digest
were rejected. The source, dependency, executable and certificate pins
remained unchanged. [pcc:unverified]

The certificate field is **F_(2^61-1)**, distinct from the Prime31 target
field. This is a same-team deterministic finite-evaluation record, with
proof:null, not a kernel proof over the integers or the Prime31 field.
The API's displayed false-accept expression is not promoted to a rigorous
probability statement for these fixed seeds. [pcc:unverified]

The [identity protocol](identity/IDENTITY_PROTOCOL.md),
[summary](evidence/identity-01/summary.json) and
[dependency/source manifest](evidence/identity-01/source-build.json)
supply the reproduction boundary. A metadata preparation failure, occurring
before any build or identity evaluation, is retained with the original
runner and correction explanation. Mandatory locked archive/source checks
were preserved; only absent optional extracted-cache checksum metadata was
handled.

Registered statement:
[PCC-20261123-894c19](../../pcc/claims/PCC-20261123-894c19.json),
unsigned with no independent verdict. [pcc:unverified]

## Standalone test closure

Prepublication review identified the production module's relative cfg(test)
fixture include. The exact baseline Schoenhage record was added at
docs/identity-certificates/schoenhage-10mult.json. A separate, frozen offline
Cargo run then passed 22 existing tests (9 identity-certificate and 13 SHA),
including the committed-certificate check. No test or production module was
changed. See [test receipt](evidence/identity-tests-01/summary.json) and
[raw results](evidence/identity-tests-01/stdout.txt). These are new test-closure
observations, not a retroactive unit-test claim for the original issuer run.
[pcc:unverified]

Test-closure statement: [PCC-20261123-ee4187](../../pcc/claims/PCC-20261123-ee4187.json), unsigned. [pcc:unverified]
