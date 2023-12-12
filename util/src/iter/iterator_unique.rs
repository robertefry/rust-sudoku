
use std::collections::HashSet;

pub trait Unique: Iterator
{
    fn unique(self) -> UniqueIterator<Self>
    where
        Self: Sized,
        Self::Item: Clone + Eq + std::hash::Hash,
    {
        UniqueIterator
        {
            iter: self,
            seen: HashSet::new(),
        }
    }
}

impl<I: Iterator> Unique for I {}

pub struct UniqueIterator<I: Iterator>
{
    iter: I,
    seen: HashSet<I::Item>,
}

impl<I: Iterator> Iterator for UniqueIterator<I>
where
    I::Item: Clone + Eq + std::hash::Hash,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item>
    {
        while let Some(next) = self.iter.next()
        {
            if self.seen.insert(next.clone())
            {
                return Some(next);
            }
        }
        return None;
    }
}
