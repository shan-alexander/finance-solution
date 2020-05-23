#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

// word problems:
// https://www.westga.edu/~rbest/finc3511/sampleprobs/sample5.pdf

// teaching resources:
// https://www.studocu.com/en-us/document/wichita-state-university/financial-management-ii/assignments/time-value-of-money-practice-problems-and-solutions/1915379/view
// http://mupfc.marshall.edu/~brozik/timevalueweb.pdf

// use finance_solution::{present_value_solution, future_value_solution, future_value, periods_solution, periods};
use finance_solution::*;
use num_format::{Locale};

pub fn main() {
    // pv_problem_1();
    // fv_problem_1();
    // nper_problem_1();
    // rate_problem_1();

    // pv_problem_2();
    // fv_problem_2();
    // periods_problem_2();
    // rate_problem_2();

    // pv_annuity_problem_1();
    // fv_annuity_problem_1();

    retirement_problem_1();
}

// If you wish to accumulate $140,000 in 13 years, 
// how much must you deposit today in an account that
// pays an annual interest rate of 14%? 
// Expect $25,489.71
fn pv_problem_1() {
    let future_value = 140_000;
    let periods = 13;
    let rate = 0.14;
    let pv_problem_1 = present_value_solution(rate, periods, future_value, false);
    dbg!(&pv_problem_1); // Outputs solution struct.
    dbg!(pv_problem_1.present_value()); // Outputs 25489.71433359101 from solution struct.
    dbg!(present_value(rate, periods, future_value, false)); // Outputs 25489.71433359101 from f64 fn.
    assert_rounded_2(25_489.71, pv_problem_1.present_value());
}

// What will $247,000 grow to be in 9 years if it is invested today 
// in an account with an annual interest rate of 11%? 
// Expect $631,835.12
fn fv_problem_1() {
    let present_value = 247_000;
    let periods = 9;
    let rate = 0.11;
    let fv_problem_1 = future_value_solution(rate, periods, present_value, false);
    dbg!(&fv_problem_1); // Outputs solution struct.
    dbg!(fv_problem_1.future_value()); // Outputs 631835.1203234661 from solution struct.
    dbg!(future_value(rate, periods, present_value, false)); // Outputs 631835.1203234661 from f64 fn.
    assert_rounded_2(631_835.12, fv_problem_1.future_value());
}

// How many years will it take for $136,000 to grow to be $468,000 
// if it is invested in an account with an annual interest rate of 8%? 
// Expect 16.06 years
fn nper_problem_1() {
    let present_value = 136_000;
    let future_value = 468_000;
    let rate = 0.08;
    let nper_problem_1 = periods_solution(rate, present_value, future_value, false);
    dbg!(&nper_problem_1); // Outputs solution struct.
    dbg!(nper_problem_1.periods()); // Outputs 17 from solution struct.
    dbg!(nper_problem_1.fractional_periods()); // Outputs 16.057649324100133 from solution struct.
    dbg!(periods(rate, present_value, future_value, false)); // Outputs 16.057649324100133 from f64 fn.
    assert_rounded_2(16.06, nper_problem_1.fractional_periods());
}

// At what annual interest rate must $137,000 be invested 
// so that it will grow to be $475,000 in 14 years?
// Expect 9.29%
fn rate_problem_1() { 
    let pv = 137_000;
    let fv = 475_000;
    let periods = 14;
    let rate_problem_1 = rate_solution(periods, pv, fv, false);
    dbg!(&rate_problem_1);
    dbg!(&rate_problem_1.rate());
    assert_rounded_4(0.0929, rate_problem_1.rate());
}

