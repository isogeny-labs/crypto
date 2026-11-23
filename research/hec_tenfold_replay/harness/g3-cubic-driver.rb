s=File.read("main.rs")
def change(s,a,b)
  raise "driver source mismatch" unless s.scan(a).size==1
  s.sub(a){b}
end
s=change(s,"let answer=candidate::solve(c,&d,&target,ell,i==2,audit);",%q{extern "C" {fn pencil_mode(m:i32,audit:bool);}unsafe{pencil_mode((i-1) as i32,audit);}let answer=candidate::solve(c,&d,&target,ell,true,audit);})
s=change(s,"for arm in 0..3","for arm in 0..4")
s=change(s,"for pass in 0..3","for pass in 0..4")
if ARGV.first=='crossover'
  s=change(s,"if i==0 {run(0,4,0,audit,None)} else {","{")
  s=change(s,"pencil_mode((i-1) as i32,audit)","pencil_mode(if i==0 || i==3 {0} else {2},audit)")
  puts change(s,"(case+rep+if std::env::args().len()>1 {2-pass}else{pass})%3","if ((case+rep+std::env::args().len())%2)==0 {pass} else {(pass+2)%4}")
else
  puts change(s,"{2-pass}else{pass})%3","{3-pass}else{pass})%4")
end
