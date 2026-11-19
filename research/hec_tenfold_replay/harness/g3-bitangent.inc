template<int P,int C,int L>
int bitangent(int *ys,int *v,unsigned long long *inverses,int *rt,unsigned long long *xs,int *len,bool audit,bool scan){
 Cubics<P> tab;auto m=tab.mod;
 struct Place{int x,y,s;};
 std::vector<Place> places;
 std::array<int,P> index{};index.fill(-1);*len=0;
 for(int x=0;x<P;x++){
   inverses[x]=tab.inv[x];int r=tab.square[x];rt[x]=r<0?-1:std::min(r,P-r);
 }
 for(int x=0;x<P;x++){
   int x2=m(x*x),x3=m(x2*x),x6=m(x3*x3);
   int y=rt[m(x6*x+x3+C*x+1)];if(y<0)continue;
   ys[x]=y;xs[(*len)++]=x;
   if(y){index[x]=places.size();places.push_back({x,y,m((7*x6+3*x2+C)*tab.inv[m(2*y)])});}
 }
 int n=places.size();fused::Basis basis;std::vector<fused::CompactRow> rows;std::vector<int> values(n);
 int tests=0,rank=0;
 for(int gap=1;gap<n;gap++)for(int i=0;i+gap<n;i++){
   int j=i+gap;auto A=places[i],B=places[j];
   int h=A.x,t=m(B.x-h),it2=m(tab.inv[t]*tab.inv[t]);
   for(int sign:{1,-1}){
     int k=m((sign*B.y-A.y-A.s*t)*it2);
     int d=m((sign*B.s-A.s-2*k*t)*it2),e=m(k-d*t);
     int b2=m(7*h-d*d+2*t);
     int b1=m(21*h*h-2*d*e+2*t*b2-t*t);
     int b0=m(35LL*h*h*h-2*d*A.s-e*e+2*t*b1-t*t*b2);
     if(audit){
       int f[8]{},w[]={A.y,A.s,e,d},factor[]={0,0,m(t*t),m(-2*t),1},q[]={b0,b1,b2,1};
       // Shift the curve polynomial into z, then check the full norm identity.
       int deg=-1;
       for(int c:{1,0,0,0,1,0,C,1}){
         for(int u=++deg;u>0;u--)f[u]=m(h*f[u]+f[u-1]);
         f[0]=m(h*f[0]+c);
       }
       for(int u=0;u<=7;u++){
         int coefficient=0;
         for(int l=0;l<=3;l++){
           if(u-l>=0 && u-l<=3)coefficient+=w[l]*w[u-l];
           if(u-l>=0 && u-l<=4)coefficient+=q[l]*factor[u-l];
         }
         assert(m(coefficient)==f[u]);
       }
     }
     tests++;typename Cubics<P>::Roots roots;
     if(scan || audit){
       int degree=3;std::array<int,4> q{b0,b1,b2,1};
       for(int z=0;z<P && degree;z++)while(degree){
         auto quotient=q;int rem=q[degree];
         for(int l=degree-1;l>=0;l--){quotient[l]=rem;rem=m(q[l]+z*rem);}
         if(rem)break;roots.add(z);q=quotient;degree--;
       }
       if(degree)roots.n=0;
     }
     if(!scan){auto found=tab.split(b2,b1,b0);
       if(audit){assert(found.n==roots.n);if(found.n==3){auto r=found.r;std::sort(r.begin(),r.end());assert(r==roots.r);}}
       roots=found;
     }
     if(roots.n!=3)continue;
     std::array<int,5> col{i,j,-1,-1,-1},weights{2,2*sign,0,0,0};
     for(int l=0;l<3;l++){int z=roots.r[l],x=m(h+z),y=m(A.y+z*(A.s+1LL*z*(e+z*d)));
       assert(ys[x]>=0 && (y==ys[x] || y==m(-ys[x])));
       col[l+2]=index[x];weights[l+2]=y==ys[x]?1:-1;
     }
     fused::CompactRow row;
     for(int a=0;a<5;a++)if(col[a]>=0){
       for(int b=a+1;b<5;b++)if(col[b]==col[a]){weights[a]+=weights[b];col[b]=-1;}
       row.add(col[a],weights[a]);
     }
     rows.push_back(row);
     if(int(rows.size())<n-1)continue;
     rank=fused::positive_rank<L>(rows,n,nullptr,values.data(),basis);
     if(rank==n-1){
       int anchor=-1;for(int a=0;a<n;a++){int x=places[a].x;v[x]=values[a];if(anchor<0 && v[x])anchor=x;}
       if(audit)printf("PAIR %d %d %d %d %d %zu %d\n",P,C,n,int(scan),tests,rows.size(),rank);
       return anchor;
     }
   }
 }
 if(audit)printf("EXHAUST %d %d %d %d %d %zu %d\n",P,C,n,int(scan),tests,rows.size(),rank);
 return -1;
}
