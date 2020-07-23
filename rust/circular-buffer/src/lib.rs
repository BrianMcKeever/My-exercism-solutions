pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    start: usize,
    length: usize,
    next: usize,
    capacity: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(None);
        }
        CircularBuffer {
            data,
            start: 0,
            length: 0,
            next: 0,
            capacity,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.length == self.capacity {
            return Err(Error::FullBuffer);
        }
        self.data[self.next] = Some(element);
        self.next = self.increment(self.next);
        self.length += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if 0 == self.length {
            return Err(Error::EmptyBuffer);
        }
        self.length -= 1;
        let r = Ok(self.data[self.start].take().unwrap());
        self.start = self.increment(self.start);
        r
    }

    pub fn clear(&mut self) {
        if self.start < self.next {
            for i in self.start..self.next {
                self.data[i] = None;
            }
        } else {
            for i in self.start..self.capacity {
                self.data[i] = None;
            }
            for i in 0..self.next {
                self.data[i] = None;
            }
        }

        self.start = 0;
        self.length = 0;
        self.next = 0;
    }

    fn increment(&self, i: usize) -> usize {
        (i + 1) % self.capacity
    }

    pub fn overwrite(&mut self, element: T) {
        self.data[self.next] = Some(element);
        self.next = self.increment(self.next);
        self.length += 1;
        if self.length > self.capacity {
            self.length -= 1;
            self.start = self.increment(self.start);
        }
    }
}
