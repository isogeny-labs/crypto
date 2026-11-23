     // The residual quintic either has five distinct rational roots away
     // from A,B or has a repeated root or meets one of the fixed points.
     std::array<unsigned char,P> hits{};
     std::array<bool,P> candidate{};
     int s3=mod((f[3]-2*s1*s2)*inv2y,P);
     candidate[mod((k-s3)*inverse(t,P),P)]=true;
     int yb=ys[positive[bj]],xb=xs[positive[bj]],xb2=xb*xb%P,xb4=xb2*xb2%P;
     int tangent_b=mod((9*xb4*xb4+3*xb2+C)*inverse(2*yb,P),P);
     candidate[mod((tangent_b-s1-2*s2*t-3*k*t*t)*1LL*inverse(t*t*t%P,P),P)]=true;
     for(size_t qi=0;qi<xs.size();qi++){
       int z=mod(xs[qi]-h,P);
       if(!z || z==t)continue;
       int invz3=inverse(z*z*z%P,P),invzt=inverse(mod(z-t,P),P);
       for(int sign:{1,-1}){
         int yy=mod(sign*ys[qi],P);if(!yy && sign<0)continue;
         int u=mod((yy-y-s1*z-s2*z*z)*1LL*invz3,P);
         int bb=mod((u-k)*invzt,P);
         if(++hits[bb]>=5)candidate[bb]=true;
         if(yy){
           int x2=xs[qi]*xs[qi]%P,x4=x2*x2%P;
           int derivative=mod(s1+2*s2*z+3LL*u*z*z+1LL*bb*z*z*z,P);
           if(!mod(2*yy*derivative-9*x4*x4-3*x2-C,P))candidate[bb]=true;
         }
       }
     }
