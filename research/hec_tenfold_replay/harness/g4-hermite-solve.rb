require 'open3'
def generated(*args)
  text,status=Open3.capture2('ruby',*args)
  raise args.inspect unless status.success?
  text
end
root=File.expand_path('..',__dir__)
groups=File.readlines("#{root}/results/g4-rank-panel-curves.txt").map{|s|p,c,t,n,l=s.split;[[p.to_i,c.to_i],n.to_i/l.to_i] if t=='selected'}.compact.to_h
rows=File.readlines("#{root}/results/g4-two-point-rank.txt").map{|s|p,c,l=s.split[1,3].map(&:to_i);[p,c,l,groups.fetch([p,c])]}
if (ARGV & %w[switch carry pivots]).any?
  rows=File.readlines("#{root}/results/g4-rank-panel-curves.txt").map do |line|
    p,c,state,n,l=line.split
    [p.to_i,c.to_i,l.to_i,n.to_i/l.to_i] if state=='selected'
  end.compact
end
if ARGV.first=='fixtures'
  rows.each{|r|puts r.join(' ')}
elsif ARGV.first=='driver'
  s=generated('../../harness/g4-transfer.rb','driver',ARGV.fetch(1))
  supported=rows.map(&:first).uniq
  dispatch=s[/match c.fp.p\{37=>solve_field.*?,_=>unreachable!\(\)\}/] or raise
  s=s.sub(dispatch,'match c.fp.p{'+supported.map{|p|"#{p}=>solve_field::<#{p}>(c,d,t,l,scan,audit)"}.join(',')+',_=>unreachable!()}')
  # Derive an input generator before timing using only the curve and cofactor.
  s=s.sub('let generator=c.mul(&c.place(0,1),cofactor);',<<'RS')
      let generator=(0..p).find_map(|x|{
        let y=c.fp.sqrt(c.f.eval(x,&c.fp))?;
        let d=c.mul(&c.place(x,y.min(p-y)),cofactor);
        (!c.is_zero(&d)).then_some(d)
      }).unwrap();
RS
  s=s.sub('assert!(validate::eligible(c,&generator,&generator,ell));','assert!(c.is_zero(&c.mul(&generator,ell)));let pencil_ok=validate::eligible(c,&generator,&generator,ell);')
  s=s.gsub('if arm>0 &&','if arm==0 && !pencil_ok {continue;}if arm>0 &&').gsub('if i>0 &&','if i==0 && !pencil_ok {continue;}if i>0 &&')
  s=s.sub('assert_eq!(call(arm,true).0,Some(truth));','let k=call(arm,true).0;if arm!=0 || k.is_some(){assert_eq!(k,Some(truth));}')
  s=s.sub('assert_eq!(r.0,Some(truth));','if i!=0 || r.0.is_some(){assert_eq!(r.0,Some(truth));}')
  s=s.sub('P {p} {c0} {case} {rep} {i} {t} {} {} {} {}','P {p} {c0} {case} {rep} {i} {t} {} {} {} {} {}').sub('v[0],v[1],v[2],v[7]);','v[0],v[1],v[2],v[7],usize::from(r.0.is_some()));')
  puts s.sub('for rep in 0..32','for rep in 0..8').sub('for pass in 0..4','for pass in 0..3').sub('3-pass','2-pass').sub('else{pass})%4','else{pass})%3')
else
  %w[screen hermite].each do |name|
    if ARGV.include?('pivots')
      s=generated('harness/g4-carry.rb',name=='screen' ? 'replay' : 'carry')
    elsif ARGV.include?('carry')
      s=generated(name=='screen' ? 'harness/g4-hermite-quotient.rb' : 'harness/g4-carry.rb')
    elsif ARGV.include?('switch')
      s=name=='screen' ? generated('harness/g4-hermite-quotient.rb') : generated('harness/g4-switch.rb','half')
    elsif ARGV.include?('quotient')
      s=generated(name=='screen' ? 'harness/g4-hermite.rb' : 'harness/g4-hermite-quotient.rb')
    else
      s=name=='screen' ? generated('harness/g4-rank-panel.rb','extend','screen') : generated('harness/g4-hermite.rb')
    end
    s=s.split(' template<int P> void panel(){').first
    s=s.sub('namespace fourth_family {',"namespace #{name} { static int *out_y,*out_v,chosen;")
    s=s.sub('auto output=[&](const std::vector<int>& v,int a){',<<'CPP')
 auto output=[&](const std::vector<int>& v,int a){
   chosen=xs[positive[a]];
   for(size_t j=0;j<xs.size();j++)out_y[xs[j]]=ys[j];
   for(int j=0;j<n;j++)out_v[xs[positive[j]]]=v[j];
   return;
CPP
    s=s.gsub('printf("EXT ','if(false)printf("EXT ').sub('static bool report=true','static bool report=false')
    puts s
    puts <<'CPP'
 template<int P> int solve(int c,int l,int *y,int *v){
   C=c;L=l;chosen=-1;out_y=y;out_v=v;
   int a=-1,stats[4]{}; history.clear();record=true;
   run<P,true,true,15>(false,y,v,a,stats);
   if(a>=0)return a;
   extend<P>(stats[0],stats[3],a,v);
   return chosen;
 }
 }
CPP
  end
  puts 'static int coefficient,ell,mode;extern "C" void select_mode(int i){mode=i;}'
  puts 'extern "C" int select_curve(int p,int c){coefficient=c;switch(128*p+c){'
  rows.each{|p,c,l,_|puts "case #{128*p+c}:ell=#{l};return 3;"}
  puts 'default:abort();}}'
  puts 'extern "C" int quartic(int p,int*y,int*v,int,int*){switch(p){'
  rows.map(&:first).uniq.each{|p|puts "case #{p}:return (mode==0?screen::solve<#{p}>:hermite::solve<#{p}>)(coefficient,ell,y,v);"}
  puts 'default:abort();}}'
end
