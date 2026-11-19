     for(int sign:{1,-1}){
       if(rank==n-1)break;
       int xb=xs[positive[bj]],yb=mod(sign*ys[positive[bj]],P);
       int t=mod(xb-h,P),inv3=inverse(t*t*t%P,P);
       int k=mod((yb-y-s1*t-s2*t*t)*1LL*inv3,P);
       int x2=xb*xb%P,x4=x2*x2%P;
       int dy=mod((9*x4*x4+3*x2+C)*inverse(2*yb%P,P),P);
       int b=mod((dy-s1-2*s2*t-3*k*t*t)*1LL*inv3,P);
