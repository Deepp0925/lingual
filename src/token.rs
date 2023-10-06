use std::borrow::Cow;

use crate::errors::Errors;

pub(crate) fn generate_token<S: AsRef<str>>(text: S) -> Result<String, Errors> {
    let b = tkk().0;

    let mut d = vec![];
    let actual_text_len = text.as_ref().encode_utf16().count();

    for mut f in 0..actual_text_len {
        let a: Vec<u16> = text.as_ref().encode_utf16().collect();
        let mut g = a[f] as i64;

        if 128 > g {
            d.push(g)
        } else {
            if 2048 > g {
                d.push(g >> 6 | 192);
            } else {
                if (55296 == (g & 64512))
                    && (f + 1 < actual_text_len)
                    && (56320 == (a[f + 1] & 64512))
                {
                    f += 1;
                    g = 65536 + ((g & 1023) << 10) + (a[f] & 1023) as i64;
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

    let mut a = b.parse::<i64>().map_err(|_| Errors::ParseIntErr)?;
    for c in d {
        a += c;
        a = wr(a, "+-a^+6").ok_or(Errors::ParseIntErr)?;
    }

    a = wr(a, "+-3^+b+-f").ok_or(Errors::ParseIntErr)?;
    a ^= tkk().1;

    if 0 > a {
        a = (a & 2147483647) + 2147483648;
    }

    a %= 1e6 as i64;

    Ok(format!(
        "{}.{}",
        a,
        a ^ b.parse::<i64>().map_err(|_| Errors::ParseIntErr)?
    ))
}

const fn tkk() -> (&'static str, i64) {
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
            (a << d.parse::<i64>().ok()?).to_string().into()
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
    let m;

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
