// A mockup interest model 
// TODO: make this in a separate contract and let someone manage this per each collaterized asset 

// constant that needs to be managed
pub static let multiplier_per_block = 0.000000237823;
pub static let base_rate_per_block = 0;
pub static let jump_multipier_per_block = 0.000000518455;
pub static let kink = 0.8

fn get_utilization_rate(cash: &Uint128, borrows: &Uint128, reserves: &Uint128) -> f64 {
    if (borrows == 0) {
        return 0;
    }
    borrows/((cash + borrows) - reserves)?
}


fn get_borrow_rate(cash: &Uint128, borrows: &Uint128, reserves: &Uint128) -> f64 {
    let util = get_utilization_rate(cash, borrows, reserves);

    if (util <= kink) {
        return util * multiplier_per_block + base_rate_per_block);
    } else {
        let normal_rate = kink * multiplier_per_block + base_rate_per_block;
        let excess_util = (util - kink)?;
        return excess_util * jump_multiplier_per_block) + normal_rate;
    }
}

fn get_supply_rate(cash: &Uint128, borrows, &Uint128, reserves: &Uint128, reserve_factor: &f64) -> f64 {
    let one_minus_reserve_factor = 1 - reserve_factor;
    let borrow_rate = get_borrow_rate(cash, borrows, reserves);
    let rate_to_pool = borrow_rate * one_minus_reserve_factor;
    get_utilization_rate(cash, borrows, reserves) * rate_to_pool
}