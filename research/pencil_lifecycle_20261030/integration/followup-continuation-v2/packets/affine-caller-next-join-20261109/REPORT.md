# Next affine caller invariant join

Dispatch-to-stored-validity already exists in dispatch-acceptance. Do not recreate it; affine-dispatch source-fidelity review is still a separate pending obligation.

## One proposed statement

Under prime2<p<2^31, canonical degree-five curve coefficients, Field.n=1/no tables, and initial WellFormed a, every source-ordered Jacobian::mul trace initialized r=Divisor::O preserves WellFormed of both r and a at every reached loop boundary and returns an r accepted by StoredValidity.test. Each reached addition receives WellFormed operands derived from the loop invariant, not a separately assumed per-call predicate.

three_leads.rs Jacobian::mul lines1062-1073: test odd bit; add accumulator if set; shift k; square base only if shifted k>0; return r. Preserve k=0 and k=1 skip behavior.

The new join is induction over the native caller loop, deriving each add call input contract from the prior loop state. Existing fast CRT, quotient/head, division, constructor and dispatch output work is reused.

After this bounded caller closure, connect Jacobian::point canonical x,y plus asserted curve equation to initial WellFormed; transfers and externally supplied divisors need their own canonical storage provenance.

- No proof that valid() alone establishes canonical inactive padding; initial WellFormed provenance remains required.
- No scalar/group-law semantic equality, physical Rust refinement, weighted dispatcher closure or operation-cost equivalence.
- No need to repeat complete fast/fallback/constructor joins already in affine-dispatch.

Prerequisite review: Independent source-fidelity review of both affine-dispatch and dispatch-acceptance remains pending; producer results are reusable prerequisites, not already validated native closure.
