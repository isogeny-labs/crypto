# Independent product accumulation review

Reviewer /root/monic_review, 2026-11-01. Scope: ProductWord, ProductRow, Product in producer packet pencil-product-accumulation-20261101-v1. Repository source reference f2075e7b23bcb41e28efb584a43e83c755dba150; exact inputs hashed separately.

## Source-first derivation

Before reading producer product proof bodies, I read native three_leads.rs pmul and recorded the following invariant in review-plan.md: completed rows contribute the corresponding prefix of A times B; within a row, processed columns contribute a_i X^i times the processed prefix of B. Buffer entries remain canonical and buffer length stays |a|+|b|-1. For nonempty inputs i<|a| and j<|b| imply i+j<|a|+|b|-1. Empty input returns [] before allocation. Final trimming preserves polynomial decoding. This invariant was not obtained from the producer proof. The inherited parent context disclosed the claim and module names, but not these new proof bodies; no sibling report was read.

## Mathematical findings

ProductWord.add_product derives modular sum/product decoding and canonical output from explicit canonical input words and p<2^31. Under those premises, the raw product is below 2^62 and the modular addition is below 2^32, so wrapping cannot affect decoding. step_correct updates exactly one array coefficient; run_correct accounts for repeated destinations by addition rather than overwrite. The generated event list preserves source operand order and starts at the selected row offset. ProductRow identifies its accumulated polynomial with C(a_i) X^i B. Product.rows_correct carries the offset and remaining rows, and its invariant decoded output = decoded initial state + X^j A_remaining B agrees with the independently derived loop invariant. Recursion decreases list lengths; no assumption of the final product is made. Product initializes a zero buffer, handles both empty branches before the nonempty allocation case, and applies polynomial-preserving trim. The theorem yields canonical output and exact polynomial product.

No mathematical defect found in these claims. Prime modulus is stronger than needed for the abstract convolution ring law, but is an explicit permissible assumption. Inputs need not be monic, nonzero, normalized or equal in length; final zero product can return [].

## Native correspondence

Native pmul lines 835-848 uses row-major i then j traversal, writes r[i+j]=add(r[i+j],mul(x,y)), and trims once after both loops. The model preserves this operational order. Native odd prime-field n=1, diagnostic tables absent has precisely the reviewed modular formulas. The Lean theorem's Nat bounds certify mathematical index validity, not Rust usize addition/allocation safety for arbitrary lengths. Native a.len()+b.len()-1, memory allocation/OOM, counter overflow, geometry_events instrumentation, binary/extension-field branches and compiled equivalence are unmodeled. Therefore fidelity is faithful_scoped for the explicit model and partial for full native execution. The small fixed genus-2 caller lengths should later discharge the native capacity arithmetic obligation explicitly.

## Fresh verification and controls

Fresh isolated builds of ProductWord, ProductRow and Product passed. Producer Controls and Readback were copied only after the parent reported stable Controls and stable Readback source. They passed fresh builds. Ten main declarations and eight producer controls passed separate axiom readback. Six reviewer-added controls passed: asymmetric internal-zero operands, swapped asymmetric operands, nonempty zero operand, both empty branches and maximum canonical residue update at p=17. Readbacks for all 24 declarations use only propext, Classical.choice, Quot.sound. Local imported helper source/binary snapshots and cached Mathlib dependency unions remain the declared trust boundary; this is not a clean rebuild of every dependency or a separate kernel checker. Detailed commands, native exits, input bindings and full logs are retained.

## Workflow assessment

Satisfied: prospective local plan before proof-body access; source-first independent invariant derivation for this exact scope; separate reviewer session/directory; no producer authorship; no sibling verdict access; fresh isolated replay of all three new modules and controls; named axiom allowlist; explicit model/native distinction; independent controls; read-only repository.

Missing: precommitted plan and immutable published producer input at dispatch; complete local/dependency source rebuild and independent kernel checker; multi-reviewer independence-checker gate; coordinator publication and official decision. This scoped adversarial review is useful evidence, not a claim that the entire formal-review workflow is complete. No scientific execution or official status transition occurred.

## Primary sources

Native three_leads.rs pmul, Field::add/mul; ProductWord/ProductRow/Product; imported Events (ordered indexed list), UInt64Residue and WordAdd11 arithmetic bridges. Standard distributivity and coefficient convolution supply the mathematical interpretation; no novelty claim. Separate source hashes bind this exact review.
