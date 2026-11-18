from pathlib import Path
import hashlib,json
root=Path(__file__).resolve().parent
src=root.parent/'feature-trace-image-envelope'
pins={}
for name in ['modes.rs','reference.rs','adapter.rs','orbit.rs','dynamic.rs','envelope.rs','block-projection.rs','blocked-image.rs','trace-adapter.rs','history.rs','trace.rs']:
 data=(src/name).read_text();pins[name]=hashlib.sha256(data.encode()).hexdigest()
 if name=='history.rs':data=data.replace('trace_image_blocked(p,&state,m,r,cap,cap,cap.checked_mul(cap).ok_or(Error::Limit)?)','adaptive_image(p,&state,m,r,cap,cap,cap.checked_mul(2).ok_or(Error::Limit)?)')
 if name=='trace.rs':
  data='include!("adaptive.rs");\n'+data
  data=data.replace('let expected=stable_envelope', '''let adaptive=adaptive_image(&mut p,&g,m,r,r as usize,r as usize,2*r as usize).unwrap();assert_eq!((&adaptive.images,&adaptive.envelope),(&got.images,&got.envelope));assert!(adaptive.moments.len()<=4*distinct.len());
   if !roots.is_empty(){let b=power_mod(&mut p,&g,m).unwrap();let mut corrupt=adaptive.images.clone();corrupt[0]=(corrupt[0]+1)%r;assert_ne!(quotient_evaluate(&mut p,&g,&b,&corrupt,r).unwrap(),vec![0]);}
   adaptive_rejections+=adaptive.rejected;if !roots.is_empty()&&adaptive.moments.len()<roots.len()+1{early+=1;}
   let expected=stable_envelope''')
  data=data.replace('fn main(){envelope_checks();', 'fn main(){adaptive_controls();envelope_checks();')
  data=data.replace('fn trace_checks(){','fn trace_checks(){let(mut adaptive_rejections,mut early)=(0usize,0usize);')
  data=data.replace('let multiset:Vec<_>=roots.iter().map(|&x|pow(x,m as u64,r)).collect();let mut distinct=multiset.clone();distinct.sort();distinct.dedup();assert_eq!(got.characteristic,schoolbook_roots(&multiset,r));assert_eq!(got.images,schoolbook_roots(&distinct,r));assert_eq!(p.divrem', 'let adaptive=adaptive_image(&mut p,&g,m,r,32,4096,64).unwrap();assert_eq!((&adaptive.images,&adaptive.envelope),(&got.images,&got.envelope));let multiset:Vec<_>=roots.iter().map(|&x|pow(x,m as u64,r)).collect();let mut distinct=multiset.clone();distinct.sort();distinct.dedup();assert_eq!(got.characteristic,schoolbook_roots(&multiset,r));assert_eq!(got.images,schoolbook_roots(&distinct,r));assert_eq!(p.divrem')
  data=data.replace('println!("trace_subsets=', '''assert!(adaptive_rejections>0);assert!(early>0);assert!(matches!(adaptive_image(&mut p,&[12,1],3,13,1,13,1),Err(Error::Limit)));assert!(matches!(adaptive_image(&mut p,&[1,11,1],3,13,3,13,6),Err(Error::Domain)));println!("adaptive_rejected_prefixes={adaptive_rejections} fewer_than_n_plus_one_moments={early}");
 println!("trace_subsets=''')
 (root/name).write_text(data)
(root/'generation.json').write_text(json.dumps({'source':'../feature-trace-image-envelope','input_hashes':pins,'output_hashes':{n:hashlib.sha256((root/n).read_bytes()).hexdigest() for n in pins}},indent=2)+'\n')
