fn allocate(steps:&[Step],outputs:&[(usize,bool)],n:usize)->(Vec<usize>,Vec<Step>,usize){
 let mut last:Vec<usize>=(0..n).collect();
 for(i,s)in steps.iter().enumerate(){let t=i+5;match *s{Step::Odd{a,b,c,d,..}=>{for x in [a,b,c,d]{last[x]=t}},Step::Even{a,b,c,d,e}=>{for x in [a,b,c,d,e]{last[x]=t}}}}
 for &(i,_)in outputs{last[i]=n}
 let mut ends=vec![Vec::<usize>::new();n+1];for(i,&end)in last.iter().enumerate(){ends[end].push(i)}
 let(mut free,mut slots,mut count)=(Vec::new(),vec![0usize;n],0usize);
 for i in 0..n{if i>0{for &old in &ends[i-1]{free.push(slots[old])}}slots[i]=free.pop().unwrap_or_else(||{let s=count;count+=1;s});}
 let physical=steps.iter().map(|s|match *s{Step::Odd{a,b,c,d,half_even}=>Step::Odd{a:slots[a],b:slots[b],c:slots[c],d:slots[d],half_even},Step::Even{a,b,c,d,e}=>Step::Even{a:slots[a],b:slots[b],c:slots[c],d:slots[d],e:slots[e]}}).collect();
 (slots,physical,count)
}
