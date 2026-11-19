rows=File.readlines(ARGV.fetch(0)).map{|l|l.split.map(&:to_i)}
s=File.read('main.rs')
start=s.index('    for (g,p) in')
finish=s.index('        let c = &inst.curve;',start)
raise unless start && finish
s[start..finish+'        let c = &inst.curve;'.size-1]=<<~RS
  extern "C" {fn audit_catalogue(p:i32,c:i32,l:i32)->i32;fn set_arm(i:i32, audit:bool);}
  let fixtures:[(u64,u64,u64,u64);#{rows.size}]=[#{rows.map{|r|'('+r.join(',')+')'}.join(',')}];
  for (p,coeff,ell,multiple) in fixtures {
    let g=3usize;
    counted::disable();
    let curve=hec_native_ic::instance::thread_curve(p,coeff,g).unwrap();
    let c=&curve;
    let generator=c.mul(&c.place(0,1),multiple);
    assert!(validate::eligible(c,&generator,&generator,ell));
    let available=unsafe{audit_catalogue(p as i32,coeff as i32,ell as i32)};
    println!("PANEL {p} {coeff} {ell} {multiple} {available}");
RS
s=s.sub('&inst.d1','&generator')
# Only the fast verifier uses a precomputed cofactor. Every group order is
# ell times a cofactor bounded by the integer Hasse upper bound.
bounds=rows.map do |p,c,l,_|
  root=Integer.sqrt(p);root+=1 if root*root<p
  top=(root+1)**6/l
  raise unless top>=1
  h=(1..top).reduce(1,:lcm)
  "#{l}=>#{h}"
end
old='let h=match c.fp.p {73=>6,127=>1,257|383=>3,_=>return c.mul(d,k)==*target};'
raise unless s.scan(old).size==1
s=s.sub(old,"let h=match l {#{bounds.join(',')},_=>return c.mul(d,k)==*target};")
s=s.sub('let answer=candidate::solve(c,&d,&target,ell,i==2,audit);','unsafe{set_arm((i-1) as i32,audit);}let answer=candidate::solve(c,&d,&target,ell,true,audit);')
start=s.index(' for arm in 0..3')
raise unless start
s=s[0...start]+<<'RS'
 let crossover=std::env::var_os("HEC_CROSSOVER").is_some();
 for i in 0..4 {
   if i>0 && (available & (1<<(i-1)))==0 {continue;}
   let result=call(i,true).0;
   if result.is_some() || i!=0 {assert_eq!(result,Some(truth));}
 }
 let reverse=std::env::args().len()>1;
 for rep in 0..32 {for pass in 0..4 {
   let slot=if crossover {
     if (case+rep+usize::from(reverse))%2==0{pass}else{(pass+2)%4}
   }else{(case+rep+if reverse{3-pass}else{pass})%4};
   let arm=if crossover{if slot==0 || slot==3{1}else{3}}else{slot};
   if arm>0 && (available & (1<<(arm-1)))==0 {continue;}
   let start=Instant::now();
   let result=black_box(call(arm,false)).0;
   let ns=start.elapsed().as_nanos();
   if result.is_some() || arm!=0 {assert_eq!(result,Some(truth));}
   println!("P {p} {coeff} {case} {rep} {slot} {ns} {}",usize::from(result.is_some()));
 }}
 }}}
RS
puts s
