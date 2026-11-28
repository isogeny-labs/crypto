# First eligible PR4992 CI review

Head8475b8a2 has13 all-attempt check runs across12 workflow runs, below the
30-check ceiling. Both failing gates remain failures: artifact hydration
requires original history/store data; PCC tests raise three NameErrors for
q/reasons. Full check/run metadata is retained here, and both complete
decoded logs remain whole at their recorded private paths with SHA/provider
URLs and remote custody pending. [pcc:unverified]

Existing tools, scripts and workflows are byte-equivalent to base816 except
the newly added main-only study workflow. This input comparison is not a
fresh baseline CI execution, merge waiver or proof that every downstream
stage would pass after the first failure. No gate/verifier/keyring/policy
change or unchanged CI rerun was made. PR4992 remains draft for the separate
formal instance-design gap; current shared library work is PR34. Check
ACTUAL_FIRST_CI_REVIEW.json for the next unchanged-head review time, at least
four hours after this completed review. [pcc:unverified]
