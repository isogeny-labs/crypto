# R27 attempts and failures

All raw inputs and logs are retained. A cap, failed process or missing exit is not mathematical nonexistence.

| Run | Stage | Status | Explanation |
|---|---|---|---|
| action27-01 | native | accepted | Accepted controls, identities and recorded completion checks. |
| cusp27-01 | native | accepted | Accepted controls, identities and recorded completion checks. |
| direct27-01 | native | implementation_rejection | Constant polynomial assigned to number without leadcoef conversion. |
| direct27-02 | native | accepted | Accepted controls, identities and recorded completion checks. |
| io27-01 | native | accepted | Accepted controls, identities and recorded completion checks. |
| lambda14-01 | native | accepted | Accepted controls, identities and recorded completion checks. |
| laurent27-01 | native | implementation_rejection | Polynomial/integer identifier collision; no branches accepted. |
| laurent27-02 | native | accepted | Accepted controls, identities and recorded completion checks. |
| laurent27-corrupt-01 | native | expected_rejection | Expected rejection of actual source update corruption. |
| lean01 | proof | proof_or_runtime_failure | Field-map/sum ordering/denominator elaboration errors. |
| lean02 | proof | proof_or_runtime_failure | Task-local import cache runtime initializer failure. |
| lean03 | proof | accepted | Accepted controls, identities and recorded completion checks. |
| lean04 | proof | gate_blocked_before_compiler | Checkpoint credential discovery refused before compiler. |
| lean05 | proof | proof_or_runtime_failure | Extra virtual address limit prevents runtime thread creation. |
| lean06 | proof | proof_or_runtime_failure | Same address-space failure despite CPU affinity. |
| lean07 | proof | proof_or_runtime_failure | Scalar-zero simplification/import elaboration errors. |
| lean08 | proof | proof_or_runtime_failure | Broad simplifier leaves natural scalar-zero goals. |
| lean09 | proof | accepted | Accepted controls, identities and recorded completion checks. |
| lowgenus27-01 | native | accepted | Accepted controls, identities and recorded completion checks. |
| mapbuild27-01 | native | implementation_rejection | Reserved argument monomial; late output unaccepted. |
| mapbuild27-02 | native | accepted | Accepted controls, identities and recorded completion checks. |
| mapsolve27-01 | native | implementation_rejection | Cross-ring control lookup failed; stop requested; native caught TERM and returned 0. |
| mapsolve27-02 | native | resource_limit | All 910 controls pass; registered 4 GiB memory cap, exit 14. |
| move27-01 | native | accepted | Accepted controls, identities and recorded completion checks. |
| node27-01 | native | accepted | Accepted controls, identities and recorded completion checks. |
| norm27-01 | native | accepted | Accepted controls, identities and recorded completion checks. |
| quadrics27-01 | native | implementation_rejection | Reserved identifier kernel. |
| quadrics27-02 | native | implementation_rejection | Reserved identifier rank. |
| quadrics27-03 | native | accepted | Accepted controls, identities and recorded completion checks. |
| reverse27-01 | native | implementation_rejection | Polynomial remainder by 11 in Q used for an integer control. |
| reverse27-02 | native | accepted | Accepted controls, identities and recorded completion checks. |
| rr27-01 | native | implementation_rejection | Polynomial/integer identifier collision; rejected/stopped. |
| rr27-02 | native | wrapper_evidence_incomplete | Live shell launcher edit lost native exit status; no strict acceptance. |
| rr27-export01 | native | accepted | Accepted controls, identities and recorded completion checks. |
| rr27-o01 | native | accepted | Accepted controls, identities and recorded completion checks. |

Bulk list writing also omitted its last entry. IO01 isolated the failure; complete RR exports were regenerated with individually framed writes. Earlier frozen files remain unchanged.
