# Miller/Frobenius feature builder

recurrence.rs implements cubic power columns in linear field work, including
the characteristic-r anchor and zero-shift factorization. lib.rs substitutes
these columns into the preserved dense constructor. Elimination is unchanged.
All1037 reference columns match across seven prime fields; the r479 output
packet is byte-identical to the dense baseline. See DERIVATION.md and receipts.
No runtime comparison was run; dense solve and source costs remain separate.
