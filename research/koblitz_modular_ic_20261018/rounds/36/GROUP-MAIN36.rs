fn product_space36(s:&Series,values:&[Series],table:&[[u8;32];32])->Space{let products:Vec<_>=values.iter().map(|v|{let p=mul(s,v,table);assert_eq!(p,mul_independent(s,v));p}).collect();series_basis36(&products)}
fn flip36(label:&str,v:&[Series],d:&[Series],table:&[[u8;32];32],out:&mut impl Write,expected:usize)->Vec<Series>{let s=d[0];let target=product_space36(&s,v,table);assert_eq!(target.basis.len(),324);quotient36(label,v,&target,d,expected,table,out)}
fn equality36(label:&str,v:&[Series],d:&[Series],e:&[Series],expected:usize,table:&[[u8;32];32],out:&mut impl Write){let target=product_space36(&d[0],e,table);assert_eq!(target.basis.len(),196);let w=quotient36(label,v,&target,d,expected,table,out);assert_eq!(w.len(),expected);println!("GROUP36_EQUALITY label={label} equivalent={} witness_dimension={expected}",expected==1);}
fn addflip36(label:&str,v:&[Series],d:&[Series],e:&[Series],table:&[[u8;32];32],out:&mut impl Write)->Vec<Series>{let complement=flip36(&format!("{label}-first-flip"),v,d,table,out,68);let target=product_space36(&d[0],e,table);assert_eq!(target.basis.len(),196);let sum=quotient36(&format!("{label}-sum-divisor"),v,&target,&complement,68,table,out);flip36(&format!("{label}-last-flip"),v,&sum,table,out,196)}
fn main(){let args:Vec<String>=env::args().collect();assert_eq!(args.len(),4);let spaces=read(&args[1]);let d=read_basis36(&args[2]);fs::create_dir(&args[3]).unwrap();let v=&spaces[2];let zero=&spaces[1];series_basis36(v);series_basis36(zero);series_basis36(&d);let mut table=[[0u8;32];32];for a in 0..32{for b in 0..32{table[a][b]=field(a as u8,b as u8);assert_eq!(table[a][b],field_independent(a as u8,b as u8));}}
 let mut out=BufWriter::new(fs::File::create(format!("{}/group-certificates.txt",args[3])).unwrap());
 equality36("self",v,&d,&d,1,&table,&mut out);
 equality36("source-versus-zero",v,&d,zero,0,&table,&mut out);
 // Addflip(0,D) with s0=1: first flip is the already complete L(D0).
 let dspace=series_basis36(&d);let middle=quotient36("negation-middle",v,&dspace,&spaces[0],68,&table,&mut out);let negative=flip36("negation-last-flip",v,&middle,&table,&mut out,196);
 equality36("negative-versus-zero",v,&negative,zero,0,&table,&mut out);
 let cancelled=addflip36("add-to-inverse",v,&d,&negative,&table,&mut out);
 equality36("cancellation-versus-zero",v,&cancelled,zero,1,&table,&mut out);
 for (label,basis) in [("negative",negative),("cancellation",cancelled)]{let mut file=BufWriter::new(fs::File::create(format!("{}/{label}-basis.txt",args[3])).unwrap());for (i,val) in basis.iter().enumerate(){write!(file,"{}",i+1).unwrap();for x in val{write!(file," {x}").unwrap();}writeln!(file).unwrap();}file.flush().unwrap();}
 out.flush().unwrap();println!("GROUP36_RESULT source_nonprincipal=true negative_nonprincipal=true self_equivalent=true add_to_inverse_equivalent_zero=true full_generator_memberships=all independent_products=all target_factor_evaluated=false");println!("GROUP36_ALL_CHECKS_PASS");}
