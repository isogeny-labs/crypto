s=File.read('main.rs')
s=s.sub('if i==0 {run(0,4,0,audit,None)} else {','{')
s=s.sub('sequence((i-1) as i32)','sequence(if i==0 || i==3 {0} else {1})')
s=s.sub('for rep in 0..8','for rep in 0..16')
key='(case+rep+if std::env::args().len()>1 {3-pass}else{pass})%4'
raise unless s.include?(key)
puts s.sub(key,'if (case+rep+usize::from(std::env::args().len()>1))%2==0 {pass}else{(pass+2)%4}')
