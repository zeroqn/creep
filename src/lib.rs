use dyn_clone::DynClone;

use std::{any::Any, collections::HashMap, fmt::Debug};

pub trait Cloneable: DynClone + Debug {
    fn as_any(&self) -> &dyn Any;
}

dyn_clone::clone_trait_object!(Cloneable);

impl<T: DynClone + Debug + 'static> Cloneable for T {
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

    #[must_use]
    pub fn with_value<V>(&self, key: &str, val: V) -> Self
    where
        V: Cloneable + 'static,
    {
        let mut ctx = self.clone();

        ctx.map.insert(key.to_owned(), Box::new(val));
        ctx
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
        let ctx = Context::new().with_value::<String>("name", "creep".to_owned());
        assert_eq!(ctx.get::<String>("name"), Some(&"creep".to_owned()));

        let ctx = ctx.with_value::<usize>("size", 2020);
        assert_eq!(ctx.get::<usize>("size"), Some(&2020));
    }

    #[test]
    fn should_clone_context_on_every_insert() {
        let ctx = Context::new().with_value::<usize>("bow", 1);
        assert_eq!(ctx.get::<usize>("bow"), Some(&1));

        let new_ctx = ctx.with_value::<usize>("player", 2);
        assert_eq!(new_ctx.get::<usize>("player"), Some(&2));
        assert_eq!(ctx.get::<usize>("player"), None);
    }

    #[test]
    fn should_get_none_on_wrong_type() {
        let ctx = Context::new().with_value::<usize>("ff7", 1);
        assert_eq!(ctx.get::<u64>("ff7"), None);
    }

    #[test]
    fn should_able_to_insert_new_type() {
        #[derive(Debug, Clone, PartialEq)]
        struct World;

        let ctx = Context::new().with_value::<World>("hello", World);
        assert_eq!(ctx.get::<World>("hello"), Some(&World));
    }
}
