mod day_1;
mod day_10;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err(format!("{} requires exactly 2 args", args[0]).into());
    }

    let day = args[1].parse::<i32>()?;
    let problem = args[2].parse::<i32>()?;
    match day {
        1 => match problem {
            1 => println!("{}", day_1::prob_1("input/day_1.txt")?),
            2 => println!("{}", day_1::prob_2("input/day_1.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        2 => match problem {
            1 => println!("{}", day_2::prob_1("input/day_2.txt")?),
            2 => println!("{}", day_2::prob_2("input/day_2.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        3 => match problem {
            1 => println!("{}", day_3::prob_1("input/day_3.txt")?),
            2 => println!("{}", day_3::prob_2("input/day_3.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        4 => match problem {
            1 => println!("{}", day_4::prob_1("input/day_4.txt")?),
            2 => println!("{}", day_4::prob_2("input/day_4.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        5 => match problem {
            1 => println!("{}", day_5::prob_1("input/day_5.txt")?),
            2 => println!("{}", day_5::prob_2("input/day_5.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        6 => match problem {
            1 => println!("{}", day_6::prob_1("input/day_6.txt")?),
            2 => println!("{}", day_6::prob_2("input/day_6.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        7 => match problem {
            1 => println!("{}", day_7::prob_1("input/day_7.txt")?),
            2 => println!("{}", day_7::prob_2("input/day_7.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        8 => match problem {
            1 => println!("{}", day_8::prob_1("input/day_8.txt")?),
            2 => println!("{}", day_8::prob_2("input/day_8.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        9 => match problem {
            1 => println!("{}", day_9::prob_1("input/day_9.txt")?),
            2 => println!("{}", day_9::prob_2("input/day_9.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        10 => match problem {
            1 => println!("{}", day_10::prob_1("input/day_10.txt")?),
            2 => println!("{}", day_10::prob_2("input/day_10.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        _ => return Err(format!("{} is an invalid day", day).into()),
    }

    Ok(())
}
