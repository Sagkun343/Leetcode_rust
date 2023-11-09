impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut temp: char = 'Z';
        let mut val: i32 = 1;
        let mut res: i32 = 0;
        let rmod = i32::pow(10, 9) + 7;
        for c in s.chars(){
            if temp == c{
                val += 1;
            }
            else{ 
                temp = c;
                val = 1;
            }
            res += val;
            if res > rmod{
                res -= rmod;
            }
        }
        return res
    }
}