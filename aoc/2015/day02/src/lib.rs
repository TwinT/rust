pub struct Solution {
    pub paper: u32,
    pub ribbon: u32,
}

struct Prism {
    l: u32,
    w: u32,
    h: u32,
}

impl Prism {
    fn from(vec: Vec<&str>) -> Prism {
        Prism {
            l: vec[0].parse().unwrap(),
            w: vec[1].parse().unwrap(),
            h: vec[2].parse().unwrap(),
        }
    }

    fn sides(&self) -> Vec<u32> {
        let v = vec![self.l * self.w, self.w * self.h, self.h * self.l];
        v
    }

    fn perimeters(&self) -> Vec<u32> {
        let mut v = Vec::new();
        v.push(2 * (self.l + self.w));
        v.push(2 * (self.w + self.h));
        v.push(2 * (self.l + self.h));
        v
    }

    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }
}

pub fn solve(input: &str) -> Solution {
    let v: Vec<&str> = input.split("x").collect();
    let prism = Prism::from(v);

    let area: u32 = prism.sides().iter().map(|x| 2 * x).sum();

    let paper = area + prism.sides().iter().min().unwrap();
    let ribbon = prism.perimeters().iter().min().unwrap() + prism.volume();

    Solution { paper, ribbon }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(solve("2x3x4").paper, 58);
        assert_eq!(solve("2x3x4").ribbon, 34);
    }

    #[test]
    fn test_1() {
        assert_eq!(solve("1x1x10").paper, 43);
        assert_eq!(solve("1x1x10").ribbon, 14);
    }
}
