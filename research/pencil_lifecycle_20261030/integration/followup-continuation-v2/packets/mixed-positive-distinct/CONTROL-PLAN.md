# Fixed controls before compilation

Full rational nonunit original-input fixture: a=(0,64,2,2), b=(-9,0,0,486,3,2), curve f=X5-X+1. Normalizes to point(0,1),u=X(X-1),v=1. Source K0,cancellationfalse,other-support distinct; original divisibility witnesses X4-1 and X3+X2+X+1. Invoke full source_branch_preserves. Independently compute normalized final(q,r,s,t)=(3/4,5/4,7/8,-3/8); this tests exact payload, not class-sum.

Boundary controls: zero ordinate a=(0,0,1,1),b=(-1,0,0,0,1,1) selects cancellation; repeated support a=(0,1,1,1),b=(0,0,0,1,1,1) gives xz2=x4 and positiveCoincidence, so distinct-other premise excludes source tripling. Invalid point constant coefficient: X does not divide (X5-X+2)-1; prove by coefficient0.

No random input, native arithmetic run, new trajectory or performance measurement. Compile controls120seconds after main module succeeds. Preserve errors/source snapshots if proof syntax needs repair. Separate readback covers every named theorem.
