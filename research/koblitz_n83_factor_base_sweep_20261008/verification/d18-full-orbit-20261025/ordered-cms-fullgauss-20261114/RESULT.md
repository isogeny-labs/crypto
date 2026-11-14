# Full-Gauss solver unlocks the ordered D18 positive control

CryptoMiniSat 5.14.7 solved the exact ordered-first-pair D18 constructed
positive XCNF in **2.0425 seconds** with 3,879 reported conflicts when all
24 recovered Gaussian matrices were enabled. Its complete assignment passed
an independent replay of all 2,807,999 original CNF clauses and 3,901 XOR
rows, then a separate characteristic-two curve, subgroup, and relation-row
replay. The same XCNF under the earlier five-matrix policy reached its frozen
one-million-conflict cap after 431.66 seconds with `INDETERMINATE` status.
These are two bounded solver-policy observations on a constructed control;
the capped arm has no measured completion time. [pcc:unverified]
Claim `PCC-20261114-2bd98e` has `unverified` standing because its retained
local transcript and replay are unsigned.

The first public-target natural query then reached its one-million-conflict
cap with `INDETERMINATE` status after 424.60 seconds. It supplies one censored
query with zero verified relations, not a natural-yield estimate.
[pcc:unverified] Claim `PCC-20261114-d174b0` has `unverified` standing.

| Item | Frozen result |
| --- | --- |
| Curve/base | primary N83 `a=0`, D18: 5,418,738 subgroup points, 32,643 folded columns |
| Positive target | `(6594052263204820946437856, 9593508870534830450764009)` |
| Source/object | `afebef39283466fca4ddff01063d26df9b59c45e`; compressed D18 object SHA-256 `eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b` |
| Positive XCNF | SHA-256 `c632c152447cb09dbe7dd7cf7e29f713588c37bd56b67dfbf1fa7dac4cb15b74`; 329,808 variables |
| Full-Gauss solver | one thread, fixed seed 20261114; `--maxnummatrices 32 --autodisablegauss 0 --gaussusefulcutoff 0`; 600 s/1,000,000 conflict cap, 660 s outer guard |
| Positive solve | SAT, 2.0425195 s process wall; 1.701798 s user CPU; 0.065882 s system CPU; 186,720 KiB sampled peak RSS; no new swapouts |
| Independent replay | all 2,807,999 CNF clauses, 3,901 XOR rows; four archived subgroup points sum to the target, and their four relation columns replay to the same target |
| First natural target | public corpus point `(1834913121178197242856238, 8797964190732846213981377)` |
| Natural export | `PASS_XCNF_EXPORT_ONLY`, 52.3456487 s process wall; XCNF SHA-256 `bafed0f783ad93fcc91dcbd4c0ab45a505fc4ee3f2b0607a561e63995d0c89c3`; 329,808 variables, 2,807,501 CNF clauses, 3,901 XOR rows |
| First natural solver preflight | `HOST_LOAD_GATE_FAILED_BEFORE_LAUNCH`; three one-minute samples 42.98, 45.49, 46.89 exceeded the frozen maximum 24 |
| Second natural solver preflight | loads 16.48, 15.48, 15.30; swapouts unchanged at 164,320 |
| Natural solver | `INDETERMINATE`, exit 15 after 1,000,001 reported conflicts and 424.5995106 s process wall; 421.023898 s user CPU, 1.512929 s system CPU; 478,880 KiB sampled peak RSS, no new swapouts; all 28 Gaussian matrices used |

The [positive protocol](PROTOCOL.md), [natural protocol](NATURAL_PROTOCOL.md),
and both solver locks were committed before their respective executions.
The [raw manifest](evidence/RAW_MANIFEST.json) binds the archived positive
solver transcript, receipt, complete original-XCNF replay, group replay,
natural export receipt, and the rejected host preflight. The separate
[natural cap manifest](evidence/NATURAL_RAW_MANIFEST.json) binds the second
host preflight, terminal solver transcript, receipt, and independent status
audit. The two 50 MB XCNFs remain in the local raw root named by those
manifests; their hashes are pinned in committed locks.

The next experiment must change a frozen solver, encoding, or base policy
and re-establish a constructed positive gate before more natural queries.
Any returned natural model needs original-XCNF and independent group/row
replay. Measuring natural relation yield, useful rank, subgroup logs,
unseen-target descent and matched same-point rho remains the path to an
end-to-end factor-base selection on both N83 curves.
