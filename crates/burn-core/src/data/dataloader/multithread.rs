use super::{
    AssignmentStrategy, DataLoader, DataLoaderIterator, DynDataLoader, Progress,
    RoundRobinAssignment, State,
};
use std::sync::mpsc;
use std::thread;

const MAX_QUEUED_ITEMS: usize = 100;

/// A multi-threaded data loader that can be used to iterate over a dataset.
pub struct MultiThreadDataLoader<O> {
    dataloaders: Vec<Box<dyn DynDataLoader<O>>>,
    num_devices: usize,
}

/// A message that can be sent between threads.
#[derive(Debug)]
pub enum Message<O> {
    /// A batch of items.
    Batch(usize, O, Progress, usize),

    /// The thread is done.
    Done,
}

struct MultiThreadsDataloaderIterator<O, A: AssignmentStrategy> {
    num_done: usize,
    workers: Vec<thread::JoinHandle<()>>,
    receiver: mpsc::Receiver<Message<O>>,
    progresses: Vec<Progress>,
    // For multi-device assignment
    assignment: A,
    queue: DeviceQueue<O>,
}

impl<O> MultiThreadDataLoader<O> {
    /// Creates a new multi-threaded data loader.
    ///
    /// # Arguments
    ///
    /// * `dataloaders` - The data loaders.
    ///
    /// # Returns
    ///
    /// The multi-threaded data loader.
    pub fn new(dataloaders: Vec<Box<dyn DynDataLoader<O>>>, num_devices: usize) -> Self {
        assert!(num_devices > 0, "Must use at least one device");
        Self {
            dataloaders,
            num_devices,
        }
    }
}

impl<O> DataLoader<O> for MultiThreadDataLoader<O>
where
    O: Send + 'static + std::fmt::Debug,
{
    fn iter<'a>(&'a self) -> Box<dyn DataLoaderIterator<O, Strategy = RoundRobinAssignment> + 'a> {
        let (sender, receiver) = mpsc::sync_channel::<Message<O>>(MAX_QUEUED_ITEMS);

        let mut progresses = Vec::with_capacity(self.dataloaders.len());

        let handlers: Vec<_> = self
            .dataloaders
            .iter()
            .enumerate()
            .map(|(index, dataloader)| {
                let dataloader_cloned = dataloader.clone_dyn();
                let sender_cloned = sender.clone();
                progresses.push(Progress::new(0, dataloader_cloned.num_items()));

                thread::spawn(move || {
                    let mut iterator = dataloader_cloned.iter();
                    while let Some(item) = iterator.next() {
                        let state = iterator.state();
                        // Default to device 0 (suboptimal, but maintains the previous strategy and does not crash)
                        let device_id = state.resource_id.unwrap_or(0);

                        match sender_cloned.send(Message::Batch(
                            index,
                            item,
                            state.progress,
                            device_id,
                        )) {
                            Ok(_) => {}
                            // The receiver is probably gone, no need to panic, just need to stop
                            // iterating.
                            Err(_) => return,
                        };
                    }
                    // Same thing.
                    sender_cloned.send(Message::Done).ok();
                })
            })
            .collect();

        Box::new(MultiThreadsDataloaderIterator::new(
            receiver,
            handlers,
            progresses,
            self.num_devices,
            RoundRobinAssignment::new(self.num_devices),
        ))
    }

    fn num_items(&self) -> usize {
        self.dataloaders.iter().map(|dl| dl.num_items()).sum()
    }
}

impl<O, A: AssignmentStrategy> MultiThreadsDataloaderIterator<O, A> {
    pub fn new(
        receiver: mpsc::Receiver<Message<O>>,
        workers: Vec<thread::JoinHandle<()>>,
        progresses: Vec<Progress>,
        num_devices: usize,
        assignment: A,
    ) -> Self {
        MultiThreadsDataloaderIterator {
            num_done: 0,
            workers,
            receiver,
            progresses,
            assignment,
            queue: DeviceQueue::new(num_devices),
        }
    }
}

impl<O: std::fmt::Debug> DataLoaderIterator<O>
    for MultiThreadsDataloaderIterator<O, RoundRobinAssignment>
{
    type Strategy = RoundRobinAssignment;

    fn progress(&self) -> Progress {
        let mut items_total = 0;
        let mut items_processed = 0;

        for progress in self.progresses.iter() {
            items_total += progress.items_total;
            items_processed += progress.items_processed;
        }

        Progress::new(items_processed, items_total)
    }

    fn state(&self) -> State {
        let progress = self.progress();
        State {
            progress,
            resource_id: None, // cannot be aggregated
        }
    }
}

struct DeviceQueue<O> {
    queues: Vec<Vec<O>>,
}

impl<O> DeviceQueue<O> {
    fn new(num_devices: usize) -> Self {
        let max_queued_items = if num_devices > 1 {
            MAX_QUEUED_ITEMS / num_devices // could probably be smaller in practice
        } else {
            0 // will never hold values since items are always assigned to one device
        };
        let queues = (0..num_devices)
            .map(|_| Vec::with_capacity(max_queued_items))
            .collect();
        Self { queues }
    }

    fn push(&mut self, item: O, device: usize) {
        self.queues[device].push(item)
    }

    fn pop(&mut self, device: usize) -> Option<O> {
        self.queues[device].pop()
    }

    fn is_empty(&self) -> bool {
        self.queues.iter().all(|q| q.is_empty())
    }
}

impl<O: std::fmt::Debug> Iterator for MultiThreadsDataloaderIterator<O, RoundRobinAssignment> {
    type Item = O;

    fn next(&mut self) -> Option<O> {
        if self.workers.is_empty() {
            return None;
        }

        loop {
            let item = self.receiver.recv();
            let item = item.unwrap();
            let current_id = self.assignment.current();

            match item {
                Message::Batch(index, item, progress, device_id) => {
                    if let Some(current) = self.progresses.get_mut(index) {
                        *current = progress;
                    }
                    if device_id == current_id {
                        self.assignment.step();
                        return Some(item);
                    } else {
                        self.queue.push(item, device_id);
                    }
                }
                Message::Done => {
                    self.num_done += 1;
                }
            };

            // Get item from queue
            if let Some(item) = self.queue.pop(current_id) {
                self.assignment.step();
                return Some(item);
            }

            if self.num_done == self.workers.len() {
                while let Some(worker) = self.workers.pop() {
                    worker.join().unwrap();
                }
            }

            if self.workers.is_empty() && self.queue.is_empty() {
                return None;
            }
        }
    }
}
