#![allow(unused)]
use std::{env, thread};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use crate::message::Message;

mod transaction;
mod logging;
mod base_types;
mod message;
mod messaging;


fn main()
{
    let args: Vec<String> = env::args().collect();

    let (transmit_main, receive_main): (Sender<bool>, Receiver<bool>) = mpsc::channel();

    &println!("Initializing with {} processes", &args[1]);
    //initialize_processes(args[1].parse::<u32>().unwrap(), &transmit_main, &receive_main);
}



/*
* REWORK THIS



fn initialize_processes( nb_process : u32, main_transmitter : &Sender<bool>, main_receiver : &Receiver<bool>)
{
    let (mut senders, receivers): (Vec<Sender<Message>>, Vec<Receiver<Message>>) =
        (0..nb_process).into_iter().map(|_| mpsc::channel()).unzip();

    for i in 0..nb_process
    {
        new_process(&senders, &receivers.get(i as usize), i+1, &main_transmitter as &Sender<bool>);
        let initialization_done_correctly = main_receiver.recv().unwrap();
        if initialization_done_correctly
        {
            eprintln!("Process {} initialized correctly!", i+1);
        }
        else
        {
            panic!("Error while trying to initialize process {}.",i+1);
        }
    }

}

fn new_process( &senders : &Vec<Sender<Message>>, receiver : &Receiver<Message>, proc_nb : u32, main_transmitter : &Sender<bool>)
{
    let real_senders = *senders;
    let real_receiver = *receiver;
    let main_transmit = *main_transmitter;
    thread::spawn(move ||
        {
            // Do initialisation
            log!(proc_nb, "Initialisation successful!");
            main_transmit.send(true).unwrap(); // telling main function I initialized correctly
            loop {
                    // Do something...
                thread::sleep(Duration::from_millis(1000));
            }
        });


}
*/