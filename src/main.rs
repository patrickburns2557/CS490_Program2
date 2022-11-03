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
    let mut num_processes: u32 = 0;
    let mut process_binary_heap: BinaryHeap<Process> = BinaryHeap::new();
    
    println!("Enther the number of process nodes you wish to generate:");

    //Read in input from user
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    print!("\n");
    //parse integer out of the input. If input is invalid, program lets user know and exits
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Invalid input."); return;}
    };
    







    let thread_handle1 = thread::spawn( || {
        for i in 1..=10 {
            println!("Thread 1: i = {}", i);
            thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(3..=25)));
            //thread::sleep(Duration::from_millis(100));
        }
    });

    thread_handle1.join().unwrap();

    println!("Creating and adding {} process nodes to the Queue and the Binary MinHeap.", input);
    //Generate process nodes and push them onto the queue
    for _i in 1..=input {
        num_processes += 1;
        let p1 = Process::new(num_processes);
        process_binary_heap.push(p1);
    }
    println!("Successfully added {} process nodes added to the Queue.", process_binary_heap.len());













    println!("Draining the Binary MinHeap, one process at a time:");
    for _i in 0..=(num_processes-1) {
        let p = process_binary_heap.pop().unwrap();
        println!(" | Id: {:>5} |  priority: {:>6} |  sleep time: {:>6} |  description: {}", p.id, p.priority, p.sleep_time, p.description);
    }
    
}