use std::{ collections::HashMap, num::ParseFloatError };

pub struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Self {
            long_hand: format!("--{}", name),
            short_hand: format!("-{}", name.chars().next().unwrap()),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.long_hand, func);
        self.flags.insert(flag.short_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if let Some(op) = self.flags.get(input) {
            if argv.len() > 1 {
                let a = op(argv[0], argv[1]).map_err(|err| err.to_string());
                return Ok(a?);
            }
        }
        return Err("Error!".to_string());
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let new_a: f64 = a.parse()?;
    let new_b: f64 = b.parse()?;
    Ok(format!("{}", new_a / new_b))
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let new_a: f64 = a.parse()?;
    let new_b: f64 = b.parse()?;
    Ok(format!("{}", new_a % new_b))
}
