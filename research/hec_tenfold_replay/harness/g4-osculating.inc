       int inv2yb=inverse(2*yb%P,P),x7=mod(1LL*x4*x2*xb,P);
       int second=mod((36*x7+3*xb-dy*dy)*inv2yb,P);
       int predicted=mod(s2+3LL*(k-b*t)*t+6LL*b*t*t,P);
       int e=mod((second-predicted)*inv3,P);
       // z^3(z-t)^2 preserves the earlier five interpolation conditions.
       int w[]={y,s1,s2,mod(k-b*t+1LL*e*t*t,P),mod(b-2*e*t,P),e};
