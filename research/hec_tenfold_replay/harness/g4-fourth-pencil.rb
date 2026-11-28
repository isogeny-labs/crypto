source=File.read("variants/g4/kernel.cpp")
def change(src, from, to)
  raise from unless src.scan(from).size==1
  src.sub(from){to}
end
source=change(source,'bool Direct=false,bool Cover=false>','bool Direct=false,bool Cover=false,int Mask=7>')
source=change(source,'add(0,a5);if(a5)','if constexpr(Mask&1){add(0,a5);}if constexpr(Mask&2){if(a5)')
source=change(source,'add(1,b);','add(1,b);}')
source=change(source,'if constexpr(true){if(!z)continue;}','if constexpr(!(Mask&4))continue; if(!z)continue;')
source=change(source,'arm++)for(int b=0;b<P;b++){','arm++)if(Mask&(1<<arm))for(int b=0;b<P;b++){')
source=change(source,'std::array<std::array<Bucket,P>,3>', 'std::array<std::array<Bucket,P>,4>')
source=change(source,'if constexpr(Mask&1){add(0,a5);}',<<'NEW')
if constexpr(Mask&8){
  // y = Taylor_3(h) + b (x-h)^4 has pole order nine at infinity.
  // Its fixed zero 4P leaves a residual divisor of degree five.
  int b=m(pole.s[4]+z*a5), mult=1;
  if(!z){
    while(mult<=5 && !pole.s[4+mult]) ++mult;
  }else if(q.sg && !m(a5+z*da5)){
    const int coeff[]={pole.s[0],pole.s[1],pole.s[2],pole.s[3],b};
    constexpr int choose[5][5]={{1},{1,1},{1,2,1},{1,3,3,1},{1,4,6,4,1}};
    for(mult=2;mult<=4;mult++){
      int val=0;
      for(int j=4;j>=mult;j--) val=m(val*z+coeff[j]*choose[j][mult]);
      if(m(q.s[mult]-val)) break;
    }
  }
  auto &bin=buckets[3][b];assert(mult<=5 && bin.count<5);
  bin.idx[bin.count]=i;bin.mult[bin.count++]=mult;bin.degree+=mult;
}
if constexpr(Mask&1){add(0,a5);}
NEW
source=change(source,'arm<3;arm++','arm<4;arm++')
source=change(source,'int deg=v.degree+inf+base,target=arm?6:5;', 'if(arm==3){inf=0;base=0;} int deg=v.degree+inf+base,target=(arm==3||arm==0)?5:6;')
source=change(source,'row[pole.col]=target+base;', 'row[pole.col]=arm==3?4:target+base;')
source=change(source,'if(audit){auto &ref=reference[P].at(accepted);', 'if(rowcheck){long long dot=0;for(auto &q:poles)dot+=1LL*row[q.col]*rowcheck[q.x];assert(dot%L==0);}if(audit){auto &ref=reference[P].at(accepted);')
if ARGV.first == 'verify'
  at='for(int arm=0;arm<4;arm++)'
  check=<<'CPP'
