use rstar::Point as RStarPoint;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Point {
    pub x: i128,
    pub y: i128,
}

impl Point {
    pub fn new(x: i128, y: i128) -> Self {
        Point { x, y }
    }
}

impl<T> PartialEq<(T, T)> for Point
where
    T: Clone,
    i128: From<T>,
{
    fn eq(&self, other: &(T, T)) -> bool {
        self.x == other.0.clone().into() && self.y == other.1.clone().into()
    }
}

impl RStarPoint for Point {
    type Scalar = i128;
    const DIMENSIONS: usize = 2;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        Point {
            x: generator(0),
            y: generator(1),
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        if index == 0 {
            return self.x;
        }
        self.y
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        if index == 0 {
            return &mut self.x;
        }
        &mut self.y
    }
}

impl<T> From<(T, T)> for Point
where
    i128: From<T>,
{
    fn from(value: (T, T)) -> Self {
        Point {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

impl<T> From<&(T, T)> for Point
where
    T: Clone,
    i128: From<T>,
{
    fn from(value: &(T, T)) -> Self {
        Point {
            x: value.0.clone().into(),
            y: value.1.clone().into(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn new() {
        let point = Point::new(1, 2);
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }

    #[test]
    fn generate() {
        let generator = |dimension: usize| -> i128 { dimension as i128 + 12 };
        let point = Point::generate(generator);
        assert_eq!(point.x, 12);
        assert_eq!(point.y, 13);
    }

    #[test]
    fn nth() {
        let generator = |dimension: usize| -> i128 { dimension as i128 + 12 };
        let point = Point::generate(generator);
        assert_eq!(point.nth(0), 12);
        assert_eq!(point.nth(1), 13);
    }

    #[test]
    fn nth_mut() {
        let generator = |dimension: usize| -> i128 { dimension as i128 + 12 };
        let mut point = Point::generate(generator);
        assert_eq!(point.nth(0), 12);
        assert_eq!(point.nth(1), 13);
        *point.nth_mut(0) = 10;
        *point.nth_mut(1) = 11;
        assert_eq!(point.nth(0), 10);
        assert_eq!(point.nth(1), 11);
    }

    #[test]
    fn from_i32_ref_tuple() {
        let tuple: (i32, i32) = (1, 2);
        let point: Point = (&tuple).into();
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }

    #[test]
    fn from_i32_tuple() {
        let tuple: (i32, i32) = (1, 2);
        let point: Point = tuple.into();
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }
}
