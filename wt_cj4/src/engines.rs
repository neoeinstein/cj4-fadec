//! Structures for treating the dual engine system of the CJ4 jointly
//! or independently as required
//!
//! ## Examples
//!
//! ```
//! use wt_cj4::engines::{EngineNumber, EngineData};
//!
//! let mut engines = EngineData::new(5.0);
//! engines[EngineNumber::Engine1] += 2.0;
//! engines[EngineNumber::Engine2] += 0.5 * engines[EngineNumber::Engine1];
//! let tee = engines.map(|_, t| t * 0.4);
//! engines.update(|e, t| {
//!     println!("{:?}: {}", e, t);
//!     if e == EngineNumber::Engine1 {
//!         *t = 0.0
//!     }
//! });
//! ```

use std::iter;

/// An indexer into an engine data structure
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineNumber {
    /// Engine 1
    Engine1,

    /// Engine 2
    Engine2,
}

impl EngineNumber {
    /// Produces an iterator to step through the engine indexes
    pub fn iter() -> impl IntoIterator<Item = Self> {
        iter::once(Self::Engine1).chain(iter::once(Self::Engine2))
    }
}

/// Bilateral engine data structure
///
/// Can be indexed into by using the `EngineNumber` structure:
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EngineData<T> {
    /// Engine 1
    pub engine1: T,

    /// Engine 2
    pub engine2: T,
}

impl<T> EngineData<T> {
    /// Assigns the same value to both engines
    pub fn new(v: T) -> Self
    where
        T: Clone,
    {
        Self {
            engine1: v.clone(),
            engine2: v,
        }
    }

    /// Assigns each engine distinct values
    pub const fn new_distinct(e1: T, e2: T) -> Self {
        Self {
            engine1: e1,
            engine2: e2,
        }
    }

    /// Runs a function for a single engine on all of the engines,
    /// producing a new engine data structure
    pub fn map<U>(self, mut f: impl FnMut(EngineNumber, T) -> U) -> EngineData<U> {
        EngineData {
            engine1: f(EngineNumber::Engine1, self.engine1),
            engine2: f(EngineNumber::Engine2, self.engine2),
        }
    }

    /// Runs a function for a single engine on all of the engines, updating
    /// engine data in place
    pub fn update(&mut self, mut f: impl FnMut(EngineNumber, &mut T)) {
        f(EngineNumber::Engine1, &mut self.engine1);
        f(EngineNumber::Engine2, &mut self.engine2);
    }

    /// Iterates through the engine values, borrowing the underlying data
    pub fn iter(&self) -> impl IntoIterator<Item = &T> {
        iter::once(&self.engine1).chain(iter::once(&self.engine2))
    }

    /// Iterates through the engine values, mutably borrowing the underlying value
    pub fn iter_mut(&mut self) -> impl IntoIterator<Item = &mut T> {
        iter::once(&mut self.engine1).chain(iter::once(&mut self.engine2))
    }
}

impl<T> IntoIterator for EngineData<T> {
    type Item = T;
    type IntoIter = iter::Chain<iter::Once<Self::Item>, iter::Once<Self::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.engine1).chain(iter::once(self.engine2))
    }
}

impl<T> std::ops::Index<EngineNumber> for EngineData<T> {
    type Output = T;
    fn index(&self, index: EngineNumber) -> &Self::Output {
        match index {
            EngineNumber::Engine1 => &self.engine1,
            EngineNumber::Engine2 => &self.engine2,
        }
    }
}

impl<T> std::ops::IndexMut<EngineNumber> for EngineData<T> {
    fn index_mut(&mut self, index: EngineNumber) -> &mut Self::Output {
        match index {
            EngineNumber::Engine1 => &mut self.engine1,
            EngineNumber::Engine2 => &mut self.engine2,
        }
    }
}
