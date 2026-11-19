// Normalize a depressed cubic Y³+aY+b by Y=(b/a)Z when ab is nonzero.
// Then Z³+u(Z+1)=0, u=a³/b². Z=-1 is never a root.
template<int P> struct Cubics {
 struct Roots {int n=0;std::array<int,3> r{};void add(int z){assert(n<3);r[n++]=z;}};
 std::array<int,P> inv{},square{};
 std::array<Roots,P> generic{},cube{};
 static int mod(long long x){return int((x%P+P)%P);}
 Cubics(){
   square.fill(-1);inv[1]=1;
   for(int j=2;j<P;j++)inv[j]=P-P/j*inv[P%j]%P;
   for(int z=0;z<P;z++){
     int zz=mod(z*z),zzz=mod(zz*z);
     square[zz]=z;
     cube[zzz].add(z);
     if(z==P-1)continue;
     int u=mod(-zzz*inv[z+1]);generic[u].add(z);
     if(mod(3*zz+u)==0){generic[u].add(z);if(!z)generic[u].add(z);}
   }
 }
 Roots split(int b2,int b1,int b0)const{
   int shift=mod(b2*inv[3]),a=mod(b1-3*shift*shift);
   int b=mod(b0+2LL*shift*shift*shift-b1*shift);
   Roots roots;
   if(!b){
     int s=square[mod(-a)];if(s<0)return roots;
     for(int z:{0,s,-s})roots.add(mod(z-shift));
   }else if(!a){
     auto bucket=cube[mod(-b)];if(bucket.n!=3)return roots;
     for(int z:bucket.r)roots.add(mod(z-shift));
   }else{
     int scale=mod(b*inv[a]),u=mod(1LL*a*inv[scale]*inv[scale]);
     auto bucket=generic[u];if(bucket.n!=3)return roots;
     for(int z:bucket.r)roots.add(mod(scale*z-shift));
   }
   return roots;
 }
};
