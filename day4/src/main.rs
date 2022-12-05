use std::fs;

fn main() {
    //Part 1
    let mut text = fs::read_to_string("Codes.txt").unwrap();
    let mut eclipse: i32 = 0;
    for i in text.lines(){
        let mut t = i.split(",")
                 .collect::<Vec<&str>>();

        // Two variables two hold two different ranges
        let mut u = t[0].split("-").map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut v = t[1].split("-").map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if ((u[0] - v[0] <= 0) && (u[u.len()-1] - v[v.len()-1] >= 0)) || ((v[0] - u[0] <= 0) && (v[v.len()-1] - u[u.len()-1] >= 0)){
            eclipse+=1;
        }
    }
    println!("The number of pairs that totally overlap (Part 1) is: {}", eclipse);


    //Part 2
    let mut foo: i32 = (text.lines().count()) as i32; 
    let mut not_overlap: i32 = 0;
    for i in text.lines(){
        let mut t = i.split(",")
                 .collect::<Vec<&str>>();

        // Two variables two hold two different ranges
        let mut u = t[0].split("-").map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut v = t[1].split("-").map(|x|x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if ((u[u.len()-1] - v[0] < 0) || (u[0] - v[v.len()-1] > 0)) || ((v[v.len()-1] - u[0] < 0) || (v[0] - u[u.len()-1] > 0)){
            not_overlap += 1;
        }
    }

    println!("The number of pairs that partially overlap is: {:?}", foo - not_overlap );
}
