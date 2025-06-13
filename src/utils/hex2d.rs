pub struct Hex2d {
    pub q: f64,
    pub r: f64,
    pub radius: f64,
}

impl Hex2d
{
    pub fn s(&self) -> f64 {
        return -self.q - self.r;
    }

    pub fn x(&self) -> f64 {

        return 3.0f64.sqrt() * (self.q + 0.5 * self.r);
    }

    pub fn y(&self) -> f64 {
        return 1.5 * self.r;
    }
}

impl Hex2d
{
    // var results = []
    // for each -N ≤ q ≤ +N:
    //     for each max(-N, -q-N) ≤ r ≤ min(+N, -q+N):
    //         var s = -q-r
    //         results.append(cube_add(center, Cube(q, r, s)))
    pub fn range(&self, radius: i32) -> Vec<Hex2d> {
        let mut results = Vec::new();
        for q_i32 in -radius..radius+1 {
            let q = q_i32 as f64;
            let min = std::cmp::max(-radius, -q_i32 - radius);
            let max = std::cmp::min(radius, -q_i32 + radius);
            for r_i32 in min..max+1 {
                let r = r_i32 as f64;
                results.push(Hex2d { q: q * self.radius, r: r * self.radius, radius: self.radius});
            }
        }
        return results;
    }
}

impl ToString for Hex2d
{
    fn to_string(&self) -> String {
        return format!("{},{},{}", self.q, self.r, self.s());
    }
}
