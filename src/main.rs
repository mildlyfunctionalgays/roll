use rand::Rng;
use std::io;
use std::io::stdin;
use std::io::BufRead;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let stdin = stdin();
    let lock = stdin.lock();
    for line in lock.lines() {
        let line = line?;
        let value: usize = line
            .split('+')
            .map(|s: &str|
                s.split('*')
                    .map(|s: &str| {
                        let parts: Vec<&str> = s.split('d').collect();
                        let count = usize::from_str(parts[0]).unwrap_or(1);
                        let die = parts.get(1).map_or(1, |&s| usize::from_str(s).unwrap());
                        (0..count)
                            .map(|_| rand::thread_rng().gen_range(1, die + 1))
                            .sum::<usize>()
                    })
                    .product::<usize>()
            )
            .sum();
        println!("{} = {}", line, value);
    }
    Ok(())
}