// Form f(h+z) - (Taylor_3(z) + b*z^4)^2, divide by z^4,
// then compare every rational root and multiplicity with the bucket.
for(int b=0;b<P;b++){
  std::array<int,10> norm{};
  for(int i=9;i>=0;i--){
    for(int j=9;j>=1;j--) norm[j]=m(norm[j]*pole.x+norm[j-1]);
    int coeff=(i==9 || i==3 || i==0)?1:(i==1?C:0);
    norm[0]=m(norm[0]*pole.x+coeff);
  }
  int line[]={pole.s[0],pole.s[1],pole.s[2],pole.s[3],b};
  for(int i=0;i<=4;i++)for(int j=0;j<=4;j++)norm[i+j]=m(norm[i+j]-line[i]*line[j]);
  for(int i=0;i<4;i++)assert(norm[i]==0);
  assert(norm[9]==1);
  std::array<int,6> residual{};
  std::copy(norm.begin()+4,norm.end(),residual.begin());
  const auto &bucket=buckets[3][b];
  int degree=5;
  for(int x=0;x<P;x++){
    int z=m(x-pole.x),multiplicity=0,reported=0,y=0;
    for(int j=4;j>=0;j--)y=m(y*z+line[j]);
    while(degree){
      int value=residual[degree];
      for(int i=degree-1;i>=0;i--)value=m(value*z+residual[i]);
      if(value)break;
      int carry=residual[degree];
      for(int i=degree-1;i>=0;i--){
        int next=m(residual[i]+carry*z);residual[i]=carry;carry=next;
      }
      assert(carry==0);--degree;++multiplicity;
    }
    for(int i=0;i<bucket.count;i++){
      const auto &point=pts[bucket.idx[i]];
      if(point.x==x){assert(point.y==y);reported+=bucket.mult[i];}
    }
    assert(reported==multiplicity);
  }
  assert(bucket.degree==5-degree);
  ++checked;split_count+=degree==0;
}
CPP
  source=change(source,at,check+at)
  source=change(source,'if(!audit && rank==n-1){','if(false){')
  puts 'static const int* rowcheck=nullptr;static int checked,split_count;'
  puts source.split('extern "C" int quartic').first
  puts 'int main(){int y[67]{},v[67]{},a=-1;'
  [[37,9,520447],[43,5,1020619],[59,4,1011001],[67,1,2804327]].each do |p,c,l|
    puts "checked=split_count=0;run<#{p},#{c},#{l},true,true,8>(false,y,v,a);printf(\"%d %d %d\\n\",#{p},checked,split_count);"
  end
  puts '}'
  exit
end
if ARGV.first == 'library'
  puts source.lines.select { |line| line.start_with?('#include') }.uniq
  puts 'namespace fourth_family { static const int* rowcheck=nullptr;'
  # The full driver uses a separate, unmodified translation unit as control.
  puts source.split('extern "C" int quartic').first
  puts 'int solve(int p,int* y,int* v,int* stats){int a=-1;switch(p){'
  [[37,9,520447],[43,5,1020619],[59,4,1011001],[67,1,2804327]].each do |p,c,l|
    puts "case #{p}:run<#{p},#{c},#{l},true,true,15>(false,y,v,a,stats);break;"
  end
  puts '};return a;}}'
  puts 'extern "C" int original_quartic(int,int*,int*,int,int*);'
  puts 'static bool enabled; extern "C" void select_fourth(int on){enabled=on;}'
  puts 'extern "C" int quartic(int p,int*y,int*v,int cover,int*s){return enabled?fourth_family::solve(p,y,v,s):original_quartic(p,y,v,cover,s);}'
  exit
end
puts 'static const int* rowcheck=nullptr;'
puts source
puts <<'CPP'
template<int P,int C,int L,int Mask>
void bench(int *y,int *v,int ref_anchor,int *ref_values,int trial){
  int S[4]{},a=-1;
  auto start=std::chrono::steady_clock::now();
  auto result=run<P,C,L,true,true,Mask>(false,y,v,a,S);
  auto ns=std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now()-start).count();
  if(a>=0){
    assert(ref_values[a]!=0 && v[ref_anchor]!=0);
    for(int x=0;x<P;x++)assert(1LL*v[x]*ref_values[ref_anchor]%L==1LL*ref_values[x]*v[ref_anchor]%L);
  }
  printf("%d %d %d %d %d %d %d %lld\n",P,Mask,trial,a>=0,result.second,S[0],result.first,(long long)ns);
}
template<int P,int C,int L>
void field(bool reverse){
  int ys[P]{},ref[P]{},a=-1;
  run<P,C,L,true,true>(false,ys,ref,a);
  assert(a>=0);
  for(int i=-1;i<60;i++)for(int j=0;j<15;j++){
    int y[P]{},v[P]{},arm=1+(15+i+(reverse?14-j:j))%15;rowcheck=i<0?ref:nullptr;
    switch(arm){
CPP
(1..15).each{|mask|puts "case #{mask}:bench<P,C,L,#{mask}>(y,v,a,ref,i);break;"}
puts <<'CPP'
    }
  }
  rowcheck=nullptr;
}
int main(int argc,char**){bool rev=argc>1;field<37,9,520447>(rev);field<43,5,1020619>(rev);field<59,4,1011001>(rev);field<67,1,2804327>(rev);}
CPP
