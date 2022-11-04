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
    sleep_time: u64,
    _description: String
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
            sleep_time: rand::thread_rng().gen_range(100..=2000),
            _description: desc
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
    
    /* Read in input from user */
    println!("Enter the number of creation phases for the producer:");
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    //parse integer out of the input. If input is invalid, program lets user know and exits
    let num_phases: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Invalid input."); return;}
    };

    input = String::new();
    println!("Enter sleep time in ms for the producer to pause between creation phases:");
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    //parse integer out of the input. If input is invalid, program lets user know and exits
    let producer_sleep_time: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Invalid input."); return;}
    };

    input = String::new();
    println!("Enter the number of processes to create each phase:");
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    //parse integer out of the input. If input is invalid, program lets user know and exits
    let num_per_phase: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Invalid input."); return;}
    };



    //Create variables to be used between threads
    let sharedheap: Arc<Mutex<BinaryHeap<Process>>> = Arc::new(Mutex::new(BinaryHeap::new()));

    /* BEGIN PRODUCER THREAD DEFINITION */
    //create reference to shared heap for the producer
    let heap_reference_producer = Arc::clone(&sharedheap);

    let thread_producer = thread::spawn( move || {
        println!("\n......Producer is starting it's work......\n");
        let mut num_processes = 0;
        for _i in 1..=num_phases {
            for _j in 1..=num_per_phase {
                num_processes += 1;
                { /* BEGIN CRITICAL REGION */
                    let mut process_binary_heap = heap_reference_producer.lock().unwrap();

                    let p1 = Process::new(num_processes);
                    process_binary_heap.push(p1);
                } /* END CRITICAL REGION */
                thread::sleep(Duration::from_millis(5));
            }
            println!("\n......Producer is sleeping......\n");
            thread::sleep(Duration::from_millis(producer_sleep_time as u64));
            
            
        }
        println!("......Producer has finished: {} process nodes were created......", num_processes);
    });
    /* END PRODUCER THREAD DEFINITION */



    //wait a short time after producer is made before consumers start
    thread::sleep(Duration::from_millis(200));


    /* BEGIN CONSUMER THREAD 1 DEFINITION */
    //create reference to shared heap for consumer 2
    let heap_reference_consumer_1 = Arc::clone(&sharedheap);

    let thread_consumer_1 = thread::spawn( move || {
        let mut num_executed1 = 0;
        loop { //keep looping until it's empty
            let sleep_time: u64; //create a local variable to store the sleep time so it can be used after the mutex lock is released
            {
                let mut process_binary_heap = heap_reference_consumer_1.lock().unwrap();
                if process_binary_heap.is_empty() { //exit thread if heap is currently empty
                    break;
                }

                let p = process_binary_heap.pop().unwrap();
                println!("Consumer1: executing process {}, priority: {}, for {} ms", p.id, p.priority, p.sleep_time);
                sleep_time = p.sleep_time;
            }
            thread::sleep(Duration::from_millis(sleep_time));
            num_executed1 += 1;
        }
        println!("\n...Consumer1 has completed and executed {} processes...", num_executed1);
    });
    /* END CONSUMER THREAD 1 DEFINITION */



    /* BEGIN CONSUMER THREAD 2 DEFINITION */
    //create reference to shared heap for consumer 2
    let heap_reference_consumer_2 = Arc::clone(&sharedheap);

    let thread_consumer_2 = thread::spawn( move || {
        let mut num_executed2 = 0;
        loop { //keep looping until it's empty
            let sleep_time: u64; //create a local variable to store the sleep time so it can be used after the mutex lock is released
            {
                let mut process_binary_heap = heap_reference_consumer_2.lock().unwrap();
                if process_binary_heap.is_empty() { //exit thread if heap is currently empty
                    break;
                }
                
                let p = process_binary_heap.pop().unwrap();
                println!("Consumer2: executing process {}, priority: {}, for {} ms", p.id, p.priority, p.sleep_time);
                sleep_time = p.sleep_time;
            }
            thread::sleep(Duration::from_millis(sleep_time));
            num_executed2 += 1;
        }
        println!("\n...Consumer2 has completed and executed {} processes...", num_executed2);
    });
    /* END CONSUMER THREAD 2 DEFINITION */



    //wait for all 3 threads to finish
    thread_producer.join().unwrap();
    thread_consumer_1.join().unwrap();
    thread_consumer_2.join().unwrap();

    println!("\nBoth consumers have completed.\n");
   
    
}