struct Container {
    inner: isize,
}

impl Container {
    fn new(inner: isize) -> Self {
        Self { inner }
    }
}

pub fn trying() {
    let mut cnt: Container = Container::new(0);
    let mut increment_and_print = {
        // let cnt = &mut cnt;
        |i: isize| {
            cnt.inner += i;
            println!("cnt is now {}", cnt.inner)
        }
    };
    increment_and_print(3);
    increment_and_print(4);
}
