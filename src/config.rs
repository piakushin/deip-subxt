use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Interval {
    None,
    Timer,
    Input,
}

impl FromStr for Interval {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "timer" => Ok(Self::Timer),
            "input" => Ok(Self::Input),
            _ => Err("possible values: none, timer, input"),
        }
    }
}
