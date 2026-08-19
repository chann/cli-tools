mod commands;
use crate::commands::Commands;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "dev-tools",
    version,
    about = "Common developer utility tools",
    long_about = "A collection of small, frequently used utilities for developers. \
                  Includes UUID generation, Base64 encoding, URL encoding, and more."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Completion { shell } => {
            commands::completion::run(shell)?;
        }
        Commands::Ascii { text, font } => {
            commands::ascii::generate(&text, font)?;
        }
        Commands::Serve { path, port } => {
            commands::serve::run(path, port).await?;
        }
        Commands::Uuid { count, v7, inspect } => {
            if let Some(id) = inspect {
                commands::uuid::inspect(&id)?;
            } else {
                commands::uuid::generate(count, v7)?;
            }
        }
        Commands::Base64 { input, decode, file, output, data_uri, url_safe } => {
            if decode {
                commands::base64::decode(&input, output, url_safe)?;
            } else {
                commands::base64::encode(&input, file, data_uri, url_safe)?;
            }
        }
        Commands::Url { text, decode } => {
            if decode {
                commands::url::decode_url(&text)?;
            } else {
                commands::url::encode_url(&text)?;
            }
        }
        Commands::Json {
            text,
            check,
            format,
            minify,
            sort,
            query,
            yaml,
            toml,
            csv,
            schema,
        } => {
            if check {
                commands::json::validate(&text)?;
                println!("Valid JSON");
            } else if yaml {
                commands::json::to_yaml(&text)?;
            } else if toml {
                commands::json::to_toml(&text)?;
            } else if csv {
                commands::json::to_csv(&text)?;
            } else if schema {
                commands::json::to_schema(&text)?;
            } else {
                let pretty = format || !minify;
                let output =
                    commands::json::transform(&text, pretty, query.as_deref(), sort)?;
                println!("{output}");
            }
        }
        Commands::Port { port, kill, wait, timeout, list } => {
            if list {
                commands::port::list()?;
            } else if let Some(p) = port {
                if kill {
                    commands::port::kill(p)?;
                } else if wait {
                    commands::port::wait(p, timeout)?;
                } else {
                    commands::port::check(p)?;
                }
            } else {
                anyhow::bail!("Port number is required for this operation or use --list");
            }
        }
        Commands::Hash { input, algo, file, compare } => {
            if let Some(c) = compare {
                commands::hash::compare(&input, &c, &algo, file)?;
            } else if file {
                commands::hash::hash_file(std::path::Path::new(&input), &algo)?;
            } else {
                commands::hash::hash_string(&input, &algo)?;
            }
        }
        Commands::Time { input } => {
            if let Some(val) = input {
                commands::time::convert(&val)?;
            } else {
                commands::time::current()?;
            }
        }
        Commands::Tree { path, depth, size, git } => {
            commands::tree::print_tree(&path, depth, size, git)?;
        }
        Commands::Ip { target } => {
            commands::ip::show(target).await?;
        }
        Commands::Random { kind, count, length, min, max, numeric, symbols, uppercase } => {
            commands::random::generate(&kind, count, length, min, max, numeric, symbols, uppercase)?;
        }
        Commands::Case { text, to } => {
            commands::case::convert(&text, to.as_deref())?;
        }
        Commands::Jwt { token, sign, payload, secret } => {
            if sign {
                if let (Some(p), Some(s)) = (payload, secret) {
                    commands::jwt::sign(&p, &s)?;
                } else {
                    anyhow::bail!("Sign operation requires --payload and --secret");
                }
            } else if let Some(t) = token {
                commands::jwt::inspect(&t, secret.as_deref())?;
            } else {
                anyhow::bail!("JWT token is required for inspection or use --sign");
            }
        }
        Commands::Env { filter } => {
            commands::env::list(filter)?;
        }
        Commands::Sys => {
            commands::sys::show()?;
        }
        Commands::Contrast { foreground, background } => {
            commands::contrast::run(&foreground, &background)?;
        }
        Commands::Color { input } => {
            commands::color::convert(&input)?;
        }
        Commands::Size { value, unit } => {
            commands::size::convert(value, &unit)?;
        }
        Commands::Lorem { count, kind } => {
            commands::lorem::generate(count, &kind)?;
        }
        Commands::Password { length, passphrase, words, no_numbers, no_symbols, no_uppercase, no_lowercase, check } => {
            if let Some(pwd) = check {
                commands::password::check(&pwd)?;
            } else if passphrase {
                commands::password::generate_passphrase(words)?;
            } else {
                commands::password::generate(length, !no_numbers, !no_symbols, !no_uppercase, !no_lowercase)?;
            }
        }
        Commands::Http { method, url, body, header, output, verbose } => {
            commands::http::request(method, url, body, header, output, verbose).await?;
        }
        Commands::Qr { text, output, level, size } => {
            commands::qr::generate(&text, output, &level, size)?;
        }
        Commands::Path { path, resolve } => {
            if resolve {
                commands::path::resolve(&path)?;
            } else {
                commands::path::normalize(&path)?;
            }
        }
        Commands::Yaml { text, json, toml } => {
            if json {
                commands::yaml::to_json(&text)?;
            } else if toml {
                commands::yaml::to_toml(&text)?;
            } else {
                commands::yaml::format(&text)?;
            }
        }
        Commands::Toml { text, json, yaml } => {
            if json {
                commands::toml::to_json(&text)?;
            } else if yaml {
                commands::toml::to_yaml(&text)?;
            } else {
                commands::toml::format(&text)?;
            }
        }
        Commands::Text { text, op, reverse, unique, from, to, line_numbers, prefix, suffix, truncate } => {
            commands::text::process(
                &text,
                &op,
                reverse,
                unique,
                from,
                to,
                line_numbers,
                prefix,
                suffix,
                truncate,
            )?;
        }
        Commands::Cron { expression } => {
            commands::cron::explain(&expression)?;
        }
        Commands::Crontab { action } => {
            use commands::CrontabAction;
            match action.unwrap_or(CrontabAction::List) {
                CrontabAction::List => commands::crontab::list()?,
                CrontabAction::Add { schedule, command, comment } => {
                    commands::crontab::add(&schedule, &command, comment.as_deref())?;
                }
                CrontabAction::Remove { index } => commands::crontab::remove(index)?,
                CrontabAction::Edit { index, schedule, command } => {
                    commands::crontab::edit(index, schedule.as_deref(), command.as_deref())?;
                }
            }
        }
        Commands::DateDiff { from, to } => {
            commands::date_diff::run(&from, to.as_deref())?;
        }
        Commands::Toc { file, min_depth, max_depth } => {
            commands::toc::run(&file, min_depth, max_depth)?;
        }
        Commands::Tz { query } => {
            commands::tz::show(query.as_deref())?;
        }
        Commands::Unicode { text } => {
            commands::unicode::inspect(&text)?;
        }
        Commands::Diff { old, new, file } => {
            commands::diff::compare(&old, &new, file)?;
        }
        Commands::Regex { pattern, text } => {
            commands::regex::test(&pattern, &text)?;
        }
        Commands::Escape { text, kind, unescape } => {
            match kind.as_str() {
                "html" => {
                    if unescape {
                        commands::escape::html_unescape(&text)?;
                    } else {
                        commands::escape::html_escape(&text)?;
                    }
                }
                "string" => {
                    commands::escape::string_escape(&text)?;
                }
                _ => anyhow::bail!("Unsupported escape kind: {}", kind),
            }
        }
        Commands::Base { value, from, to, all } => {
            commands::base::convert(&value, from, to, all)?;
        }
        Commands::Checksum { input, algo, file, check } => {
            if let Some(c) = check {
                commands::checksum::verify(&input, &c, &algo, file)?;
            } else {
                commands::checksum::calculate(&input, &algo, file)?;
            }
        }
        Commands::Hexview { input, file } => {
            commands::hexview::view(&input, file)?;
        }
        Commands::Dns { domain, record } => {
            commands::dns::lookup(&domain, &record).await?;
        }
        Commands::Gitignore { list, targets } => {
            if list {
                commands::gitignore::list().await?;
            } else {
                commands::gitignore::generate(targets).await?;
            }
        }
        Commands::License { list, kind, year, holder } => {
            if list {
                commands::license::list()?;
            } else if let Some(k) = kind {
                commands::license::generate(&k, year, &holder)?;
            } else {
                anyhow::bail!("Please specify a license kind or use --list");
            }
        }
        Commands::Stat { input, file } => {
            commands::stat::analyze(&input, file)?;
        }
        Commands::Whois { domain } => {
            commands::whois::lookup(&domain)?;
        }
        Commands::Cert { host, port, file } => {
            if let Some(h) = host {
                commands::cert::inspect_remote(&h, port).await?;
            } else if let Some(f) = file {
                commands::cert::inspect_file(&f)?;
            } else {
                anyhow::bail!("Either host or file must be provided");
            }
        }
        Commands::Unit { value, from, to } => {
            commands::unit::convert(value, &from, &to)?;
        }
        Commands::Currency { amount, from, to } => {
            commands::currency::convert(amount, &from, &to).await?;
        }
        Commands::Ulid { count, inspect } => {
            if let Some(id) = inspect {
                commands::ulid::inspect(&id)?;
            } else {
                commands::ulid::generate(count)?;
            }
        }
        Commands::Nanoid { length, count, alphabet } => {
            commands::nanoid::generate(length, count, alphabet.as_deref())?;
        }
        Commands::Totp { secret, digits, skew, generate_secret } => {
            if generate_secret {
                commands::totp::generate_new_secret()?;
            } else if let Some(s) = secret {
                commands::totp::generate(&s, digits, skew)?;
            } else {
                anyhow::bail!("Secret key is required for TOTP generation, or use --generate-secret");
            }
        }
        Commands::Csv { input, json, yaml, markdown } => {
            commands::csv::convert(&input, json, yaml, markdown)?;
        }
        Commands::Shorten { url } => {
            commands::shorten::shorten(&url).await?;
        }
        Commands::Extract { input, kind, file } => {
            commands::extract::extract(&input, &kind, file)?;
        }
        Commands::Sql { query, file, indent, tabs, lowercase } => {
            let sql = if let Some(f) = file {
                std::fs::read_to_string(f)?
            } else {
                query.unwrap_or_default()
            };
            commands::sql::format(&sql, indent, tabs, lowercase)?;
        }
        Commands::Slug { text } => {
            commands::slug::generate(&text)?;
        }
        Commands::Chmod { input } => {
            commands::chmod::calculate(&input)?;
        }
        Commands::Base32 { input, decode, file, output } => {
            if decode {
                commands::base32::decode_base32(&input, output)?;
            } else {
                commands::base32::encode_base32(&input, file)?;
            }
        }
        Commands::Base58 { input, decode, file, output } => {
            if decode {
                commands::base58::decode_base58(&input, output)?;
            } else {
                commands::base58::encode_base58(&input, file)?;
            }
        }
        Commands::Base85 { input, decode, file, output } => {
            if decode {
                commands::base85::decode_base85(&input, output)?;
            } else {
                commands::base85::encode_base85(&input, file)?;
            }
        }
        Commands::Punycode { input, decode } => {
            if decode {
                commands::punycode::decode_puny(&input)?;
            } else {
                commands::punycode::encode_puny(&input)?;
            }
        }
        Commands::Hmac { text, key, algo } => {
            commands::hmac::calculate(&text, &key, &algo)?;
        }
        Commands::Binary { input, from } => {
            if from {
                commands::binary::from_binary(&input)?;
            } else {
                commands::binary::to_binary(&input)?;
            }
        }
        Commands::UserAgent { ua } => {
            if let Some(val) = ua {
                commands::user_agent::inspect(&val)?;
            } else {
                commands::user_agent::generate()?;
            }
        }
        Commands::Mime { input, extension } => {
            if extension {
                commands::mime::from_extension(&input)?;
            } else {
                commands::mime::guess(&input)?;
            }
        }
        Commands::Joke => {
            commands::joke::random()?;
        }
        Commands::Secret { length, count, kind } => {
            commands::secret::generate(length, count, &kind)?;
        }
        Commands::Snowflake { inspect, count } => {
            if let Some(id) = inspect {
                commands::snowflake::inspect(id)?;
            } else {
                commands::snowflake::generate(count)?;
            }
        }
        Commands::Semver { text, op, increment, compare } => {
            match op.as_str() {
                "parse" => commands::semver::parse(&text)?,
                "increment" => {
                    if let Some(part) = increment {
                        commands::semver::increment(&text, &part)?;
                    } else {
                        anyhow::bail!("Increment operation requires --increment <major|minor|patch>");
                    }
                }
                "compare" => {
                    if let Some(other) = compare {
                        commands::semver::compare(&text, &other)?;
                    } else {
                        anyhow::bail!("Compare operation requires --compare <version>");
                    }
                }
                _ => anyhow::bail!("Unsupported semver operation: {}", op),
            }
        }
        Commands::Ksuid { inspect, count } => {
            if let Some(id) = inspect {
                commands::ksuid::inspect(&id)?;
            } else {
                commands::ksuid::generate(count)?;
            }
        }
        Commands::Xml { text, minify } => {
            commands::xml::format(&text, !minify)?;
        }
        Commands::Uptime { urls } => {
            commands::uptime::check_multiple(urls).await?;
        }
        Commands::Subnet { cidr } => {
            commands::subnet::run(&cidr)?;
        }
        Commands::Portscan { host, start, end, timeout } => {
            commands::portscan::scan(&host, start, end, timeout).await?;
        }
        Commands::Dotenv { op, path, example } => {
            match op.as_str() {
                "init" => commands::dotenv::init(&path)?,
                "example" => commands::dotenv::example(&path)?,
                "load" => commands::dotenv::load(&path)?,
                "compare" => commands::dotenv::compare(&path, &example)?,
                _ => anyhow::bail!("Unsupported dotenv operation: {}", op),
            }
        }
        Commands::Mask { text, kind } => {
            commands::mask::process(&text, &kind)?;
        }
        Commands::Fake { kind, count, ko } => {
            commands::fake::generate(&kind, count, ko)?;
        }
        Commands::Mac { local, count } => {
            if local {
                commands::mac::show_local()?;
            } else {
                commands::mac::generate(count)?;
            }
        }
        Commands::Bcrypt { password, hash, cost } => {
            if let Some(h) = hash {
                commands::bcrypt::verify_password(&password, &h)?;
            } else {
                commands::bcrypt::hash_password(&password, cost)?;
            }
        }
        Commands::Compress { input, output, decompress } => {
            if decompress {
                commands::compress::gunzip(&input, &output)?;
            } else {
                commands::compress::gzip(&input, &output)?;
            }
        }
        Commands::HttpStatus { code, list, search } => {
            if list {
                commands::http_status::list()?;
            } else if let Some(q) = search {
                commands::http_status::search(&q)?;
            } else if let Some(c) = code {
                commands::http_status::lookup(c)?;
            } else {
                anyhow::bail!("Status code is required, or use --list or --search");
            }
        }
        Commands::AsciiTable => {
            commands::ascii_table::show()?;
        }
        Commands::UrlParse { url } => {
            commands::url_parse::parse(&url)?;
        }
        Commands::Github { issue, pr, user, repo, search } => {
            if let Some(u) = user {
                commands::github::get_user(&u).await?;
            } else if let Some(r) = repo {
                commands::github::get_repo(&r).await?;
            } else if let Some(q) = search {
                commands::github::search(&q).await?;
            } else {
                commands::github::open(issue, pr)?;
            }
        }
        Commands::Crates { query } => {
            commands::crates::search(&query).await?;
        }
        Commands::Typescript { text, name } => {
            commands::typescript::from_json(&text, &name)?;
        }
        Commands::Weather { location } => {
            commands::weather::get_weather(location).await?;
        }
        Commands::Dictionary { word } => {
            commands::dictionary::lookup(&word).await?;
        }
        Commands::Translate { text, to } => {
            commands::translate::translate(&text, &to).await?;
        }
        Commands::Bench { command, args, count } => {
            commands::bench::run(&command, args, count)?;
        }
        Commands::Whoami => {
            commands::whoami::show()?;
        }
        Commands::JsonDiff { left, right } => {
            commands::json_diff::compare(&left, &right)?;
        }
        Commands::Rust { text, name } => {
            commands::rust::from_json(&text, &name)?;
        }
        Commands::Go { text, name } => {
            commands::go::from_json(&text, &name)?;
        }
        Commands::Java { text, name } => {
            commands::java::from_json(&text, &name)?;
        }
        Commands::Md { input } => {
            commands::md::render(&input)?;
        }
        Commands::Speedtest => {
            commands::speedtest::run().await?;
        }
        Commands::Morse { text, decode } => {
            if decode {
                commands::morse::decode(&text)?;
            } else {
                commands::morse::encode(&text)?;
            }
        }
        Commands::Cheat { query } => {
            commands::cheat::run(&query).await?;
        }
        Commands::Scan { path, min_size, duplicates, empty, dirs, links, all } => {
            let mut opts = commands::scan::ScanOptions {
                duplicates,
                empty,
                dirs,
                links,
                min_size,
            };
            if all || (!duplicates && !empty && !dirs && !links) {
                opts.duplicates = true;
                opts.empty = true;
                opts.dirs = true;
                opts.links = true;
            }
            commands::scan::run(&path, opts)?;
        }
        Commands::CheckLinks { path } => {
            commands::check_links::run(&path).await?;
        }
        Commands::Detach { command, args } => {
            commands::detach::run(&command, args)?;
        }
        Commands::Silent { command, args } => {
            commands::silent::run(&command, args)?;
        }
        Commands::Image { input, output, format, info, thumbnail, width, height, blur, rotate, flip_h, flip_v, grayscale, invert, crop, brighten, contrast, hue } => {
            commands::image::process(&input, output, format, info, thumbnail, width, height, blur, rotate, flip_h, flip_v, grayscale, invert, crop, brighten, contrast, hue)?;
        }
    }

    Ok(())
}