// If you wish to accumulate $197,000 in 5 years, 
// how much must you deposit today in an account that pays
// a quoted annual interest rate of 13% with semi-annual compounding of interest?
// Expect $104,947.03
fn pv_problem_2() {
    let years = 5;
    let apr = 0.13;
    let fv = 197_000;

    // First solve using one period per year.

    // The interest is compounded twice per year so we need to convert the APR to an effective
    // annual rate (EAR) which will be slightly larger.
    let ear = convert_apr_to_ear(apr, 2);
    dbg!(apr, ear);
    assert!(ear > apr);

    // There is one period per year. This works only because we're using the EAR calculated above.
    let periods = years;

    let pv_problem_2_ear = present_value_solution(ear, periods, fv, false);
    dbg!(&pv_problem_2_ear);
    assert_rounded_2(104_947.03, pv_problem_2_ear.present_value());

    // Solve it again using one period for each compounding period which in this case is twice per
    // year.

    // Convert the APR to an effective periodic rate (EPR) which means simply dividing the APR by
    // the number of compounding periods, in this case two.
    let epr = convert_apr_to_epr(apr, 2);
    dbg!(apr, ear, epr);
    assert_approx_equal!(apr / 2.0, epr);

    // The interest is compounded semiannually so the number of compounding periods is twice the
    // number of years.
    let periods = years * 2;

    let pv_problem_2_epr = present_value_solution(epr, periods, fv, false);
    dbg!(&pv_problem_2_epr);
    assert_rounded_2!(104_947.03, pv_problem_2_epr.present_value());

    // Both approaches return the same answer.
    assert_approx_equal!(pv_problem_2_ear.present_value(), pv_problem_2_epr.present_value());
}

// What will $153,000 grow to be in 13 years if it is invested today 
// in an account with a quoted annual interest rate of 10% with monthly compounding of interest?
// Expect $558,386.38
fn fv_problem_2() {
    let pv = 153_000;
    // finish here
}


//  How many years will it take for $197,000 to grow to be $554,000 
// if it is invested in an account with a quoted annual interest rate of 8% with monthly compounding of interest? 
// Expect 12.97 years
fn periods_problem_2() {
    let present_value = 197_000;
    let future_value = 554_000;

    // The annual interest rate is 8%.
    let apr = 0.08;

    // The interest is compounded monthly, so we need to convert the APR into the effective
    // periodic rate per month.
    let rate = convert_apr_to_epr(apr, 12);

    let months = periods(rate, present_value, future_value, false);
    dbg!(months);

    // We know the number of months, which may contain a fractional amount. Convert to years.
    let years = months / 12.0;
    dbg!(years);
}

// At what quoted annual interest rate must $134,000 be invested 
// so that it will grow to be $459,000 in 15 years 
// if interest is compounded weekly? For simplicity assume there are exactly 52 weeks per year.
// Expect 8.21%
fn rate_problem_2() {
    let present_value = 134_000.0;
    let future_value = 459_000.0;
    let years = 15;
    let periods_per_year = 52;
    let periods = years * periods_per_year;

    let weekly_rate = rate(periods, present_value, future_value, false);
    dbg!(weekly_rate);

    let apr = convert_epr_to_apr(weekly_rate, periods_per_year);
    dbg!(apr);

    // Check the answer by calculating a future value as if the interest is compounded once per
    // year.

    // First calculate the effective annual rate (EAR). Calculating the future value with this EAR
    // and compounding once per year should be the same as using the weekly rate and compounding
    // once per week.
    let ear = convert_apr_to_ear(apr, periods_per_year);

    // Starting with the same present value and compounding once per year using the EAR should give
    // us the future value we started with. If so this means we correctly calculated the rate above.
    let check_future_value = finance_solution::future_value(ear, years, present_value, false);
    dbg!(check_future_value);
    assert_rounded_4(check_future_value, future_value as f64);
}

// You are offered an investment with a quoted annual interest rate 
// of 13% with quarterly compounding of interest. 
// What is your effective annual interest rate?
// Expect  13.65% 
fn rate_conversion_problem_1() {
    let apr = 0.013;
    let compounding_periods_per_year = 4;
    // finish here
}

// You are offered an annuity that will pay $24,000 per year 
// for 11 years (the first payment will occur one year from today).
// If you feel that the appropriate discount rate is 13%, 
// what is the annuity worth to you today? 
// Expect $136_486.59
fn pv_annuity_problem_1() {
    let annuity = -24_000; // payments/annuities are typically negative, to indicate cashflow direction
    let periods = 11;
    let rate = 0.13;
    let pv_annuity_problem_1 = present_value_annuity_solution(rate, periods, annuity, false);
    dbg!(pv_annuity_problem_1); 
}

// If you deposit $16,000 per year for 12 years 
// (each deposit is made at the end of each year) in an account
// that pays an annual interest rate of 14%, 
// what will your account be worth at the end of 12 years?
// Expect $436,331.98
fn fv_annuity_problem_1() {
    let rate= 0.14;
    let periods = 12;
    let payment = -16_000; // payments/annuities are typically negative, to indicate cashflow direction
    let fv_annuity_problem_1 = future_value_annuity_solution(rate, periods, payment, false);
    dbg!(&fv_annuity_problem_1);
}

