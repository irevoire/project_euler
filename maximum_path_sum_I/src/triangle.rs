pub struct Triangle {
    pub val: u32,
    pub left: Box<Triangle>,
    pub right: Box<Triangle>,
    max: u32,
}

impl Triangle {
    pub fn new(n: u32) -> Triangle {
        Triangle {
            val: n,
            left: Box::new(),
            right: Box::new(),
            max: 0,
        };
    }

    pub fn print(&self) {
        print!("{}", val);
        self.left.print();
        self.right.print();
    }

    pub fn insert(&self, x: u32, y: u32, el: u32) {
        if x == 0 && y == 0 {
            self.val = el;
        } else if x == 0 {
            insert(*self.left, x, y - 1);
        } else { // x > 0
            insert(*self.right, x - 1, y - 1);
        }
    }
}
