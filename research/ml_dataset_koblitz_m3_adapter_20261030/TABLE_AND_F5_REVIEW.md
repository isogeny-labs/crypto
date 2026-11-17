# Independent S4 table and F5 admission review

S4 table patch reviewed SHA256 `cd8b97065ece7e8429ec0af777fe19cef9481b97fd615e5337d183bd0a042c93`. No source-level blocker found.

The old multiplication sequence computes z^0 through z^(2n-2). The replacement starts at1 and advances by shifting, removing the z^n bit and XORing the lower modulus exactly when that bit is set. This preserves every power, n-by-n table slice and square entry. With authenticated sorted lower exponents below n and n<=61, tail/mask remain n-bit and shifted operands fit u64. Irreducibility is not required for this quotient identity.

Admission precedes powers allocation and tail construction. Two modeled units per modulus term cover shift/OR, eight per advance cover fixed arithmetic/control, and retained n²+3n+fixed terms cover table population/setup under the unchanged coefficient32. The unchanged1MiB byte envelope and Stage::Table remain; this removes actual general multiply calls rather than merely renaming work. No parent-storage coefficient changes. Per-term deadline checks may honestly stop earlier with retained reservation.

Controls compare all24 requested degrees to legacy FieldStructure, plus dense/high-tail quotient edges n2/3/23/61, exact work, one-less work/bytes, expired deadline and observer refusal. The n2 fixture formerly supplied coefficient5 outside its two-bit domain; masking and explicit range assertion now supply valid two-bit operands and protect the dense reference index. F2mElement was moved to test-only imports after review.

F5 initial patch `d3829737bfb89d5e05fc1936e68ef27d415858b1abd796d285ae29456b8ad2d5`: production split reviewed without a mathematical/accounting blocker. Initial draft test placement was a compile blocker (new control in legacy mod tests, bounded helpers/types imported in bounded_k2_tests only); producer notified for relocation. Final hash pending.

The split is an explicitly authorized accounting-contract correction, not an execution speedup. Gate::prepare first charges entry work and recomputes degrees from validated original canonical equations. Criterion setup4096+128e is charged before inspecting that bounded degree slice or mutating fixed pruned/status storage. Empty input or any degree0 polynomial skips only the unused1024*(32e+1) allowance. Nonconstant input reserves that full active allowance before criterion work; active total equals the prior contract. Entry and setup charges remain on a later refusal. Memory, active loops, support selection, row reduction, stages and global ceilings are unchanged. Worst-case required_reservation remains a conservative structural bound; it is no longer a claim of exact charge for skipped inputs.

The original one-less active admission control now expects paid setup retained, with backend/status not claimed active and zero criterion/main/allocation counts. Bypass controls keep the explicit EmptyInput/SkippedConstantInput states. New exact/one-less total controls compare result row space against the independent oracle and repeat deterministic stage/reservation/row replay. No test/build/CLI/cloud actions were run by this reviewer; current-source runtime replay and retention gates remain necessary.

## Final F5 correction

Final F5 patch SHA256 `c39fdf3a3974500250031a9912bb81602acc2c20c10be2747002d20264c1da89` moves the new test into bounded_k2_tests, where its helpers/types are available. Confirmed source placement and diff; no further source-level blocker found. Production accounting is unchanged from the reviewed split above. Table patch remains `cd8b97065ece7e8429ec0af777fe19cef9481b97fd615e5337d183bd0a042c93`.

For n61 with four modulus low terms, independently checked table formula616480 ->156160 (460320 difference). For61 equations the skipped active criterion allowance is1024*(32*61+1)=1999872. Their sum2460192 is a conditional modeled allowance difference. It is neither a measured speedup nor proof that a new complete worker delivery succeeds, especially because serialized counter widths and actual deadlines/storage can change. No tests or builds executed by reviewer.
