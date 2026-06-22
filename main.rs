// start of the selected ci engine. for now this just sets up the determinant type so the crate builds, the real work comes later :))

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Determinant {
    // each bit is an orbital, a set bit means that orbital is occupied
    alpha: u64,
    beta: u64,
}

impl Determinant {
    fn new(alpha: u64, beta: u64) -> Self {
        Determinant { alpha, beta }
    }

    fn n_electrons(&self) -> u32 {
        self.alpha.count_ones() + self.beta.count_ones()
    }
}

fn main() {
    // hartree fock determinant for h2 in a minimal basis, one electron
    // per spin in the lowest orbital
    let hf = Determinant::new(0b1, 0b1);
    println!("{:?} has {} electrons", hf, hf.n_electrons());
}
