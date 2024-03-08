pub struct PositionTracker<I> {
    pub iterator: I,
    position: usize,
}

impl<I> PositionTracker<I> {
    pub fn new(iterator: I) -> Self {
        PositionTracker {
            iterator,
            position: 0,
        }
    }

    pub fn current_position(&self) -> usize {
        self.position
    }
}

impl<I> Iterator for PositionTracker<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(item) => {
                self.position += 1;
                Some(item)
            }
            None => None,
        }
    }
}
