use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let vals = vec![
//         1,2,3,4,5,6,7,8
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs_f32(0.1));
//         }
//     });

//     for received in rx {
//         println!("Got: {:?}", received);
//     }
// }


fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
        thread::sleep(Duration::from_secs_f32(0.1));
        tx.send(1).unwrap();
        thread::sleep(Duration::from_secs_f32(0.1));
        tx.send(1).unwrap();
        thread::sleep(Duration::from_secs_f32(0.1));
        tx.send(1).unwrap();
        thread::sleep(Duration::from_secs_f32(0.1));
        tx.send(1).unwrap();
        thread::sleep(Duration::from_secs_f32(0.1));
        tx.send(1).unwrap();
        thread::sleep(Duration::from_secs_f32(0.1));
        tx.send(1).unwrap();
    });


    // thread::spawn(move || {
    //     println!("receive {:?}", rx.try_recv());
    // });

    println!("receive {:?}", rx.try_recv());
    thread::sleep(Duration::from_secs_f32(0.1));
    println!("receive {:?}", rx.try_recv());
    thread::sleep(Duration::from_secs_f32(0.1));
    println!("receive {:?}", rx.try_recv());
    thread::sleep(Duration::from_secs_f32(0.1));
    println!("receive {:?}", rx.try_recv());
    thread::sleep(Duration::from_secs_f32(0.1));
    println!("receive {:?}", rx.try_recv());
    thread::sleep(Duration::from_secs_f32(0.1));
}