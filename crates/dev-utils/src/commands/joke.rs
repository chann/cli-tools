use anyhow::Result;
use rand::seq::SliceRandom;

const JOKES: &[&str] = &[
    "Why do programmers prefer dark mode? Because light attracts bugs.",
    "A SQL query walks into a bar, walks up to two tables, and asks, 'Can I join you?'",
    "How many programmers does it take to change a light bulb? None, that's a hardware problem.",
    "Real programmers count from 0.",
    "Why did the programmer quit his job? Because he didn't get arrays.",
    "To understand what recursion is, you must first understand what recursion is.",
    "!false (It's funny because it's true.)",
    "There are 10 types of people in the world: those who understand binary, and those who don't.",
    "A programmer had a problem. He thought, 'I know, I'll use Java.' Now he has a ProblemFactory.",
    "Why do Java programmers have to wear glasses? Because they don't C#.",
    "Hardware: The part of a computer that you can kick.",
    "An optimist says: 'The glass is half full.' A pessimist says: 'The glass is half empty.' A programmer says: 'The glass is twice as large as it needs to be.'",
];

pub fn random() -> Result<()> {
    let mut rng = rand::thread_rng();
    if let Some(joke) = JOKES.choose(&mut rng) {
        println!("{}", joke);
    }
    Ok(())
}
