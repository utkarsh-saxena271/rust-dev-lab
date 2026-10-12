use std::fmt::Error;

trait Serialize{
    fn serialize(&self)->Vec<u8>;
}

trait Deserialize: Sized{
    fn deserialize(base: &[u8]) -> Result<Self, Error>;
}


#[derive(Debug)]
struct Swap{
    qt_1 : u32,
    qt_2 : u32
}

// manual implementation of traits for the struct
// impl Serialize for Swap {
//     fn serialize(&self)->Vec<u8> {
//         let q_1 = self.qt_1.to_be_bytes();
//         let q_2 = self.qt_2.to_be_bytes();

//         let mut bytes: Vec<u8> = vec![];

//         bytes.extend_from_slice(&q_1);
//         bytes.extend_from_slice(&q_2);

//         bytes
//     }
// }

// impl Deserialize for Swap{
//     fn deserialize(base: &[u8]) -> Result<Self, Error> {
//         if base.len() != 8 {
//             return Err(Error);
//         }

//         let qt_1 = u32::from_be_bytes(
//             base[0..4].try_into().unwrap()
//         ); 

//         let qt_2 = u32::from_be_bytes(
//             base[4..8].try_into().unwrap()
//         );

//         Ok(Swap { qt_1, qt_2 })
//     }
// }




fn main(){
    let s1 = Swap{
        qt_1: 1,
        qt_2: 33
    };
    
}