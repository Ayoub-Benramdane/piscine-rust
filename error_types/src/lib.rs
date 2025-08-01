 use chrono::Utc;
#[derive(Debug, Eq, PartialEq)]
pub struct FormError <'a> {
   pub form_values: (&'a str, String),
   pub date: String,
   pub err: &'a str,
}

impl FormError <'static> {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let now = Utc::now();
        Self {
            form_values: (field_name, field_value),
            date: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }
        if self.password.len() < 8 {
            return Err(
                FormError::new(
                    "password",
                    self.name.clone(),
                    "Password should be at least 8 characters long"
                )
            );
        } else {
            let valid = {
                let mut alpha = false;
                let mut digit = false;
                let mut sym = false;

                for c in self.password.chars() {
                    if c.is_ascii_alphabetic() {
                        alpha = true;
                    } else if c.is_ascii_digit() {
                        digit = true;
                    } else {
                        sym = true;
                    }
                }
                alpha && digit && sym
            };

            if !valid {
                return Err(
                    FormError::new(
                        "password",
                        self.password.clone(),
                        "Password should be a combination of ASCII numbers, letters and symbols"
                    )
                );
            };
        }
        Ok(())
    }
}
