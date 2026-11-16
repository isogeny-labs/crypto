fn allocate(steps:&[Step],outputs:&[(usize,bool)],flags:&[u8])->(Vec<[usize;3]>,Vec<Step>,usize){
 let n=flags.len();let mut last:Vec<[usize;3]>=(0..n).map(|i|[i;3]).collect();
 for(i,s)in steps.iter().enumerate(){let t=i+5;match *s{Step::Odd{a,b,c,d,..}=>{for (x,k)in [(a,0),(b,2),(c,0),(d,2)]{last[x][k]=t}},Step::Even{a,b,c,d,e}=>{for(x,k)in [(a,0),(b,0),(c,1),(d,0),(e,1)]{last[x][k]=t}}}}
 for &(i,_)in outputs{last[i][0]=n}
 let present=|i:usize,k:usize|k==0||flags[i]&(1<<(k-1))!=0;
 let mut ends=vec![Vec::<(usize,usize)>::new();n+1];for i in 0..n{for k in 0..3{if present(i,k){ends[last[i][k]].push((i,k))}}}
 let(mut free,mut slots,mut count)=(Vec::new(),vec![[usize::MAX;3];n],0usize);
 for i in 0..n{if i>0{for &(old,k)in &ends[i-1]{free.push(slots[old][k])}}for k in 0..3{if present(i,k){slots[i][k]=free.pop().unwrap_or_else(||{let s=count;count+=1;s})}}}
 let physical=steps.iter().map(|s|match *s{Step::Odd{a,b,c,d,half_even}=>Step::Odd{a:slots[a][0],b:slots[b][2],c:slots[c][0],d:slots[d][2],half_even},Step::Even{a,b,c,d,e}=>Step::Even{a:slots[a][0],b:slots[b][0],c:slots[c][1],d:slots[d][0],e:slots[e][1]}}).collect();
 (slots,physical,count)
}
