/*
// Assignment 1
use std::thread;
use std::time::Duration;

fn main() {
    println!("Main thread starting");
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 3 threads
    for i in 1..=3 {
        // TODO: Spawn a thread and store its handle
        let handle = thread::spawn(move || {
            // Simulate some work
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} finished", i);
        });
        
        // TODO: Store the handle
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("All threads completed.");
}

//Assignment 2
// getting reference starter code from git assignment
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // TODO: Create a shared counter using Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 5 threads
    for _i_ in 1..=5 {
        // TODO: Clone the Arc for the thread
        let counter_clone = Arc::clone(&counter); //following example here
        
        // TODO: Spawn a thread that increments the counter 10 times
        let handle = thread::spawn(move || {
            // TODO: Increment counter 10 times
                let mut num = counter_clone.lock().unwrap();
                for _ in 0..10 {
                *num += 1;
            }
            
        });
        
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // TODO: Print the final value of the counter
    println!("This is the final counter value: {}", *counter.lock().unwrap());
    // The arc is for the ownership shared across these threads.
}


//Assignment 3
// Creating a simple thread pool with a configurable number of worker threads...
// Submit at least 10 tasks to the thread pool
 // Each worker thread should print its identifier and the task it's processing
// Implement a clean shutdown mechanism for the thread pool
// The main thread should wait for all tasks to complete before exiting
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // TODO: Create a channel for sending jobs
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        // TODO: Create and store workers
        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        // TODO: Return the ThreadPool
        ThreadPool { workers, sender }
        
    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Create a job from the closure and send it to a worker
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: Send terminate message to all workers
        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        
        // TODO: Wait for all workers to finish
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // TODO: Create a thread that loops and receives jobs from the channel
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} has a job", id);
                        //calling job();
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} terminating", id);
                        break;
                    }
                }
            }
        });
        
        // TODO: Return the Worker
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    
    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }
    
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
}
*/

// Assignment 4 
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;
    // TODO: Create a channel for sending numbers
    
    let (tx,rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];
    let tx_main = tx.clone();

    // TODO: Create 2 producer threads
    for i in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(i, tx_clone,ITEM_COUNT);
        });
        handles.push(handle);
    }
    
    // TODO: Create 3 consumer threads
    for i in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(i, rx_clone);
        });
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to finish
    for handle in handles.drain(..NUM_PRODUCERS) {
        handle.join().unwrap();
    }
    for _ in 0..NUM_CONSUMERS {
        tx_main.send(TERMINATION_SIGNAL).unwrap();
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("All items has been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let num = rng.gen_range(1..100);
        println!("Producer {} produced {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} finished", id);

}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        let value = {
            let rx_lock = rx.lock().unwrap();
            rx_lock.recv().unwrap()
        };
        if value == TERMINATION_SIGNAL {
            break;
        }
        println!("Consumer {} consumed {}", id, value);
        thread::sleep(Duration::from_millis(150));
    }
}