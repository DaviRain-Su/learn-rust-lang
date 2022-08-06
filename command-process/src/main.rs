use regex::Regex;
use std::process::Command;

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn main() -> anyhow::Result<()> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;

    if !output.status.success() {
        eprintln!("Command executed with failing error code");
    }

    let pattern = Regex::new(
        r"(?x)
                               ([0-9a-fA-F]+) # commit hash
                               (.*)           # The commit message",
    )?;

    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| Commit {
            hash: cap[1].to_string(),
            message: cap[2].trim().to_string(),
        })
        .take(10)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}