// You plan to borrow $389,000 now and repay it 
// in 25 equal annual installments (payments will be made
// at the end of each year). If the annual interest rate 
// is 14%, how much will your annual payments be?
// Expect $56,598.88

// You are told that if you invest $11,000 per year 
// for 23 years (all payments made at the end of each year) 
// you will have accumulated $366,000 at the end of the period. 
// What annual rate of return is the investment offering? 
// Expect 3.21%

// You are offered an annuity that will pay $17,000 per year 
// for 7 years (the first payment will be made today). 
// If you feel that the appropriate discount rate is 11%, 
// what is the annuity worth to you today?
// Expect $88,919.14

// If you deposit $15,000 per year for 9 years 
// (each deposit is made at the beginning of each year) in an
// account that pays an annual interest rate of 8%, 
// what will your account be worth at the end of 9 years?
// Expect  $202,298.44

// You plan to accumulate $450,000 over a period of 12 years 
// by making equal annual deposits in an account that pays an 
// annual interest rate of 9% (assume all payments will occur 
// at the beginning of each year). 
// What amount must you deposit each year to reach your goal?
// Expect $20,497.98

// You are told that if you invest $11,100 per year for 19 years 
// (all payments made at the beginning of each year) you will 
// have accumulated $375,000 at the end of the period. 
// What annual rate of return is the investment offering? 
// Expect 5.48%

// You plan to buy a car that has a total "drive-out" cost 
// of $25,700. You will make a down payment of $3,598. 
// The remainder of the car’s cost will be financed over a 
// period of 5 years. You will repay the loan by making equal 
// monthly payments. Your quoted annual interest rate is 8% with 
// monthly compounding of interest. (The first payment will be 
// due one month after the purchase date.) 
// What will your monthly payment be?
// Expect $448.15

// You are considering leasing a car. You notice an ad that says you can 
// lease the car you want for $477.00 per month. The lease term is 60 months 
// with the first payment due at inception of the lease. You must also make 
// an additional down payment of $2,370. The ad also says that the residual 
// value of the vehicle is $20,430. After much research, you have concluded 
// that you could buy the car for a total "driveout" price of $33,800. 
// What is the quoted annual interest rate you will pay with the lease? 
// Expect 13.47%

// You are valuing an investment that will pay you $12,000 the first year, 
// $14,000 the second year, $17,000 the third year, $19,000 the fourth year, 
// $23,000 the fifth year, and $29,000 the sixth year (all payments are at the 
// end of each year). What it the value of the investment to you now 
// if the appropriate annual discount rate is 11.00%? 
// Expect  $76,273.63

// You are valuing an investment that will pay you $27,000 per year 
// for the first ten years, $35,000 per year for the next ten years, 
// and $48,000 per year the following ten years (all payments are 
// at the end of each year). If the appropriate annual discount rate 
// is 9.00%, what is the value of the investment to you today?
// Expect  $323,123.04


// John and Peggy recently bought a house. They financed the house 
// with a $125,000, 30-year mortgage with a nominal interest rate 
// of 7 percent. Mortgage payments are made at the end of each month. 
// What total dollar amount of their mortgage payments during the first 
// three years will go towards repayment of principal?
// Expect   $25,847.31 

// You are valuing an investment that will pay you $26,000 per year 
// for the first 9 years, $34,000 per year for the next 11 years, 
// and $47,000 per year the following 14 years (all payments are at 
// the end of each year). Another similar risk investment alternative is 
// an account with a quoted annual interest rate of 9.00%
// with monthly compounding of interest. 
// What is the value in today's dollars of the set of cash flows 
// you have been offered?
// Expect   $314,517.85


// You have just won the Georgia Lottery with a jackpot of $40,000,000. 
// Your winnings will be paid to you in 26 equal annual installments with the 
// first payment made immediately. If you feel the appropriate
// annual discount rate is 8%, what is the present value of the 
// stream of payments you will receive? 
// Expect $17,961,194.14

