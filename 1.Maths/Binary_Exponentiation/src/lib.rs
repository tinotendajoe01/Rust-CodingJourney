pub fn binary_exponentiation_compound_interest(mut base: f64, mut exponent: u64) -> f64 {
    let mut result: f64 = 1.0;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result *= base;
        }
        base *= base;
        exponent >>= 1;
    }
    result
}

pub fn compound_interest(principal: f64, annual_rate: f64, comp_per_year: u64, years: u64) -> f64 {
    let rate_per_period = annual_rate / comp_per_year as f64;
    let periods = comp_per_year * years;
    principal * binary_exponentiation_compound_interest(1.0 + rate_per_period, periods)
}
