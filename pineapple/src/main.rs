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
    let mut total: f32 = 0.0;
    let groups: Vec<&str> = line.trim_end().split(" ").collect();

    // no dicts :/
    let mut groups_w: Vec<i32> =  Vec::new();
    let mut groups_got: Vec<Vec<String>> =  Vec::new();
    let mut groups_out: Vec<Vec<String>> =  Vec::new();

    let mut check_desire: bool = false;

    for group in groups {
        let mut weight = String::new();
        let mut grade_str = String::new();
        
        print!("What is the weight of this group:");
        io::stdout().flush().unwrap(); // print to screen NOW
        std::io::stdin().read_line(&mut weight).unwrap(); // get input for weight
        groups_w.push(weight.trim_end().parse().unwrap()); // convert input to i32 and add to "dict"

        let mut grades_got: Vec<String> = Vec::new();
        let mut grades_out: Vec<String> = Vec::new();

        print!("Enter grades separated by space (50/60)");
        print!("If you do not know put \"*\"");
        print!("eg. final1: */100");
        print!("{}:", group);
        io::stdout().flush().unwrap(); // print all print! to screen NOW
        std::io::stdin().read_line(&mut grade_str).unwrap(); // get input of grades

        let grades = grade_str.trim_end().split(" "); // create vector (i think) of grades
        for grade_pair in grades {
            let halves:Vec<&str> = grade_pair.split("/").collect(); // split by delim of "/"
            grades_got.push(halves[0].to_string()); // create deep clone (i think)
            grades_out.push(halves[1].to_string()); // (75/100) <- this part
        }

        // add to "dict"
        groups_got.push(grades_got);
        groups_out.push(grades_out);
    }

    for d in &groups_got {
        if d.contains(&"*".to_string()) { // im beginning to hate this language
            check_desire = true;
            break; // dont need to keep checking
        }
    }

    if check_desire {
        let mut desired = String::new();
        print!("What grade do you desire (eg. 70): ");
        io::stdout().flush().unwrap(); // print out the buffer NOW
        std::io::stdin().read_line(&mut desired).unwrap(); // get input
        let f: i32 = desired.trim_end().parse().unwrap();

        let mut d: f32 = 0.0;
        let mut group_unknown_i = 0;
        for (i, di) in groups_got.iter().enumerate() {
            if di.contains(&"*".to_string()) {
                group_unknown_i = i;
            } else {
                let int_got: Vec<i32> = di.iter().map(|x| x.parse::<i32>().unwrap()).collect();
                let int_out: Vec<i32> = groups_out[i].iter().map(|x| x.parse::<i32>().unwrap()).collect();
                d += (int_got.iter().sum::<i32>() / int_out.iter().sum::<i32>() * (groups_w[i]/100)) as f32;
            }
        }

        let cs: Vec<i32> = groups_out[group_unknown_i].iter().map(|x| x.parse::<i32>().unwrap()).collect();
        let c: i32 = cs.iter().sum::<i32>();

        let mut a: i32 = 0;
        for grade in &groups_got[group_unknown_i] {
            if grade != "*" {
                a += grade.trim_end().parse::<i32>().unwrap();
            }
        }

        let b = groups_w[group_unknown_i];
        let star = ((((f) as f32 - d) * (c) as f32) / (b) as f32) - (a) as f32;
        println!("You need a {} on the * to get an {}", star, desired);
        if star < 0.0 {
            println!("You can not get a LOW enough score to get that grade");
        } else if star > 100.0 {
            println!("You can not get a HIGH enough score to get that grade");
        } 
    } else { // if there is no star and you are solving for total (because youre boring)
        for (i, di) in groups_got.iter().enumerate() {
            let int_got: Vec<i32> = di.iter().map(|x| x.parse::<i32>().unwrap()).collect();
            let int_out: Vec<i32> = groups_out[i].iter().map(|x| x.parse::<i32>().unwrap()).collect();
            total += (int_got.iter().sum::<i32>() / int_out.iter().sum::<i32>() * (groups_w[i]/100)) as f32;
        }
        println!("total: {}", total);
    }
}