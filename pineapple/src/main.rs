use std::io;
use std::io::prelude::*;

fn main() {
    // flex
    println!("Hello World!");

    // get user input
    let mut line = String::new();
    println!("Enter Groups separated by a space:");
    println!("eg. \"quizes tests\"");
    std::io::stdin().read_line(&mut line).unwrap();

    // loop through each
    let mut total: f64 = 0.0;
    let groups: Vec<&str> = line.trim_end().split(" ").collect();
    for group in groups {
        let mut n_str = String::new();
        let mut weight = String::new();
        
        print!("How many {} are there? ", group);
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut n_str).unwrap();
        let n: i32 = n_str.trim_end().parse().unwrap();
        print!("What is the weight of this group:");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut weight).unwrap();
        let w: f64 = weight.trim_end().parse().unwrap();

        // get each in group
        println!("What did you get on (55/65): ");
        let mut grades_got_tot: f64 = 0.0;
        let mut grades_out_tot: f64 = 0.0;
        for i in 0..n {
            let mut grade = String::new();
            print!("{}{}:", group, i);
            std::io::stdin().read_line(&mut grade).unwrap();
            grades_got_tot += &grade[..1].parse().unwrap();
            grades_out_tot += &grade[1..].parse().unwrap();
        }

        total += grades_got_tot/grades_out_tot*w;
    }

    println!("Total: {}", total);
}