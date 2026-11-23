require 'open3'
mode=ARGV.fetch(0)
check,adaptive=case mode
when 'early' then [1,true]
when 'half' then [2,true]
when 'late' then [3,true]
when 'fixed' then [2,false]
when 'full' then [4,false]
else raise 'unexpected switch mode'
end
s,status=Open3.capture2('ruby','harness/g4-hermite-quotient.rb')
raise unless status.success?
key='for(int step=0;step<n;step++){'
raise unless s.scan(key).size==1
s=s.sub(key,<<~CPP)
  int used_poles=0;
  for(int step=0;step<n;step++){
    if(step==(n*#{check}+3)/4 && #{adaptive ? 'rank<n-1 && rank<step' : 'true'})break;
    used_poles++;
CPP
s=s.sub('stats[1]=pos;stats[2]=accepted;stats[3]=rank;} if(audit)', 'stats[1]=used_poles;stats[2]=accepted;stats[3]=rank;} if(audit)')
puts s
