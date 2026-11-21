# Source receipt

- KB unavailable: no MCP/catalog; repository wrapper exit 127. The ecdb guide
  was read from main; no token/AWS identity is configured, so no database
  lookup was made and no missing database result is treated as evidence.
- Smith, https://arxiv.org/abs/0806.2995, primary PDF sections 3–8. In
  particular, Proposition 5 assumes absolute simplicity, which fails here.
  Downloaded PDF SHA-256:
  44b809946008b6c5d1e9dcadb54b6103fa46ab56a13c0ebeadf9a191b2763140.
- Steven Galbraith, Mathematics of Public Key Cryptography, primary author PDF:
  https://www.math.auckland.ac.nz/~sgal018/crypto-book/main.pdf,
  section 14.4, Theorem 14.4.3 and section 14.4.2. Read the arithmetic plus
  representative-cost model, examples, cycle handling and endomorphism walk.
  Downloaded PDF SHA-256:
  ca75ee62b96daa0d46faaa2d4dc28205683455ad3a2a48586b9b39299c25022d.
- Primary author slides, https://www.math.auckland.ac.nz/~sgal018/ECC2012.pdf,
  pages 15–17: orbit walks and short-cycle problems. The IACR PKC2010 full
  paper endpoint returned 403; its abstract is not a full-group runtime bound.
- Repository sources: native/classification study modules; existing
  smith_kernel_coverage, smith_prop24, identity_certificate; p256_isogeny_walk,
  p256_isogeny_campaign, p256_isogeny_task and isogeny/velu. The degree-three
  edge is already present in the prior P-256 census; no novelty claim.
- Policy/workflow sources: ecdlp-surfaces and higher-genus reference,
  report-evidence, research-visuals, endomorphism-rules, transfer-assessment
  methodology and isolab-eks. No performance run was submitted.

The exact source/evidence snapshot is in sources.sha256. Cryptographic
research executes in Rust; Python is used only for the existing PCC metadata
tool. No archived Python algorithm is invoked.
