/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?

pub struct Matcher<T> {
    test: Box<dyn Fn(T) -> bool>,
    sub: String,
}

impl<T> Matcher<T> {
    pub fn new<F: 'static + Fn(T) -> bool>(test: F, subs: &str) -> Matcher<T> {
        Matcher {
            test: Box::new(test),
            sub: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Clone> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: std::iter::Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |t| {
            let mut addition = String::new();
            for matcher in &self.matchers {
                if (matcher.test)(t.clone()) {
                    addition.push_str(&matcher.sub.clone());
                }
            }
            if addition == "" {
                t.to_string()
            } else {
                addition
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<
    T: 'static + Copy + ToString + Clone + From<u8> + PartialEq + Default + std::ops::Rem<Output = T>,
>() -> Fizzy<T> {
    let mut f = Fizzy::new();
    let three = 3.into();
    let five = 5.into();
    f = f
        .add_matcher(Matcher::new(move |x| x % three == T::default(), "fizz"))
        .add_matcher(Matcher::new(move |x| x % five == T::default(), "buzz"));
    f
}
