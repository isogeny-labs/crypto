# Generate a sparse scan of pencil parameters with enough rational intersection degree.
s=File.read('variants/g4/kernel.cpp')
def replace(s,a,b)
  raise "source mismatch: #{a}" unless s.scan(a).size==1
  s.sub(a){b}
end
if ARGV.include?('packed')
  s=replace(s,'std::array<std::array<Bucket,P>,3> buckets;',<<'CPP')
 std::array<std::array<Bucket,P>,3> buckets;
 constexpr int padded=(P+7)/8*8;
 std::array<std::array<unsigned char,padded>,3> degree{};
 degree[0].fill(1);degree[0][0]=2;
 degree[1].fill(1);degree[1][0]=pole.s[5]?2:3;
 degree[2].fill(fixedOrder+(!pole.s[5] && !pole.s[6]));
 auto reset=[&](int b){
   int infinity=m(pole.s[6]+b*pole.s[5])?0:(b?1:pole.s[5]?2:3);
   degree[2][b]=fixedOrder+infinity+(b==exception?extra:0);
 };
 reset(0);if(exception>=0)reset(exception);
 if(pole.s[5])reset(m(-pole.s[6]*inv[pole.s[5]]));
 for(int b=P;b<padded;b++)degree[2][b]=0;
CPP
  s=replace(s,'bucket.degree+=order;', 'degree[arm][b]+=order;')
  s=replace(s,'for(int arm=0;arm<3;arm++)for(int b=0;b<P;b++){',<<'CPP')
 for(int arm=0;arm<3;arm++)for(int block=0;block<padded;block+=8){
 unsigned long long lanes;
 __builtin_memcpy(&lanes,degree[arm].data()+block,8);
 #if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
 lanes=__builtin_bswap64(lanes);
 #endif
 auto mask=(lanes+0x7a7a7a7a7a7a7a7aULL)&0x8080808080808080ULL;
 while(mask){int b=block+__builtin_ctzll(mask)/8;mask&=mask-1;
CPP
  a=s.index('int inf=arm==2');b=s.index(' Row row{};',a)
  s[a...b]="int base=arm==2?fixedOrder+(b==exception?extra:0):0,target=arm?6:5; assert(degree[arm][b]==6);\n"
  s=replace(s,'if(audit)assert(accepted', '} if(audit)assert(accepted')
else
s=replace(s,'std::array<std::array<Bucket,P>,3> buckets;',<<'CPP')
 std::array<std::array<Bucket,P>,3> buckets;
 constexpr int words=(P+63)/64;
 std::array<std::array<unsigned long long,words>,3> ready{};
 auto mark=[&](int arm,int b){ready[arm][b/64]|=1ULL<<(b%64);};
 // b=0 can gain degree at infinity. In the third family, so can K=0.
 // The exceptional parameter can have extra multiplicity at the pole.
 for(int arm=0;arm<3;arm++)mark(arm,0);
 if(exception>=0)mark(2,exception);
 if(pole.s[5])mark(2,m(-pole.s[6]*inv[pole.s[5]]));
 const int third_min=6-fixedOrder-(!pole.s[5] && !pole.s[6]); if(third_min<=0)for(int b=0;b<P;b++)mark(2,b);
CPP
s=replace(s,'bucket.degree+=order;','bucket.degree+=order; if(bucket.degree>=(arm==2?third_min:5))mark(arm,b);')
s=replace(s,'for(int arm=0;arm<3;arm++)for(int b=0;b<P;b++){',<<'CPP')
 for(int arm=0;arm<3;arm++)for(int word=0;word<words;word++)
 for(auto mask=ready[arm][word];mask;mask&=mask-1){
 int b=64*word+__builtin_ctzll(mask);
CPP
end
s=replace(s,'++pos;auto &v=', 'pos=(3*step+arm)*P+b+1;auto &v=')
orig=File.read('variants/g4/kernel.cpp')
puts orig.lines.grep(/^#include/).uniq
[orig,s].zip(%w[original screened]).each do |source,ns|
  puts "namespace #{ns} {"
  # In the independent exhaustive audit, collect every row before elimination.
  if ARGV.include?('audit')
    puts 'static int visits[68]{};'
    source=replace(source,'auto &v=buckets[arm][b];','++visits[P];auto &v=buckets[arm][b];')
    source=replace(source,'if(audit){auto &ref=reference[P].at(accepted);assert(ref.pos==pos && ref.row==row);}', 'if(audit)reference[P].push_back({pos,row});')
    source=replace(source,'if(audit)assert(accepted==(int)reference[P].size());','')
  end
  puts source.split('extern "C" int quartic').first
  puts '}'
end
puts 'static bool screen;extern "C" void select_screen(int on){screen=on;}'
puts 'extern "C" int quartic(int p,int*y,int*v,int,int*stats){int a=-1;switch(p){'
fixtures=[[37,9,520447],[43,5,1020619],[59,4,1011001],[67,1,2804327]]
fixtures.each do |p,c,l|
  puts "case #{p}: (screen?screened::run<#{p},#{c},#{l},true,true>:original::run<#{p},#{c},#{l},true,true>)(false,y,v,a,stats);break;"
end
puts '}return a;}'
if ARGV.include?('audit')
  puts 'int main(){'
  fixtures.each do |p,c,l|
    puts "{ int a=-1,y[67]{},v[67]{};"
    %w[original screened].each{|ns|puts "#{ns}::run<#{p},#{c},#{l},true,true>(true,y,v,a);"}
    puts "auto &left=original::reference[#{p}];auto &right=screened::reference[#{p}];assert(left.size()==right.size());for(unsigned j=0;j<left.size();j++){assert(left[j].pos==right[j].pos && left[j].row==right[j].row);}printf(\"#{p} %zu %d %d\\n\",left.size(),original::visits[#{p}],screened::visits[#{p}]);}"
  end
  puts '}'
end
