use models::Document;
use regex::Regex;

pub struct Validator;

impl Validator {
    pub fn is_valid_email(email: &String) -> bool {
        let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
        re.is_match(email)
    }

    pub fn is_valid_password(pswd: &String) -> bool {
        let re = Regex::new(r".{8,}").unwrap();
        re.is_match(pswd)
    }

    fn is_number(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_digit())
    }

    pub fn is_valid_passport(passport: &Document) -> bool {
        let mut is_correct = false;
        if passport.serial.len() == 4 && passport.number.len() == 6 {
            if Self::is_number(&passport.serial) && Self::is_number(&passport.number) {
                is_correct = true;
            }
        }
        is_correct
    }

    pub fn is_valid_gos_num(gos_num: &String) -> bool {
        let re = Regex::new(r"^[АВЕКМНОРСТУХ]\d{3}[АВЕКМНОРСТУХ]{2}\d{2,3}$").unwrap();
        re.is_match(gos_num)
    }

    pub fn is_valid_gos_num_mask(gos_num_mask: &String) -> bool {
        let re = Regex::new(r"^([АВЕКМНОРСТУХ*])(\d|\*){3}([АВЕКМНОРСТУХ*]{2})(\d{2,3}|\*{1})$")
            .unwrap();
        re.is_match(gos_num_mask)
    }

    pub fn is_valid_date(date: &String) -> bool {
        let re = Regex::new(r"^\d{2}\.\d{2}\.\d{4}$").unwrap();
        re.is_match(date)
    }
    pub fn is_valid_time(time: &String) -> bool {
        let re = Regex::new(r"^\d{1,2}\:\d{2}$").unwrap();
        if !re.is_match(time) {
            return false;
        }

        let parts: Vec<&str> = time.split(':').collect();
        let hours: u32 = parts[0].parse().unwrap_or(24);
        let minutes: u32 = parts[1].parse().unwrap_or(60);

        hours < 24 && minutes < 60
    }
}
