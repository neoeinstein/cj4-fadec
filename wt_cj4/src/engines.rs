use std::iter;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineNumber {
    Engine1,
    Engine2
}

impl EngineNumber {
    pub const fn sim_index(self) -> u32 {
        match self {
            Self::Engine1 => 1,
            Self::Engine2 => 2,
        }
    }

    pub fn iter() -> impl IntoIterator<Item = Self> {
        iter::once(Self::Engine1)
            .chain(iter::once(Self::Engine2))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EngineData<T> {
    pub engine1: T,
    pub engine2: T,
}

impl<T> EngineData<T> {
    pub fn new(v: T) -> Self
    where
        T: Clone,
    {
        Self {
            engine1: v.clone(),
            engine2: v,
        }
    }

    pub const fn new_distinct(e1: T, e2: T) -> Self {
        Self {
            engine1: e1,
            engine2: e2,
        }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> EngineData<U> {
        EngineData {
            engine1: f(self.engine1),
            engine2: f(self.engine2),
        }
    }

    pub fn iter(&self) -> impl IntoIterator<Item = &T> {
        iter::once(&self.engine1)
            .chain(iter::once(&self.engine2))
    }

    pub fn iter_mut(&mut self) -> impl IntoIterator<Item = &mut T> {
        iter::once(&mut self.engine1)
            .chain(iter::once(&mut self.engine2))
    }
}

impl<T> IntoIterator for EngineData<T> {
    type Item = T;
    type IntoIter = iter::Chain<iter::Once<Self::Item>, iter::Once<Self::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.engine1)
            .chain(iter::once(self.engine2))
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
