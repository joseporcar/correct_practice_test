use std::{fs, io::Error};
fn main() {
    match (decode_answers(), decode_response()) {
        (Err(e), _) => println!("{e}"),
        (_, Err(e)) => println!("{e}"),
        (Ok(v), Ok(u)) => {
            if v.len() != u.len() {
                println!("there is an unequal number of answers and responses");
                return;
            }
            let mistakes = v.iter().zip(u).enumerate().filter(|x| x.1 .1 != *x.1 .0);
            
            let mut count = 0;
            for m in mistakes {
                println!("{}: correct({}) yours({})", m.0 + 1, m.1.0, m.1.1);
                count += 1;
            }
            println!("You got {}/{} correct. {:.1}%", v.len() - count, v.len(), ((v.len() - count ) as f32 / v.len() as f32) * 100.);
        }
    }
    println!("{:?}", decode_answers());
    println!("{:?}", decode_response());
}
fn decode_answers() -> Result<Vec<char>, Error> {
    fs::read_to_string("answers.txt")?
        .trim()
        .split('\n')
        .map(|s| {
            s.trim()
                .chars()
                .find(|c| c.is_alphabetic())
                .ok_or(Error::other("answer file empty"))
        })
        .collect()
}
fn decode_response() -> Result<Vec<char>, Error> {
    fs::read_to_string("response.txt")?
        .trim()
        .split('\n')
        .map(|s| {
            s.trim()
                .chars()
                .next()
                .ok_or(Error::other("answer file empty"))
        })
        .collect()
}
