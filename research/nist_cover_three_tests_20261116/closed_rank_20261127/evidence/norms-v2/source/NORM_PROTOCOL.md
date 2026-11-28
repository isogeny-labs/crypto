# Necessary signed-norm test before auxiliary fibre expansion

The completed rational old-base lift and line scan gave no additional rank.
Before performing a second geometric lift, test the necessary elliptic condition
for every canonical section whose individual images belong to the existing
429 signed elliptic columns or O. Keep the first 225 columns old and the
remaining 204 auxiliary. Their original rank-six baseline is unchanged.

Let kappa be the unchanged reference section's elliptic sum. Every such line
section pushes forward to a multiset of four signed column symbols, including
O and multiplicities, whose sum is kappa. This also covers closed curve points
mapping individually to rational column points: their degrees give repeated
symbols. It does not cover traces of nonrational elliptic images or new columns.

Enumerate all unordered signed-symbol pairs with repetition. For 429 nonzero
columns the alphabet has 859 symbols and 369370 pairs. Compare the entire
ordered pair-sum and kappa-complement streams using OpenSSL named-curve and
separate GMP affine addition. Join all matching complementary pairs by both
grouped and sorted algorithms. Deduplicate coefficient vectors, not mere word
representations, before the bound of 4096 distinct rows. Preserve one complete
four-symbol witness for each row, including the zero reference row separately.
The old 225-column mode is a separate diagnostic, not a changed baseline.

Require every original row (up to overall sign) to be represented among the
retained norm constraints. Check all retained elliptic rows directly. Append
them to the unchanged original matrix and compute full rank, rank restricted
to auxiliary columns, closed rank and gain over six. A graph cycle, repeated
column or rank growth alone is insufficient. Positive norm candidates still
require the declared geometric/KM and separate replay before establishment.
[pcc:unverified]

If the union has closed rank six, every subset realizable by those canonical
sections also has gain zero. In that outcome, reject positive gain from H-SA
without performing its auxiliary fibre lift. This is a finite necessary-
condition result for a fixed column domain, not a general DLP conclusion.
[pcc:unverified]

Controls: a separate two-column G,2G fixture with target 4G must retain the
nonzero row (-4,2); grouped/sorted joins and both arithmetic streams must
agree; missing pair data, a mutated witness coefficient and omission of a
required original row must reject. The fixture never enters the actual matrix.
One CPU,1GiB address-space limit,300s strict build and600s native workers;
all source/input/binary hashes, exits and costs retained. No new PCC,scalar
recovery or performance comparison. Reuses the previous signed-word/pair-join
code and Diem's canonical line relation, already reviewed in this study.
