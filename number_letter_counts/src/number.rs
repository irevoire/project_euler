fn reverse(n: u32) -> u32 {
    let mut rev = 0;
    let mut n = n;

    loop {
        let digit = n % 10;
        rev = (rev * 10) + digit;
        n = n / 10;

        if n == 0 {
            break
        }
    }
    return rev
}

pub struct Number {
    num: std::collections::HashMap<u32, String>,
}

impl Number {
    pub fn new() -> Number {
        let mut s = Number { num: std::collections::HashMap::new() };

        s.num.insert(0, "".to_string());
        s.num.insert(1, "one".to_string());
        s.num.insert(2, "two".to_string());
        s.num.insert(3, "three".to_string());
        s.num.insert(4, "four".to_string());
        s.num.insert(5, "five".to_string());
        s.num.insert(6, "six".to_string());
        s.num.insert(7, "seven".to_string());
        s.num.insert(8, "eight".to_string());
        s.num.insert(9, "nine".to_string());
        s.num.insert(10, "ten".to_string());
        s.num.insert(11, "eleven".to_string());
        s.num.insert(12, "twelve".to_string());
        s.num.insert(13, "thirteen".to_string());
        s.num.insert(14, "fourteen".to_string());
        s.num.insert(15, "fifteen".to_string());
        s.num.insert(16, "sixteen".to_string());
        s.num.insert(17, "seventeen".to_string());
        s.num.insert(18, "eighteen".to_string());
        s.num.insert(19, "nineteen".to_string());
        s.num.insert(20, "twenty".to_string());
        s.num.insert(30, "thirty".to_string());
        s.num.insert(40, "forty".to_string());
        s.num.insert(50, "fifty".to_string());
        s.num.insert(60, "sixty".to_string());
        s.num.insert(70, "seventy".to_string());
        s.num.insert(80, "eighty".to_string());
        s.num.insert(90, "ninety".to_string());
        s.num.insert(100, "hundred".to_string());
        s.num.insert(1000, "thousand".to_string());
        
        return s;
    }

    pub fn print(&self) {
        println!("{:?}", self.num);
    }

    pub fn parse(&mut self, n: u32) -> String {
        match self.num.get(&n) {
            Some(s) => return s.to_string(),
            None => (),
        }
        let rev = reverse(n);
        let mut res: String = "".to_owned();

        if n >= 100 {
            let base = rev % 10;
            res.push_str(&self.parse(base));
            res.push_str(" ");
            res.push_str(&self.num.get(&100).unwrap());

            if n % 100 != 0 {
                res.push_str(" and ");
                res.push_str(&self.parse(n % 100));
                res.push_str(" ");
            }
        } else { // 20 < n < 100
            let base = (rev % 10) * 10;
            res.push_str(&self.num.get(&base).unwrap());
            res.push_str("-");

            res.push_str(&self.parse(n % 10));
            res.push_str(" ");
        }

        self.num.insert(n, res);

        return self.parse(n);
    }
}
