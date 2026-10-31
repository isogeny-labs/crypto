# Corrected exact control freeze (before Lean control compilation)
Correct expansion of the planned simple fixture:
x=0,y=1,u=X²+1,v=0,k=1,m=1,h=X²+1,u'=X²-X-1.
f=h²+X*u*u'=X5+X²-X+1. Thus(a0,a1,a2,a3)=(1,-1,1,0).
The scratch expansion in initial PLAN.md was erroneous and is retained there as a planning failure; this exact identity will be kernel-checked, not treated as assumed data.
Weighted realization: point a=(0,64,2,2), since y=64/(2⁵*2)=1; b=(0,81,0,0,3,2), since r=81/3⁴=1. All weights nonzero, k!=0.
Check full theorem, explicit output normalized q'=-1,r'=-1,s'=-1,t'=-2. Negative control k0 boundary and altered invalid f/ordinate, plus nonzero slope independent fixture if tractable. No scientific sampling or measurement.
