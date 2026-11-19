require 'open3'
def generated(*args)
  text,status=Open3.capture2('ruby',*args)
  raise args.inspect unless status.success?
  text
end
def change(source,old,replacement)
  raise "expected one occurrence of #{old}" unless source.scan(old).size==1
  source.sub(old){replacement}
end
if ARGV.first=='driver'
  s=generated('../../harness/g3-transfer-driver.rb',ARGV.fetch(1))
  old='if let Some(roots)=split(&current.u,&c.fp,&mut OracleStats::default()){'
  s=change(s,old,old+' if roots.as_slice().iter().any(|&(x,_)|ys[x as usize]<0){current=c.add_place(&current,anchor as u64,ys[anchor] as u64);continue;}')
  completion=File.read('../../sources/complete.rs')
  completion=change(completion,'let mut log=0;', 'if ys[first as usize]<0 || ys[second as usize]<0{continue;} let mut log=0;')
  if ARGV.include?('shift')
    s=change(s,'set_arm((i-1) as i32,audit);','set_arm(if i==2 {0} else {2},audit);')
    s=change(s,'candidate::solve(c,&d,&target,ell,true,audit)','candidate::solve(c,&d,&target,ell,i==3,audit)')
    marker="        old()\n    };"
    s=change(s,marker,<<'RS')
        if probe {
            let x=a as usize;
            let shifted=c.add_place(input,x as u64,ys[x] as u64);
            let second=crate::complete::coordinate::<P>(c,&shifted,l,&ys,&values,&xs,&inv,&roots,audit);
            if audit {eprintln!("RETRY {} {}",P,u8::from(second.is_some()));}
            if let Some(k)=second {
                let original=(k+l-values[x] as u64)%l;
                if audit {assert_eq!(old(),original);}
                return original;
            }
        }
        old()
    };
RS
  end
  puts change(s,'#[path="../../sources/complete.rs"] mod complete;', "mod complete {#{completion}}")
  exit
end
s=generated('harness/g3-transfer-kernels.rb',ARGV.fetch(0))
s=change(s,'namespace fused {','namespace fused {static bool selected;')
s=change(s,'fused::dual=i==2;', 'fused::dual=true;fused::selected=i==2;')
s=s.gsub(/if\(active\)\{fused::(run<[^;]+);\}else\{control::[^;]+;\}/,'fused::\1;')
pos=s.index('namespace fused {')
kernel=s[pos..]
kernel=change(kernel,'std::vector<Point> pts,poles;', 'std::vector<Point> pts,poles;int extra=0;')
census=File.read('harness/g3-character-census.rb')
predicate=census[/ auto positive=.*?\n };/m] or raise
kernel=change(kernel,'if(y<0)continue;',<<~CPP)
  if(y<0)continue;
  if constexpr((P==127 && C==15) || P==257 || P==383){
    #{predicate}
    if(selected && y && !positive(x) && extra++ >= (P==127?10:0))continue;
  }
CPP
kernel=change(kernel,'assert(v[base] && ref[a]);','assert(v[a] && ref[a]);')
kernel=change(kernel,'for(int j=0;j<p;j++)assert(1LL*v[j]*ref[base]%l==1LL*ref[j]*v[base]%l);',
 'for(int i=0;i<len;i++){int j=xs[i];assert(1LL*v[j]*ref[a]%l==1LL*ref[j]*v[a]%l);}')
if ARGV.include?('positive-only') || ARGV.include?('shift')
  kernel=change(kernel,'(P==127 && C==15) || P==257 || P==383','P==257 || P==383')
end
puts s[0...pos]+kernel
