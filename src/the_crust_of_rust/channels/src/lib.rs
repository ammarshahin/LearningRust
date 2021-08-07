use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Inner<T> {
  queue: Mutex<VecDeque<T>>,
  available: Condvar,
}
pub struct Sender<T> {
  inner: Arc<Inner<T>>,
}

impl<T> Sender<T> {
  pub fn send(&mut self, t: T) {
    let mut queue = self.inner.queue.lock().unwrap();
    queue.push_back(t);
    drop(queue);
    self.inner.available.notify_one();
  }
}

impl<T> Clone for Sender<T> {
  fn clone(&self) -> Self {
    Sender {
      inner: Arc::clone(&self.inner),
    }
  }
}

pub struct Receiver<T> {
  inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
  pub fn recv(&self) -> T {
    let mut queue = self.inner.queue.lock().unwrap();
    loop {
      match queue.pop_front() {
        Some(value) => return value,
        None => {
          queue = self.inner.available.wait(queue).unwrap();
        }
      }
    }
  }

  pub fn try_recv(&self) -> Option<T> {
    let mut queue = self.inner.queue.lock().unwrap();
    queue.pop_front()
  }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
  let inner: Inner<T> = Inner {
    queue: Mutex::default(), // new(VecDeque::new()),
    available: Condvar::new(),
  };
  let inner = Arc::new(inner);

  (
    Sender {
      inner: inner.clone(),
    },
    Receiver {
      inner: inner.clone(),
    },
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::mpsc::{self, Receiver as Std_Receiver, Sender as Std_Sender};
  use std::thread;

  #[test]
  #[ignore]
  fn std_works() {
    let (tx, rx): (Std_Sender<i32>, Std_Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();
    for id in 0..3 {
      // the sender endpoint can be copied
      let thread_tx = tx.clone();
      let child = thread::spawn(move || {
        thread_tx.send(id).unwrap();
      });
      children.push(child);
    }

    let mut ids = Vec::new();
    for _ in 0..3 {
      // The `recv` method picks a message from the channel
      // `recv` will block the current thread if there are no messages available
      ids.push(rx.recv());
    }

    // Wait for the threads to complete any remaining work
    for child in children {
      child.join().expect("oops! the child thread panicked");
    }

    assert_eq!(ids, vec![Ok(0), Ok(1), Ok(2)]);
  }

  #[test]
  fn our_impl_works() {
    let (mut tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    tx.send(42);
    assert_eq!(42, rx.recv());
  }

  #[test]
  fn non_blocking_impl_works() {
    let (mut tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    tx.send(42);
    assert_eq!(Some(42), rx.try_recv()); // consumes the 42
    assert_eq!(None, rx.try_recv()); // nothing left
  }

  #[test]
  fn multi_send_impl_works() {
    let (mut tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let mut actual = Vec::new();
    let expected = vec![42, 43, 44, 45];

    tx.send(42);
    tx.send(43);
    tx.send(44);
    tx.send(45);

    while let Some(element) = rx.try_recv() {
      actual.push(element);
    }
    assert_eq!(actual, expected);
  }

  #[test]
  fn zero_senders_left() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let _ = tx;
    assert_eq!(42, rx.recv()); // consumes the 42
  }
}
