# Producer validation, round02

- C++17 `-Wall -Wextra -Werror -O2` build and residue replay: exit0.
- UBSan `-fsanitize=undefined -fno-sanitize-recover=all -O1`: exit0; same output.
- All260 embeddings per prime retained; expected single/double/Frobenius-orbit counts agree. Parent full primality certificate is unchanged.
- Added exact Legendre-symbol checks:131 inert and263 split in Q(sqrt(-7)).
- Initial build failed `-Werror=misleading-indentation`; braces corrected before first successful census.
- `pcc lint-prose round02/REPORT.md`: zero findings after a line-local status tag was added before claim pinning. A preceding negated performance word had been flagged.
- PCC-20261123-b7901e standing: unverified. No signed execution, independent geometric verification or Lean theorem is claimed.
- Full Lean dependencies remain absent; parent lean-attempt.txt/root-test-attempt.txt retain the unavailable tools. This round changes no production Rust code.
- Typst0.15.1 renders a four-page report PDF. All final page renders were visually inspected; text, diagram, citations and statuses are legible.
- No curve construction, DLP, timing benchmark, EKS job or cost comparison was executed.

The theorem's full statement, direction of isogeny, coefficient orders,
Artin class and general-order invariance argument are in REPORT.md.
The native checker verifies only residue arithmetic, not these bridges.
The large-prime degree necessity does not assert a corresponding lower
bound on map evaluation time or rule out larger-genus factor embeddings.
