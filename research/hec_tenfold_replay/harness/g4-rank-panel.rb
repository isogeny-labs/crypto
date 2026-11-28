require 'open3'
s,status=Open3.capture2('ruby','harness/g4-fourth-pencil.rb','library')
raise unless status.success?
s=s.split('int solve(int p,int* y').first
def swap(s,a,b)
  raise a unless s.scan(a).size==1
  s.sub(a){b}
end
s=swap(s,'template<int P,int C,int L,','static int C,L; template<int P,')
s=swap(s,'std::array<std::array<Bucket,P>,4>','std::array<std::array<Bucket,P>,5>')
s=swap(s,'if constexpr(Mask&8){',<<'CPP')
 if constexpr(Mask&16){
   // Subtract the Taylor polynomial of degree two and a free cubic term.
   int b=m(pole.s[3]+z*m(pole.s[4]+z*a5));
   int mult=1;
   if(!z){while(mult<=6 && !pole.s[3+mult])++mult;}
   else if(q.sg){
     int slope=m(pole.s[1]+2*pole.s[2]*z+3*b*m(z*z));
     if(q.s[1]==slope){
       mult=2;
       if(q.s[2]==m(pole.s[2]+3*b*z)){
         mult=3;
         if(q.s[3]==b){mult=4;while(mult<=6 && !q.s[mult])++mult;}
       }
     }
   }
   auto &bin=buckets[4][b];assert(mult<=6 && bin.count<6);
   bin.idx[bin.count]=i;bin.mult[bin.count++]=mult;bin.degree+=mult;
 }
 if constexpr(Mask&8){
CPP
s=swap(s,'arm<4;arm++','arm<5;arm++')
s=swap(s,'if(arm==3){inf=0;base=0;}','if(arm>=3){inf=0;base=0;}')
s=swap(s,'arm==3?4:target+base','arm==4?3:arm==3?4:target+base')
s=swap(s,'if(audit)assert(accepted','if(stats){stats[0]=n;stats[1]=pos;stats[2]=accepted;stats[3]=rank;} if(audit)assert(accepted')
if ARGV.include?('extend')
  s=swap(s,'static int C,L;', 'static std::vector<Row> history;static bool record;static int C,L;')
  s=swap(s,'if(rowcheck){','if(record)history.push_back(row);if(rowcheck){')
end
puts s
if ARGV.include?('extend')
  ext=File.read('harness/g4-two-point.inc')
  if ARGV.include?('screen')
    key='     for(int b=0;b<P && rank<n-1;b++){'
    ext=swap(ext,key,File.read('harness/g4-two-point-screen.inc')+key)
    ext=swap(ext,'       tested++;','       if(!candidate[b])continue;tested++;')
  end
  if ARGV.include?('tangent')
    a=ext.index('     int t=mod(xs[positive[bj]]-h,P);')
    b=ext.index('       tested++;',a)
    ext[a...b]=File.read('harness/g4-tangent.inc')
    ext=swap(ext,'assert(divide(t));','assert(divide(t));assert(divide(t));')
    ext=swap(ext,'row[bj]=1;', 'row[bj]=2*sign;')
  end
  if ARGV.include?('osculate')
    ext=swap(ext,'std::array<int,10> f{};', 'std::array<int,11> f{};')
    ext=swap(ext,'int w[]={y,s1,s2,mod(k-b*t,P),b};',File.read('harness/g4-osculating.inc'))
    ext=swap(ext,'for(int i=0;i<5;i++)for(int j=0;j<5;j++)','for(int i=0;i<6;i++)for(int j=0;j<6;j++)')
    ext=swap(ext,'int degree=9;', 'int degree=e?10:9;')
    ext=swap(ext,'assert(divide(t));assert(divide(t));','for(int j=0;j<3;j++)assert(divide(t));')
    ext=swap(ext,'row[bj]=2*sign;', 'row[bj]=3*sign;')
    ext=swap(ext,'for(int i=4;i>=0;i--)','for(int i=5;i>=0;i--)')
    # Interchanging the two triple points yields the same row up to conjugation.
    ext=swap(ext,'if(aj!=bj){','if(aj<bj){')
  end
  puts ext
end
panel=<<'CPP'
 template<int P> void panel(){
   int baseline[67]{},base=-1;
   for(int arm:{7,15,31}){
     int a=-1,y[67]{},v[67]{},stats[4]{};
     (arm==7?run<P,true,true,7>:arm==15?run<P,true,true,15>:run<P,true,true,31>)(false,y,v,a,stats);
     int n=0;
     for(int x=0;x<P;x++){
       int x3=x*x*x%P,fx=(x3*x3*x3+x3+C*x+1)%P;
       for(int h=1;h<=P/2;h++)if(h*h%P==fx){++n;break;}
     }
     if(a>=0){
       if(base<0){base=a;std::copy_n(v,P,baseline);}
       assert(v[a] && baseline[a] && stats[3]==n-1);
       for(int j=0;j<P;j++)assert(1LL*v[j]*baseline[a]%L==1LL*baseline[j]*v[a]%L);
     }
     printf("%d %d %d %d %d %d %d %d %d\n",P,C,L,arm,n,stats[0],stats[2],stats[3],a);
   }
 }
 }
 int main(){
   int p,c,l;
   while(scanf("%d%d%d",&p,&c,&l)==3){
     fourth_family::C=c;fourth_family::L=l;
     switch(p){
CPP
if ARGV.include?('extend')
  panel=panel.sub('for(int arm:{7,15,31}){','for(int arm:{15}){history.clear();record=true;')
  panel=panel.sub('if(a>=0){','extend<P>(n,stats[3],a,v);if(a>=0){')
end
puts panel
[31,37,41,43,47,53,59,61,67].each{|p|puts "case #{p}:fourth_family::panel<#{p}>();break;"}
puts 'default:abort();}}}'
