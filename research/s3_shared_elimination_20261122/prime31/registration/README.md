# Prime31 unsigned statement registration

Statement [PCC-20261122-95c625](../../../../pcc/claims/PCC-20261122-95c625.json)
pins the raw [development receipt](../development-receipt.json) at committed
source/evidence revision `26f6880421d5fcb85cb33c68ec2f56aedfae6803`.
The statement is unsigned and remains **[pcc:unverified]**. Registration records
producer data; it does not execute a scientific verifier or sign a verdict.

The existing native [registration helper](../../register_statement.sh) first
produced [preview.json](preview.json). After inspection, its exact timestamp,
claim hash, source commit and expected ledger tail were supplied to the append
call. [append.postcall.json](append.postcall.json) records the result. The helper
checked the 1,369-entry ledger prefix and appended one statement entry without
rewriting that prefix. No keyring or verdict file was modified.

| Binding | SHA-256 |
| --- | --- |
| Receipt bytes | `95c6250d6bfceb15409b9cdd94f05d6659310791a0ca4f2b4e1123baecf12c54` |
| Canonical claim | `d9ad28a3d858767f9ee4802097dd3d7ef6b5a96d00da6f7b15acbba45a8f8f36` |
| Appended ledger entry | `940122e0959dc32a7503866647627ed3fc3f8358f9bcfca9f38642524aa1fa5b` |

The claim covers the retained local Prime31 correctness observations, including
the declared structured target/base and separate coordinate and subgroup
moduli. It does not assert a uniform-target solver result, a full-root-library
pass, complete point-decomposition coverage, or a performance comparison.
