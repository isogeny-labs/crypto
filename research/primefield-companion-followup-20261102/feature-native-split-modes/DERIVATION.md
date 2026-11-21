# Split mode evaluation on canonical fibers

T-1871. Prime-field implementation construction. Producer assertion; independent signed verdict pending. [pcc:unverified]

## Construction

Use the predecessor's disjoint monic supports B (both roots) and S (one selected root), with v(W) modulo S. B is unramified, and v²−Wv+1=0 modulo S. The candidate coordinate V is represented in the product of F_r[W,V]/(B,V²−WV+1) and F_r[W]/S, mapping V to (V,v). Empty supports use the unit polynomial and omit their component. All field/order and initial squarefree-splitting assumptions are inherited.

Polynomial evaluation commutes with these projections. For the fixed anchor and scale λ=t/t0, evaluate F_e(λV) with base λV in the pair component and λv in the selected component. A balanced block returns (value,base^length). Combining consecutive descending-coefficient blocks gives (left_value*right_power+right_value,left_power*right_power). One coefficient reader supplies both components, including mixed states; no second coefficient sequence is constructed for the selected component.

The C0 and C2 equations are formed componentwise. Filter the old selected support by gcd(S,C_selected), retaining v modulo the result. Apply the predecessor canonical keep operation on B alone, producing new both/selected supports and a new selected root. Old and new selected supports remain disjoint, so merge their root functions by the predecessor's coprime CRT formula. Thus each update preserves exactly the predecessor's canonical triple, including ramified roots initialized as selected. No total-support pair is reconstructed for evaluation.

## Charged work and limits

For a streamed polynomial with L coefficients, the balanced evaluator reads exactly L coefficients and makes 2(L−1) ring products per nonempty component, including the returned power at the root. One scalar ring product invokes one base polynomial multiplication and modular reduction; the pair kernel invokes three base polynomial multiplications plus its additions/reductions. These are source-level kernel calls, not scalar field-operation counts or timings. A mixed state pays both component costs at their respective support degrees. Ring construction, cloned supports/root, coefficient-stream setup, gcds, inversions, CRT and all temporary/capacity memory remain charged. No universal benefit is inferred for mixed or small states. Current code rebuilds component rings for each C0/C2 stage and uses ordinary extended Euclid for inverses.

All public-source grids, initial C0, order checks and source calls remain charged. The four-radius limit, lift/image caps and fallback behavior are inherited. Original 4,563-case cohort and prior immutable evidence are unchanged. No cloud or timed workload is run here.

## Validation design

Native projections compare split values with the total-support quadratic evaluation after reducing modulo B or substituting V=v modulo S. Masked constraints progressively select fibers, including ramification and mixed/selected-only states. Coefficient reads and product calls are asserted against the balanced recursion identity. A separate panel compares canonical triples and materialized sets after each of four radii for all scalar fixtures in five configurations. Two public-point recovery panels compare against recover_canonical at caps64/16. Scalar enumeration occurs only in checker fixtures.

The new adapter preserves the predecessor unchanged. Compiler invocation, input hashes, stdout/stderr and return codes are in validation01/receipt.json and sibling streams. generate.py pins copied source files. See RESULTS.md for observed output and any failures.

## Sources checked and next step

Read feature-native-canonical-fibers/{canonical.rs,canonical-image.rs,canonical-checks.rs}, quadratic.rs and persistent.rs before implementation. Official FLINT polynomial quotient conventions: https://flintlib.org/doc/nmod_poly/index.html (prime/unit preconditions and reduced polynomial arithmetic). Standard quotient projection and CRT are reused; no novelty claim. The research-kb MCP was unavailable and scripts/research-kb status returned `no executable found`; exact predecessor and primary documentation supplied the fallback.

Next remove repeated evaluator powers or compile shared branch work while charging setup, coefficient production and initial-support construction. A measured end-to-end improvement requires its separately admitted frozen workload; these correctness controls do not substitute for it.
