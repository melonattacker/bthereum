use std::sync::{Arc, Weak};


#[derive(Clone, Debug)]
pub struct Readiness(Weak<()>);

#[derive(Clone, Debug)]
pub struct Latch(Arc<()>);

// impl Readiness {
//     pub fn new() -> (Readiness) {
//         let a = Arc::new(());
//         Readiness(Arc::downgrade(&a))
//     }

//     pub fn is_ready(&self) -> bool {
//         self.0.upgrade().is_none()
//     }
// }

// impl Latch {
//     pub fn new() -> (Latch) {
//         let a = Arc::new(());
//         Latch(a)
//     }
//     pub fn release(self) {
//         drop(self);
//     }
// }

impl Readiness {
    pub fn new() -> (Readiness, Latch) {
        let a = Arc::new(());
        (Readiness(Arc::downgrade(&a)), Latch(a))
    }

    pub fn is_ready(&self) -> bool {
        self.0.upgrade().is_none()
    }
}

impl Latch {
    pub fn release(self) {
        drop(self);
    }
}
