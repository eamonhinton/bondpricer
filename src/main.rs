fn bond_pricer(face: f32, coup_rate: f32, disc_rate: f32, years: f32, freq: f32) -> f32 {
    let mut price:f32 = 0.0;
    let coup_rate = coup_rate / freq;
    let new_discr: f32 = 1.0 + (disc_rate / freq);
    let periods_to_maturity = years * freq;
    for payment_num in 1..=periods_to_maturity as i32{
        price += (face * coup_rate) / new_discr.powf(payment_num as f32);
    }
    price += face / new_discr.powf(periods_to_maturity);
    return price;
}

fn main() {
    bond_pricer(37603.0 , 0.05, 0.01,3.0, 4.0);
}