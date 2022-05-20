mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
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
        11 => match problem {
            1 => println!("{}", day_11::prob_1("input/day_11.txt")?),
            2 => println!("{}", day_11::prob_2("input/day_11.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        12 => match problem {
            1 => println!("{}", day_12::prob_1("input/day_12.txt")?),
            2 => println!("{}", day_12::prob_2("input/day_12.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        13 => match problem {
            1 => println!("{}", day_13::prob_1("input/day_13.txt")?),
            2 => println!("{}", day_13::prob_2("input/day_13.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        14 => match problem {
            1 => println!("{}", day_14::prob_1("input/day_14.txt")?),
            2 => println!("{}", day_14::prob_2("input/day_14.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        15 => match problem {
            1 => println!("{}", day_15::prob_1("input/day_15.txt")?),
            2 => println!("{}", day_15::prob_2("input/day_15.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        16 => match problem {
            1 => println!("{}", day_16::prob_1("input/day_16.txt")?),
            2 => println!("{}", day_16::prob_2("input/day_16.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        17 => match problem {
            1 => println!("{}", day_17::prob_1("input/day_17.txt")?),
            2 => println!("{}", day_17::prob_2("input/day_17.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        18 => match problem {
            1 => println!("{}", day_18::prob_1("input/day_18.txt")?),
            2 => println!("{}", day_18::prob_2("input/day_18.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        19 => match problem {
            1 => println!("{}", day_19::prob_1("input/day_19.txt")?),
            2 => println!("{}", day_19::prob_2("input/day_19.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        20 => match problem {
            1 => println!("{}", day_20::prob_1("input/day_20.txt")?),
            2 => println!("{}", day_20::prob_2("input/day_20.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        21 => match problem {
            1 => println!("{}", day_21::prob_1("input/day_21.txt")?),
            2 => println!("{}", day_21::prob_2("input/day_21.txt")?),
            _ => return Err(format!("{} is an invalid problem", problem).into()),
        },
        _ => return Err(format!("{} is an invalid day", day).into()),
    }

    Ok(())
}
