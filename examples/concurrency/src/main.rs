
fn main() {
    test_simple();
    test_simple_n_threads();
    test_drop_on_rx();
    test_drop_on_tx();
    test_sample();
}

fn do_not_wait_to_finnish() {
    for _i in 1..10 {
        // we are not joining the threads, so is likely that they will not end up executing.
        let _handle = std::thread::spawn(|| println!("Hello world!"));
    }
}

fn wait_to_finnish() {
    for i in 1..=10 {
        let handle = std::thread::spawn(move || println!("{} Hello world", i));
        let _ = handle.join();
    }
}

fn test_sample() {
    let a = (1..=10).collect::<Vec<u64>>();
    let b = (21..=30).collect::<Vec<u64>>();
    let (tx, rx) = std::sync::mpsc::channel::<String>();

    assert_eq!(a.len(), b.len());

    for i in 0..a.len() {
        let a = a.clone();
        let b = b.clone();
        let tx = tx.clone();

        let handle = std::thread::spawn(move || {
            let s = format!("Thread {} added {} and {}, result {}", i, a[i], b[i], a[i] + b[i]);
            tx.send(s).unwrap();
        });
        _ = handle.join().unwrap(); // Why? It seems that it is not needed
    }

    drop(tx);
    for result in rx {
        println!("{}", result);
    }
}

fn test_simple() {
    let value = 10;
    let (tx, rx) = std::sync::mpsc::channel::<i32>();

    std::thread::spawn(move || tx.send(value).unwrap());

    assert_eq!(rx.recv_timeout(std::time::Duration::from_secs(1)).unwrap(), value);
}

fn test_simple_n_threads() {
    let n_threads = 10;
    let (tx, rx) = std::sync::mpsc::channel::<i32>();

    for i in 1..=n_threads {
        let tx = tx.clone();
        std::thread::spawn(move || tx.send(i).unwrap());
    }

    for _ in 1..=n_threads {
        let i = rx.recv_timeout(std::time::Duration::from_secs(1)).unwrap();
        println!("{}", i);
        assert!(i >= 1 || i <= 10);
    }
}

fn test_drop_on_rx() {
    let (tx, rx) = std::sync::mpsc::channel::<i32>();

    _ = tx.send(10).unwrap();

    drop(rx);
    let err: std::sync::mpsc::SendError::<i32> = tx.send(1).unwrap_err();

    assert_eq!(format!("{}", err), "sending on a closed channel");  // std::fmt::Display
    assert_eq!(format!("{:?}", err), "SendError { .. }");           // std::fmt::Debug
    assert_eq!(err.0, 1);                                           // i32
}

fn test_drop_on_tx() {
    let (tx, rx) = std::sync::mpsc::channel::<i32>();

    drop(tx);
    assert!(rx.recv().is_err());
}
