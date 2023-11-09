impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        if (sx, sy) == (fx, fy) && t == 1{
            return false
        }
        let d_x: i32 = (sx - fx).abs();
        let d_y: i32 = (sy - fy).abs();
        if d_x > d_y{
            return t>= d_x
        }
        else{ return t >= d_y}
    }
}