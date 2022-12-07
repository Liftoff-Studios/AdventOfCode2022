use std::fs;

fn main() {
    let text = fs::read_to_string("Codes.txt").unwrap();
    /*for i in 0..(text.len()-3){
        let d = algo(&text,i);
        if d==false{
            println!("{}",i);
            break;
        }
    }*/

    let duhm = text.as_bytes().windows(4).position(|x|{
            check_equal(x.to_vec())
    }).unwrap() as i32;

    println!("Answer is (Part 1): {:?}",duhm+4);

    let duhm2 = text.as_bytes().windows(14).position(|x|{
            check_equal(x.to_vec())
    }).unwrap() as i32;

    println!("Answer is (Part 2): {:?}",duhm2+14);

}


fn check_equal(x: Vec<u8>)->bool{
    let mut y = x.clone();
    y.sort();
    for i in 0..(y.len()-1){
        if y[i]==y[i+1]{
            
            return false;
        }
    }
    true
}