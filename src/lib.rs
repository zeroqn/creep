use dyn_clone::DynClone;

use std::{any::Any, collections::HashMap, fmt::Debug};

pub trait Cloneable: DynClone + Send + Sync + Debug {
    fn as_any(&self) -> &dyn Any;
}

dyn_clone::clone_trait_object!(Cloneable);

impl<T: DynClone + Send + Sync + Debug + 'static> Cloneable for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Context {
    map: HashMap<String, Box<dyn Cloneable>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            map: Default::default(),
        }
    }

    pub fn with_value<V>(&mut self, key: &str, val: V)
    where
        V: Cloneable + 'static,
    {
        self.map.insert(key.to_owned(), Box::new(val));
    }

    pub fn get<V>(&self, key: &str) -> Option<&V>
    where
        V: Cloneable + 'static,
    {
        let opt_val = self.map.get(key);

        opt_val.and_then(|v| v.as_ref().as_any().downcast_ref::<V>())
    }
}

#[cfg(test)]
mod tests {
    use super::Context;

    #[test]
    fn should_insert_and_get_back_value_reference() {
        let mut ctx = Context::new();
        ctx.with_value::<String>("name", "creep".to_owned());
        assert_eq!(ctx.get::<String>("name"), Some(&"creep".to_owned()));

        ctx.with_value::<usize>("size", 2020);
        assert_eq!(ctx.get::<usize>("size"), Some(&2020));
    }

    #[test]
    fn should_get_none_on_wrong_type() {
        let mut ctx = Context::new();
        ctx.with_value::<usize>("ff7", 1);
        assert_eq!(ctx.get::<u64>("ff7"), None);
    }

    #[test]
    fn should_able_to_insert_new_type() {
        #[derive(Debug, Clone, PartialEq)]
        struct World;

        let mut ctx = Context::new();
        ctx.with_value::<World>("hello", World);
        assert_eq!(ctx.get::<World>("hello"), Some(&World));
    }
}
