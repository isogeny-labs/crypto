require 'open3'
rows=File.readlines(ARGV.fetch(1)).map{|s|s.split.map(&:to_i)}
def generate(*command)
  out,status=Open3.capture2(*command)
  raise command.inspect unless status.success?
  out
end
if ARGV.first=='driver'
  s=File.read('main.rs')
  script=rows.map{|p,c,l,_| "print(subst(hyperellcharpoly(Mod(1,#{p})*(x^9+x^3+#{c}*x+1)),x,1)/#{l})"}.join("\n")
  out,status=Open3.capture2('gp','-q',stdin_data:script)
  cofactors=out.lines.map{|line|Integer(line)}
  raise unless status.success? && cofactors.size==rows.size && cofactors.all?(&:positive?)
  by_curve=rows.map{|r|r[2]}.uniq.size!=rows.size
  selector=by_curve ? '(c.fp.p,c.f.c[1],l)' : 'l'
  cases=rows.zip(cofactors).map{|(p,c,l,_),h| by_curve ? "(#{p},#{c},#{l})=>#{h}" : "#{l}=>#{h}"}.join(',')
  key='match c.fp.p {37|43=>4,59=>12,67=>8,'
  verifier=File.read('../../sources/f28de4e9af-verify.rs')
  raise unless [s,verifier].all?{|src|src.scan(key).size==1}
  s=s.sub(key,"match #{selector} {#{cases},")
  s=s.sub('#[path="../../sources/f28de4e9af-verify.rs"] mod verify;'){"mod verify {#{verifier.sub(key,"match #{selector} {#{cases},")}}"}
  a=s.index('    for (g,p) in')
  b=s.index('        let mut rng',a)
  s[a...b]=<<~RS
    extern "C"{fn select_curve(p:i32,c:i32)->i32;fn select_mode(mode:i32);}
    for (p,c0,ell,cofactor) in [#{rows.map{|r|'('+r.join(',')+')'}.join(',')}]{
      let g=4;
      let curve=hec_native_ic::instance::thread_curve(p,c0,g).unwrap();
      let c=&curve;
      let generator=c.mul(&c.place(0,1),cofactor);
      assert!(validate::eligible(c,&generator,&generator,ell));
      let available=unsafe{select_curve(p as i32,c0 as i32)};
      println!("CAT {p} {c0} {available}");
  RS
  s=s.gsub('&inst.d1','&generator')
  s=s.sub('let answer=candidate::solve(c,&d,&target,ell,i==2,audit);','unsafe{select_mode((i-1) as i32);}let answer=candidate::solve(c,&d,&target,ell,true,audit);')
  s=s.sub('for arm in 0..3 {assert_eq!', 'for arm in 0..4 {if arm>0 && available & (1<<(arm-1))==0 {continue;}assert_eq!')
  s=s.sub('for pass in 0..3','for pass in 0..4').sub('2-pass','3-pass').sub('else{pass})%3;', 'else{pass})%4;if i>0 && available & (1<<(i-1))==0{continue;}')
  s=s.sub('P {g} {p} {case}', 'P {p} {c0} {case}')
  puts s
  exit
end
packed=generate('ruby','harness/g4-degree-screen.rb','packed').split('static bool screen;').first
fourth=generate('ruby','harness/g4-fourth-pencil.rb','library').split('namespace fourth_family').last.split('int solve(int p').first
source=packed+'namespace fourth_family'+fourth+'}'
puts source.gsub('if(audit)assert(accepted', 'if(stats){stats[0]=n;stats[1]=pos;stats[2]=accepted;stats[3]=rank;} if(audit)assert(accepted')
puts 'static int coefficient,mode; extern "C" void select_mode(int i){mode=i;}'
puts 'extern "C" int quartic(int p,int*y,int*v,int,int*stats){int a=-1;switch(p*128+coefficient){'
rows.each do |p,c,l,_|
  puts "case #{p*128+c}:"
  %w[original screened fourth_family].each_with_index do |ns,i|
    params=[p,c,l,true,true];params<<15 if i==2
    puts "if(mode==#{i})#{ns}::run<#{params.join(',')}>(false,y,v,a,stats);"
  end
  puts 'break;'
end
puts '}return a;}'
puts 'int modulus(int p,int c){switch(p*128+c){'+rows.map{|p,c,l,_|"case #{128*p+c}:return #{l};"}.join+'default:abort();}}'
puts <<'CPP'
 extern "C" int select_curve(int p,int c){
   coefficient=c;int available=0,ref[67]{},base=-1;
   for(mode=0;mode<3;mode++){
     int y[67]{},v[67]{},stats[4]{};
     int a=quartic(p,y,v,0,stats);
     printf("RANK %d %d %d %d %d %d %d\n",p,c,mode,a,stats[0],stats[2],stats[3]);
     if(a<0)continue;
     available|=1<<mode;
     if(base<0){base=a;std::copy_n(v,p,ref);}
     auto l=modulus(p,c);for(int j=0;j<p;j++)assert(1LL*v[j]*ref[a]%l==1LL*ref[j]*v[a]%l);
     assert(v[a] && ref[a]);
     if(mode==1){assert(a==base);for(int j=0;j<p;j++)assert(v[j]==ref[j]);}
   }
   return available;
 }
CPP
puts 'int main(){'+rows.map{|p,c,_,_|"select_curve(#{p},#{c});"}.join+'}' if ARGV.first=='audit'
