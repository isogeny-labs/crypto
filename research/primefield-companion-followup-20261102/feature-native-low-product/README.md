# Native truncated polynomial products

`Poly::mul_low(a,b,n)` computes the canonical low n coefficients of a product,
with [0] for n=0 or a zero result. It normalizes and truncates inputs, preserves
the cutoff12 schoolbook/Karatsuba strategy, and omits schoolbook terms i+j>=n.
Full-output requests use the existing full multiplier.

Newton series updates, reversed quotient products and low-remainder reconstruction
now call mul_low. Product-tree construction keeps full multiplication. Source and
recovery copies are byte-identical to the monic component's routed files; the changed
dependency is polynomial.rs. Previous components and comparison drafts are unchanged.

One optimized native build and one bounded test invocation passed all66 methods,
with no filtered tests. The new grid has936 cases over p=7,509, empty/noncanonical
inputs, balanced/unbalanced lengths, cutoff boundaries and truncation lengths.
Each result matches both full multiplication followed by truncation and independent
schoolbook convolution. Sparse cancellation cases and all8 complete recovery
receipts also pass. Inherited tests cover division, inverse series, ordered values,
resultants, geometry, seeds and invalid inputs. Seven prebuild source locks pass.

No counted comparison or timing panel ran. This is a separately checked candidate;
it has not replaced the binary bound by the existing monic comparison draft.
Integration into a matched counted panel requires instrumenting the new additions
and product loops and publishing a new protocol/candidate identity.

See DERIVATION.md, receipt.json, raw build/test logs, source-lock.json and report.pdf.
The module retains the existing word-size prime-field domain (p<=u64::MAX/2,
caller-certified primality). The original4563-case full cohort is unchanged.
