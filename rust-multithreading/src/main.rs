use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let str1 = String::from("Hello");
        tx.send(str1).unwrap();
    });

    let val = rx.recv().unwrap();
    println!("{}", val);
}

// move variable

// fn main(){
//     let v = vec![1,2,3,4,5];

//     let handle = thread::spawn(move|| {
//         println!("{:?}", v);
//     });
// }

// fn main() {
//     thread::spawn(|| {
//         for i in 1..6{
//             println!("{} from spawn thread",i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..6{
//         println!("{} from main thread", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }
