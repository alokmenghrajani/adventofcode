// Maybe I'm nuts? I find converting things to Strings to be a little annoying in Rust.
// format!{:?} works but doesn't give me what I want.
//
// So I started going down this rabbit hole. This code assumes we are only dealing with ascii and
// probably creates unnecessary copies of the underlying bytes.

pub trait Sfy {
    fn sfy(&self) -> String;
}

pub trait MutSfy {
    fn sfy(&mut self) -> String;
}

impl Sfy for u8 {
    fn sfy(&self) -> String {
        return (*self as char).to_string();
    }
}

impl Sfy for char {
    fn sfy(&self) -> String {
        return self.to_string();
    }
}

impl Sfy for String {
    fn sfy(&self) -> String {
        return (*self).clone();
    }
}

impl<'a> Sfy for &'a str {
    fn sfy(&self) -> String {
        return self.to_string();
    }
}

impl<T> Sfy for [T]
    where T: Sfy
{
    fn sfy(&self) -> String {
        let v: Vec<String> = self.into_iter().map(|e| e.sfy()).collect();
        return v.join("");
    }
}

impl<T, I: Iterator<Item = T>> MutSfy for I
    where T: Sfy
{
    fn sfy(&mut self) -> String {
        let v: Vec<String> = self.map(|e| e.sfy()).collect();
        return v.join("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sfy() {
        let t = "abc"; // &str
        assert_eq!(t.sfy(), "abc");

        let t = b"haha"; // T[u8]
        assert_eq!(t.sfy(), "haha");

        let mut t = "hello world".chars(); // Iter<char>
        assert_eq!(t.sfy(), "hello world");

        let t = vec!['a' as u8, 'b' as u8, 'c' as u8]; // Vec<u8>
        assert_eq!(t.sfy(), "abc");

        let t = vec!["hello", " ", "world"]; // Vec<&str>
        assert_eq!(t.sfy(), "hello world");
    }
}
