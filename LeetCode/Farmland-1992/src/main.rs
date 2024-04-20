struct Solution(Vec<Vec<i32>>);

impl Solution {
    pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();

        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if land[i][j] == 1 {

                    let mut x = i;
                    let mut y = j;

                    // Check the right and bottom of the land
                    while x < land.len() && land[x][j] == 1 {
                        // land[x][j] = 2;
                        x += 1;
                    }

                    // Check the right and bottom of the land
                    while y < land[0].len() && land[i][y] == 1 {
                        // land[i][y] = 2;
                        y += 1;
                    }

                    // Update the land
                    for a in i..x {
                        for b in j..y {
                            land[a][b] = 2;
                        }
                    }

                    res.push(vec![i as i32, j as i32, x as i32 - 1, y as i32 - 1]);
                }
            }
        }
        res
    }

}




fn main() {

    let land = vec![vec![1, 0, 0],
                                  vec![0, 1, 1],
                                  vec![0, 1, 1]];
    let res = Solution::find_farmland(land);
    println!("{:?}", res);

    let land = vec![vec![1, 1],
                                  vec![1, 1]];
    let res = Solution::find_farmland(land);
    println!("{:?}", res);
}
