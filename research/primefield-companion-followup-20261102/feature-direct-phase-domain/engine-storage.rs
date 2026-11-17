/// Inline ownership or a scoped borrow of the same dynamically checked cell.
enum EngineStorage<'a,T>{Owned(std::cell::RefCell<T>),Borrowed(&'a std::cell::RefCell<T>)}
impl<T> std::ops::Deref for EngineStorage<'_,T>{
 type Target=std::cell::RefCell<T>;
 fn deref(&self)->&Self::Target{match self{Self::Owned(cell)=>cell,Self::Borrowed(cell)=>cell}}
}
