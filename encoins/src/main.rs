mod transaction;
mod processus;
extern crate mpi;

use mpi::request::WaitGuard;
use mpi::traits::*;

const M: usize = 10;

fn main() {
    // An example of a transaction
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();
    let mu:[usize;M] = [0;M];
    let mut p = processus::Processus::init(rank, mu);
    let root_rank = 0;

    p.transfert(0,1,0); 
}