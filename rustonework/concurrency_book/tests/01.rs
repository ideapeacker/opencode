use std::thread;

#[test]
#[ignore]
fn start_n_threads() {
    const N: usize = 10;
    let m: Vec<_> = (0..N)
        .map(|i| {
            std::thread::spawn(move || {
                println!("{}", i);
            })
        })
        .collect();
    for handle in m {
        handle.join().unwrap();
    }
}

#[test]
#[ignore]
fn test_lazy_static() {}

#[test]
#[ignore]
fn test_arc_refcell() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || match counter.lock() {
            Ok(mut num) => {
                *num += 1;
                if *num == 3 {}
                panic!("Simulated panic!");
            }
            Err(e) => {
                println!("PoisonError:{}", e);
                let mut num = e.into_inner();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count : {}", *counter.lock().unwrap());
}

#[test]
#[ignore]
fn test_once() {
    use std::sync::Once;
    static START: Once = Once::new();
    START.call_once(|| {
        println!("init 1");
    });
    START.call_once(|| {
        println!("init 3");
    });
}

#[test]
fn test_futures_channel() {
    use futures_channel::oneshot;
    use std::time::Duration;

    let (sender, receiver) = oneshot::channel::<i32>();
    thread::spawn(|| {
        println!("sleeping...");
        thread::sleep(Duration::from_secs(10));
        println!("awake..");
        sender.send(3).unwrap();
    });

    println!("Main...");

    futures::executor::block_on(async {
        println!("MAIN: waiting for msg...");
        println!("MAIN: got:{:?}", receiver.await);
    });
}
