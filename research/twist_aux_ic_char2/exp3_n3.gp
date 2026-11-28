\\ EXP3: n=3 (odd), q=83,89 (above Weil threshold q>=83). Theorem: no all-lift and no all-twist line exists.
\\ Sample random lines (random c0, c1) on random curves; report max/min lift density and the Weil envelope.
read("common.gp");
run(q, ncurves, nlines, seed) = {
  setrand(seed); my(t=ffgen(q^3,'t), one=t^0, mx=0, mn=1, tot=0);
  for(cv=1,ncurves, my(c=[0,random(t),random(t)]);
    for(i=1,nlines, my(c1=random(t), c0=random(t)); if(c1==0,next);
      my(L=linelift(c,c0,c1,q,one), d=L[1]/(q-L[2])); mx=max(mx,d); mn=min(mn,d); tot++));
  printf("n=3 q=%d: %d curves x %d random lines: lift density in [%.3f, %.3f]; Weil envelope for nondegenerate g: [%.3f, %.3f]\n",
     q, ncurves, nlines, mn*1., mx*1., 0.5-(8*sqrt(q)+9)/(2*(q-9)), 0.5+(8*sqrt(q)+9)/(2*(q-9)));
}
run(83, 4, 1500, 11); run(89, 4, 1500, 12);
