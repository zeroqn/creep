use std::{
    any::Any,
    collections::HashMap,
    fmt::Debug,
    marker::{Send, Sync},
};

pub trait Cloneable: CloneableImpl + Debug + Send + Sync {}

pub trait CloneableImpl {
    fn box_clone(&self) -> Box<dyn Cloneable>;
    fn as_any(&self) -> &dyn Any;
}

impl<T> CloneableImpl for T
where
    T: Cloneable + Clone + Debug + 'static,
{
    fn box_clone(&self) -> Box<dyn Cloneable> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for Box<dyn Cloneable> {
    fn clone(&self) -> Box<dyn Cloneable> {
        self.box_clone()
    }
}

impl Cloneable for String {}
impl Cloneable for usize {}
impl Cloneable for u64 {}

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

    pub fn with_value<V>(&self, key: &str, val: V) -> Self
    where
        V: Cloneable + 'static,
    {
        let mut ctx = self.clone();

        ctx.map.insert(key.to_owned(), val.box_clone());
        ctx
    }

    pub fn get<V>(&self, key: &str) -> Option<&V>
    where
        V: Cloneable + 'static,
    {
        self.map
            .get(key)
            .and_then(|any| any.as_any().downcast_ref::<V>())
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

        impl super::Cloneable for World {}

        let ctx = Context::new().with_value::<World>("hello", World);
        assert_eq!(ctx.get::<World>("hello"), Some(&World));
    }
}
