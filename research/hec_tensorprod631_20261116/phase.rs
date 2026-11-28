use std::{time::Instant,sync::{OnceLock,Mutex},collections::BTreeMap};
static ENABLED:OnceLock<bool>=OnceLock::new();
static TOTALS:OnceLock<Mutex<BTreeMap<String,(u128,u64)>>>=OnceLock::new();
pub struct Span{name:String,start:Option<Instant>}
impl Span{pub fn new(name:&str)->Self{let enabled=*ENABLED.get_or_init(||std::env::var_os("HEC_PHASE_LOG").is_some());Self{name:name.into(),start:enabled.then(Instant::now)}}}
impl Drop for Span{fn drop(&mut self){if let Some(t)=self.start{let ns=t.elapsed().as_nanos();if self.name.starts_with("group_")||self.name=="section_coords"{let mut m=TOTALS.get_or_init(||Mutex::new(BTreeMap::new())).lock().unwrap();let e=m.entry(self.name.clone()).or_default();e.0+=ns;e.1+=1;}else{println!("TRACE_PHASE name={} wall_ns={ns} complete={} nested=true",self.name,!std::thread::panicking());}}}}
pub fn report(){if let Some(m)=TOTALS.get(){for(name,(ns,n))in m.lock().unwrap().iter(){println!("TRACE_AGGREGATE name={name} wall_ns={ns} calls={n} inclusive=true nested=true");}}}
