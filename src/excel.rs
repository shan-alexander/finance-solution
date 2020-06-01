#![allow(unused_variables)]

pub fn fv<N, M, P>(rate: f64, n_per: N, pmt: P, pv: P, type_: u8) -> f64
where
    N: Into<f64> + Copy,
    M: Into<f64> + Copy,
    P: Into<f64> + Copy,
{
    unimplemented!()
}

pub fn pv<N, M, F>(rate: f64, nper: N, pmt: M, fv: F, type_: u8) -> f64
    where
        N: Into<f64> + Copy,
        M: Into<f64> + Copy,
        F: Into<f64> + Copy,
{
    unimplemented!()
}

pub fn pmt<N, P, F>(rate: f64, nper: N, pv: P, fv: F, type_: u8) -> f64
    where
        N: Into<f64> + Copy,
        P: Into<f64> + Copy,
        F: Into<f64> + Copy,
{
    unimplemented!()
}

pub fn ipmt<N, P, F>(rate: f64, per: u32, n_per: N, pv: P, fv: F, type_: u8) -> f64
    where
        N: Into<f64> + Copy,
        P: Into<f64> + Copy,
        F: Into<f64> + Copy,
{
    unimplemented!()
}

pub fn nper<M, P, F>(rate: f64, pmt: M, pv: P, fv: F, type_: u8) -> f64
    where
        M: Into<f64> + Copy,
        P: Into<f64> + Copy,
        F: Into<f64> + Copy,
{
    unimplemented!()
}

pub fn rate<N, M, P, F, G>(nper: N, pmt: M, pv: P, fv: F, type_: u8, guess: Option<G>) -> f64
    where
        N: Into<f64> + Copy,
        M: Into<f64> + Copy,
        P: Into<f64> + Copy,
        F: Into<f64> + Copy,
        G: Into<f64> + Copy,
{
    unimplemented!()
}







