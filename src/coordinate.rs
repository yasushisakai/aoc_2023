#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coordinates<T>
where
    T: PartialOrd + PartialEq,
{
    pub x: T,
    pub y: T,
}

impl<T> Coordinates<T>
where
    T: PartialOrd + PartialEq,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Coordinates<usize> {
    pub fn region_min(&mut self, other: &Self) {
        self.x = self.x.min(other.x);
        self.y = self.y.min(other.x);
    }

    pub fn region_max(&mut self, other: &Self) {
        self.x = self.x.max(other.x);
        self.y = self.y.max(other.y);
    }
}

pub type Delta = Coordinates<isize>;

pub const UP: Delta = Coordinates { x: 0, y: -1 };
pub const DOWN: Delta = Coordinates { x: 0, y: 1 };
pub const RIGHT: Delta = Coordinates { x: 1, y: 0 };
pub const LEFT: Delta = Coordinates { x: -1, y: 0 };

pub struct BBox<T>
where
    T: PartialEq + PartialOrd,
{
    pub x_max: T,
    pub y_max: T,
}

impl BBox<usize> {
    pub fn new(x_limit: usize, y_limit: usize) -> Self {
        Self {
            x_max: x_limit,
            y_max: y_limit,
        }
    }

    pub fn check_within(
        &self,
        coord: &Coordinates<usize>,
        delta: &Delta,
    ) -> Option<Coordinates<usize>> {
        let Coordinates { x, y } = coord;

        let (nx, flag_x) = x.overflowing_add_signed(delta.x);
        let (ny, flag_y) = y.overflowing_add_signed(delta.y);

        if flag_y || flag_x {
            return None;
        }

        if nx > self.x_max || ny > self.y_max {
            return None;
        }

        Some(Coordinates::new(nx, ny))
    }
}
