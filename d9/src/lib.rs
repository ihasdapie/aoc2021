
const FLOOR_X: usize = 100;
const FLOOR_Y: usize = 100;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    height: u32,
    risk_level: u32,
    is_lowpoint: bool,
    checked: bool,
    basin_member: bool,
    coord: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct Floor {
    pub points: Vec<Point>,
    pub risk_sum: u32,
    pub basin_sizes: Vec<u32>,
    lowpoints_found: bool,
}

impl Floor {
    fn display_raw(&self) {
        for y in 0..(FLOOR_Y + 2) {
            for x in 0..(FLOOR_X + 2) {
                let point = &self.points[y * (FLOOR_X + 2) + x];
                if point.is_lowpoint || point.checked {
                    print!("\x1b[0;31m{}\x1b[0m ", point.height);
                } else {
                    print!("{} ", point.height);
                }
            }
            println!();
        }
        println!();
    }

    pub fn find_basin_size(&mut self, x: i32, y: i32) -> u32 {

        if x < 0
            || y < 0
            || x == FLOOR_X as i32
            || y == FLOOR_Y as i32
            || self.get(x as usize, y as usize).height == 9 // we can do this because logical operators are short-circuited
            || self.get(x as usize, y as usize).checked
        {
            return 0;
        }

        self.set_checked(x as usize, y as usize, true);
        self.set_basin_member(x as usize, y as usize, true);

        return 1
            + self.find_basin_size(x - 1, y)
            + self.find_basin_size(x + 1, y)
            + self.find_basin_size(x, y - 1)
            + self.find_basin_size(x, y + 1);
    }

    pub fn display(&self) {
        for y in 0..FLOOR_Y {
            for x in 0..FLOOR_X {
                let point = self.get(x, y);
                if point.is_lowpoint || point.checked {
                    print!("\x1b[0;31m{}\x1b[0m ", point.height);
                } else {
                    print!("{} ", point.height);
                }
            }
            println!("");
        }
        println!("");
    }

    pub fn from_heights(heights: Vec<u32>) -> Floor {
        // note that we'll pad points s.t. there is an outer ring of 10s
        // this way we won't have to worry about bounds checking
        let points: Vec<Point> = vec![
            Point {
                height: 10,
                risk_level: 0,
                is_lowpoint: false,
                checked: false,
                basin_member: false,
                coord: (0, 0),

            };
            (FLOOR_X + 2) * (FLOOR_Y + 2)
        ];

        let mut floor = Floor {
            points,
            risk_sum: 0,
            basin_sizes: Vec::<u32>::new(),
            lowpoints_found: false,
        };

        for y in 0..FLOOR_Y {
            for x in 0..FLOOR_X {
                floor.set_height(x, y, heights[x + y * FLOOR_X]);
                floor.get_mut(x, y).coord = (x, y);
            }
        }

        floor
    }

    // these get and set aren't working!
    fn set_height(&mut self, x: usize, y: usize, height: u32) {
        self.points[(x + 1) + (y + 1) * (FLOOR_X + 2)].height = height;
    }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Point {
        &mut self.points[(x + 1) + (y + 1) * (FLOOR_X + 2)]
    }

    fn get(&self, x: usize, y: usize) -> Point {
        self.points[(x + 1) + (y + 1) * (FLOOR_X + 2)]
    }
    fn set_checked(&mut self, x: usize, y: usize, checked: bool) {
        let idx = (x + 1) + (y + 1) * (FLOOR_X + 2);
        self.points[idx].checked = checked;
    }
    fn set_basin_member(&mut self, x: usize, y: usize, checked: bool) { let idx = (x + 1) + (y + 1) * (FLOOR_X + 2); self.points[idx].basin_member = checked;
    }

    fn set_lowpoint(&mut self, x: usize, y: usize) {
        let idx = (x + 1) + (y + 1) * (FLOOR_X + 2);
        self.points[idx].is_lowpoint = true;
        self.points[idx].risk_level = self.points[idx].height + 1;
    }

    fn get_unguarded(&self, x: usize, y: usize) -> Point {
        self.points[x + y * (FLOOR_X + 2)]
    }

    fn is_lowpoint(&self, x: usize, y: usize) -> bool {
        let left = self.get_unguarded(x, y + 1).height;
        let right = self.get_unguarded(x + 2, y + 1).height;
        let up = self.get_unguarded(x + 1, y).height;
        let down = self.get_unguarded(x + 1, y + 2).height;
        let pt = self.get(x, y).height;
        (pt < left) && (pt < right) && (pt < up) && (pt < down)
    }

    pub fn set_lowpoints(&mut self) {
        for y in 0..FLOOR_Y {
            for x in 0..FLOOR_X {
                if self.is_lowpoint(x, y) {
                    self.set_lowpoint(x, y);
                }
            }
        }
    }
    fn clear_checked(&mut self) {
        for y in 0..FLOOR_Y {
            for x in 0..FLOOR_X {
                self.set_checked(x, y, false);
            }
        } }
    pub fn calc_risk_sum(&mut self) {
        self.risk_sum = self.points.iter().map(|p| p.risk_level).sum();
    }

    pub fn get_all_basin_sizes(&mut self) {
        // returns them in sorted order for convenience
        
        // all basins will be centered on a lowpoint
        if !self.lowpoints_found {
            self.set_lowpoints(); 
        }
        let lowpoints: Vec<(usize, usize)> = self.points.iter().filter(|p| p.is_lowpoint).map(|p| p.coord).collect();

        let mut basin_sizes: Vec<u32> = vec![];

        for pt in lowpoints {
            let (x, y) = pt;
            let size = self.find_basin_size(x as i32, y as i32);
            basin_sizes.push(size);
            self.display();
            self.clear_checked()

        }
        basin_sizes.sort_unstable();
        basin_sizes.reverse();
        self.basin_sizes = basin_sizes.clone();
    }

}
