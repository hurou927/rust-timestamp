use std::cell::RefCell;
use std::time::SystemTime;

pub struct TimeStamp {
    pub times: RefCell<Vec<u128>>,
}
impl TimeStamp {
    pub fn new(max :usize) -> TimeStamp {
        TimeStamp {times: RefCell::new(Vec::with_capacity(max))}
    }
    pub fn stamp(&self) {
        self.times.borrow_mut().push(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("duration_since failed").as_micros());
    }
    pub fn print(&self) {
        let t = self.times.borrow();
        for x in 1..t.len() {
            println!("{} {}", x, (t[x]-t[x-1]) as f64 / 1000_000 as f64);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::utils::*;
    use std::{thread, time};
    #[test]
    fn it_works() {
        let ts = TimeStamp::new(10);
        ts.stamp();
        thread::sleep(time::Duration::from_secs(1));
        ts.stamp();
        thread::sleep(time::Duration::from_secs(1));
        ts.stamp();
        thread::sleep(time::Duration::from_secs(1));
        ts.stamp();
        ts.print();
        assert_eq!(ts.times.borrow().len(), 4);
    }
}
