use anyhow::Result;
use rand::seq::SliceRandom;

const LOREM_IPSOMS: &[&str] = &[
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
    "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
    "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
    "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
    "Curabitur pretium tincidunt lacus. Nulla gravida orci a odio.",
    "Nullam varius, turpis et commodo pharetra, est eros bibendum elit, nec luctus magna felis sollicitudin mauris.",
    "Integer in mauris eu nibh euismod gravida.",
    "Duis ac tellus et risus vulputate vehicula.",
    "Donec lobortis risus a elit. Etiam tempor.",
    "Ut ullamcorper, ligula eu tempor congue, eros est euismod turpis, id tincidunt sapien risus a quam.",
    "Maecenas fermentum consequat mi. Donec fermentum. Pellentesque malesuada nulla a mi.",
    "Duis sapien nunc, commodo et, interdum suscipit, sollicitudin et, dolor.",
    "Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.",
    "Aliquam id diam maecenas ultricies mi eget mauris.",
];

pub fn generate(paragraphs: usize) -> Result<()> {
    let mut rng = rand::thread_rng();
    
    for i in 0..paragraphs {
        let mut p = Vec::new();
        let sentence_count = rand::random::<usize>() % 5 + 3;
        
        for _ in 0..sentence_count {
            if let Some(s) = LOREM_IPSOMS.choose(&mut rng) {
                p.push(*s);
            }
        }
        
        println!("{}", p.join(" "));
        if i < paragraphs - 1 {
            println!();
        }
    }
    
    Ok(())
}
