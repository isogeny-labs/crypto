source=File.read("variants/g4/kernel.cpp")
def replace_once(source, old, new_text)
  raise "kernel source mismatch" unless source.scan(old).size==1
  source.sub(old){new_text}
end
puts '#include <unordered_map>', '#include <cstdint>'
puts 'static bool validate_places=false;'
puts 'struct Counts {int partial=0, joins=0, independent=0, ramified=0;};static Counts counts;'
source=replace_once(source,'bool Direct=false,bool Cover=false>','bool Direct=false,bool Cover=false,bool Large=false>')
source=replace_once(source,'const int n=poles.size();','struct Entry{int pole,b,len;short col[5],value[5];};std::array<short,P*P> slot{};std::vector<Entry> entries;if constexpr(Large)entries.reserve(2*P); const int n=poles.size();')
source=replace_once(source,'if(deg!=target)continue;', 'bool partial=Large && arm==0 && target-deg==2;if(deg!=target && !partial)continue;')
key=' if constexpr(false){'+ "\n" +'   seen[pole.col]'
insert=<<'CPP'
 if constexpr(Large){
   if(partial){
     ++counts.partial;
     // Leading coefficients of (f - line^2)/z^5 in z=x-h.
     int h=pole.x,s4=pole.s[4],s3=pole.s[3];
     int alpha,beta;
     if(b){
       int scale=m(-inv[b]*inv[b]);
       alpha=m((1-2*b*s4)*scale);
       beta=m((9*h-2*b*s3-s4*s4)*scale);
     }else{
       alpha=m(9*h-s4*s4);
       beta=m(36*h*h-2*s4*s3);
     }
     for(int i=0;i<v.count;i++){
       int root=m(pts[v.idx[i]].x-h);
       for(int k=0;k<v.mult[i];k++){
         alpha=m(alpha+root);
         beta=m(beta+alpha*root);
       }
     }
     // Lift the monic quadratic and the line ordinate into global x coordinates.
     int A=m(alpha-2*h),B=m(beta-alpha*h+h*h);
     if(validate_places){
       // Irreducibility and the curve equation are checked in F_p[x]/(x^2+A*x+B).
       assert(root[m(A*A-4*B)]<0);
       auto eval=[&](int degree,auto coefficient){
         int u=0,w=0;
         for(int k=degree;k>=0;k--){int next=m(u-alpha*w);u=m(coefficient(k)-beta*w);w=next;}
         return std::array<int,2>{u,w};
       };
       auto y=eval(5,[&](int k){return k==5?b:pole.s[k];});
       int u=y[0],w=y[1];
       // Compute f(h+z) by Horner evaluation at x=h+z.
       int f0=0,f1=0;
       for(int k=9;k>=0;k--){
         int next=m(f0+(h-alpha)*f1);
         f0=m(h*f0-beta*f1+(k==9||k==3||k==0?1:k==1?C:0));f1=next;
       }
       assert(m(u*u-beta*w*w)==f0 && m(2*u*w-alpha*w*w)==f1);
       counts.ramified+=!u&&!w;
     }
     int key=A*P+B;
     int index=slot[key]-1;
     if(index<0){
       Entry e{};e.pole=id;e.b=b;
       e.col[0]=pole.col;e.value[0]=5;e.len=1;
       for(int j=0;j<v.count;j++){
         auto q=pts[v.idx[j]];if(!q.sg)continue;
         e.col[e.len]=q.col;e.value[e.len++]=q.sg*v.mult[j];
       }
       entries.push_back(e);slot[key]=entries.size();
       continue;
     }
     // Each irreducible support has just two conjugate ordinates.
     // Compute orientations only after observing a repeated support.
     auto ordinate=[&](const auto &point,int slope){
       int h=point.x,al=m(A+2*h),be=m(B+A*h+h*h),u=slope,w=0;
       for(int k=4;k>=0;k--){int next=m(u-al*w);u=m(point.s[k]-be*w);w=next;}
       return w*P+m(u-h*w);
     };
     const auto &e=entries[index];
     int left=ordinate(pole,b),right=ordinate(poles[e.pole],e.b);
     int factor=left==right?1:-1;
     assert(m(left/P-factor*(right/P))==0 && m(left%P-factor*(right%P))==0);
     for(int j=0;j<e.len;j++)row[e.col[j]]-=factor*e.value[j];
     ++counts.joins;
   }
 }
CPP
source=replace_once(source,key,insert+key)
source=replace_once(source,'std::array<long long,P> work{};','int previous_rank=rank;std::array<long long,P> work{};')
source=replace_once(source,' if(!audit && rank==n-1){',' if constexpr(Large){if(partial && rank>previous_rank)++counts.independent;} if(!audit && rank==n-1){')
if ARGV.first=='library'
  puts source.lines.select{|line| line.start_with?('#include')}.uniq
  puts 'namespace quadratic_places {'
  puts source.split('extern "C" int quartic').first
  puts 'int solve(int p,int*y,int*v,int*stats){int a=-1;switch(p){'
  [[37,9,520447],[43,5,1020619],[59,4,1011001],[67,1,2804327]].each do |p,c,l|
    puts "case #{p}:run<#{p},#{c},#{l},true,true,true>(false,y,v,a,stats);break;"
  end
  puts '}return a;}}'
  puts 'extern "C" int original_quartic(int,int*,int*,int,int*);'
  puts 'static bool enabled;extern "C" void select_quadratics(int on){enabled=on;}'
  puts 'extern "C" int quartic(int p,int*y,int*v,int cover,int*s){return enabled?quadratic_places::solve(p,y,v,s):original_quartic(p,y,v,cover,s);}'
  exit
end
puts source.split('extern "C" int quartic').first
puts <<'CPP'
template<int P,int C,int L>
void compare(bool reverse){
  int ref[P]{},ys[P]{},base=-1;
  run<P,C,L,true,true>(false,ys,ref,base);
  assert(base>=0);
  for(int rep=-1;rep<128;rep++)for(int pass=0;pass<2;pass++){
    validate_places=(rep<0); bool extra=(2+rep+pass+reverse)%2;int y[P]{},v[P]{},anchor=-1,s[4]{};
    counts={};
    auto start=std::chrono::steady_clock::now();
    (extra?run<P,C,L,true,true,true>:run<P,C,L,true,true,false>)(false,y,v,anchor,s);
    auto ns=std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now()-start).count();
    assert(anchor>=0 && v[base] && ref[anchor]);
    for(int x=0;x<P;x++)assert(y[x]==ys[x] && 1LL*v[x]*ref[base]%L==1LL*ref[x]*v[base]%L);
    printf("%d %d %d %d %d %d %d %d %d %lld\n",P,rep,extra,s[0],s[1],counts.partial,counts.joins,counts.ramified,counts.independent,(long long)ns);
  }
}
int main(int n,char**){compare<37,9,520447>(n>1);compare<43,5,1020619>(n>1);compare<59,4,1011001>(n>1);compare<67,1,2804327>(n>1);}
CPP
