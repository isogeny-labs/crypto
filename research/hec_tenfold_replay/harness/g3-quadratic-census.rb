s=File.read('variants/g3/kernel.cpp')
def insert_before(s, key, code)
  raise "source mismatch" unless s.scan(key).size==1
  s.sub(key){code+key}
end
key='if(distinct<3)continue;'
s=insert_before(s,key,<<'CPP')
{
  // Norm in z=x-h, after removing the four fixed copies of the pole.
  std::array<int,9> f{};
  for(int j=7;j>=0;j--){
    for(int k=8;k>0;k--)f[k]=m(f[k]*h+f[k-1]);
    f[0]=m(f[0]*h+(j==7||j==3||j==0?1:j==1?C:0));
  }
  int line[]={pole.y,s1,s2,s3,b};
  for(int i=0;i<5;i++)for(int j=0;j<5;j++)f[i+j]=m(f[i+j]-line[i]*line[j]);
  for(int j=0;j<4;j++)assert(f[j]==0);
  int deg=b?4:3;
  std::array<int,5> rem{};
  for(int j=0;j<=deg;j++)rem[j]=m(f[j+4]*inv[f[deg+4]]);
  std::vector<std::pair<int,int>> rational{{h,4}};
  for(int i=0;i<bin.count;i++){
    auto q=pts[bin.point[i]];
    int z=m(q.x-h),mult=0;
    while(deg){
      int val=rem[deg];
      for(int j=deg-1;j>=0;j--)val=m(val*z+rem[j]);
      if(val)break;
      int carry=rem[deg];
      for(int j=deg-1;j>=0;j--){int next=m(rem[j]+carry*z);rem[j]=carry;carry=next;}
      assert(!carry);--deg;++mult;
    }
    assert(mult>0);
    if(q.sg)rational.emplace_back(q.x,q.sg*mult);
  }
  std::vector<std::pair<int,int>> quadratic;
  auto add_place=[&](int a,int d){
    assert(root[m(a*a-4*d)]<0);
    int u=b,w=0;
    for(int j=3;j>=0;j--){int next=m(u-a*w);u=m(line[j]-d*w);w=next;}
    int A=m(a-2*h),B=m(d-a*h+h*h),v0=m(u-h*w);
    int f0=0,f1=0;
    for(int j=7;j>=0;j--){int next=m(f0-A*f1);f0=m((j==7||j==3||j==0?1:j==1?C:0)-B*f1);f1=next;}
    assert(m(v0*v0-B*w*w)==f0 && m(2*v0*w-A*w*w)==f1);
    int ordinate=w*P+v0,neg=m(-w)*P+m(-v0);
    quadratic.emplace_back(P+A*P+B,ordinate==0?0:ordinate<neg?1:-1);
  };
  if(deg==2)add_place(rem[1],rem[0]);
  if(deg==4){
    // (z^2+a*z+d)(z^2+c*z+e). The constant and linear
    // coefficients determine e and a for each d, except when d=e.
    bool found=false;
    for(int d=1;d<P && !found;d++){
      int e=m(rem[0]*inv[d]);
      auto try_a=[&](int a){
        int c=m(rem[3]-a);
        if(m(a*c+d+e)!=rem[2] || m(a*e+c*d)!=rem[1])return false;
        assert(m(d*e)==rem[0]);
        add_place(a,d);add_place(c,e);return true;
      };
      if(d!=e){found=try_a(m(m(rem[1]-d*rem[3])*inv[m(e-d)]));}
      else if(rem[1]==m(d*rem[3])){
        int r=root[m(rem[3]*rem[3]-4*(rem[2]-2*d))];
        if(r>=0)found=try_a(m((rem[3]+r)*inv[2]));
      }
    }
  }
  if(!quadratic.empty()){
    printf("Q %d %d %d %zu",P,rep.fibers,sequence+1,quadratic.size());
    for(auto [x,k]:rational)printf(" %d %d",x,k);
    for(auto [x,k]:quadratic)printf(" %d %d",x,k);
    puts("");
  }
}
CPP
s=insert_before(s,'   if(rep.rows>=n-1)',<<'CPP')
printf("F %d %d %d 0",P,rep.fibers,sequence+1);
for(int j=0;j<row.size;j++){auto e=row.terms[j];printf(" %d %d",poles[e.col].x,e.value);}
puts("");
CPP
puts s
puts <<'CPP'
int main(){
  for(int p:{73,127,257,383}){
    int y[383]{},v[383]{},rt[383]{},len=0;
    unsigned long long inv[383]{},xs[383]{};
    int anchor=quartic(p,y,v,1,inv,rt,xs,&len);
    assert(anchor>=0);
    printf("V %d",p);
    for(int j=0;j<len;j++){int x=xs[j];if(y[x])printf(" %d %d",x,v[x]);}puts("");
  }
}
CPP
