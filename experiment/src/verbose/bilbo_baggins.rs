#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::verbose::convert_rates;
use crate::verbose::present_value;
use crate::verbose::present_value_annuity;
use crate::verbose::future_value;
use crate::verbose::future_value_annuity;
use crate::verbose::payment;

pub fn main() {
    try_bilbo_baggins();
}
// Bilbo Baggins wants save money to meet three objectives. 
// First, he would like to be able to retire {{bilbo.retireInYrs}} years from now, 
// with a retirement income of ${{bilbo.retireIncome}} per month for {{bilbo.retireIncomeYrs}} years,
// with the first payment received {{bilbo.retireInYrs}} years and 1 month from now. 
// Second, he would like to purchase a cabin in Rivendell in {{bilbo.buyCabinInYrs}} years at an estimated cost of ${{bilbo.cabin}}. 
// Third,after he passes on at the end of {{bilbo.retireIncomeYrs}} years of withdrawals, he would like to leave 
// an inheretance of ${{bilbo.inheretance}} to his nephew Frodo. He can afford to save ${{bilbo.savePerMonth1}} per month 
// for the next {{bilbo.buyCabinInYrs}} years. 
// If he can earn a {{bilbo.firstEAR}}% EAR before he retires and a {{bilbo.secondEAR}}% EAR after he retires, 
// how much will he have to save each month in Years {{bilbo.buyCabinInYrs + 1}} through {{bilbo.retireInYrs}}?
fn try_bilbo_baggins() {
    // inputs
    let retire_in_years_from_now: u8 = 30;
    let retirement_income_per_month: f64 = 4_000.;
    let retirement_income_for_how_many_years: u8 = 20;
    let buy_cabin_in_years_from_now: u8 = 20;
    let cost_of_cabin: f64 = 50_000.;
    let inheretance_left_behind: f64 = 200_000.;
    let currently_save_per_month: f64 = 2_500.;
    let effective_annual_rate_before_retire: f64 = 0.045;
    let effective_annual_rate_after_retire: f64 = 0.028;

    // to solve:
    // step one: convert EARs to EPRs (EPR = periodic rate)
    // EPR = (1 + EAR)^(1/#ofPeriodsPerYear) 
    let epr_before_retire = convert_rates::convert_ear_to_periodic(effective_annual_rate_before_retire, 12);
    let epr_after_retire = convert_rates::convert_ear_to_periodic(effective_annual_rate_after_retire, 12);
    dbg!(epr_before_retire);
    dbg!(epr_after_retire);

    // step two: Calculate the Future Value of Annuity of years 0 - &buy_cabin_in_years_from_now
    // Now we need the Future Value of Bilbo's savings cashflow (i.e., Annuity) from years 0 - &buy_cabin_in_years_from_now
    let months_of_savings_before_buying_cabin = buy_cabin_in_years_from_now * 12;
    let fv_savings_before_buying_cabin = future_value_annuity::future_value_annuity(currently_save_per_month, epr_before_retire, months_of_savings_before_buying_cabin as u16);
    dbg!(&fv_savings_before_buying_cabin);

    // step three: how much money do we have at yearX when we buy the cabin?
    let money_remaining_after_cabin_purchase = &fv_savings_before_buying_cabin.future_value_annuity - cost_of_cabin;
    dbg!(&money_remaining_after_cabin_purchase);

    // step four: calculate the Present Value of Bilbo's intended retirement income at year &retire_in_years_from_now
    // thus, use present_value_annuity

    let retirement_income_at_beginning_of_retirement = present_value_annuity::present_value_annuity(retirement_income_per_month, epr_after_retire, (retirement_income_for_how_many_years*12) as u16);
    dbg!(&retirement_income_at_beginning_of_retirement);
    dbg!(&retirement_income_at_beginning_of_retirement.present_value_annuity);
    
    // step five: Calculate the Present Value of the inheretance Bilbo plans to leave his nephew Frodo
    // we can use ear here because the periods are yearly, not monthly
    let inheretance_at_time_of_retirement = present_value::present_value(effective_annual_rate_after_retire, inheretance_left_behind, retirement_income_for_how_many_years as u16);
    dbg!(&inheretance_at_time_of_retirement.present_value);

    // step six: Determine how much Bilbo needs at year &retire_in_years_from_now to acheive his retirement goals
    let how_much_needed_at_moment_of_retirement_to_achieve_goals = &inheretance_at_time_of_retirement.present_value + &retirement_income_at_beginning_of_retirement.present_value_annuity;
    dbg!(&how_much_needed_at_moment_of_retirement_to_achieve_goals);

    // step seven: determine future_value (at time of retirement) of remaining money after buying the cabin
    let years_between_cabin_purchase_and_retirement = retire_in_years_from_now - buy_cabin_in_years_from_now;
    let fv_of_money_after_cabin_purchase_at_retirement = future_value::future_value(effective_annual_rate_before_retire, money_remaining_after_cabin_purchase, years_between_cabin_purchase_and_retirement as u16);
    dbg!(&fv_of_money_after_cabin_purchase_at_retirement);

    // step eight: compare and solve the final problem.
    // bilbo has &fv_of_money_after_cabin_purchase_at_retirement.future_value 
    // and he needs &how_much_needed_at_moment_of_retirement_to_achieve_goals
    // so get the "PMT" (Excel) aka the payment/annuity needed in the years between cabin purchase and retirement.
    let amount_needed = &how_much_needed_at_moment_of_retirement_to_achieve_goals - &fv_of_money_after_cabin_purchase_at_retirement.future_value;
    dbg!(&amount_needed);
    let months_between = (retire_in_years_from_now - buy_cabin_in_years_from_now) * 12;
    dbg!(&months_between);
    fn payment(r: f64, n: u16, fv: f64) -> f64 {
        // C = FV / [ ((1 + i)^n -1) / i ]
        fv / (((1. + r).powi(n as i32) -1.) / r)
    } 
    let monthly_savings_needed = payment(epr_before_retire, months_between as u16, amount_needed);
    dbg!(monthly_savings_needed);
    

    let monthly_savings_needed = payment::payment(epr_before_retire, months_between as u16, 0., amount_needed, false);
    dbg!(monthly_savings_needed);


}