use anyhow::Result;
use rand::{thread_rng, Rng, seq::SliceRandom};
use owo_colors::OwoColorize;

const ADJECTIVES: &[&str] = &[
    "abandoned", "able", "absolute", "adorable", "adventurous", "academic", "acceptable", "acclaimed", "accomplished",
    "accurate", "aching", "acidic", "acrobatic", "active", "actual", "adept", "admirable", "admired", "adolescent",
    "adorable", "adored", "advanced", "afraid", "affectionate", "aged", "aggravating", "aggressive", "agile", "agitated",
    "agonizing", "agreeable", "ajar", "alarmed", "alarming", "alert", "alienated", "alive", "all", "altruistic", "amazing",
    "ambitious", "ample", "amused", "amusing", "anchored", "ancient", "angelic", "angry", "anguished", "animated", "annual",
    "another", "antique", "anxious", "any", "apprehensive", "appropriate", "apt", "arctic", "arid", "aromatic", "artistic",
    "ashamed", "assured", "astonishing", "athletic", "attached", "attentive", "attractive", "austere", "authentic",
    "authorized", "automatic", "avaricious", "average", "aware", "awesome", "awful", "awkward", "babyish", "bad", "back",
    "baggy", "bare", "barren", "basic", "beautiful", "belated", "beloved", "beneficial", "better", "best", "bewildered",
    "big", "bigger", "biggest", "binary", "bitter", "black", "bland", "blank", "blaring", "bleak", "blind", "blissful",
    "blond", "blue", "blushing", "bogus", "boiling", "bold", "bony", "boring", "bossy", "both", "bouncy", "bountiful",
    "bowed", "brave", "breakable", "brief", "bright", "brilliant", "brisk", "broken", "bronze", "brown", "bruised",
    "bubbly", "bulky", "bumpy", "buoyant", "burly", "bustling", "busy", "buttery", "buzzing",
];

const NOUNS: &[&str] = &[
    "apple", "arm", "banana", "bird", "boat", "book", "bottle", "box", "boy", "brain", "bridge", "brother", "brush",
    "bus", "bush", "button", "cake", "camera", "canvas", "car", "card", "carrot", "case", "cat", "chain", "chair",
    "chalk", "chart", "cheese", "chest", "chick", "church", "circle", "city", "clock", "cloud", "coat", "coin", "comb",
    "cord", "cow", "cup", "curtain", "desk", "dog", "doll", "door", "drain", "drawer", "dress", "drop", "ear", "egg",
    "engine", "eye", "face", "farm", "feather", "finger", "fish", "flag", "floor", "fly", "foot", "fork", "fowl", "frame",
    "garden", "girl", "glass", "glove", "goat", "goose", "grass", "hammer", "hand", "hat", "head", "heart", "hook", "horn",
    "horse", "hospital", "house", "island", "jewel", "kettle", "key", "knee", "knife", "knot", "leaf", "leg", "library",
    "line", "lip", "lock", "map", "match", "monkey", "moon", "mouth", "muscle", "nail", "neck", "needle", "nerve", "net",
    "nose", "nut", "office", "orange", "oven", "parcel", "pen", "pencil", "picture", "pig", "pin", "pipe", "plane", "plate",
    "plow", "pocket", "pot", "potato", "prison", "pump", "rail", "rat", "receipt", "ring", "river", "road", "roof", "root",
    "rose", "route", "sail", "school", "scissors", "screw", "seed", "sheep", "shelf", "ship", "shirt", "shoe", "skin",
    "skirt", "snake", "sock", "spade", "sponge", "spoon", "spring", "square", "stamp", "star", "station", "stem", "stick",
    "stocking", "stomach", "store", "street", "sun", "table", "tail", "thread", "throat", "thumb", "ticket", "toe",
    "tongue", "tooth", "town", "train", "tray", "tree", "trousers", "umbrella", "wall", "watch", "wheel", "whip", "whistle",
    "window", "wing", "wire", "worm",
];

