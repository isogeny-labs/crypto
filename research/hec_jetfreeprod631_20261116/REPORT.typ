#set page(paper:"a4",margin:24mm)
#set text(size:11pt)
= Production section shortcut: correctness gate
Genus 3 · 15 November 2026

#table(columns:(1fr,1fr),[Check],[Outcome],[No-base shortcut evaluations],[48],[Degree-two base fallback],[1],[Original destination classes],[Both unchanged],[Public adjacency corruption],[Rejected],[Process],[Exit zero; stderr empty])

== Production path
Evaluate reduced numerators → compute common base factor → factor one: return residues without Hensel/reference work → otherwise: retain the full cancellation implementation.

Both direct quadratic outputs reconstruct the same unique prime-subgroup classes as the preceding independent replay. Public primary/adjacent coordinate and sign checks pass. An explicit divisor containing the fixed base exercises the retained exceptional path.

The previous comparison gate established coefficient-by-coefficient reference equality. This gate checks production execution and final classes without running reference evaluation on the no-base branch.

== Logging correction
The raw branch-count line inherits a label claiming every call is compared with the reference. That label is incorrect for this production gate. Raw output is preserved, and the source label is corrected afterward. No arithmetic or acceptance condition changes. Corrected binaries must be rebuilt and hashed before timing.

== Custody and next measurement
Freeze536159b62708 and checkpoint20261116T032857Z precede one correctness execution. Source files removed from the working copy by sparse rules after the earlier merge were restored from committed content using additive checkout entries.

The prepared five-block collector compares the previous direct path with the production shortcut, changing only reconstruction executable. It has not run. A fresh benchmark record, binary/input hashes and checkpoint are required first. The earlier measured1.621xresult remains unchanged; no new timing or tenfold gain is claimed. Full cold integration and genus-two/four mechanisms remain open.
