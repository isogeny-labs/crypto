# Retained development outcomes

1. An initial metadata setup command used a repository-relative path while
   already inside the new Lean directory. It failed before writing its
   intended manifest. The shell nevertheless started Lake; that first
   fresh-cache fetch was interrupted with exit130. Its log is retained.
   The interrupted cache stays private outside the repository. This paragraph
   transcribes the setup error from the tool response; it is not a captured
   raw stderr receipt. The corrected setup used the actual current directory
   and the already-pinned Mathlib checkout/cache.
2. The guessed `Mathlib.Tactic.Decide` import does not exist at the pin. The
   second full build log and original source snapshot are retained. Core
   finite decide needs no such import; it was removed.
3. The next elaboration required a Nontrivial ZMod101 instance. The failed
   source/log, including the printed sorryAx from the unsolved goal, remain
   retained. Adding the concrete Fact(1<101) closes that obligation. No failed
   elaboration is counted as a theorem or accepted proof.
4. The fourth build passed with a letI style suggestion; that source/log is
   retained. The final source uses let and passed both the maintained focused
   gate and the containing Lake target. The maintained axiom auditor itself
   prints a String.mk deprecation warning; its bytes were not patched.
5. First TeX render failed on an unescaped underscore and reported one
   overflow. Its exact source/log are retained. The corrected final2-page PDF
   has no overfull box and both pages were visually inspected before hashing.
6. Whole docs/ACTIVE prose lint has eight pre-existing findings outside the
   edited line. Those baseline lines were not changed. New assessment prose
   is checked separately; no whole-repository prose pass is inferred.

No scientific native experiment, solver, benchmark or paid job is concealed
by these build/layout failures. No frozen source/evidence was overwritten.
