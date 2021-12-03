mod day_1;

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
        _ => return Err(format!("{} is an invalid day", day).into()),
    }

    Ok(())
}
