// Start with the complete four-family matrix on each deficient curve.
template<int P> void extend(int expected,int previous,int anchor,const int *prior){
 std::vector<int> xs,ys,positive;
 for(int x=0;x<P;x++){
   int cube=x*x*x%P, f=(cube*cube*cube+cube+C*x+1)%P;
   for(int y=0;y<=P/2;y++)if(y*y%P==f){
     xs.push_back(x);ys.push_back(y);
     if(y)positive.push_back(xs.size()-1);
     break;
   }
 }
 int n=positive.size();
 auto output=[&](const std::vector<int>& v,int a){
   printf("VALUES %d %d %d %d",P,C,L,xs[positive[a]]);
   for(int j=0;j<n;j++)printf(" %d",v[j]);
   puts("");
 };
 if(anchor>=0){
   std::vector<int> v;int a=-1;
   for(int j:positive){if(xs[j]==anchor)a=v.size();v.push_back(prior[xs[j]]);}
   assert(a>=0);output(v,a);return;
 }
 auto mod=[](long long a,int p){return int((a%p+p)%p);};
 std::vector<std::vector<int>> basis(n);
 int rank=0;
 auto insert=[&](std::vector<int> row){
   for(int j=0;j<n;j++)if(int v=mod(row[j],L)){
     if(basis[j].empty()){
       int scale=inverse(v,L);
       for(int k=j;k<n;k++)row[k]=mod(1LL*scale*row[k],L);
       basis[j]=std::move(row);rank++;return;
     }
     for(int k=j;k<n;k++)row[k]=mod(row[k]-1LL*v*basis[j][k],L);
   }
 };
 for(auto &r:history)insert(std::vector<int>(r.begin(),r.begin()+n));
 assert(rank==previous && n==expected);
 int pairs=0,tested=0,split=0;
 for(int aj=0;aj<n && rank<n-1;aj++){
   int h=xs[positive[aj]],y=ys[positive[aj]];
   std::array<int,10> f{};int deg=-1;
   for(int a:{1,0,0,0,0,0,1,0,C,1}){
     for(int j=++deg;j>0;j--)f[j]=mod(h*f[j]+f[j-1],P);
     f[0]=mod(h*f[0]+a,P);
   }
   int inv2y=inverse(2*y,P),s1=mod(f[1]*inv2y,P),s2=mod((f[2]-s1*s1)*inv2y,P);
   // B runs over distinct nonbranch x-coordinates, with canonical ordinate.
   for(int bj=0;bj<n && rank<n-1;bj++)if(aj!=bj){
     ++pairs;
     int t=mod(xs[positive[bj]]-h,P);
     int k=mod((ys[positive[bj]]-y-t*s1-t*t*s2)*1LL*inverse(t*t*t%P,P),P);
     for(int b=0;b<P && rank<n-1;b++){
       tested++;
       int w[]={y,s1,s2,mod(k-b*t,P),b};
       auto residual=f;
       for(int i=0;i<5;i++)for(int j=0;j<5;j++)residual[i+j]=mod(residual[i+j]-w[i]*w[j],P);
       int degree=9;
       auto divide=[&](int z){
         auto q=residual;
         for(int j=degree-1;j>=0;j--)q[j]=mod(residual[j+1]+(j+1<degree?z*q[j+1]:0),P);
         if(mod(residual[0]+z*q[0],P))return false;
         residual=q;--degree;return true;
       };
       for(int i=0;i<3;i++)assert(divide(0));
       assert(divide(t));
       std::vector<int> row(n);row[aj]=3;row[bj]=1;
       int column=0;
       for(size_t j=0;j<xs.size() && degree;j++){
         int z=mod(xs[j]-h,P),mult=0;
         while(degree && divide(z))mult++;
         if(mult){
           int value=0;for(int i=4;i>=0;i--)value=mod(value*z+w[i],P);
           assert(value==ys[j] || mod(-value,P)==ys[j]);
           if(ys[j])row[column]+=(value==ys[j]?mult:-mult);
         }
         column+=ys[j]!=0;
       }
       if(!degree){split++;insert(row);}
     }
   }
 }
 printf("EXT %d %d %d %d %d %d %d %d %d\n",P,C,L,n,previous,rank,pairs,tested,split);
 if(rank==n-1){
   std::vector<int> v(n);int free=-1;
   for(int j=n-1;j>=0;j--){
     if(basis[j].empty()){free=j;v[j]=1;continue;}
     for(int k=j+1;k<n;k++)v[j]=mod(v[j]-1LL*basis[j][k]*v[k],L);
   }
   assert(free>=0);output(v,free);
 }
}
