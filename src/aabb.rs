#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AABB {
    pub p: [u32; 2],
    pub d: [u32; 2],
}

impl AABB {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        AABB {
            p: [x, y],
            d: [w, h],
        }
    }

    pub fn split_along(&self, axis: usize, coord: u32) -> (AABB, AABB) {
        let mut split = self.p.clone();
        split[axis] = coord;

        let mut d1 = self.d.clone();
        d1[axis] = coord - self.p[axis];
        let mut d2 = self.d.clone();
        d2[axis] -= d1[axis];

        (
            AABB {
                p: self.p.clone(),
                d: d1,
            },
            AABB {
                p: split.clone(),
                d: d2,
            },
        )
    }

    pub fn quads(&self) -> [AABB; 4] {
        let [x, y] = self.p;
        let w = self.d[0] / 2;
        let h = self.d[1] / 2;
        [
            AABB::new(x, y, w, h),
            AABB::new(x + w, y, w, h),
            AABB::new(x, y + h, w, h),
            AABB::new(x + w, y + h, w, h),
        ]
    }

    pub fn center(&self) -> (u32, u32) {
        (self.p[0] + self.d[0] / 2, self.p[1] + self.d[1] / 2)
    }
}
