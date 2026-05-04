use anyhow::Result;
use fake::faker::name::en as name;
use fake::faker::internet::en as internet;
use fake::faker::address::en as address;
use fake::faker::company::en as company;
use fake::faker::job::en as job;
use fake::faker::phone_number::en as phone;
use fake::faker::name::raw::*;
use fake::faker::address::raw::*;
use fake::locales::EN;
use fake::Fake;
use rand::{seq::SliceRandom, Rng};

pub fn generate(kind: &str, count: usize, ko: bool) -> Result<()> {
    let mut rng = rand::thread_rng();
    
    let ko_names = ["김철수", "이영희", "박지민", "최민수", "정소연", "강현우", "조예진", "윤도현", "장미래", "한지석"];
    let ko_cities = ["서울", "부산", "인천", "대구", "대전", "광주", "울산", "수원", "성남", "고양"];
    let ko_companies = ["(주)가나다", "삼성전자", "현대자동차", "카카오", "네이버", "쿠팡", "배달의민족", "토스", "직방", "당근마켓"];

    for _ in 0..count {
        let result = if ko {
            match kind.to_lowercase().as_str() {
                "name" => ko_names.choose(&mut rng).unwrap().to_string(),
                "city" => ko_cities.choose(&mut rng).unwrap().to_string(),
                "company" => ko_companies.choose(&mut rng).unwrap().to_string(),
                "address" => format!("{} {}", ko_cities.choose(&mut rng).unwrap(), StreetName(EN).fake::<String>()),
                "phone" => format!("02-{}-{}", rng.gen_range(1000..9999), rng.gen_range(1000..9999)),
                "cell-phone" => format!("010-{}-{}", rng.gen_range(1000..9999), rng.gen_range(1000..9999)),
                _ => {
                    // Fallback to EN for other types
                    match kind.to_lowercase().as_str() {
                        "first-name" => FirstName(EN).fake::<String>(),
                        "last-name" => LastName(EN).fake::<String>(),
                        _ => anyhow::bail!("Unsupported fake data kind for Korean: {}", kind),
                    }
                }
            }
        } else {
            match kind.to_lowercase().as_str() {
                "name" => name::Name().fake::<String>(),
                "first-name" => name::FirstName().fake::<String>(),
                "last-name" => name::LastName().fake::<String>(),
                "email" => internet::SafeEmail().fake::<String>(),
                "username" => internet::Username().fake::<String>(),
                "password" => internet::Password(8..16).fake::<String>(),
                "ipv4" => internet::IPv4().fake::<String>(),
                "ipv6" => internet::IPv6().fake::<String>(),
                "mac" => internet::MACAddress().fake::<String>(),
                "user-agent" => internet::UserAgent().fake::<String>(),
                "address" => format!("{} {}", address::BuildingNumber().fake::<String>(), address::StreetName().fake::<String>()),
                "city" => address::CityName().fake::<String>(),
                "country" => address::CountryName().fake::<String>(),
                "zip" => address::ZipCode().fake::<String>(),
                "company" => company::CompanyName().fake::<String>(),
                "industry" => company::Industry().fake::<String>(),
                "profession" => job::Field().fake::<String>(),
                "title" => job::Title().fake::<String>(),
                "phone" => phone::PhoneNumber().fake::<String>(),
                "cell-phone" => phone::CellNumber().fake::<String>(),
                _ => anyhow::bail!("Unsupported fake data kind: {}. Supported: name, first-name, last-name, email, username, password, ipv4, ipv6, mac, user-agent, address, city, country, zip, company, industry, profession, title, phone, cell-phone", kind),
            }
        };
        println!("{}", result);
    }
    Ok(())
}
