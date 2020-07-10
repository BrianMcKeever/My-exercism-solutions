use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;
use std::rc::Weak;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(Clone, Debug)]
struct BucketData {
    cap_1: u8,
    cap_2: u8,
    amount_1: u8,
    amount_2: u8,
}

impl BucketData {
    fn new(cap_1: u8, cap_2: u8, amount_1: u8, amount_2: u8) -> BucketData {
        BucketData {
            cap_1,
            cap_2,
            amount_1,
            amount_2,
        }
    }
    fn key(&self) -> (u8, u8) {
        (self.amount_1, self.amount_2)
    }
    fn fill_1(&self) -> BucketData {
        BucketData {
            amount_1: self.cap_1,
            ..*self
        }
    }
    fn fill_2(&self) -> BucketData {
        BucketData {
            amount_2: self.cap_2,
            ..*self
        }
    }
    fn empty_1(&self) -> BucketData {
        BucketData {
            amount_1: 0,
            ..*self
        }
    }
    fn empty_2(&self) -> BucketData {
        BucketData {
            amount_2: 0,
            ..*self
        }
    }
    fn pour_1_into_2(&self) -> BucketData {
        let max_transfer = self.cap_2 - self.amount_2;
        if self.amount_1 <= max_transfer {
            BucketData {
                amount_1: 0,
                amount_2: self.amount_1 + self.amount_2,
                ..*self
            }
        } else {
            BucketData {
                amount_1: self.amount_1 - max_transfer,
                amount_2: self.cap_2,
                ..*self
            }
        }
    }
    fn pour_2_into_1(&self) -> BucketData {
        let max_transfer = self.cap_1 - self.amount_1;
        if self.amount_2 <= max_transfer {
            BucketData {
                amount_1: self.amount_1 + self.amount_2,
                amount_2: 0,
                ..*self
            }
        } else {
            BucketData {
                amount_1: self.cap_1,
                amount_2: self.amount_2 - max_transfer,
                ..*self
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Node<T> {
    parent: Option<Weak<RefCell<Node<T>>>>,
    data: T,
    children: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            parent: None,
            data,
            children: Vec::new(),
        }
    }
    fn add_child(&mut self, child: Rc<RefCell<Node<T>>>) {
        self.children.push(child);
    }
    fn set_parent(&mut self, parent: Option<Rc<RefCell<Node<T>>>>) {
        match parent {
            None => self.parent = None,
            Some(p) => self.parent = Some(Rc::downgrade(&p)),
        }
    }
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > capacity_1 + capacity_2 {
        return None;
    }
    let amount_1: u8;
    let amount_2: u8;

    match start_bucket {
        Bucket::One => {
            amount_1 = capacity_1;
            amount_2 = 0;
        }
        Bucket::Two => {
            amount_1 = 0;
            amount_2 = capacity_2;
        }
    }
    let starting_buckets = BucketData::new(capacity_1, capacity_2, amount_1, amount_2);
    let head = Rc::new(RefCell::new(Node::new(starting_buckets)));
    let mut processed_keys: HashSet<(u8, u8)> = HashSet::new();
    let mut bredth_search = VecDeque::new();
    bredth_search.push_back(Rc::clone(&head));

    let success = |node: &Node<BucketData>, goal_bucket, other_bucket| {
        let mut moves = 1;
        let mut n: Node<BucketData> = node.clone();
        dbg!(n.data);
        while let Some(weak) = n.parent {
            moves += 1;
            n = weak.upgrade().unwrap().borrow().clone();
            dbg!(n.data);
        }

        Some(BucketStats {
            moves,
            goal_bucket,
            other_bucket,
        })
    };
    loop {
        let node_rc_option = bredth_search.pop_front();
        let node_rc = node_rc_option?;
        let mut node = node_rc.borrow_mut();
        let buckets = &node.data;
        let key = buckets.key();
        if processed_keys.contains(&key) {
            continue;
        }
        match start_bucket {
            //we aren't alowed to change to the other start position
            Bucket::One => {
                if key.0 == 0 && key.1 == capacity_2 {
                    continue;
                }
            }
            Bucket::Two => {
                if key.1 == 0 && key.0 == capacity_1 {
                    continue;
                }
            }
        }
        if buckets.amount_1 == goal {
            return success(&node, Bucket::One, buckets.amount_2);
        }
        if buckets.amount_2 == goal {
            return success(&node, Bucket::Two, buckets.amount_1);
        }
        processed_keys.insert(key);

        let f1 = buckets.fill_1();
        let f2 = buckets.fill_2();
        let p1 = buckets.pour_1_into_2();
        let p2 = buckets.pour_2_into_1();
        let e1 = buckets.empty_1();
        let e2 = buckets.empty_2();

        let mut add_child = |buckets: BucketData| {
            let child = Rc::new(RefCell::new(Node::new(buckets)));
            child.borrow_mut().set_parent(Some(Rc::clone(&node_rc)));
            bredth_search.push_back(Rc::clone(&child));
            node.add_child(child);
        };
        add_child(f1);
        add_child(f2);
        add_child(p1);
        add_child(p2);
        add_child(e1);
        add_child(e2);
    }
}
