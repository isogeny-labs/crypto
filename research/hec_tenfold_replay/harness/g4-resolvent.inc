// For X^4+p X²+q X+r, opposite quadratic factors give w=s² satisfying
// w((p+w)²-4r)=q². Splitting into four linear factors splits this cubic too.
template<int P> struct Quartics: Cubics<P> {
 using Cubics<P>::mod;using Cubics<P>::inv;using Cubics<P>::square;
 int split4(const int *q,int *roots)const{
   if(!q[4]){
     int scale=inv[q[3]];
     auto c=this->split(mod(q[2]*scale),mod(q[1]*scale),mod(q[0]*scale));
     if(c.n!=3)return 0;
     std::copy_n(c.r.begin(),3,roots);return 3;
   }
   int scale=inv[q[4]],h=mod(q[3]*scale%P*inv[4]);
   int hh=mod(h*h),a=mod(q[2]*scale-6*hh);
   int b=mod(q[1]*scale-2*h*a-4*h*hh);
   int c=mod(q[0]*scale-h*mod(b+h*(a+hh)));
   auto quadratic=[&](int u,int v,int offset){
     int s=square[mod(u*u-4*v)];if(s<0)return false;
     roots[offset]=mod((-u+s)*inv[2]-h);
     roots[offset+1]=mod((-u-s)*inv[2]-h);return true;
   };
   if(!b){
     int s=square[mod(a*a-4*c)];if(s<0)return 0;
     for(int i=0;i<2;i++){
       int z=square[mod((-a+(i?-s:s))*inv[2])];if(z<0)return 0;
       roots[2*i]=mod(z-h);roots[2*i+1]=mod(-z-h);
     }
   }else{
     auto cub=this->split(mod(2*a),mod(a*a-4*c),mod(-b*b));
     if(cub.n!=3)return 0;
     int w=cub.r[0],s=square[w];if(s<=0)return 0;
     int delta=mod(b*inv[s]);
     if(!quadratic(s,mod((a+w-delta)*inv[2]),0) ||
        !quadratic(-s,mod((a+w+delta)*inv[2]),2))return 0;
   }
   return 4;
 }
};
