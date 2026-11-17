default(parisizemax,1000000000);
read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/jac.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/glue.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/ic.gp");
Q=65521; QH=(Q-1)\2; FBS=12345; FBTH=Q; l=16381;
mk(m) =
{
  my(v, x1, x2, e1, e2, xc, a, b, nv = (Q+1)\2);
  ROWS = vector(m);
  for(r=1,m, x1=random(Q); x2=random(Q); while(x2==x1,x2=random(Q)); if(x1>x2,[x1,x2]=[x2,x1]);
    e1=if(random(2),1,l-1); e2=if(random(2),1,l-1); xc=1+random(l-1); a=random(l);
    ROWS[r]=[[x1,x2,Q],[e1,e2,xc],a]);
}
t(m, ol) = { my(t0=getwalltime(), r); mk(m); print("make ",getwalltime()-t0); t0=getwalltime(); r=icsolve(m,l,ol); print("m=",m," orderlarge=",ol," res=",r," ms=",getwalltime()-t0); }
t(32768,0); t(40000,0); t(65521,0); t(65521,1);
quit
