use std::{fmt::{Display, Write}, str::FromStr};

use anyhow::{anyhow, ensure};

#[derive(Debug, Clone, Copy)]
pub enum Key {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Key {
    pub fn chars(&self) -> &'static [char] {
        match self {
            Key::Two => &['a', 'b', 'c'],
            Key::Three => &['d', 'e', 'f'],
            Key::Four => &['g', 'h', 'i'],
            Key::Five => &['j', 'k', 'l'],
            Key::Six => &['m', 'n', 'o'],
            Key::Seven => &['p', 'q', 'r', 's'],
            Key::Eight => &['t', 'u', 'v'],
            Key::Nine => &['w', 'x', 'y', 'z'],
        }
    }
}

macro_rules! try_from_num {
    ($type:ty) => {
        impl TryFrom<$type> for Key {
            type Error = anyhow::Error;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                let key = match value {
                    9 => Self::Nine,
                    8 => Self::Eight,
                    7 => Self::Seven,
                    6 => Self::Six,
                    5 => Self::Five,
                    4 => Self::Four,
                    3 => Self::Three,
                    2 => Self::Two,
                    _ => return Err(anyhow!("key must be between 2..=9")),
                };

                Ok(key)
            }
        }
    };
}

try_from_num!(u8);
try_from_num!(u16);
try_from_num!(u32);
try_from_num!(u64);

try_from_num!(i8);
try_from_num!(i16);
try_from_num!(i32);
try_from_num!(i64);

impl TryFrom<char> for Key {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        ensure!(
            matches!(value, '2'..='9'),
            "key must be ascii numeric between 2..=9"
        );

        let key = value.to_digit(10).unwrap();

        Ok(key.try_into().unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct Keys(Vec<Key>);

impl Display for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &key in self.0.iter() {
            let c = char::from_digit(key as u32, 10).unwrap();
            f.write_char(c)?;
        }

        Ok(())
    }
}

impl FromStr for Keys {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keys: Vec<Key> = s.chars().map(|c| c.try_into()).try_collect()?;

        Ok(Keys(keys))
    }
}
