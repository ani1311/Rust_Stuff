use std::vec;
use std::fmt;

struct Graph {
    n: usize,
    edges: Vec<Vec<usize>>,
}

impl fmt::Display for Graph{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.n {
            write!(f, "{}: ", i)?;
            for j in self.edges[i].iter() {
                write!(f, " {}", j)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Graph {
    pub fn new(n: usize,p: f32) -> Self {
        let mut g = Graph {
            n,
            edges: vec![vec![]; n],
        };

        for i in 0..n {
            for j in i+1..n {
                if rand::random::<f32>() < p {
                    g.edges[i].push(j);
                    g.edges[j].push(i);
                }
            }
        }
        return g;
    }

}
