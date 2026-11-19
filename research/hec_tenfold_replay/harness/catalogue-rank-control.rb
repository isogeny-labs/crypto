g = Integer(ARGV.fetch(0))
s = File.read("variants/g#{g}/kernel.cpp")
def replace_once(s, a, b)
  raise "source mismatch: #{a}" unless s.scan(a).size == 1
  s.sub(a) { b }
end
puts '#include <array>', '#include <cassert>', '#include <chrono>', '#include <cstdio>'
puts 'static bool replay, capture; static int stop_row; static std::array<int,383> recorded;'
if g == 3
  key='rep.rank=(Mask?positive_rank<L>:ordered_rank<L>)(input,n,audit,values,retain?persistent:fresh);'
  s=replace_once(s,key,<<~CPP)
    if(replay){
      if(rep.rows==stop_row){
        std::copy_n(recorded.data(),n,values);
        rep.rank=n-1;
      }
    }else{
      #{key}
      if(capture && rep.rank==n-1){
        std::copy_n(values,n,recorded.data());
        stop_row=rep.rows;
      }
    }
  CPP
else
  key='std::array<long long,P> work{};std::copy_n(row.begin(),n,work.begin());'
  s=replace_once(s,key,'asm volatile("" : : "g"(&row) : "memory"); if(!replay){'+key)
  s=replace_once(s,'if(!audit && rank==n-1){','}else if(accepted==stop_row){rank=n-1;} if(!audit && rank==n-1){')
  s=replace_once(s,'std::array<int,P> kernel{};','std::array<int,P> kernel{}; if(replay){std::copy_n(recorded.data(),n,kernel.data());}else{')
  key='for(auto &q:pts) if(q.sg==0) ys[q.x]=0;'
  s=replace_once(s,key,'if(capture){stop_row=accepted;std::copy_n(kernel.data(),n,recorded.data());}} '+key)
end
puts s
call = g==3 ? 'quartic(p,y.data(),v.data(),1,inv.data(),rt.data(),xs.data(),&len)' : 'quartic(p,y.data(),v.data(),1,nullptr)'
puts <<~CPP
  int main(int argc,char**){
    for(int p: {#{g==3 ? '73,127,257,383' : '37,43,59,67'}}){
      std::array<int,383> expected_y{}, expected_v{};
      int expected_anchor=-1;
      for(int rep=-1;rep<128;++rep){
        for(int pass=0;pass<(rep<0?1:2);++pass){
          capture=rep<0;
          replay=rep>=0 && ((rep+pass+(argc>1))%2);
          std::array<int,383> y{},v{},rt{};
          std::array<unsigned long long,383> inv{},xs{};
          int len=0;
          const auto begin=std::chrono::steady_clock::now();
          int anchor=#{call};
          auto ns=std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now()-begin).count();
          if(rep<0){expected_anchor=anchor;expected_y=y;expected_v=v;}
          else{
            assert(anchor==expected_anchor && y==expected_y && v==expected_v);
            printf("%d %d %d %d %lld\\n",#{g},p,rep,int(replay),(long long)ns);
          }
        }
      }
    }
  }
CPP
