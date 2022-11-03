/**********************************
Patrick Burns
2022-11-03
CS 490 Programming Assignment 2

Purpose: Simulate the creation and execution
of processes in RUST using concurrent threads
for creating and then executing the processes.

Written using VSCode on Windows
**********************************/

/* Use declarations */
use std::{collections::BinaryHeap, cmp::Ordering, io, thread, time::Duration, sync::{Arc, Mutex}};
use rand::Rng;

/* Structs and their associated Methods */
#[derive(Debug)]
#[derive(Eq)]
#[derive(Clone)]
struct Process {
    priority: u8,
    id: u32,
    sleep_time: u8,
    description: String
}
impl Process {
    //associated function to return a new Process instance
    fn new(id: u32) -> Self {
        //Creating the string to store in the description field
        let mut desc = String::from("Process Node: ");
        desc.push_str(&id.to_string());
        
        Self {
            id,
            priority: rand::thread_rng().gen_range(1..=100),
            sleep_time: rand::thread_rng().gen_range(100..=200),
            description: desc
        }
    }
}
//Implements ordering to be used in the Binary Heap to sort the Process instances by priority
//Reverses the normally expected output of cmp to create a minimum binary heap
impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //Reverse the Ordering result to create a minimum binary heap
        match self.priority {
            b if b > other.priority => Ordering::Less,
            b if b < other.priority => Ordering::Greater,
            b if b == other.priority => Ordering::Equal,
            _ => Ordering::Equal
        }
    }
}
//Implements PartialOrd for Process to use Ord
impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
//Implements PartialEq for Process to use Ord
impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

fn main() {
    let mut input = String::new();
    
    println!("Enther the number of process nodes you wish to generate:");

    //Read in input from user
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    print!("\n");
    //parse integer out of the input. If input is invalid, program lets user know and exits
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Invalid input."); return;}
    };
    





    let sharedcounter = Arc::new(Mutex::new(0));
    let sharedheap: Arc<Mutex<BinaryHeap<Process>>> = Arc::new(Mutex::new(BinaryHeap::new()));

    
    let counter_reference_producer = Arc::clone(&sharedcounter);
    let heap_reference_producer = Arc::clone(&sharedheap);

    let thread_producer = thread::spawn( move || {
        for _i in 1..=input {
            { /* BEGIN CRITICAL REGION */
                let mut num = counter_reference_producer.lock().unwrap();
                let mut process_binary_heap = heap_reference_producer.lock().unwrap();
                
                *num += 1;
                println!("thread 1 creating process {}", *num);
                let p1 = Process::new(*num);
                process_binary_heap.push(p1);

            } /* END CRITICAL REGION */
            
            thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(50..=100)));
        }
    });

    thread::sleep(Duration::from_millis(200));

    
    let counter_reference_consumer_1 = Arc::clone(&sharedcounter);
    let heap_reference_consumer_1 = Arc::clone(&sharedheap);

    let thread_consumer_1 = thread::spawn( move || {
        while true { //keep looping until it's empty
            let mut sleep_time: u8;
            {
                let mut process_binary_heap = heap_reference_consumer_1.lock().unwrap();
                let p = process_binary_heap.pop().unwrap();
                println!("Consumer1: executing process {}, priority: {}, for {} ms", p.id, p.priority, p.sleep_time);
                sleep_time = p.sleep_time;
                if(process_binary_heap.is_empty()) {
                    break; //only pop one process at a time
                }
            }
            thread::sleep(Duration::from_millis(sleep_time as u64));
        }
        //thread::sleep(Duration::from_millis(700));
    });



    thread_producer.join().unwrap();
    thread_consumer_1.join().unwrap();
    let mut process_binary_heap = sharedheap.lock().unwrap();
    let num_processes = sharedcounter.lock().unwrap();












    

    println!("Draining the Binary MinHeap, one process at a time:");
    //for _i in 0..=(*num_processes-1) {
    while !process_binary_heap.is_empty() {
        let p = process_binary_heap.pop().unwrap();
        println!(" | Id: {:>5} |  priority: {:>6} |  sleep time: {:>6} |  description: {}", p.id, p.priority, p.sleep_time, p.description);
    }
    
}