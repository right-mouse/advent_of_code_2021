mod day_1;
mod day_2;
mod day_3;
mod day_7;

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
        7 => match problem {
            1 => println!("{}", day_7::prob_1("input/day_7.txt")?),
            2 => println!("{}", day_7::prob_2("input/day_7.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        _ => return Err(format!("{} is an invalid day", day).into()),
    }

    Ok(())
}
