
/*
 * https://exercism.io/tracks/rust/exercises/leap/solutions/8e94607e2a69474e86b757c596969299
 * The match tuple solution that others have called 'Rusty'. 
 * Looks like a matrix. Harder to read as a noob. 
 * There's a couple not so immediately obvious things going on here. 
 * Of great importance is the order of the matches. 
 * 
 * In the (,0,) line we don't care about two of the results, 
 * because the 400 case was already checked before we return 
 * false for divisible by 100. If we make it to the last match 
 * arm, clearly it's not a leap year.
 */
pub fn is_leap_year(year: u64) -> bool {
    match (year % 400, year % 100, year % 4) {
        (0, _, _) => true,
        (_, 0, _) => false,
        (_, _, 0) => true,
        (_, _, _) => false
    }
}
