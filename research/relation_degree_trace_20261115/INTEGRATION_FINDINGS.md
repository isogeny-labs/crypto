# Native integration findings

The immutable qualifier at source 6cf33f1c39579dd3e5564824c597ef2fa1baf245
passed five public-API development checks. Complete bases match the actual
native F4 control on the fixed 31-bit three-/four-variable cases. The wide-field
case passes exact arithmetic, duplicate/scaled input and tamper checks. A capped
root-input trace preserves null/incomplete status. The six-variable case is an
explicit expansion-budget failure, not a successful solve `[pcc:unverified]`.

This sharpens H1: codebase interoperability and the fixed small cases pass,
but the existing expanded-certificate representation does not yet establish
six-variable coverage under its frozen budget. The generic failure does not
yet distinguish polynomial terms, polynomial degree and proof terms.

The next diagnostic keeps all caps unchanged and adds failing phase, variable,
projected-root degree and actual term/degree counts to the error. A separately
copied diagnostic qualifier will run only the existing six-variable development
test. Its pre-call/post-call manifests are separate from round 01.

Conditional H3: if the projected root remains small and proof terms alone
exceed the cap, preserving a shared certificate DAG may remove storage
duplication. It must retain exact local identities and original-input
membership, charge graph construction and checking, and have independent
node/word caps. Increasing the old cap or accepting a partial basis would not
test that hypothesis. If polynomial degree/terms are the limit instead, proof
compression alone is not supported by this diagnostic `[pcc:unverified]`.

No root-codebase speed ratio, six-variable success, regularity change,
factor-base yield or ECDLP result is inferred. Unit-test elapsed times remain
development output. Producer record PCC-20261115-9fe802 pins round 01's
immutable post-call receipt and remains unverified.
