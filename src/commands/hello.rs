use std::error::Error;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        return Err("usage: hello <name> [--yell]".into());
    }

    let mut name = None::<String>;
    let mut yell = false;
    for a in args {
        match a.as_str() {
            "--yell" => yell = true,
            s if name.is_none() => name = Some(s.to_string()),
            _ => return Err("unexpected argument".into()),
        }
    }

    let mut msg = format!("Hello, {}!", name.unwrap());
    if yell {
        msg = msg.to_uppercase();
    }
    println!("{}", msg);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn hello_basic() {
        let args = vec!["Alice".to_string()];
        run(&args).unwrap();
    }

    #[test]
    fn hello_yell() {
        let args = vec!["Bob".to_string(), "--yell".to_string()];
        run(&args).unwrap();
    }
}
