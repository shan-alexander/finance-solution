use std::fmt;
use itertools::Itertools;

pub fn main() {
    let a = A {
        b: B {
            schedule: Schedule {
                0: vec![0.01, 0.02, 0.03],
            }
        }
    };
    dbg!(&a);
    /*
    let values = format!("[{}]", a.b.schedule.0.iter().map(|x| x.to_string()).join(", "));
    println!("{}", values);
    let a = "a".to_string();
    println!("{}", a);
    println!("{:?}", a);
    dbg!(a);
    */
}

#[derive(Debug)]
struct A {
    b: B,
}

#[derive(Debug)]
struct B {
    schedule: Schedule,
}

// #[derive(Debug)]
struct Schedule(Vec<f64>);



/*
impl fmt::Debug for Schedule {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // fmt.debug_list().entries(self.0.iter()).finish()
        fmt.debug_set().entries(self.0.iter()).finish()
    }
}
*/

/*
impl fmt::Debug for B {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let values = format!("[{}]", self.0.iter().map(|x| x.to_string()).join(", "));
        fmt.debug_struct("Schedule")
            .field("values", &values)
            .finish()
    }
}
*/

impl fmt::Debug for Schedule {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let values = format!("[{}]", self.0.iter().map(|x| x).join(", "));
        write!(fmt, "{}", values)
    }
}

/*
impl fmt::Debug for Schedule {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let values = format!("[{}]", self.0.iter().map(|x| x.to_string()).join(", "));
        fmt.debug_struct("Schedule")
            //.field("values", &("a".to_string().replace("\"", "")))
            .field("values", &values)
            .finish()
    }
}
*/


