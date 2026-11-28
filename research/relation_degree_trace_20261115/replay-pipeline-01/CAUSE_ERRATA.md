# Correction to the initial failure attribution

PCC-20261116-b4b6bd correctly records null selection, absent complete-roster
ratios and matching completed answers. Its statement that every Std import
exceeded100000 total cofactor terms is too broad [pcc:unverified].

Four cases (star-last-n10, both n12, star-last-n16, all seed7) explicitly report
replay: total cofactor budget. The star-first-n10/n16 import processes instead
terminate without a normal exit code, with empty stderr and witness files exactly
16777216 bytes, the declared file-size limit. This is consistent with that
limit; the recorded process JSON does not identify a signal number. Do not
invent one. No case's success, selection or timing status changes [pcc:unverified].

The first renderer's report-01 draft repeats the overbroad attribution and is
retained. Use the corrected report-02 once generated. Original claims, summaries,
process files and failed outputs remain byte-identical. No experiment is rerun.
