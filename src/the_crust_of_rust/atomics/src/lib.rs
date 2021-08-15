use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED: bool = true;
const UNLOCKED: bool = false;
pub struct Mutex<T> {
  lock: AtomicBool,
  v: UnsafeCell<T>,
}

impl<T> Mutex<T> {
  pub fn new(t: T) -> Self {
    Self {
      lock: AtomicBool::new(UNLOCKED),
      v: UnsafeCell::new(t),
    }
  }

  /// ! WARNING: we are using spin lock >> which is really really BAD !!!!
  pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
    while self
      .lock
      .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
      .is_err()
    {
      while self.lock.load(Ordering::Relaxed) == LOCKED {}
    }
    self.lock.store(LOCKED, Ordering::Relaxed);
    let ret = f(unsafe { &mut *self.v.get() });
    self.lock.store(UNLOCKED, Ordering::Release);
    ret
  }
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

#[cfg(test)]
mod tests {
  use super::*;
  use std::thread;
  #[test]
  #[ignore]
  fn the_wrong_way_works() {
    const THREADS_NUM: i32 = 1000;
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handles: Vec<_> = (0..THREADS_NUM)
      .map(|_| {
        thread::spawn(move || {
          for _ in 0..THREADS_NUM {
            l.with_lock(|v: &mut i32| *v += 1);
          }
        })
      })
      .collect();

    for handle in handles {
      handle.join().unwrap();
    }
    assert_eq!(l.with_lock(|v| *v), THREADS_NUM * THREADS_NUM);
  }

  #[test]
  fn seq_Racing_test() {
    let x: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let y: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let z: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    
    let tx = thread::spawn(move || {
      
    });
    let ty = false;
    let z =0;
    let tx = thread::spawn(move || {
        
    } );
    
  }
}
