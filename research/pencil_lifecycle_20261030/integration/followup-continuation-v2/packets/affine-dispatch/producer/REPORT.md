# Source-ordered affine addition: execution, uniqueness and preservation

Fourteen main declarations establish total modeled execution, unique branch/output, and preservation of stored polynomial validity for the affine Jacobian::add source path. Nine fixed controls and twenty-three separate named axiom readbacks passed. This packet composes existing word/polynomial primitives, stored-input extraction, strict fallback and array construction; it changes no native source.

## Construction

AffineConstruct proves a general normalized monic list has last word one, identifies the active stored slot after raw construction, and derives construct=Some(raw) plus StoredInput.WellFormed from normalized canonical valid lists. Inactive padding is initialized by this constructor; identity returns preserve the original object's padding.

AffineRelation classifies source branches in order: left identity, right identity, then degree2/2 with norm-zero, slope-zero or fast; other nonidentity degrees use fallback. Runs retains exact execution evidence: fast exactDiv and remainder equations, leading-word assertion and constructor equation; fallback includes both extended-gcd Run/finish traces through Chain, exact quotient calls through Divided, both remainders, reduction Run and final constructor equation. Output validity is not a premise in the Runs constructors. The tags are disjoint by the actual classifier and every modeled execution has that unique tag.

AffineTotal constructs an execution and a wellformed output under prime2<p<2^31, canonical quintic f and two wellformed stored inputs. AffineDeterminism proves uniqueness of extended-gcd terminal registers, reduction-loop outputs and fallback-chain coefficient results. AffineUnique combines these function/trace equalities with constructor equality to prove unique stored output and wellformedness of every modeled execution, not only of the selected existence witness.

## Source and domain

Native source is the immutable e62d035ab852be5bbfd27bbe8f11c8f6bfe25f2e snapshot SHA25626e99b56dace8f848d055e8709bb989622e4caea65aec26d63e00805153dc3c6, constructor/accessors943–964 and add1004–1061. The stored-input source was subsequently published and independently reviewed at4bc1aa36a8. Exact imported source/cache hashes are in every receipt. The construction follows existing primary source and formal components, with no new-method claim.

This is the n=1/no-diagnostic-table source value model. Native Field::validate admits p>=5; the algebraic helper domain remains p>2. WellFormed explicitly requires canonical slots and capacities; native validity does not itself enforce canonical inactive padding. No preprocessing or canonicalization is inferred from acceptance. Identity precedence returns the other original object.

The classifier models source branch outcomes, not evaluation costs: native computes s1 before evaluating norm!=0 && s1!=0, whereas the pure classifier need only consult s1 after a nonzero norm. Fast polynomial V is computed before U in source; the retained call/value definitions do not assert a temporal scheduling/counter trace. Runtime allocation, compiler and counter semantics, extension-field arithmetic and divisor-class equality are not part of these statements.

## Checks and fixed controls

Successful checks, all unchanged bindings and exit0:
- AffineConstruct-attempt-HG9mxa5c
- AffineRelation-attempt-M0Rfp7PY
- AffineTotal-attempt-pWTA6RaF
- AffineDeterminism-attempt-siMNtmWo
- AffineUnique-attempt-Lok9DIcQ
- AffineControls-attempt-Rk0sn5Q6
- AffineReadback-attempt-8qOHTGL7

Separate readback names every main/control declaration. Axioms are subsets of propext, Classical.choice and Quot.sound. ExpectedFailure rejects0=1. Imported caches are reused; independent source-fidelity review and fresh independent replay are next.

Controls establish identity precedence and explicit left/right returns; all four nonidentity classifications; f=X5 and f=X5+X3 decoding; valid slope-zero inputs; and existence of complete preserving modeled executions for degree fallback, norm-zero, slope-zero and fast. The norm-zero fixture has equal supports/zero ordinates; slope-zero uses U=X2 and U=X2+1 with V=0 on X5+X3. Fast uses the stored-input p17 fixture. These polynomial controls need no smoothness claim and are not fresh native scientific trajectories.

One failed control attempt, AffineControls-attempt-bRf3ZMss, is retained: simplification left nested multiplication equal to X5 and two natDegree goals. Explicit ring normalization and natDegree_X_pow rewrites repaired the proofs without changing fixtures or claims. No other source-module failure occurred.

## Next joint

Compose the separately produced stored native-validity wrapper with every_execution_preserves, retaining canonical inputs. Then perform independent source-fidelity review of dispatch/order, exact call evidence and deterministic output proof. Class-sum/representative equality is a separate subsequent semantic task. No cohort, runtime qualification, cost or campaign authorization is changed.
