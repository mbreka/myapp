use std::thread;
use std::time::Duration;


fn main() {
    let mut a: Vec<i32> = (0..10).collect();
    let mut b: Vec<i32> = a.iter().map(do_mul).collect();

    fn do_something(vec: &Vec<i32>) {
        for i in 0..vec.len() {
            println!("{}", vec[i]);
        }
    }

    fn do_mul(i: &i32) -> i32 {
        return i * 2;
    }

    let thr = thread::spawn(move || {
        do_something(&a);
        
    });

    thr.join().unwrap();

    do_something(&b);
    trait Quack {
        fn quack(&self);
    }
    
    struct Duck ();
    
    impl Quack for Duck {
        fn quack(&self) {
            println!("quack!");
        }
    }
    
    struct RandomBird {
        is_a_parrot: bool
    }
    
    impl Quack for RandomBird {
        fn quack(&self) {
            if ! self.is_a_parrot {
                println!("quack!");
            } else {
                println!("squawk!");
            }
        }
    }
    
    let duck1 = Duck();
    let duck2 = RandomBird{is_a_parrot: false};
    let parrot = RandomBird{is_a_parrot: true};
    
    let ducks: Vec<&dyn Quack> = vec![&duck1,&duck2,&parrot];
    
    for d in &ducks {
        d.quack();
    }
}