// You have just won the Georgia Lottery with a jackpot of $11,000,000. Your winnings will be paid to
// you in 26 equal annual installments with the first payment made immediately. If you had the money now,
// you could invest it in an account with a quoted annual interest rate of 9% with monthly compounding of
// interest. What is the present value of the stream of payments you will receive?
// Expect  $4,453,789.97

// You are planning for retirement 34 years from now. You plan to invest $4,200 per year for the first 7
// years, $6,900 per year for the next 11 years, and $14,500 per year for the following 16 years (assume all
// cash flows occur at the end of each year). If you believe you will earn an effective annual rate of return of
// 9.7%, what will your retirement investment be worth 34 years from now?
// Expect  $1,542,217.26

// You plan to retire 33 years from now. You expect that you will live 27 years after retiring. You want
// to have enough money upon reaching retirement age to withdraw $180,000 from the account at the
// beginning of each year you expect to live, and yet still have $2,500,000 left in the account at the time of
// your expected death (60 years from now). You plan to accumulate the retirement fund by making equal
// annual deposits at the end of each year for the next 33 years. You expect that you will be able to earn 12%
// per year on your deposits. However, you only expect to earn 6% per year on your investment after you
// retire since you will choose to place the money in less risky investments. What equal annual deposits must
// you make each year to reach your retirement goal?
// Expect $8,874.79
fn retirement_problem_1() {
    let retire_in = 33;
    let live_how_many_years_after_retirement = 27;
    let retirement_income = 180_000;
    let total_inheritance = 2_500_000;
    let rate_before_retire = 0.12;
    let rate_after_retire = 0.06;

    // First calculate the amount of money needed at the moment of retirement to achieve these
    // goals. The retirement income of $180,000 can be treated as annuity for which we can calculate
    // the present value at the time of retirement.
    let due_at_beginning = true;
    // Use the negative of the retirement income so that the value ends up positive.
    let value_at_retirement_for_income= present_value_annuity(rate_after_retire, live_how_many_years_after_retirement, -retirement_income, due_at_beginning);
    dbg!(value_at_retirement_for_income);

    // What amount of money is needed at the moment of retirement so that it can be invested and
    // eventually grow to $2.5 million at the moment of death?
    let value_at_retirement_for_inheritance = present_value(rate_after_retire, live_how_many_years_after_retirement, total_inheritance, false);
    dbg!(value_at_retirement_for_inheritance);

    let total_at_retirement = value_at_retirement_for_income + value_at_retirement_for_inheritance;
    dbg!(total_at_retirement);

    let due_at_beginning = false;
    // Calculate the payments needed from now until retirement to end up with the total amount
    // needed at retirement. In this case we'll call payment_solution() to get a struct containing
    // more information.
    let payment_before_retirement = payment_solution(rate_before_retire, retire_in, 0, total_at_retirement, due_at_beginning);
    dbg!(&payment_before_retirement);
    // Since the desired future value was positive the payment is negative.
    assert_rounded_2(-8_874.79, payment_before_retirement.payment());

    // To double check the logic, start with the payments calculated just now and work forward.

    // You start with zero dollars and will be paying in $8_874.79 per year (the amount that we're
    // double checking) into an investment that pays 12% per year. What will be the value of that
    // investment after 33 years, that is at the moment of retirement?
    let payment = payment_before_retirement.payment();
    let due_at_beginning = false;
    let check_value_at_retirement = future_value_annuity(rate_before_retire, retire_in, payment, due_at_beginning);
    dbg!(total_at_retirement, check_value_at_retirement);
    // Compare this to the total amount at retirement we calculated earlier by working backward from
    // the amounts needed at the moment of death.
    assert_approx_equal!(total_at_retirement, check_value_at_retirement);

    // Continuing to work forward and given the total amount at retirement, what retirement income
    // could be drawn from an investment while still leaving $2.5 million at the end as an
    // inheritance?
    let rate = rate_after_retire;
    let periods = live_how_many_years_after_retirement;
    let present_value = check_value_at_retirement;
    let future_value = -total_inheritance;
    // The retirement income will be taken at the beginning of each year.
    let due_at_beginning = true;
    let check_payment_after_retirement = payment_solution(rate, periods, present_value, future_value, due_at_beginning);
    dbg!(&check_payment_after_retirement);
    assert_approx_equal!(-retirement_income as f64, check_payment_after_retirement.payment());
}
