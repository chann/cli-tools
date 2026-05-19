use anyhow::Result;
use rand::seq::SliceRandom;
use cli_core::ui::Theme;

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
    "A programmer is told to 'go to hell'. He finds the worst part of that is the lack of version control.",
    "Why was the JavaScript developer sad? Because he didn't Know How To 'null' his feelings.",
    "In order to understand recursion, one must first understand recursion.",
    "Documentation is like sex; when it's good, it's very, very good, and when it's bad, it's better than nothing.",
    "Why do C++ programmers always get lost? Because they can't find their headers.",
    "A guy walks into a bar and asks for a beer. The bartender says, 'That'll be 5 dollars.' The guy gives him 5 dollars and the bartender gives him a beer. The guy then asks for another beer. The bartender says, 'That'll be 5 dollars.' The guy gives him 5 dollars and the bartender gives him a beer. This goes on for 10 beers. The guy then asks for another beer. The bartender says, 'That'll be 5 dollars.' The guy says, 'Wait, I've already had 10 beers, I should get a discount!' The bartender says, 'No, this is a bar, not a subscription service!'",
    "Why did the functional programmer get thrown out of the bar? Because he kept asking for 'just one more' without any side effects.",
    "A programmer's wife tells him, 'Go to the store and get a loaf of bread. If they have eggs, get a dozen.' He returns with 12 loaves of bread.",
];

pub fn random() -> Result<()> {
    let mut rng = rand::thread_rng();
    if let Some(joke) = JOKES.choose(&mut rng) {
        println!("\n{} {}", Theme::info("Programmer Joke:"), Theme::highlight(joke));
    }
    Ok(())
}
