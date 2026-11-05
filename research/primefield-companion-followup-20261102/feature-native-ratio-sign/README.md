# Division-free normalized sign source

The sign-only source now compares N*calibration_inverse directly to D or -D,
avoiding formation of the affine field value N/D. It rejects noncanonical or zero
ratio inputs. Public identity points retain sign0. The raw-value API keeps its
original division and metadata through a private shared RawParts record.

The direct sign, legacy window and prepared seed-window APIs use the predicate.
The previously checked low-product polynomial engine and recovery implementation
are unchanged. This construction does not replace any existing comparison binary.

One optimized native build and one bounded invocation pass all106 tests, with none
filtered.1944 nonzero ratio cases compare against Fermat-inverse classification;
3888 common-rescaling checks and6 invalid-tuple checks pass. Saved raw/seed/window
checks and all8 complete recovery receipts against low-product baseline pass.
All24 prebuild source/data locks pass. See receipt.json and raw logs.

The algebra removes one final inversion and one product per finite sign. This is
a local derived operation count, not a complete-cost or timing result. The explicit
ratio checks and denominator negation remain, as do other source inversions and
the scale-dependent source/decoder work. Original4563-case cohort unchanged.

Next: instrument this source change separately and evaluate its complete effects
under a new frozen comparison when preflight/budget/EKS admission are available.
No panel or cloud job ran here. DERIVATION.md contains assumptions, proof and prior
context; report.pdf is the inspected editable construction report.
