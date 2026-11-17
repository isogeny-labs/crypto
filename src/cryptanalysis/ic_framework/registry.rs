//! Application-owned, typed plug-in factories.
//!
//! Registries are optional: direct trait injection works without names or
//! parsing. Each registry has its own namespace, lifetime and output type, so
//! applications can register borrowed builders, stateful oracles, matrix
//! factories or polynomial solvers without changing this crate. Factories
//! create fresh instances for each run; they are ordinary trusted Rust code.

use std::collections::BTreeMap;

use super::stages::Params;

type Factory<'a, T> = dyn Fn(&Params) -> Result<T, String> + 'a;

pub struct PluginRegistry<'a, T> {
    factories: BTreeMap<String, Box<Factory<'a, T>>>,
}

impl<T> Default for PluginRegistry<'_, T> {
    fn default() -> Self {
        Self {
            factories: BTreeMap::new(),
        }
    }
}

impl<'a, T> PluginRegistry<'a, T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register without silently replacing an existing implementation. Names
    /// cannot contain whitespace or `:` (the spec's parameter separator).
    pub fn register(
        &mut self,
        name: impl Into<String>,
        factory: impl Fn(&Params) -> Result<T, String> + 'a,
    ) -> Result<&mut Self, String> {
        let name = name.into();
        if name.is_empty() || name.contains(':') || name.chars().any(char::is_whitespace) {
            return Err(format!("invalid plug-in name `{name}`"));
        }
        if self.factories.contains_key(&name) {
            return Err(format!("plug-in `{name}` is already registered"));
        }
        self.factories.insert(name, Box::new(factory));
        Ok(self)
    }

    /// Deterministic discovery without constructing any plug-in.
    pub fn names(&self) -> impl ExactSizeIterator<Item = &str> {
        self.factories.keys().map(String::as_str)
    }

    /// Explicitly remove a registration before replacing it.
    pub fn remove(&mut self, name: &str) -> bool {
        self.factories.remove(name).is_some()
    }

    /// Instantiate from typed parameters, without imposing a text format.
    pub fn create_named(&self, name: &str, params: &Params) -> Result<T, String> {
        let factory = self.factories.get(name).ok_or_else(|| {
            format!(
                "unknown plug-in `{name}`; registered: {}",
                self.names().collect::<Vec<_>>().join(", ")
            )
        })?;
        factory(params).map_err(|error| format!("plug-in `{name}`: {error}"))
    }

    /// Convenience parser for `name:key=value,key=value`.
    pub fn create(&self, spec: &str) -> Result<T, String> {
        let (name, params) = Params::parse_spec(spec)?;
        self.create_named(&name, &params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn factories_borrow_state_and_construct_fresh_values() {
        let calls = Cell::new(0);
        let mut registry = PluginRegistry::new();
        registry
            .register("counter", |p| {
                calls.set(calls.get() + 1);
                Ok((calls.get(), p.u64("size")?))
            })
            .unwrap();
        assert_eq!(registry.names().collect::<Vec<_>>(), ["counter"]);
        assert_eq!(calls.get(), 0);
        assert_eq!(registry.create("counter:size=7").unwrap(), (1, 7));
        assert_eq!(registry.create("counter:size=9").unwrap(), (2, 9));
        assert!(registry
            .create("counter")
            .unwrap_err()
            .contains("missing parameter `size`"));
    }

    #[test]
    fn registrations_are_local_and_duplicates_fail_closed() {
        let mut registry = PluginRegistry::new();
        registry.register("engine", |_| Ok(1)).unwrap();
        assert!(registry.register("engine", |_| Ok(2)).is_err());
        assert_eq!(registry.create("engine").unwrap(), 1);
        let separate = PluginRegistry::<u32>::new();
        assert!(separate.create("engine").is_err());
        assert!(registry.remove("engine"));
        assert!(!registry.remove("engine"));
        registry.register("engine", |_| Ok(2)).unwrap();
        assert_eq!(registry.create("engine").unwrap(), 2);
        for name in ["", "two words", "engine:version"] {
            assert!(registry.register(name, |_| Ok(0)).is_err());
        }
    }
}
