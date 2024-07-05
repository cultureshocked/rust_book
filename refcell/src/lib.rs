// refcell is a kind of smartpointer that allows for interior mutability
// it lets you modify its contents, but the object it points to must remain constant
// while data in C may be const T *, a refcell is effectively T const *

// normally, interior mutability is disallowed by the borrow checker, because only one mutable
// reference may exist at a time.
// the borrow rules are instead enforced at runtime rather than compile-time. a refcell contains
// single ownership of the data it holds (which is compile-time) but it can only be used for
// mutations by one thing at a time. this way, we can pass around some _mutable_ data to multiple
// places, effectively "mimicking" passing around and having multiple mutable references.

// since issues may happen at runtime, using a RefCell may result in a panic.

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let total_percentage = self.value as f64 / self.max as f64; // as keyword does typecasting
        if total_percentage >= 1.0 {
            self.messenger.send("ERR: You are over quota!");
        } else if total_percentage >= 0.9 {
            self.messenger.send("URGENT: You have used up 90% of your quota.");
        } else if total_percentage >= 0.75 {
            self.messenger.send("Warning: You've used 75% of quota.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell; // where RefCell lives

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg)); // gets mutable reference
            // its the only one, so it's OK
            // if we took another mutable reference here, this code would panic.
        }
    }

    #[test]
    fn sends_75_warning() {
        let m = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&m, 100);

        limit_tracker.set_value(80);

        assert_eq!(m.sent_messages.borrow().len(), 1); // gets immutable reference, OK
    }
}
