require 'open3'
curves=ARGV.fetch(0)
s,ok=Open3.capture2('ruby','harness/g3-transfer-kernels.rb',curves)
raise unless ok.success?
s=s.sub('namespace fused {','namespace fused {static int strategy;')
start=s.index("template<int L>\nint positive_rank",s.index('namespace fused {'))
finish=s.index('template<int P,int C',start)
raise unless start && finish
old=s[start...finish]
new_rank=old.sub('template<int L>','template<int L,int Strategy>').sub('int positive_rank','int exchange_rank')
if ARGV.include?('mask')
body_start=new_rank.index('    const auto &row=rows[basis.consumed')
body_end=new_rank.index('  if(rank==n-1)',body_start)
raise unless body_start && body_end
new_rank[body_start...body_end]=<<'CPP'
    const auto &row=rows[basis.consumed<basis.perm.size()?basis.perm[basis.consumed]:basis.consumed];
    auto &links=basis.pivot_support;
    if(links.empty())links.resize(n);
    std::fill(work.begin(),work.end(),0);
    std::array<unsigned long long,4> live{};
    int top=0;
    for(int i=0;i<row.size;i++){
      auto t=row.terms[i];int j=index[t.col];
      live[j/64]|=1ULL<<(j%64);top=std::max(top,j);
      work[j]=t.value<0?t.value+L:t.value;
    }
    auto process=[&](){
      for(int block=top/64;block>=0;--block)while(live[block]){
        int j=64*block+63-__builtin_clzll(live[block]);
        live[block]&=~(1ULL<<(j%64));
        int v=work[j]%L;if(!v)continue;
        auto previous=piv[j];
        bool replace=false;
        int length=previous.end-previous.first;
        if constexpr(Strategy==2){
          if(previous.end>=0 && length>=4){
            int bound=0;
            for(int b=0;b<=block;b++)bound+=__builtin_popcountll(live[b]);
            replace=bound<length;
          }
        }
        auto old_mask=links[j];
        if(previous.end<0 || replace){
          int scale=inverse(v,L);
          auto &span=piv[j];span.first=entries.size();
          for(int b=0;b<=block;b++){
            auto bits=live[b];unsigned long long mask=0;
            while(bits){
              int k=__builtin_ctzll(bits);bits&=bits-1;
              int col=b*64+k,a=(work[col]%L)*scale%L;
              if(a){entries.push_back({col,L-a});mask|=1ULL<<k;}
            }
            links[j][b]=mask;
          }
          span.end=entries.size();
          if(previous.end<0){++rank;return;}
          TRACK_EXCHANGE
        }
        for(int i=previous.first;i<previous.end;i++){
          auto t=entries[i];work[t.col]+=1ULL*v*t.value;
        }
        for(int b=0;b<=block;b++)live[b]|=old_mask[b];
      }
    };
    process();
  }
CPP
else
new_rank=new_rank.sub('      if(audit) audit->updates+=span.end-span.first;',<<'CPP')
      // Save the old equation for the current elimination. If we keep a
      // shorter new equation as pivot, this same difference spans the old
      // equation together with the replacement, preserving the row space.
      const auto previous=span;
      const int length=previous.end-previous.first;
      if(length>1 && ((j==top && row.size-1<length) ||
                       (Strategy==2 && j!=top && length>=8))){
        int nnz=0;
        for(int k=0;k<j && nnz<length;++k)nnz+=work[k]!=0 && work[k]%L!=0;
        if(nnz<length){
          const int scale=inverse(v,L);
          span.first=entries.size();
          for(int k=0;k<j;k++)if(work[k]){
            int a=(work[k]%L)*scale%L;
            if(a)entries.push_back({k,L-a});
          }
          span.end=entries.size();
          TRACK_EXCHANGE
        }
      }
      if(audit) audit->updates+=length;
CPP
new_rank=new_rank.sub('for(int i=span.first;i<span.end;i++){','for(int i=previous.first;i<previous.end;i++){')
raise unless new_rank.include?('previous.first;i<previous.end')
end
s.insert(finish,new_rank)
k=s.index('namespace fused {')
a,b=s[0...k],s[k..]
if ARGV.include?('mask')
  b=b.sub('struct Basis {','struct Basis {std::vector<std::array<unsigned long long,4>> pivot_support;')
end
key='(Mask?positive_rank<L>:ordered_rank<L>)'
raise unless b.scan(key).size==1
b=b.sub(key,'(Mask?(strategy==0?positive_rank<L>:(strategy==1?exchange_rank<L,1>:exchange_rank<L,2>)):ordered_rank<L>)')
s=a+b
s=s.sub('fused::dual=i==2;', 'fused::dual=true;fused::strategy=i;')
if ARGV.include?('audit')
  puts 'static unsigned long long updates, exchanges;'
  s=s.gsub('TRACK_EXCHANGE','++exchanges;')
  s=s.gsub('auto t=entries[i];work[t.col]+=1ULL*v*t.value;', '++updates;auto t=entries[i];work[t.col]+=1ULL*v*t.value;')
  s=s.sub('    int a=quartic','    updates=exchanges=0;int a=quartic')
  s=s.sub('    if(a>=0){','    printf("OPS %d %d %d %llu %llu\\n",p,c,mode,updates,exchanges);if(a>=0){')
else
  s=s.gsub('TRACK_EXCHANGE','')
end
# Arm zero in this comparison is the unchanged two-pencil elimination.
s=s.gsub('if(active){fused::','if(active>=0){fused::')
puts s
if ARGV.include?('audit')
  puts 'int main(){'
  File.foreach(curves){|line|p,c,l,_=line.split.map(&:to_i);puts "assert(audit_catalogue(#{p},#{c},#{l})==7);"}
  puts '}'
end
