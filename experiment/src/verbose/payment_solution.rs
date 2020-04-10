







pub struct Schedule(Vec<f64>);

impl Deref for Schedule {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Schedule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for Schedule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let min = self.0.min();
        let max = self.0.max();
        if min == max {
            write!(f, "{:.4}", min)
        } else {
            let values = self.0.iter()
                .map(|x| format!("{:.4}", x))
                .join(", ");
            write!(f, "[{}]", values)
        }
    }
}

#[derive(Debug)]
pub struct PaymentSolution {
    pub calculated_field: finance::TvmVariable,
    pub periods: u32,
    pub present_value: f64,
    pub future_value: f64,
    pub rates: Schedule,
    pub payments: Schedule,
    pub formula: String,
}
