# Independent source derivation before producer proof

Read retained gate lines703-752 and890-938. double22 computes n1=q*s-t,n2=r*s,B=2*(n1*t-n2*s), then tests B0 before s0. add22 compares crossed q/r support, then inverse crossed s/t, then equal crossed s/t and delegates original a to double22.

If s=0, literal B simplifies to -2*t². For 2!=0, B0 forces t0. Hence crossed s1,t1 are both zero regardless of weights. Equal crossed ordinate guards then force crossed s2,t2 to zero. The preceding inverse predicate is therefore true, contradicting the equal-arm's actual path condition. Nonzero weights or Mumford validity are not needed for this guard-only argument.

Direct calls to double22 with s=t=0 still return identity legitimately. Characteristic2 makes the algebraic implication B0,s0=>t0 false without the explicit characteristic hypothesis; however equal-vs-inverse precedence is itself special there. Do not infer a direct-function invariant from the nested-call exclusion. Positive nonvacuity can use equal unit representations with s!=0; B0 may select the point-producing arm. Review must keep support equality as a source reachability context rather than assert it is needed for the algebraic contradiction.
