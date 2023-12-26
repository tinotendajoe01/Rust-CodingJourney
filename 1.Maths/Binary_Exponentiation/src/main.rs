use binary_exponentiation::compound_interest;

fn main() {
    let principal = 1000.0; // For example, $1000
    let annual_rate = 0.05; // 5% annual interest
    let comp_per_year = 12; // Compounded monthly
    let years = 5; // For 5 years

    let future_value = compound_interest(principal, annual_rate, comp_per_year, years);
    println!("Future value of the investment: {}", future_value);
}
