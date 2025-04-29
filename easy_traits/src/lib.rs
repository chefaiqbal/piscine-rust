#[derive(Clone, Debug, PartialEq)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;
    fn append_number(&mut self, nb_to_append: f64) -> Self;
    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self {
        self.value.push_str(&str_to_append);
        self.clone()
    }

    fn append_number(&mut self, nb_to_append: f64) -> Self {
        self.value = format!("{}{}", self.value, nb_to_append);
        self.clone()
    }

    fn remove_punctuation_marks(&mut self) -> Self {
        self.value = self.value
            .chars()
            .filter(|&c| !matches!(c, '!' | '.' | ',' | '?'))
            .collect();
        self.clone()
    }
}