use crate::point::Point;
use rstar::{RTreeObject, AABB};
use std::any::Any;
use std::fmt::Debug;

pub trait Element: RTreeObject<Envelope = AABB<Point>> + Debug + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[cfg(test)]

pub(crate) mod mocks {
    use super::*;
    use mockall::mock;

    mock! {
        pub Element {
            pub fn action(&self);
            pub fn action_mut(&mut self);
        }

        impl RTreeObject for Element {
            type Envelope = AABB<Point>;

            fn envelope(&self) -> AABB<Point>;
        }

        impl Debug for Element {
            fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result;
        }
    }

    impl Element for MockElement {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }
}
