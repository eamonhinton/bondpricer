fn bond_pricer(face: f32, coup_rate: f32, disc_rate: f32, years: f32, freq: f32) -> f32 {
    let coup_rate = coup_rate / freq;
    let new_discr: f32 = 1.0 + (disc_rate / freq);
    let periods_to_maturity = years * freq;
    let discount_factor = new_discr.recip(); // Equivalent to 1.0 / new_discr

    let mut price: f32 = 0.0;
    let mut discounted_cash_flow: f32 = 1.0;

    for _ in 1..=periods_to_maturity as i32 {
        discounted_cash_flow *= discount_factor; // Accumulate the power of the discount factor
        price += (face * coup_rate) * discounted_cash_flow;
    }
    
    price += face * discounted_cash_flow;
    price
}

fn main() {
    bond_pricer(100.0 , 0.03875, 0.03, 10.0, 2.0);
}
