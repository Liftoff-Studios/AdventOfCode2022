use std::fs;

fn main() {
    // Part 1
    let mut text = fs::read_to_string("Codes.txt").unwrap();
    let mut fii: Vec<i32> = vec![0];
    for i in text.lines(){
        if(i==""){
            fii.push(0);
        }else{
            let mut d = fii.len();
            fii[d-1] += i.parse::<i32>().unwrap();
        }

    }

    println!("The least hungry elf has to be carrying {:?}",fii.iter().max().unwrap());

    //Part 2
    let mut foo: i32 = 0;
    for i in [0,1,2]{
        let max = fii.iter().max().unwrap();
        foo+=max;
        let index = fii.iter().position(|element| element == max).unwrap();
        fii.remove(index);
    }

    println!("Sum of top 3 elves calories: {}",foo);
}
