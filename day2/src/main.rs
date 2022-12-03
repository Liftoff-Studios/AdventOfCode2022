use std::fs;

fn main() {
    /** Part 1 **/
    let mut text = fs::read_to_string("Codes.txt").unwrap();
    let mut score: i32 = 0;
    for i in text.lines(){

        let s = match i{
            "A X"=>4, /* Rock - Rock*/
            "A Y"=>8, // Rock - Paper
            "A Z"=>3, // Rock - Scissor
            "B X"=>1, // Paper - Rock
            "B Y"=>5, // Paper - Paper
            "B Z"=>9, // Paper - Scissor
            "C X"=>7, // Scissor - Rock
            "C Y"=>2, // Scissor - Paper
            "C Z"=>6, // Scissor - Scissor
            _=>0,
        };
        
        score += s;
    }

    println!("Score is (Part 1): {}",score);


    /** Part 2 **/
    let mut new_score: i32 = 0;
    for i in text.lines(){

        let d = match i{
            "A X"=>3, /* Rock - Lose*/
            "A Y"=>4, // Rock - Draw
            "A Z"=>8, // Rock - Win
            "B X"=>1, // Paper - Lose
            "B Y"=>5, // Paper - Draw
            "B Z"=>9, // Paper - Win
            "C X"=>2, // Scissor - Lose
            "C Y"=>6, // Scissor - Draw
            "C Z"=>7, // Scissor - Win
            _=>0,
        };
        
        new_score += d;
    }
    println!("Updated Score is (Part 2): {}", new_score);
}
