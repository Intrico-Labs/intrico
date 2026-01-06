use smallvec::SmallVec;

pub mod state;
pub mod gate;

pub type Complex = SmallVec<[f64; 2]>;