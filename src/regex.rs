use {
    once_cell::sync::Lazy,
    regex::Regex,
    std::{collections::HashMap, sync::Mutex},
};

static REGEX_CACHE: Lazy<Mutex<HashMap<&'static str, &'static Regex>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub(crate) fn get_regex(re: &'static str) -> &'static Regex {
    *REGEX_CACHE
        .lock()
        .unwrap()
        .entry(re)
        .or_insert_with(|| Box::leak(Box::new(Regex::new(re).unwrap())))
}

macro_rules! regex {
    ($re:literal) => {
        crate::regex::get_regex($re)
    };
}

macro_rules! matches_regex {
    ($expr:expr, $re:literal) => {
        regex!($re).is_match($expr)
    };
}
