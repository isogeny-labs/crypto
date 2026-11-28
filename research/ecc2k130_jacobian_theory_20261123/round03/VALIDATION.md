# Producer checks and exact scope

- C++17/GMP optimized build and exact five-trace screen pass.
- UBSan initial screen produced identical summary and all112897 raw TSV bytes.
- Later source adds all32 Weierstrass tuples, explicit field polynomial, derivative47 and quadratic norm1052; full262-degree TSV remains byte-identical.
- Prime-factor products and full-factorization primality certificates are retained in primes.cpp/primes.txt and factors.txt.
- PARI2.19.1 static replay: degree130, irreducible1, real roots130, all H(t), derivative47 and norm1052. This is separate implementation evidence, not signed execution or an independent geometric proof.
- New all-degree place-count conclusion combines exact first19 rows with a written uniform tail bound; the source also computed through262 as a finite check.
- First GMP build failed -Werror misleading-indentation; braces repaired before data generation. One GP replay used a wrong working-directory path; failed output retained, then replay corrected.
- Full raw places.tsv upload failed: AWS credentials absent. RAW_STATUS.md gives size/hash and exact regeneration command. The raw file is gitignored and is not represented as remotely archived.
- Official PARI source SHA-256419553b11ff8c3ecb8e9031b7d91d7cee003a5d721e685fcb499565c96fe2943; version2.19.1. Local dependency build succeeded.
- Fixed ideal attempt:30-second deadline,512MiB maximum PARI stack,1GiB address-space cap. Exit124 before bnfinit returned; no ideal test result. No nonprincipality conclusion.
- No paid cloud compute launched. Local AWS STS unavailable; retained GitHub telemetry run45219644370 logs show OIDC AssumeRoleWithWebIdentity denied, credential outcome failure, budget outcome skipped. The connector supplied decoded logs after gh CLI returned empty logs and the artifact storage host refused download.
- Full mathematical/CM/gluing and all-degree bound formalization remain absent. The parent Lean tool attempt records missing Lean. No production Rust code changes; no new root Cargo test is claimed.

A larger class-group computation is not admitted until budget/capacity
requirements can be read. The first bounded certificate timeout has a
materially different successor: prepare the explicit real-cyclotomic
integral basis to avoid opaque power-order initialization. The original
curve and computable subgroup map remain missing.
