s=File.read('variants/g3/kernel.cpp')
def change(s,old,new_text)
  raise "unexpected kernel source" unless s.scan(old).size==1
  s.sub(old){new_text}
end
a=s.index(' struct Bucket{int count=0;')
b=s.index(' if constexpr(Adaptive){',a)
raise unless a && b
s[a...b]=<<'CPP'
 // Family 0 fixes 4A. Family 1 fixes 3A and has a pole of order 7 at infinity.
 struct Bucket{short point[4];};
 std::array<std::array<Bucket,P>,2> buckets;
 std::array<std::array<unsigned char,P>,2> counts{};
 constexpr int words=(P+63)/64;
 std::array<unsigned long long,2*words> ready{};
 auto emit=[&](int arm,int param,int i){
   int size=counts[arm][param]++;
   assert(size<4);
   buckets[arm][param].point[size]=i;
   int threshold=3-(arm==0 && param==0);
   if(size+1==threshold)ready[arm*words+param/64]|=1ULL<<(param%64);
 };
 for(int i=0;i<(int)pts.size();i++){
   auto q=pts[i];int z=m(q.x-h);
   if(!z){emit(0,s4,i);if(dual)emit(1,s3,i);if(q.sg)++i;continue;}
   int t=s[0]+s1*z+s2*z2[z]+s3*z3[z];
   int center=m(-m(t)*inv4[z]),radius=m(q.y*inv4[z]);
   auto add=[&](int param,int point){
     emit(0,param,point);
     if(dual)emit(1,m(s3+z*param),point);
   };
   int plus=center+radius;add(plus>=P?plus-P:plus,i);
   if(q.sg){int minus=center-radius;add(minus<0?minus+P:minus,++i);}
 }
 for(int block=0;block<(dual?2:1)*words;block++)while(ready[block]){
   int bit=__builtin_ctzll(ready[block]);ready[block]&=ready[block]-1;
   int arm=block/words,b=64*(block%words)+bit;
   auto &bin=buckets[arm][b];int count=counts[arm][b];
   int distinct=count+(!arm && !b);
   int degree=distinct;int mult[4]={1,1,1,1};
   if(distinct==3)for(int i=0;i<count;i++){
     auto q=pts[bin.point[i]];int z=m(q.x-h);
     bool repeated;
     if(arm==1)repeated=z?(q.sg && m(q.slope-s1-2*s2*z-3*b*z2[z])==0):s4==0;
     else repeated=z?(q.sg && m(q.slope-s1-2*s2*z-3*s3*z2[z]-4*b*z3[z])==0):s5==0;
     if(repeated){degree++;mult[i]++;}
   }
   if(degree<4)continue;assert(degree==4);++rep.rows;
   std::fill(work.begin(),work.end(),0);work[pole.col]=arm?3:4;
   for(int i=0;i<count;i++){auto q=pts[bin.point[i]];if(q.sg)work[q.col]+=q.sg*mult[i];}
   if(validation && arm){
     // The monic residual quartic must equal the product of the four
     // reported factors, including repetitions, in z=x-h.
     std::array<int,8> f{};
     for(int j=7;j>=0;j--){
       for(int k=7;k>0;k--)f[k]=m(f[k]*h+f[k-1]);
       f[0]=m(f[0]*h+(j==7||j==3||j==0?1:j==1?C:0));
     }
     int line[]={s[0],s1,s2,b};
     for(int i=0;i<4;i++)for(int j=0;j<4;j++)f[i+j]=m(f[i+j]-line[i]*line[j]);
     std::array<int,5> product{};product[0]=1;
     int nroots=0;
     for(int i=0;i<count;i++){
       auto q=pts[bin.point[i]];int z=m(q.x-h);
       assert(m(((b*z+s2)*z%P+s1)*z+pole.y)==q.y);
       for(int k=0;k<mult[i];k++){
         for(int j=++nroots;j>0;j--)product[j]=m(product[j-1]-z*product[j]);
         product[0]=m(-z*product[0]);
       }
     }
     assert(nroots==4 && !f[0] && !f[1] && !f[2]);
     for(int i=0;i<=4;i++)assert(product[i]==f[i+3]);
   }
   if(validation){last_poles=sequence+1;used_rows[arm]++;}
CPP
s=s.gsub('for(int i=0;i<bin.count;i++)','for(int i=0;i<count;i++)')
puts s.lines.select{|line|line.start_with?('#include')}.uniq
puts 'namespace fused {static bool dual=true,validation=false;static int last_poles,used_rows[2];'
puts s.split('extern "C" int quartic').first
puts 'int go(int p,int*y,int*v,unsigned long long*inv,int*root,unsigned long long*xs,int*len){int a=-1;switch(p){'
[[73,22,73277],[127,12,2214137],[257,15,5561089],[383,35,19226639]].each do |p,c,l|
  puts "case #{p}:run<#{p},#{c},#{l},true,true,true>(nullptr,y,v,&a,true,inv,root,xs,len);break;"
end
puts '}return a;}}'
puts 'extern "C" int original_quartic(int,int*,int*,int,unsigned long long*,int*,unsigned long long*,int*);'
puts 'static int mode;extern "C" void pencil_mode(int m,bool audit){mode=m;fused::dual=m==2;fused::validation=audit;}'
dispatch = ARGV.include?('rule') ? '(mode==1 || (mode==2 && (p==127 || p==257)))' : 'mode'
puts "extern \"C\" int quartic(int p,int*y,int*v,int opt,unsigned long long*i,int*r,unsigned long long*x,int*n){return #{dispatch}?fused::go(p,y,v,i,r,x,n):original_quartic(p,y,v,opt,i,r,x,n);}"
if ARGV.first=='audit'
puts <<'CPP'
int main(){
 for(int p:{73,127,257,383}){
   int baseline[383]{},base=-1;
   int ell=p==73?73277:p==127?2214137:p==257?5561089:19226639;
   for(int m=0;m<3;m++){
     pencil_mode(m,true);fused::last_poles=0;fused::used_rows[0]=fused::used_rows[1]=0;
     int ys[383]{},v[383]{},rt[383]{},len=0;unsigned long long xs[383]{},inv[383]{};
     int a=quartic(p,ys,v,1,inv,rt,xs,&len);assert(a>=0);
     if(!m){base=a;std::copy_n(v,p,baseline);}
     assert(baseline[a] && v[base]);
     for(int x=0;x<p;x++)assert(1LL*v[x]*baseline[base]%ell==1LL*baseline[x]*v[base]%ell);
     printf("%d %d %d %d %d\n",p,m,fused::last_poles,fused::used_rows[0],fused::used_rows[1]);
   }
 }
}
CPP
end
