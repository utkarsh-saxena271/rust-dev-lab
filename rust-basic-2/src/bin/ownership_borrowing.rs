fn main(){
    let str = String::from("Utkarsh");
    let l = get_length(&str);
    println!("{}",l);
    println!("{}",str);
}

fn get_length(str:&String) -> usize {
    return str.len();
}