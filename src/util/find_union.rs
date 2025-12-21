pub struct FindUnion {
    pub parents: Vec<usize>,
    pub sizes: Vec<u32>,
}

/// FindUnion with sizes
impl FindUnion {
    pub fn new(size: usize) -> Self {
        let parents: Vec<usize> = (0..size).collect();
        let sizes: Vec<u32> = vec![1u32; size];
        FindUnion { parents, sizes }
    }

    pub fn find(&mut self, x: usize) -> usize {
        let p = self.parents[x];

        if self.parents[p] == p {
            return p;
        }

        let root = self.find(p);
        self.parents[x] = root;
        root
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);

        if a == b {
            return;
        }

        if self.sizes[a] < self.sizes[b] {
            self.parents[a] = b;
            self.sizes[b] += self.sizes[a];
        } else {
            self.parents[b] = a;
            self.sizes[a] += self.sizes[b];
        }
    }

    pub fn size_of(&mut self, x: usize) -> u32 {
        let rep_x = self.find(x);
        self.sizes[rep_x]
    }
}
