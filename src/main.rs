// Thanks and credit to bheisler of the Rust Slack community!
mod foo {
    // This struct can be created outside this module
    pub struct Public {
        pub x: u64,
    }
    
    // This one can't, because it has a private field
    pub struct NonZero {
        x: u64,
    }
    impl NonZero {
        // The name 'new' isn't special, but it's used for simple 
        // constructor-like functions by convention
        pub fn new(x: u64) -> NonZero {
            assert!(x != 0);
            NonZero { x }
        }
    }
    
    // This struct also can't be created outside this module
    pub struct PartiallyPublic {
        pub x: u64,
        y: u64,
    }
    
    // This one uses PhantomData to create a fake private field, so it can't 
    // be created outside this module either
    pub struct Phantom {
        pub x: u64,
        
        phantom: std::marker::PhantomData<()>,
    }
    impl Phantom {
        // Example of how you would create a Phantom
        pub fn new(x: u64) -> Phantom {
            Phantom {
                x,
                phantom: std::marker::PhantomData,
            }
        }
    }
}

pub fn main() {
    // I can create a Public
    let _zero = foo::Public {
        x: 0
    };
    // And a NonZero if I use the function
    let _one = foo::NonZero::new(1);
    
    // And even a Phantom
    let phn = foo::Phantom::new(1);
    dbg!(&phn.x);
    // Note that the PhantomData is always zero-bytes, so there's no performance cost
    // to having the extra private field.
    dbg!(std::mem::size_of::<u64>());
    dbg!(std::mem::size_of::<foo::Phantom>());
}

/*
pub fn doesnt_work() {
    // But I can't create a NonZero directly like I can with Public
    // This gives a compiler error. Uncomment this function and see!
    let _zero = foo::NonZero {
        x: 0
    };
}
*/

// This code is thanks to Jon of the Rust Slack community
type Length = u32;
type Point = (u32, u32);

struct Rectangle {
    top_left: Point,
    width: Length,
    height: Length,
}

use std::cmp::{max, min};

impl Rectangle {
    fn with_points(p1: Point, p2: Point) -> Self {
        let top_left = (min(p1.0, p2.0), min(p1.1, p2.1));
        let width = max(p1.0, p2.0) - top_left.0;
        let height = max(p1.1, p2.1) - top_left.1;
        
        Self { top_left, width, height }
    }
}
