use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess, SeqAccess};
use serde_json::{Value, Map, Number, json};
use sha2::{Digest, Sha256};
use std::{fmt,fs::{self,File,Metadata},io::Read,path::{Path,Component},collections::BTreeMap};
use std::os::unix::fs::MetadataExt;
pub mod reviewed_adapter;
type R<T> = Result<T,String>;
fn require(b:bool,s:&str)->R<()> {if b{Ok(())}else{Err(s.into())}}
struct Strict(Value);
impl<'de> Deserialize<'de> for Strict {
 fn deserialize<D:Deserializer<'de>>(d:D)->Result<Self,D::Error>{
  struct V;
  impl<'de> Visitor<'de> for V {
   type Value=Strict;
   fn expecting(&self,f:&mut fmt::Formatter)->fmt::Result {f.write_str("JSON without duplicate keys")}
   fn visit_bool<E:de::Error>(self,v:bool)->Result<Strict,E>{Ok(Strict(json!(v)))}
   fn visit_i64<E:de::Error>(self,v:i64)->Result<Strict,E>{Ok(Strict(json!(v)))}
   fn visit_u64<E:de::Error>(self,v:u64)->Result<Strict,E>{Ok(Strict(json!(v)))}
   fn visit_f64<E:de::Error>(self,v:f64)->Result<Strict,E>{Number::from_f64(v).map(|n|Strict(Value::Number(n))).ok_or(E::custom("nonfinite JSON number"))}
   fn visit_str<E:de::Error>(self,v:&str)->Result<Strict,E>{Ok(Strict(Value::String(v.into())))}
   fn visit_string<E:de::Error>(self,v:String)->Result<Strict,E>{Ok(Strict(Value::String(v)))}
   fn visit_unit<E:de::Error>(self)->Result<Strict,E>{Ok(Strict(Value::Null))}
   fn visit_none<E:de::Error>(self)->Result<Strict,E>{Ok(Strict(Value::Null))}
   fn visit_seq<A:SeqAccess<'de>>(self,mut a:A)->Result<Strict,A::Error>{let mut v=Vec::new();while let Some(x)=a.next_element::<Strict>()?{v.push(x.0)} Ok(Strict(Value::Array(v)))}
   fn visit_map<A:MapAccess<'de>>(self,mut a:A)->Result<Strict,A::Error>{let mut m=Map::new();while let Some(k)=a.next_key::<String>()?{if m.contains_key(&k){return Err(de::Error::custom("duplicate JSON key"))}let v=a.next_value::<Strict>()?;m.insert(k,v.0);}Ok(Strict(Value::Object(m)))}
  }
  d.deserialize_any(V)
 }
}
pub fn strict_json(raw:&[u8],cap:usize)->R<Value>{
 require(raw.len()<=cap,"JSON byte cap")?;
 let mut d=serde_json::Deserializer::from_slice(raw);
 let v=Strict::deserialize(&mut d).map_err(|e|e.to_string())?;
 d.end().map_err(|e|e.to_string())?;Ok(v.0)
}
pub fn digest(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn hex64(s:&str)->bool{s.len()==64 && s.bytes().all(|x| x.is_ascii_digit() || (b'a'..=b'f').contains(&x))}
fn stamp(m:&Metadata)->(u64,u64,u64,u32,u64,i64,i64,i64,i64){(m.dev(),m.ino(),m.len(),m.mode(),m.nlink(),m.mtime(),m.mtime_nsec(),m.ctime(),m.ctime_nsec())}
fn canonical_nosymlink(p:&Path)->R<()> {
 require(p.is_absolute(),"absolute path")?;
 require(p.components().all(|c|matches!(c,Component::RootDir|Component::Normal(_))),"normal path components")?;
 require(p.as_os_str()==fs::canonicalize(p).map_err(|e|e.to_string())?.as_os_str(),"canonical path spelling")?;
 for a in p.ancestors(){require(!fs::symlink_metadata(a).map_err(|e|e.to_string())?.file_type().is_symlink(),"symlink component")?;}
 Ok(())
}
fn regular(m:&Metadata,mode:u32)->R<()> {require(m.is_file() && m.nlink()==1 && m.mode()&0o7777==mode,"file type/mode/link count")}
pub fn bound_file(path:&Path,sha:&str,mode:u32,cap:usize)->R<Value>{
 require(hex64(sha),"digest shape")?;canonical_nosymlink(path)?;
 let before=fs::symlink_metadata(path).map_err(|e|e.to_string())?;regular(&before,mode)?;
 require(before.len()<=cap as u64,"file byte cap")?;
 let mut f=File::open(path).map_err(|e|e.to_string())?;
 let opened=f.metadata().map_err(|e|e.to_string())?;require(stamp(&before)==stamp(&opened),"opened file identity")?;
 let mut b=Vec::new();(&mut f).take((cap as u64).saturating_add(1)).read_to_end(&mut b).map_err(|e|e.to_string())?;
 require(b.len()<=cap && b.len() as u64==before.len(),"read length")?;
 require(digest(&b)==sha,"file digest")?;
 require(stamp(&opened)==stamp(&f.metadata().map_err(|e|e.to_string())?),"descriptor changed")?;
 canonical_nosymlink(path)?;
 require(stamp(&before)==stamp(&fs::symlink_metadata(path).map_err(|e|e.to_string())?),"path changed")?;
 Ok(json!({"path":path,"sha256":sha,"bytes":b.len(),"mode":mode,"device":before.dev(),"inode":before.ino(),"links":before.nlink()}))
}
fn listing(path:&Path)->R<Vec<String>>{let mut names=Vec::new();for e in fs::read_dir(path).map_err(|e|e.to_string())?{names.push(e.map_err(|e|e.to_string())?.file_name().into_string().map_err(|_|"UTF8 fixture name")?)}names.sort();Ok(names)}
#[derive(Clone)]
pub struct Custody {
 pub fixtures: std::path::PathBuf,
 pub files:BTreeMap<String,String>,
 pub gate: std::path::PathBuf,
 pub gate_sha256:String,
 pub file_cap:usize,
}
pub fn snapshot(c:&Custody)->R<Value>{
 canonical_nosymlink(&c.fixtures)?;
 let before=fs::symlink_metadata(&c.fixtures).map_err(|e|e.to_string())?;
 require(before.is_dir() && before.mode()&0o7777==0o555,"fixture directory type/mode")?;
 require(!c.files.is_empty(),"empty fixture inventory")?;
 for name in c.files.keys(){require(!name.is_empty() && Path::new(name).components().count()==1 && matches!(Path::new(name).components().next(),Some(Component::Normal(_))),"fixture member name")?;}
 let wanted:Vec<_>=c.files.keys().cloned().collect();require(listing(&c.fixtures)?==wanted,"exact fixture inventory")?;
 let mut files=Vec::new();for (name,sha) in &c.files{files.push(bound_file(&c.fixtures.join(name),sha,0o444,c.file_cap)?)}
 let gate=bound_file(&c.gate,&c.gate_sha256,0o555,c.file_cap)?;
 require(listing(&c.fixtures)?==wanted,"fixture inventory changed")?;
 require(stamp(&before)==stamp(&fs::symlink_metadata(&c.fixtures).map_err(|e|e.to_string())?),"fixture directory changed")?;
 Ok(json!({"files":files,"gate":gate,"race_free_claim":false,"external_pin_admission":false}))
}
pub fn qualify_inputs(case:&str,rc:i64,diff:&[u8],inspect_raw:&[u8],expected_raw:&[u8],c:&Custody)->R<Value>{
 let inspect=strict_json(inspect_raw,2*1024*1024)?;let expected=strict_json(expected_raw,16*1024)?;
 require(expected["fixtures_source"].as_str()==c.fixtures.to_str() && expected["gate_source"].as_str()==c.gate.to_str(),"mount source custody join")?;
 let before=snapshot(c)?;
 let adapter=reviewed_adapter::qualify(case,rc,diff,&inspect,&expected)?;
 let after=snapshot(c)?;require(before==after,"custody snapshot changed")?;
 Ok(json!({"schema":"software.native-input-custody/v1","accepted":true,"adapter":adapter,"custody":after,"runtime_qualification":false,"scientific_execution":false}))
}
