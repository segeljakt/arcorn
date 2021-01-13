use arcon::prelude::state::backend::handles::ActiveHandle;
use arcon::prelude::state::data::Key as ArconKey;
use arcon::prelude::state::data::Value as ArconValue;
use arcon::prelude::state::Appender;
use arcon::prelude::state::Backend;
use arcon::prelude::state::Map;
use arcon::prelude::state::MapState;
use arcon::prelude::state::Value;
use arcon::prelude::state::ValueState;
use arcon::prelude::state::VecState;
use std::hash::Hash;

/// Data abstraction over Arcon Values.
pub struct ArcValue<T: ArconValue, B: Backend> {
    data: Value<T, B>,
}

impl<T: ArconValue, B: Backend> ArcValue<T, B> {
    #[inline(always)]
    pub fn new(handle: ActiveHandle<B, ValueState<T>, (), ()>) -> Self {
        Self {
            data: Value::new(handle),
        }
    }
    #[inline(always)]
    pub fn put(&mut self, v: T) {
        self.data.put(v)
    }

    #[inline(always)]
    pub fn get(&mut self) -> T {
        self.data.get().unwrap().clone()
    }
}

/// Data abstraction over Arcon Appenders.
pub struct ArcAppender<T: ArconValue, B: Backend> {
    data: Appender<T, B>,
}

impl<T: ArconValue, B: Backend> ArcAppender<T, B> {
    #[inline(always)]
    pub fn new(handle: ActiveHandle<B, VecState<T>, (), ()>) -> Self {
        Self {
            data: Appender::new(handle),
        }
    }
    #[inline(always)]
    pub fn append(&mut self, data: T) {
        self.data.append(data).unwrap();
    }

    #[inline(always)]
    pub fn fold<A>(&mut self, init: A, folder: fn(A, T) -> A) -> A {
        self.data.consume().unwrap().into_iter().fold(init, folder)
    }
}

/// Data abstraction over Arcon Maps.
pub struct ArcMap<K: Hash + ArconKey, V: ArconValue, B: Backend> {
    data: Map<K, V, B>,
}

impl<K: Hash + Eq + ArconKey, V: ArconValue, B: Backend> ArcMap<K, V, B> {
    pub fn new(handle: ActiveHandle<B, MapState<K, V>, (), ()>) -> Self {
        Self {
            data: Map::new(handle),
        }
    }

    #[inline(always)]
    pub fn insert(&mut self, key: K, val: V) {
        self.data.put(key, val).unwrap();
    }

    #[inline(always)]
    pub fn get(&mut self, key: K) -> Option<V> {
        self.data.get(&key).unwrap().cloned()
    }

    #[inline(always)]
    pub fn remove(&mut self, key: K) {
        self.data.remove(&key).unwrap().unwrap();
    }
}
