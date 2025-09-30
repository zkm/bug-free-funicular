use std::error::Error;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        return Err("usage: sum <n1> <n2> ...".into());
    }

    let mut total: i64 = 0;
    for a in args {
        let n: i64 = a.parse()?;
        total += n;
    }
    println!("{}", total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn sum_basic() {
        let args = vec!["1".to_string(), "2".to_string(), "-3".to_string()];
        run(&args).unwrap();
    }
}
