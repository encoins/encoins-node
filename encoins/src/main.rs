mod transaction;
mod processus;
extern crate mpi;

use mpi::request::WaitGuard;
use mpi::traits::*;


fn main() {
    // An example of a transaction
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();
    let p = processus::Processus::init(rank);

    /* 
    Thread 1 :
        loop:
            if p.toValidate not empty:
                (q, t, h) = toValidate.pop()
                if p.valid(q, t, h):
                    p.adjust(q, t, h)
    
    Thread 2 :
        if rank == 0:
            p.transfert(0, 1, 0)
        loop:
            Rcv(q,m)
            p.verify(q,m)
    */

}
