# Symbolic S4 multiplier-row recurrence — external patch

Modified native source: src/cryptanalysis/koblitz_s4_bounded.rs only. Patch s4-recurrence.patch SHA256ceb83dc7a7c3e57b113106aadc9b5b3b3c0243e4ae9927855acc7a25c952f454. No module/profile/domain/equation/schema change. Parent requested original32primitive coefficient and parent-storage32tariff remain. No worktree edits, builds, Rust tests or cloud operations; rustfmt parsed/formatted external draft successfully.

## Retained evidence

Read only /tmp/verified-worker-3518616-1793463700154608798/live/events.jsonl and original input. Construction native work6252720: Product5394624, Table616480, Selector196864, Square33856, Coordinates7824, Add3072. Native matrix total797376, giving7050096overall native work. Construction has49events including17Products; the complete native stream has273events. No rerun occurred.

s4-recurrence-fixture-audit.json binds saved event bytes/source/patch and lists each Product delta. Given three selector variables, support lengths0..8; every old Product delta uniquely determines left/right support counts under the existing tariff. Right support counts total41. Replacing only n² row preparation by8n yields modeled removal4241696 on these SAME supports. This is arithmetic from retained reservations, not a completed fresh attempt, timing result or prediction of downstream completion.

## Actual algorithm improvement

Old Gate::mul builds each nentry coefficient-multiplication row by independently scanning coefficient bits against each of the n structure-table rows. New private fill_multiplier_row builds the identical values f,zf,...,z^(n-1)f by polynomial-basis recurrence. With tail=st.reduced[n-1][1]=z^n modp and mask=2^n-1:

row[i]=value;
carry=0-(value>>(n-1));
value=((value<<1)&mask)^(tail&carry).

The table is created by this Gate from the original modulus, n23..31/47..61, so the selected tail is exactly the original modulus's reduced leading power. Induction: the row starts atf, and each update multiplies byz in the same quotient; removing the high term and XORing its reduction is precisely polynomial division. No irreducibility assumption is needed for this identity. Coefficients stay nbit, so carry is0 or allones, and n<=61 avoids shift overflow. This equals the old sum_j f_j*reduced[i][j] coefficient-for-coefficient.

Pairwise support multiplication, Boolean mask OR, coefficient accumulation, canonical sort/cancellation, the entire fixed S4 expression, and output coordinate ordering are unchanged. No scalar truth-table/resultant substitution. Both twists continue to use the original target/domain/field; this helper does not read or change curvea. Existing full adapter positive/both-twist/real-worker controls must still validate the resulting source; they have not run here.

## Admission, memory, events

Original Product work32*(rhs_support*n² +q*(n+1)+q*ceil_log2(max(q,2))+q+1) becomes32*(8*rhs_support*n +same remainingterms). Eight modeled fixed steps per row cover store, carry shift/negation, data shift/mask, tail mask, XOR and loop/control; existing deadlines/checking remain control overhead under the same modeled convention, not exact instructions. Setup uses existing fixed term. Gate::admit executes before row/out allocations or mutations; unchanged reservation byte formula and capacities, no refund or limit increase. The same one Stage::Product observer event occurs at admission with truthful new cumulative reservation. Additional per-row deadline checks may stop earlier, preserve admitted reservation and expose existing Deadline outcome; no extra events or hidden retry.

## Draft validation

1. Compare recurrence directly against the OLD dense-table XOR implementation for every requested integer degree23..31/47..61, zero/unit/allbasis/dense/topbit/repeated multibit coefficients. Add dense and high-tail quotient edge moduli n2/3/23/61 explicitly without claiming irreducibility.
2. Compare complete FieldSum products/support/order against the old dense-row reference. Exact/one-less original work and bytes, expired deadline, and observer failure verify pre-mutation admission and preserved charge/stage position.
3. Stop each of the17Product observer positions in the unchanged49event expression and check exact event prefix/reservation against a completed same-input construction.

Existing canonical full-equation equivalence against legacy sym_semaev_s4 and independent scalar/resultant controls are expanded from five representative sizes to ALL24 requested integer degrees; their fixed targets and all eight assignments remain unchanged. Selector padding and all existing refusal tests remain unchanged. Draft tests are uncompiled/unrun. The new native source path requires parent scope review before application; no Conductor scope or runtime validation is inferred from this external patch.
