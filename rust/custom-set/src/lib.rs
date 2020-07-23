use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: PartialEq> {
    data: Vec<T>,
}

impl<T: PartialEq + Clone + PartialOrd + Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut data = input.to_vec();
        data.sort();
        CustomSet { data }
    }

    pub fn contains(&self, element: &T) -> bool {
        if self.data.is_empty() {
            return false;
        }
        let index = self.search(element);
        self.data[index] == *element
    }

    pub fn add(&mut self, element: T) {
        if self.data.is_empty() {
            self.data.push(element);
            return;
        }
        let i = self.search(&element);
        if self.data[i] != element {
            self.data.insert(i, element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.is_empty() {
            return true;
        }
        for i in 0..other.data.len() {
            let mut subset = true;
            for (j, element) in self.data.iter().cloned().enumerate() {
                if other.data[i + j] != element {
                    subset = false;
                    break;
                }
            }
            if subset {
                return true;
            }
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            return true;
        }
        !self.data.iter().any(|x| other.contains(x))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .cloned()
            .filter(|x| other.contains(&x))
            .collect();
        CustomSet { data }
    }

    fn search(&self, element: &T) -> usize {
        //binary search that returns the closest index
        self.search_inner(0, self.data.len() - 1, element)
    }

    fn search_inner(&self, start: usize, end: usize, element: &T) -> usize {
        if start == end {
            return start;
        }
        let mid = (start + end) / 2;
        match self.data[mid].cmp(element) {
            Ordering::Less => {
                if mid == start {
                    end
                } else {
                    self.search_inner(mid, end, element)
                }
            }
            Ordering::Greater => self.search_inner(start, mid, element),
            Ordering::Equal => mid,
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .cloned()
            .filter(|x| !other.contains(&x))
            .collect();
        CustomSet { data }
    }

    pub fn union(&self, other: &Self) -> Self {
        let data = self.data.clone();
        let mut new = CustomSet { data };
        for element in other.data.iter().cloned() {
            new.add(element);
        }
        new
    }
}
