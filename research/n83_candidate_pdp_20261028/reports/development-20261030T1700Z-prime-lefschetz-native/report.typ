#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Serif",size:10pt)
= Native Lefschetz development dataset
Observations only. The native certificate and replay path replaces the Python execution dependency that prevented completion of the previous42-case production attempt. Historical data and the interrupted run remain unchanged.

#image("pipeline.svg",width:100%)

*Recorded controls.*14tests passed: six new native controls and eight inherited donor tests. The new controls check block Hilbert dimensions, degree17 truncation boundary, the explicit8192-term squarefree witness and a coefficient mutation, leading terms for m3–9 and tied-weight negative controls, complete42-key coverage and duplicate/missing rejection, source/base/normalization binding mutations, and retained arithmetic artifacts.

*Raw structure.* The17JSON arithmetic artifacts contain structure and Hilbert rows, every normalized witness coefficient and squarefree mask, seven leading-term gates, seven negative gates, and source identity. NATIVE_LEFSCHETZ_FORMAT.json explains every file, bit index, variable order and numeric encoding. Binary and source snapshots, logs from every test invocation and all journal controls are retained in the development archive.

*Signal handling.* Success, deliberate exit7 and direct controller SIGTERM143 tests pass. The active phase records interruption; all unattempted phases retain status125. The first test only terminated the child; it is retained as narrower evidence alongside the corrected direct-controller test. The original production launcher and its incorrect terminal remain immutable with the previously published correction.

*S3 replay.* The archive was downloaded and its SHA256 checked. Every archived payload passed its manifest before and after replay. The restored exact native test executable passed all14tests; all17recreated arithmetic JSON files match byte-for-byte. These are development checks, not42production certificates or independent mathematical review.

*Remaining boundary.* No native production case has run; no new FFD,LFD,regularity or elite-base result is claimed. The written Lefschetz/Koszul argument remains conditional and independently unreviewed. Next work is a new frozen protocol with both native binary identities, a qualified EKS image/execution route, then the complete42case native production and fresh/S3replay. The current EKS overview confirms the earlier source probe ended DeadlineExceeded; it supplies no source-access verdict.

*Evidence paths.* validation-prime-lefschetz-native-development/custody.json; prime-lefschetz-native-development-archives/.artifacts.json; NATIVE_LEFSCHETZ_FORMAT.json. The full Koblitz/general-binary/prime m2–9 fixed-base objective remains active.
