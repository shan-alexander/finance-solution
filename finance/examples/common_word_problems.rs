// word problems:
// https://www.westga.edu/~rbest/finc3511/sampleprobs/sample5.pdf

// teaching resources:
// https://www.studocu.com/en-us/document/wichita-state-university/financial-management-ii/assignments/time-value-of-money-practice-problems-and-solutions/1915379/view
// http://mupfc.marshall.edu/~brozik/timevalueweb.pdf

// use finance::{present_value_solution, future_value_solution, future_value, periods_solution, periods};
use finance::*;

pub fn main() {
    pv_problem_1();
    fv_problem_1();
    nper_problem_1();
    rate_problem_1();

    pv_problem_2();
    fv_problem_2();
    nper_problem_2();
    rate_problem_2();

    pv_annuity_problem_1();
    fv_annuity_problem_1();

    // retirement_problem_1();

}

// If you wish to accumulate $140,000 in 13 years, 
// how much must you deposit today in an account that
// pays an annual interest rate of 14%? 
// Expect $25,489.71
fn pv_problem_1() {
    let future_value = 140_000;
    let periods = 13;
    let rate = 0.14;
    let pv_problem_1 = present_value_solution(rate, periods, future_value);
    dbg!(&pv_problem_1); // Outputs solution struct.
    dbg!(pv_problem_1.present_value()); // Outputs 25489.71433359101 from solution struct.
    dbg!(present_value(rate, periods, future_value)); // Outputs 25489.71433359101 from f64 fn.
}

// What will $247,000 grow to be in 9 years if it is invested today 
// in an account with an annual interest rate of 11%? 
// Expect $631,835.12
fn fv_problem_1() {
    let present_value = 247_000;
    let periods = 9;
    let rate = 0.11;
    let fv_problem_1 = future_value_solution(rate, periods, present_value);
    dbg!(&fv_problem_1); // Outputs solution struct.
    dbg!(fv_problem_1.future_value()); // Outputs 631835.1203234661 from solution struct.
    dbg!(future_value(rate, periods, present_value)); // Outputs 631835.1203234661 from f64 fn.
}

// How many years will it take for $136,000 to grow to be $468,000 
// if it is invested in an account with an annual interest rate of 8%? 
// Expect 16.06 years
fn nper_problem_1() {
    let present_value = 136_000;
    let future_value = 468_000;
    let rate = 0.08;
    let nper_problem_1 = periods_solution(rate, present_value, future_value);
    dbg!(&nper_problem_1); // Outputs solution struct.
    dbg!(nper_problem_1.periods()); // Outputs 17 from solution struct.
    dbg!(nper_problem_1.fractional_periods()); // Outputs 16.057649324100133 from solution struct.
    dbg!(periods(rate, present_value, future_value)); // Outputs 16.057649324100133 from f64 fn.
}

// At what annual interest rate must $137,000 be invested 
// so that it will grow to be $475,000 in 14 years?
// Expect 9.29%
fn rate_problem_1() { 
    let pv = 137_000;
    let fv = 475_000;
    let periods = 14;
    let rate_problem_1 = rate_solution(periods, pv, fv);
    dbg!(rate_problem_1);
}

// If you wish to accumulate $197,000 in 5 years, 
// how much must you deposit today in an account that pays
// a quoted annual interest rate of 13% with semi-annual compounding of interest?
// Expect $104,947.03
fn pv_problem_2() {
    let fv = 197_000;
    // finish here
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
fn nper_problem_2() {
    let pv = 197_000;
    let fv = 554_000;
    // finish here
}


// At what quoted annual interest rate must $134,000 be invested 
// so that it will grow to be $459,000 in 15 years 
// if interest is compounded weekly? 
// Expect 8.21%
fn rate_problem_2() {
    let pv = 134_000;
    let fv = 459_000;
    // finish here
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
    let annuity = 24_000;
    let periods = 11;
    let rate = 0.13;
    let pv_annuity_problem_1 = present_value_annuity_solution(rate, periods, annuity);
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
    let payment = 16_000;
    let fv_annuity_problem_1 = future_value_annuity_solution(rate, periods, payment);
    dbg!(fv_annuity_problem_1);
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
// Expect  $8,874.79
fn retirement_problem_1() {
    let retire_in = 33;
    let live_how_many_years_after_retirement = 27;
    let retirement_income = 180_000;
    let total_inheretance = 2_500_000;
    let rate_before_retire = 0.12;
    let rate_after_retire = 0.06;

    let pv_income = present_value_annuity_due(rate_after_retire, live_how_many_years_after_retirement, retirement_income);
    let pv_inheretance = present_value(rate_after_retire, live_how_many_years_after_retirement, total_inheretance);
    let total_at_retirement = pv_income + pv_inheretance;
    let retirement_problem_1 = payment_solution(rate_before_retire, retire_in, 0, total_at_retirement);
    dbg!(&retirement_problem_1);
    let pv_total_at_retirement = present_value(rate_before_retire, retire_in, total_at_retirement);
    let retirement_problem_1 = payment_solution(rate_before_retire, retire_in, pv_total_at_retirement, 0);
    dbg!(&retirement_problem_1);
}
