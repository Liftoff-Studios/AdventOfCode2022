use std::fs;
fn main() {
    // Part 1
    let mut text = fs::read_to_string("Codes.txt").unwrap();
    let mut score: i32 =0; //Score
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();


    for i in text.lines(){
       
        let len = i.len();
        let duh = i.split_at(len/2);
        for j in duh.0.chars(){
            if(duh.1.contains(j)){
                
                let index = letters.iter().position(|&r| r == j).unwrap();
                
                score+=(index+ 1) as i32;
                break;
            }
            
        }

        
    }

    println!("Score is (Part 1): {}",score );

    // Part 2
    let mut sum: i32 = 0;
    let mut groups: Vec<&str>= vec![];
    let mut counter = 0;
    for i in text.lines(){
        //println!("{:?}", counter);
        groups.push(i);
        counter+=1;
        if counter > 2{
            for j in groups[0].chars(){
                if(groups[1].contains(j)&&groups[2].contains(j)){
                    let index = letters.iter().position(|&r| r == j).unwrap();
                
                    sum+=(index+ 1) as i32;
                    break;
                }
            }

            groups.retain(|x| *x=="0");
            counter = 0;
        }
    }

    println!("Sum is (Part 2): {:?}", sum);
}
