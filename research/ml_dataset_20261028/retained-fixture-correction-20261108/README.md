# Retained caller fixture correction

The bounded libtest build on 1cfe91d3173f failed with E0063 because the retained caller test fixture lacked two required CellManifest fields. This correction supplies the cell schema and the digest of the existing synthetic run binding. It changes only the cfg(test) fixture, preserving the two tests of sticky rejection after run mutation or deadline expiry.

Source inspection and git diff --check passed. This correction has not been compiled or executed. SOURCE_HASHES.json preserves the preceding 1470-input manifest with only the corrected file digest changed; the prior failed attempt remains immutable. No retry, native cell, cloud job, budget renewal or production deadline change is authorized by this packet.
