#include <iostream>
#include <vector>
#include <cstdint>
using U=uint64_t; using V=std::vector<U>;
U choose(unsigned n,unsigned k){U r=1;for(unsigned j=1;j<=k;j++)r=r*(n-j+1)/j;return r;}
V product(V a,V b,unsigned d){V c(d+1);for(unsigned i=0;i<=d;i++)for(unsigned j=0;j+i<=d;j++)c[i+j]+=a[i]*b[j];return c;}
int main(){std::cout<<"m,degree,variables,raw_columns,quotient_columns\n";for(unsigned m=3;m<=6;m++)for(unsigned d=2;d<=8;d++){V leaf(d+1,2),node(d+1,2),q(d+1);leaf[0]=node[0]=q[0]=1;node[1]=3;for(unsigned i=0;i<m+1;i++)q=product(q,leaf,d);for(unsigned i=0;i<m-2;i++)q=product(q,node,d);U sum=0;for(U v:q)sum+=v;unsigned n=5*m-4;std::cout<<m<<','<<d<<','<<n<<','<<choose(n+d,d)<<','<<sum<<'\n';}}
