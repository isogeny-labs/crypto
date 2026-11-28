# H116 independent verifier binding

No H116 candidate/direct moment packet existed when this binding was filed.

Independent repository binding:

- repository: `isogeny-labs/cryptanalysis`
- draft PR: `#1744`
- protocol freeze revision: `742ab180`
- verifier source revision: `c7892317`
- current tracking revision: `dea90e1f`
- verifier source SHA-256:
  `d99493788c1b722e56270a8e5fb2503e2144b417e1da7b5e09451ea2caa40e0b`
- self-test stdout SHA-256:
  `65dd5559c04805e8dd4b624617be1c85d42973c075e0e7c1b0e66148c1012b98`

The Python 3 standard-library verifier independently implements tuple-valued
extension fields, elliptic-curve arithmetic, `ker(V-1)`, section/fiber
enumeration, the normalized Laurent recurrence, blinded target reconstruction,
positive and negative controls, JSONL parsing and exact cardinality checks. It
does not import or execute the Rust producer. [pcc:unverified]

Strict compilation and an excluded H115 p=31 test passed all 16 moments for
native and lambda-two models. The source test also exercised 32 planted
separable-root controls, the p=5/7/13 boundary and 12 one-at-a-time synthetic
mutations. These are source-test observations, not H116 verification.
[pcc:unverified]

Target admission remains unrun. The cryptanalysis repository requires an S3
resume checkpoint before the command; this environment lacks both the
least-privilege upload token and permitted AWS profile. The access blocker is
recorded at
`cryptanalysis:problems/h116-verifier-checkpoint-access.md`. The producer must
not invoke `--run`, and no scorer may join labels, until target admission is
checkpointed, executed, retained and cross-bound here. [pcc:unverified]
