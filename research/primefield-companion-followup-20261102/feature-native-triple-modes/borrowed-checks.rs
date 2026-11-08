#[cfg(escape_probe)]
fn escape_products()->RecoveryProducts<'static>{
 let owner=RecoveryProducts::new(1009,63,7).unwrap();
 let stream=StreamModes{r:1009,m:126,zeta:1,block:7};
 // Must fail at compile time: for_doubled returns a borrow of owner.
 owner.for_doubled(&stream).unwrap()
}
fn borrowed_checks(){
 let mut nested=0;
 for cap in [1usize,7,64]{
  let owner=RecoveryProducts::new(1009,63,cap).unwrap();
  let stream=StreamModes{r:1009,m:126,zeta:1,block:cap};
  let second=owner.for_doubled(&stream).unwrap();
  let stream2=StreamModes{r:1009,m:252,zeta:1,block:cap};
  let third=second.for_doubled(&stream2).unwrap();
  assert!(std::ptr::eq(&*owner.inner,&*third.inner));
  assert_eq!(rescue_stream_values(1009,252,cap,&third),rescue_stream_values(1009,252,cap,&RecoveryProducts::new(1009,252,cap).unwrap()));
  drop(third);drop(second);assert!(owner.inner.try_borrow_mut().is_ok());nested+=1;
 }
 struct PriorRcLayout{r:u64,m:usize,cap:usize,inner:std::rc::Rc<std::cell::RefCell<IntervalProducts>>}
 println!("borrowed_nested_panels={nested} inline_engine_bytes={} borrowed_storage_enum_bytes={} recovery_products_bytes={} prior_rc_products_bytes={} ownership_heap_constructor=false reference_count_operations=false",std::mem::size_of::<IntervalProducts>(),std::mem::size_of::<EngineStorage<'_,IntervalProducts>>(),std::mem::size_of::<RecoveryProducts<'_>>(),std::mem::size_of::<PriorRcLayout>());
}
