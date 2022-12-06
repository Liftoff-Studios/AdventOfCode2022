use std::fs;
use std::str;
fn main() {
    //Part 1
    let mut text = fs::read_to_string("Codes.txt").unwrap();
    let mut foo: Vec<String> = vec!["".to_string()];

    for i in text.lines(){
        if(i==""){
            foo.push("".to_string());
        }
        let du = foo.len();
        foo[du-1].push_str(i);
    }

    

    let mut dii = foo[0].split(" 1").collect::<Vec<&str>>();
    let row_numbers: i32 = (dii[1].replace(" ","").len() + 1) as i32;
    let subs = dii[0].as_bytes()
                    .chunks(((row_numbers*4)-1) as usize)
                    .map(|a|str::from_utf8(a))
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap();
    //Rows
    let mut rows: Vec<Vec<&str>> = vec![Vec::new(); row_numbers as usize];
    
    for i in 0..subs.len(){
        let g = subs[i].as_bytes()
                       .chunks(4)
                       .map(|a|str::from_utf8(a))
                       .collect::<Result<Vec<&str>, _>>()
                       .unwrap();   
        for j in 0..g.len(){
            if(g[j].replace(" ", "").len() == 0){
                continue;
            }else{
                rows[j].push(g[j].trim());
            }
            
        }

    }   

    let mut instructions = foo[1].split("move ").collect::<Vec<&str>>();
    instructions.remove(0);

    
    for i in 0..instructions.len(){
        let ab = instructions[i].replace(" to ", " from ");
        let cd = ab.split(" from ").collect::<Vec<&str>>();

        let gh = cd[0].parse::<usize>().unwrap();

        let no1 = (cd[1].parse::<usize>().unwrap() - 1);
        let no2 = (cd[2].parse::<usize>().unwrap() - 1);
        let xyz = rows[no1].len();
        let borrowed: Vec<&str> = rows[no1].split_off(gh);
        let duhf = rows[no1].clone();
        rows[no2].splice(0..0, duhf);
        rows[no1] = borrowed;
        //println!("{:?}",borrowed );        
    }
    
    let mut final_string = String::new();
    let mut tmp = [0u8; 4];
    for k in rows{
        final_string+=k[0].chars().collect::<Vec<char>>()[1].encode_utf8(&mut tmp);
    }

   println!("Final Top Row is (Part 2): {:?}",final_string );
}
