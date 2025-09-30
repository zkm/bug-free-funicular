use std::error::Error;
use std::fs;
use std::path::Path;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.len() != 1 {
        return Err("usage: read <path>".into());
    }
    let path = Path::new(&args[0]);
    let content = fs::read_to_string(path)?;
    let bytes = content.as_bytes().len();
    let lines = content.lines().count();
    let preview = content.lines().next().unwrap_or("");
    println!(
        "{} bytes, {} lines. First line: {}",
        bytes,
        lines,
        if preview.is_empty() { "<empty>" } else { preview }
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::fs;

    #[test]
    fn read_usage_error() {
        let args: Vec<String> = vec![];
        assert!(run(&args).is_err());
    }

    #[test]
    fn read_tempfile() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("sample.txt");
        fs::write(&file_path, "hello\nworld\n").unwrap();
        let args = vec![file_path.to_string_lossy().to_string()];
        run(&args).unwrap();
    }
}
