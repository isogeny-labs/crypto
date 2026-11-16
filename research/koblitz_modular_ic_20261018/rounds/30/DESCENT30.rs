// Exact permutation consequences of recorded scalar-kernel Frobenius colors.
// A nonzero formal augmentation vector is NOT a nonzero Jacobian class.
use std::{env,fs,path::Path};
fn colors(text:&str)->Vec<(usize,usize)>{let lines:Vec<_>=text.lines().collect();let mut out=vec![];for(i,line)in lines.iter().enumerate(){if *line=="COLOR30_VERIFIED_COLORS first,second"{out.push((lines[i+1].parse().unwrap(),lines[i+2].parse().unwrap()));}}out}
fn main(){let args:Vec<_>=env::args().collect();let out=Path::new(&args[1]);assert!(!out.exists());fs::create_dir_all(out).unwrap();let root=Path::new(&args[2]);let mut rows=vec![];let mut total=0;
 for(d,name,expected)in[(12,"color12-01",vec![(3,4),(4,3)]),(436,"color436-01",vec![(3,3)])]{let log=fs::read_to_string(root.join("runs").join(name).join("native.log")).unwrap();assert!(log.contains("COLOR30_ALL_CHECKS_PASS"));assert!(!log.lines().any(|s|s.trim_start().starts_with('?')));let found=colors(&log);assert_eq!(found,expected);
  for (candidate,(a,b)) in found.iter().enumerate(){assert_eq!((a+b)%5,d%5,"determinant character modulo sign");for(side,k)in[(*a,0),(*b,1)].iter().map(|(color,side)|(*side,*color)) {assert!((1..5).contains(&k));assert_ne!(d%5,0);let inv=(1..5).find(|u|u*k%5==1).unwrap();let length=5*d;let step=d*inv;
   let mut visited=vec![false;length];let mut cycle=0;let mut at=0;loop{assert!(!visited[at]);visited[at]=true;cycle+=1;at=(at+1)%length;if at==0{break;}}assert_eq!(cycle,length);
   let mut component_sizes=[0usize;5];for i in 0..length{component_sizes[i%5]+=1;assert_eq!((i+d)%length,(i+k*step)%length);assert_eq!(((i+5)%length)%5,i%5);assert_eq!(((i+step)%length)%5,(i%5+step%5)%5);}assert_eq!(component_sizes,[d;5]);
   let mut perm=[0usize;5];for i in 0..5{perm[i]=(i+step)%5;}let mut j=0;let mut count=0;loop{j=perm[j];count+=1;if j==0{break;}assert!(count<5);}assert_eq!(count,5);
   let mut single=[0i64;5];single[0]=1;let mut difference=single;difference[perm[0]]-=1;assert_ne!(difference,[0;5]);assert_eq!(difference.iter().sum::<i64>(),0);
   let full=[1i64;5];let mut full_difference=full;for i in 0..5{full_difference[perm[i]]-=full[i];}assert_eq!(full_difference,[0;5]);assert_ne!((k*(step+1))%length,d%length,"corrupted monodromy must reject");
   println!("DESCENT30 d={d} candidate={candidate} side={side} color={k} absolute_degree={length} F32_components=5 component_degree={d} deck_component_shift={} nonzero_formal_component=true full_norm_difference=0",step%5);
   rows.push(format!("{{\"base_degree\":{d},\"candidate\":{candidate},\"side\":{side},\"color\":{k},\"absolute_algebra_degree\":{length},\"F32_component_count\":5,\"F32_component_degree\":{d},\"deck_component_permutation\":{:?},\"individual_augmentation_vector_nonzero\":true,\"whole_orbit_augmentation_vector_zero\":true,\"Jacobian_class_nonzero\":null}}",perm));total+=1;
  }}
 }assert_eq!(total,6);fs::write(out.join("results.json"),format!("{{\"schema\":\"koblitz.descent30/v1\",\"scope\":\"finite-field Frobenius/deck permutation module, not Jacobian class evaluation\",\"records\":[{}]}}\n",rows.join(","))).unwrap();println!("DESCENT30_ALL_CHECKS_PASS");}
