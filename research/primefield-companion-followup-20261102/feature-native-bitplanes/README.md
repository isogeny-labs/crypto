# Exact bitplane character decoder

A separate decoder compiles support, odd-multiplicity and coefficient bits across
terms into machine-word planes. OR masks identify zero terms, XOR masks identify
negative terms, and signed population counts reconstruct the exact polynomial
through modular Horner accumulation. Source and coefficient packet are unchanged.

All237 native methods pass, including the full479-input identity,729 word-boundary
cases and8 complete recovery receipts. Derived saved-packet layout:328 u64 plane
entries (2624 payload bytes),144 population-count calls and9 modular Horner steps.
These are not measured time/complete-cost gains. Setup transposition and integer
work remain to be counted. See DERIVATION.md, control.json, source-lock.json,
receipt.json and raw logs. The source is reused directly; recovery routing is the
only integration change. The report includes an editable diagram and inspected PDF.
