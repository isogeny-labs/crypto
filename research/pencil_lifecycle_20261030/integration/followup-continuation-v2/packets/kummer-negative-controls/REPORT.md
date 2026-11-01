# Boundary recovery: fixed negative controls

Three new fixed negative controls reject with the expected statuses; one contextual positive succeeds. The locked offline software check exits0. Candidate recovery implementation is unchanged from the preceding84-case packet; only a test wrapper was appended. The original84/80 failure and84/84 provided-neighbour results remain unchanged.

## Fixed inputs and results
Singular C1: set theta diagonal a=0 in the retained p739 model while supplying nonzero vectors. Result: singular_coordinate_map, outputNone. This intentionally invalid model tests rejection only.
Nonsquare ordinate: x=2 on y²=x(x-1)(x-296)(x-16)(x-10) overF739 gives654. The native check verifies654^369=738 modulo739. Forward-map(0:1:2:4), then recover. Result: degree1_nonsquare, outputNone.
No signed neighbour: retain valid degree1 d=2Q, but supply kappa(G) as its neighbour. Native checks establish d!=0 and d!=±2G. Result: degree1_no_signed_lift, outputNone.
Contextual positive: supply kappa(d+G), constructed using signed d, and recover exactly d. Result: degree1_unique_signed_lift. This is a helper check given a separately supplied correct neighbour.

## Explicit scope correction
The earlier84 recovery fixtures compute external_plus=project(expected+G). That neighbour is derived from the signed expected divisor in the assertion harness, and its construction was not separately priced. Thus those checks do not establish end-to-end signed recovery directly from the selected torsion-difference pair, or oracle-independent collector execution. The immutable correction is /var/tmp/pencil-kummer-recovery-scope-addendum-20261031/{ADDENDUM.md,scope.json}. Actual state retention or a charged construction of the non-torsion neighbour remains a separate obligation.

## Custody and next construction
PLAN.md fixed all inputs, expected statuses and120s bound before execution. Files and candidate-prefix identity are in pre-execution-manifest.json; exact command and exit0 in receipt.json. No field-value scan, collector solve, benchmark, cloud operation or production edit occurred. These controls cover one retained model and do not prove universal completeness.
A possible next state construction retains the four quotient points R,R+T,R+G,R+G+T for2T=0. It may generate the needed neighbour with differential operations using fixed differences G,T,G+T. This is a proposed algebraic construction, not an implemented result in this packet.
Sources: pinned historical N19 forward projection and signed-recovery code; Chung-Costello-Smith sections2,6.3,6.4,7.4, https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf .
