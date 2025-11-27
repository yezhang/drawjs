use std::collections::HashMap;
use std::ops::Deref;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}

impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.w < other.w && self.h > other.h
    }
}

struct Cacher<T> 
    where T: Fn(u32)-> u32
{
    run: T,
    inner_value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32)-> u32
{
    fn new(cal: T) -> Cacher<T> {
        Cacher {
            run: cal,
            inner_value: HashMap::new(),
        }
    }

    fn value(&mut self, args: u32) -> u32{
        let mut value_by_key = self.inner_value.get(&args);

        match value_by_key{
            Some(v) => *v,
            None => {
                let result = (self.run)(args);
                self.inner_value.insert(args, result);
                result
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_run() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn failed_run() {
        panic!("failed intented");
    }

    #[test]
    fn test_can_hold() {
        let big_rect = Rect {
            w: 10,
            h: 8,
        };
        let small_rect = Rect {
            w: 6,
            h: 2,
        };
        assert!(big_rect.can_hold(&small_rect));
    }

    #[test]
    fn test_closure() {
        let mut cacher = Cacher::new(|x: u32| -> u32 {
            x+1
        });

        println!("{}", cacher.value(2));

        assert!(cacher.value(2) == 3, "plus one")
    }

    #[test]
    fn test_iter() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        for v in v1_iter {
            println!("{}", v);
        }

        let v2= vec![4, 5, 6];
        let v2_iter = v2.iter();

        let count: i32 = v2_iter.sum();
        println!("{}", count);

    }
    #[test]
    fn test_box() {
        struct Mybox<T>(T);

        impl<T> Mybox<T> {

            fn new(x: T) -> Mybox<T> {
                Mybox(x)
            }
        }
        impl<T> Deref for Mybox<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        let x = 5;
        let y = Mybox::new(x);

        println!("{}", *y);
    }
}
