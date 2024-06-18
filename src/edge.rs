use std::cmp::Ordering;

#[derive(Debug)]
pub struct Edge {
    pub u : usize,
    pub v : usize,
    pub w : i32,
}

impl Edge {
    pub fn new(u_ : usize, v_ : usize, w_ : i32) -> Edge {
        Edge { u : u_, v : v_, w : w_ }
    }
}

impl Default for Edge {
    fn default() -> Edge {
        Edge { u : 0, v : 0, w : 0 }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.w.cmp(&other.w)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
    }
}

impl Eq for Edge { }