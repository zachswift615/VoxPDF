use std::sync::{Arc, Mutex};

pub struct StringPool {
    pool: Arc<Mutex<Vec<String>>>,
    capacity: usize,
}

impl StringPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(Vec::with_capacity(capacity))),
            capacity,
        }
    }

    pub fn acquire(&self) -> String {
        if let Ok(mut pool) = self.pool.lock() {
            pool.pop().unwrap_or_else(|| String::with_capacity(32))
        } else {
            String::with_capacity(32)
        }
    }

    pub fn release(&self, mut s: String) {
        s.clear();
        if let Ok(mut pool) = self.pool.lock() {
            if pool.len() < self.capacity {
                pool.push(s);
            }
        }
    }
}

pub struct VecPool<T> {
    pool: Arc<Mutex<Vec<Vec<T>>>>,
    capacity: usize,
}

impl<T> VecPool<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(Vec::with_capacity(capacity))),
            capacity,
        }
    }

    pub fn acquire(&self) -> Vec<T> {
        if let Ok(mut pool) = self.pool.lock() {
            pool.pop().unwrap_or_else(|| Vec::with_capacity(64))
        } else {
            Vec::with_capacity(64)
        }
    }

    pub fn release(&self, mut v: Vec<T>) {
        v.clear();
        if let Ok(mut pool) = self.pool.lock() {
            if pool.len() < self.capacity {
                pool.push(v);
            }
        }
    }
}
