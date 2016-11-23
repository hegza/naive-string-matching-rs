
// TODO: move to another file
/*
pub fn rk_match_cpu(text: &[u8], pattern: &[u8]) -> i32 {
    let d = 4;
    let q = 13; // d*q should be less than word length (64?) and a prime
    let n = text.len();
    let m = pattern.len();
    let h = d.pow(m-1) % q;
    let mut p = 0;
    let mut t = 0;
    
    // Preprocessing
    for i in 1..m+1 {
        p = (d*p + pattern[i]) % q;
        t = (d*t + text[i]) % q;
    }
    // Matching
    for s in 0..n-m+1 {
        if p == t {
            if pattern[1..m] == text[s..s+m] {
                return s;
            }
        }
        if s < n-m {
            t = (d(t-text[s]*h)+text[s+m]) % q;
        }

    }
}
*/

