
pub fn main() {
//     // BilboBagginsProblem
//     // Bilbo Baggins wants save money to meet three objectives. 
//     // First, he would like to be able to retire {{bilbo.retireInYrs}} years from now, 
//     // with a retirement income of ${{bilbo.retireIncome}} per month for {{bilbo.retireIncomeYrs}} years,
//     // with the first payment received {{bilbo.retireInYrs}} years and 1 month from now. 
//     // Second, he would like to purchase a cabin in Rivendell in {{bilbo.buyCabinInYrs}} years at an estimated cost of ${{bilbo.cabin}}. 
//     // Third,after he passes on at the end of {{bilbo.retireIncomeYrs}} years of withdrawals, he would like to leave 
//     // an inheritance of ${{bilbo.inheritance}} to his nephew Frodo. He can afford to save ${{bilbo.savePerMonth1}} per month 
//     // for the next {{bilbo.buyCabinInYrs}} years. 
//     // If he can earn a {{bilbo.firstEAR}}% EAR before he retires and a {{bilbo.secondEAR}}% EAR after he retires, 
//     // how much will he have to save each month in Years {{bilbo.buyCabinInYrs + 1}} through {{bilbo.retireInYrs}}?
    
//     // Inputs here are listed in order that occured in the word problem.
//     let retire_in_years_from_now= 30;
//     let retirement_income_per_month: f64 = 4_000.;
//     let retirement_income_for_how_many_years = 20;
//     let buy_cabin_in_years_from_now= 20;
//     let cost_of_cabin: f64 = 50_000.;
//     let inheritance_left_behind: f64 = 200_000.;
//     let currently_save_per_month: f64 = 2_500.;
//     let effective_annual_rate_before_retire: f64 = 0.045;
//     let effective_annual_rate_after_retire: f64 = 0.028;
    
//     // to solve:
//         // step one: convert EARs to EPRs (EPR = periodic rate)
//         // EPR = (1 + EAR)^(1/#ofPeriodsPerYear) 
//         let epr_before_retire = convert_rate::convert_ear_to_epr(b.effective_annual_rate_before_retire, 12);
//         let epr_after_retire = convert_rate::convert_ear_to_epr(b.effective_annual_rate_after_retire, 12);
//         // dbg!(epr_before_retire);
//         // dbg!(epr_after_retire);
    
//         // step two: Calculate the Future Value of Annuity of years 0 - &buy_cabin_in_years_from_now
//         // Now we need the Future Value of Bilbo's savings cashflow (i.e., Annuity) from years 0 - &buy_cabin_in_years_from_now
//         let months_of_savings_before_buying_cabin = b.buy_cabin_in_years_from_now * 12;
//         let fv_savings_before_buying_cabin: FutureValueAnnuitySolution = future_value_annuity(b.currently_save_per_month, epr_before_retire, months_of_savings_before_buying_cabin);
//         // dbg!(&fv_savings_before_buying_cabin);
    
//         // step three: how much money do we have at yearX when we buy the cabin?
//         let money_remaining_after_cabin_purchase = &fv_savings_before_buying_cabin.future_value_annuity - b.cost_of_cabin;
//         // dbg!(&money_remaining_after_cabin_purchase);
    
//         // step four: calculate the Present Value of Bilbo's intended retirement income at year &retire_in_years_from_now
//         // thus, use present_value_annuity
    
//         let retirement_income_at_beginning_of_retirement: PresentValueAnnuitySolution = present_value_annuity(b.retirement_income_per_month, epr_after_retire, b.retirement_income_for_how_many_years*12);
//         // dbg!(&retirement_income_at_beginning_of_retirement);
//         // dbg!(&retirement_income_at_beginning_of_retirement.present_value_annuity);
        
//         // step five: Calculate the Present Value of the inheritance Bilbo plans to leave his nephew Frodo
//         // we can use ear here because the periods are yearly, not monthly
//         let inheritance_at_time_of_retirement = finance::present_value_solution(b.effective_annual_rate_after_retire, b.retirement_income_for_how_many_years, b.inheritance_left_behind);
//         // dbg!(&inheritance_at_time_of_retirement.present_value);
    
//         // step six: Determine how much Bilbo needs at year &retire_in_years_from_now to acheive his retirement goals
//         let how_much_needed_at_moment_of_retirement_to_achieve_goals = &inheritance_at_time_of_retirement.present_value + &retirement_income_at_beginning_of_retirement.present_value_annuity;
//         // dbg!(&how_much_needed_at_moment_of_retirement_to_achieve_goals);
    
//         // step seven: determine future_value (at time of retirement) of remaining money after buying the cabin
//         let years_between_cabin_purchase_and_retirement = b.retire_in_years_from_now - b.buy_cabin_in_years_from_now;
//         let fv_of_money_after_cabin_purchase_at_retirement = finance::future_value_solution (
//             b.effective_annual_rate_before_retire,
//             years_between_cabin_purchase_and_retirement,
//             money_remaining_after_cabin_purchase);
//         // dbg!(&fv_of_money_after_cabin_purchase_at_retirement);
    
//         // step eight: compare and solve the final problem.
//         // bilbo has &fv_of_money_after_cabin_purchase_at_retirement.future_value 
//         // and he needs &how_much_needed_at_moment_of_retirement_to_achieve_goals
//         // so get the "PMT" (Excel) aka the payment/annuity needed in the years between cabin purchase and retirement.
//         let net_amount_needed_at_retirement = &how_much_needed_at_moment_of_retirement_to_achieve_goals - &fv_of_money_after_cabin_purchase_at_retirement.future_value;
//         // dbg!(&net_amount_needed_at_retirement);
//         let months_between_cabin_and_retirement = (b.retire_in_years_from_now - b.buy_cabin_in_years_from_now) * 12;
//         // dbg!(&months_between_cabin_and_retirement);
        
//         fn payment(r: f64, n: u32, fv: f64) -> f64 {
//             // C = FV / [ ((1 + i)^n -1) / i ]
//             fv / (((1. + r).powi(n as i32) -1.) / r)
//         }
//         // let monthly_savings_needed_after_cabin = payment(epr_before_retire, months_between_cabin_and_retirement, net_amount_needed_at_retirement);
//         let monthly_savings_needed_after_cabin = finance::payment(epr_before_retire, months_between_cabin_and_retirement, 0.0, net_amount_needed_at_retirement);
//         // dbg!(&monthly_savings_needed_after_cabin);
    
    
//         // let monthly_savings_needed_after_cabin: PaymentSolution = super::payment::payment(epr_before_retire, months_between_cabin_and_retirement, 0., net_amount_needed_at_retirement);
//         let monthly_savings_needed_after_cabin: finance::TvmCashflowSolution = finance::payment_solution(epr_before_retire, months_between_cabin_and_retirement, 0., net_amount_needed_at_retirement);
//         // dbg!(&monthly_savings_needed_after_cabin);
}