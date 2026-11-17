# Round06 split-prime instance gate

1. **Structure above:** Mathlib `Matrix (Fin 2) (Fin 2) (ZMod 199)` and
   functions on `Fin 2 → ZMod 199`. Matrix multiplication, matrix-vector
   multiplication, identity, left/right inverse and injectivity are inherited.
2. **Theorems used as stated:** `Matrix.mulVec_mulVec`,
   `Matrix.one_mulVec`, `Function.LeftInverse.injective` and
   `Function.RightInverse.surjective`. Literal finite entries use extensionality
   and `norm_num`, rather than a new reconstruction law.
3. **Object as instance:** the final good-prime receipt for u7 records199 and
   omega106. Its conjugate is92. Projection matrix P=[[1,106],[1,92]] sends
   (a,b) to the two residues a+106b,a+92b. D=[[164,36],[128,71]] is the
   decoder:128 is the inverse of106-92. These are the exact finite arithmetic
   instances of the formulas in frozen v6/modular_kernel.hpp. No code-equivalence
   theorem is inferred from this transcription.
4. **Assembly probe:** before endpoints, type-check the matrix-vector identity
   with variable matrices and inverse as a hypothesis, then at P,D with their
   literal entries. Use the named multiplication and identity laws as stated.
5. **Missing components:** the native multi-prime CRT/rational-reconstruction
   loop, exact numerator divisions, good-reduction/coprimality-to-global-GCD
   bridge, CM torsion-kernel theorem, specialization and point/scalar transfer
   are not formally proved here. These reusable interfaces belong in crypto-lean
   before a complete transfer theorem. The unrelated height draft and oriented
   ratio lead remain unproved/unimplemented; neither is an endpoint.
6. **Inheritance table:** ZMod ring arithmetic is Mathlib; matrix operations
   are Mathlib; the two root and two inverse equalities are finite instance
   obligations; vector round trips inherit multiplication associativity and
   identity; injection/surjection inherit the existing inverse-function laws. [pcc:unverified]

Proof chain: D(Pv)=(DP)v by `Matrix.mulVec_mulVec`; (DP)v=Iv by the literal
inverse certificate; Iv=v by `Matrix.one_mulVec`. Apply the left-inverse law
for injectivity. The reverse chain uses PD=I and the right-inverse law.
This certifies the coefficient pair reconstruction at the displayed prime,
not the complete modular kernel constructor or ECDLP transfer.

| Planned declaration | Kind / owner | Exact scope |
| --- | --- | --- |
| `root`, `conjugate_root` | instance / SplitPrime199V1 | roots of T²+T+1 in ZMod199 |
| `left_certificate`, `right_certificate` | instance / SplitPrime199V1 | the displayed2x2 matrix products equal I |
| `roundtrip`, `reverse_roundtrip` | endpoint / SplitPrime199V1 | existing matrix-vector laws at P,D |
| `projection_injective`, `projection_surjective` | endpoint / SplitPrime199V1 | existing inverse-function laws at P,D |
| `left_row_zero`, `left_row_one`, `right_row_zero`, `right_row_one` | instance / SplitPrime199V1 | four literal integer linear identities DP-I=199K and PD-I=199L, for IDC1 pointers |

Only the finite matrix certificate opens the two literal matrices for entry
calculation. They are not composites of the native constructor; no program or
curve interface is unfolded. No new generic law, limit raise or native_decide.
The toolchain is Lean4.24/Mathlib f897ebcf72cd16f89ab4577d0c826cd14afaafc7.
Final gate and explicit axiom audit will be recorded once after endpoints.

Sources checked (KB): MCP KB tools absent; repository wrapper status fails127
because its executable is unavailable. Fallback: frozen round06 protocol,
v6/modular_kernel.hpp and setup-06-u7.jsonl; Mathlib Data/Matrix/Mul.lean
(`mulVec_mulVec`, `one_mulVec`) and Logic/Function inverse laws. Existing
LLL1982/FLINT modular-GCD sources from the protocol inform construction scope;
no new prior-art or novelty claim is made in this packaging step.

Assembly03 (V2) passes. Assembly01 was stopped while Lake unnecessarily cloned
Mathlib; the existing exact cache is now reused. Assembly02 preserves the
finite numeral goals left by norm_num; ordinary kernel-checked decide closes
them in V2. No limit changes. The four integer row identities apply the existing
commutative-ring normalizer; they are coefficient certificates, not copied
general reconstruction laws.

Final unchanged gate01 passes 77 audited theorems with 0 failures and0 trusted
native exceptions. The twelve explicit axiom lists contain only propext,
Classical.choice and Quot.sound (integer rows use a subset). Gate claim
PCC-20261116-cff31a is unsigned; exact statement hashes and twelve individual
claim ids are in receipts/round-06/lean-claims-01.json. Four integer row IDC1
records bind the same final source, with producer issuance and mutation controls,
PCC-20261116-1025eb unsigned. No independent execution or full-transfer proof
is inferred. Failed preparation/assembly/root-directory commands remain separate
raw receipts. [pcc:unverified]
