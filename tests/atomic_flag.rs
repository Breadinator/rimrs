use rimrs::helpers::AtomicFlag;
use std::thread::{self, spawn, yield_now};

#[test]
fn mostly_trivial() {
    let flag = AtomicFlag::new();
    assert!(!flag.check());
    flag.set();
    assert!(flag.check());
    flag.reset();
    assert!(!flag.check());
}

/// Spawns 100 threads then within those have a flag that is set and unset a bunch of times over
/// multiple threads then then after those threads are joined asserts the flag is unset.
///
/// High chance of failure if the assert is put before sub-threads `a` and `b` join, which is expected.
#[test]
fn set_reset_threaded() {
    let handles: Vec<_> = (0..100)
        .map(|i| {
            thread::Builder::new()
                .name(format!("{i}"))
                .spawn(move || {
                    let flag: &'static _ = Box::leak(Box::new(AtomicFlag::new()));
                    let a = spawn(move || {
                        for _ in 0..21 {
                            flag.set();
                            yield_now();
                            flag.reset();
                            yield_now();
                        }
                    });
                    let b = spawn(move || {
                        for _ in 0..10 {
                            flag.set();
                            yield_now();
                        }
                        flag.reset();
                        yield_now();
                    });

                    a.join().unwrap();
                    b.join().unwrap();

                    assert!(!flag.check());
                })
                .unwrap()
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Doesn't actually check if it's working as intended,
/// but at least checks it doesn't panic when used as intended
#[test]
fn less_contrived() {
    use std::sync::mpsc::{sync_channel, TryRecvError};

    let flag: &'static _ = Box::leak(Box::new(AtomicFlag::new()));
    let (tx, rc) = sync_channel::<()>(1);

    // basically trying to simulate a (very heavy) load
    let setters: Vec<_> = (0..100)
        .map(|i| {
            thread::Builder::new()
                .name(format!("less_contrived::setter{i}"))
                .spawn(move || {
                    for _ in 0..100 {
                        flag.set();
                        yield_now();
                        thread::sleep(std::time::Duration::from_millis(1));
                    }
                })
                .unwrap()
        })
        .collect();

    let getter = thread::Builder::new()
        .name(String::from("less_contrived::getter"))
        .spawn(move || {
            loop {
                match rc.try_recv() {
                    Ok(_) => return,
                    Err(TryRecvError::Disconnected) => panic!("channel disconnected"),
                    Err(TryRecvError::Empty) => (),
                }
                yield_now();

                if flag.check() {
                    // do something more complicated
                    yield_now();
                    println!("doing something...");

                    flag.reset(); // if i comment this out it runs way way less so seems to be mostly working but not be perfect
                }
            }
        })
        .unwrap();

    for setter in setters {
        setter.join().unwrap();
    }

    tx.send(()).unwrap();
    getter.join().unwrap();
}