pub fn generate(length: usize, use_numbers: bool, use_symbols: bool, use_uppercase: bool, use_lowercase: bool) -> Result<()> {
    let mut charset = String::new();
    let mut pools = 0;
    
    if use_lowercase {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
        pools += 26;
    }
    if use_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        pools += 26;
    }
    if use_numbers {
        charset.push_str("0123456789");
        pools += 10;
    }
    if use_symbols {
        charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
        pools += 26;
    }

    if charset.is_empty() {
        charset.push_str("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
        pools = 62;
    }

    let mut rng = thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    println!("{}", password.bright_white().bold());
    
    let entropy = (length as f64) * (pools as f64).log2();
    println!("{} {:.2} bits", "Entropy:".dimmed(), entropy.cyan());
    
    Ok(())
}

pub fn generate_passphrase(words_count: usize) -> Result<()> {
    let mut rng = thread_rng();
    let mut words = Vec::new();
    
    for i in 0..words_count {
        let word = if i % 2 == 0 {
            ADJECTIVES.choose(&mut rng).unwrap()
        } else {
            NOUNS.choose(&mut rng).unwrap()
        };
        words.push(*word);
    }
    
    let passphrase = words.join("-");
    println!("{}", passphrase.bright_white().bold());
    
    let pool_size = ADJECTIVES.len() + NOUNS.len();
    let entropy = (words_count as f64) * (pool_size as f64).log2();
    println!("{} {:.2} bits", "Entropy:".dimmed(), entropy.cyan());
    
    Ok(())
}

pub fn check(password: &str) -> Result<()> {
    let mut score = 0;
    let length = password.len();
    
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_numeric = password.chars().any(|c| c.is_numeric());
    let has_symbols = password.chars().any(|c| !c.is_alphanumeric());

    if length >= 8 { score += 1; }
    if length >= 12 { score += 1; }
    if length >= 16 { score += 1; }
    if has_lower { score += 1; }
    if has_upper { score += 1; }
    if has_numeric { score += 1; }
    if has_symbols { score += 1; }

    let mut pool_size = 0;
    if has_lower { pool_size += 26; }
    if has_upper { pool_size += 26; }
    if has_numeric { pool_size += 10; }
    if has_symbols { pool_size += 32; } // Approximate
    
    if pool_size == 0 { pool_size = 1; }
    let entropy = (length as f64) * (pool_size as f64).log2();

    println!("{} {}", "Password:".dimmed(), password.bright_white());
    println!("{} {} characters", "Length:".dimmed(), length.yellow());
    println!("{} {:.2} bits", "Entropy:".dimmed(), entropy.cyan());
    
    print!("{}: ", "Strength".bold());
    let (label, color_func): (&str, Box<dyn Fn(String) -> String>) = match score {
        0..=3 => ("Very Weak", Box::new(|s| s.red().to_string())),
        4 => ("Weak", Box::new(|s| s.yellow().to_string())),
        5 => ("Medium", Box::new(|s| s.blue().to_string())),
        6 => ("Strong", Box::new(|s| s.green().to_string())),
        _ => ("Very Strong", Box::new(|s| s.bright_green().bold().to_string())),
    };
    
    println!("{}", color_func(label.to_string()));

    // Visual meter
    let meter_width = 20;
    let filled = (score as f64 / 7.0 * meter_width as f64) as usize;
    let bar = format!("{}{}", "■".repeat(filled), "□".repeat(meter_width - filled));
    
    println!("[{}]", color_func(bar));
    
    if score < 5 {
        println!("\n{}", "Suggestions:".yellow().bold());
        if length < 12 { println!("  - Make it longer (at least 12 characters)"); }
        if !has_upper { println!("  - Add uppercase letters"); }
        if !has_numeric { println!("  - Add numbers"); }
        if !has_symbols { println!("  - Add special characters"); }
    }

    Ok(())
}
