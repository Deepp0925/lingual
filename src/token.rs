use std::{
    borrow::Cow,
    num::{IntErrorKind, ParseIntError},
};

pub enum Errors {
    ParseIntErr,
}

pub fn generate_token<S: AsRef<str>>(mut text: S) -> Result<String, Errors> {
    let _tkk = tkk();
    let b = _tkk.0;

    let mut d = vec![];

    for f in 0..text.as_ref().len() {
        let mut g: u32 = text.as_ref().as_bytes()[f].into();
        if g < 128 {
            d.push(g);
        } else {
            if g < 2048 {
                d.push(g >> 6 | 192);
            } else {
                if (g & 64512) == 55296
                    && f + 1 < text.as_ref().len()
                    && (text.as_ref().as_bytes()[f + 1] as u16 & 64512) == 56320
                {
                    g = 65536 + ((g & 1023) << 10) + (text.as_ref().as_bytes()[f] as u32 & 1023);
                    d.push(g >> 18 | 240);
                    d.push(g >> 12 & 63 | 128);
                } else {
                    d.push(g >> 12 | 224);
                }
                d.push(g >> 6 & 63 | 128);
            }
            d.push(g & 63 | 128);
        }
    }

    let mut a = b.parse::<u32>().map_err(|_| Errors::ParseIntErr)? + d[0];

    for e in 1..d.len() {
        a += d[e];
        a = wr(a.into(), "+-a^+6").ok_or(Errors::ParseIntErr)? as u32;
    }

    todo!()
}

const fn tkk() -> (&'static str, u32) {
    ("406398", 561666268 + 1526272306)
}

fn wr(mut a: i64, b: &str) -> Option<i64> {
    let mut d;

    for c in (0..(b.len() - 2)).step_by(3) {
        d = Cow::Borrowed(&b[c + 2..c + 3]);
        d = if d >= Cow::Borrowed("a") {
            Cow::Owned((d.as_bytes()[0] - 87).to_string())
        } else {
            d
        };

        d = if Cow::Borrowed("+") == b[c + 1..c + 2] {
            unsigned_right_shift(a, d.parse::<i64>().ok()?)
                .to_string()
                .into()
        } else {
            (a << d.parse::<i32>().ok()?).to_string().into()
        };

        a = if Cow::Borrowed("+") == b[c..c + 1] {
            let k = a + d.parse::<i64>().ok()?;
            k & 4294967295
        } else {
            a ^ d.parse::<i64>().ok()?
        };
    }

    Some(a)
}

fn unsigned_right_shift(mut a: i64, mut b: i64) -> i64 {
    let mut m;

    if !(-32..32).contains(&b) {
        m = b / 32;
        b -= m * 32;
    }

    if b < 0 {
        b += 32;
    }

    if b == 0 {
        return ((a >> 1) & 0x7fffffff) * 2 + ((a >> b) & 1);
    }

    if a < 0 {
        a >>= 1;
        a &= 2147483647;
        a |= 0x40000000;
        a >>= b - 1;
    } else {
        a >>= b;
    }

    a
}
