// example output in the terminal:
present_value_3 = PresentValue {
    rate: 1.034,
    periods: 5,
    present_value: 7181.005605506769,
    future_value: 250000.0,
    period_values: [
        PresentValuePeriod {
            period: 0,
            rate: 1.034,
            future_value: 250000.0,
            period_value: 7181.005605506769,
            present_value: 7181.005605506769,
        },
        PresentValuePeriod {
            period: 1,
            rate: 1.034,
            future_value: 250000.0,
            period_value: 14606.165401600765,
            present_value: 7181.005605506769,
        },
        PresentValuePeriod {
            period: 2,
            rate: 1.034,
            future_value: 250000.0,
            period_value: 29708.940426855956,
            present_value: 7181.005605506769,
        },
        PresentValuePeriod {
            period: 3,
            rate: 1.034,
            future_value: 250000.0,
            period_value: 60427.984828225,
            present_value: 7181.005605506769,
        },
        PresentValuePeriod {
            period: 4,
            rate: 1.034,
            future_value: 250000.0,
            period_value: 122910.52114060965,
            present_value: 7181.005605506769,
        },
        PresentValuePeriod {
            period: 5,
            rate: 1.034,
            future_value: 250000.0,
            period_value: 250000.0,
            present_value: 7181.005605506769,
        },
    ],
}




// this could be the default? Vecs inside a struct show the vec<type> and vec length, but omit the values
// #flat_dbg!() 
present_value_3 = PresentValue {
    rate: 1.034, periods: 5, present_value: 7181.005605506769, future_value: 250000.0,
    period_values: [ PresentValuePeriod (5) ],
}

// an option for the coder:
// #flat_dbg!(allow_struct_vecs) 
present_value_3 = PresentValue {
    rate: 1.034, periods: 5, present_value: 7181.005605506769, future_value: 250000.0,
    period_values: [
        PresentValuePeriod {period: 0,rate: 1.034,future_value: 250000.0,period_value: 7181.005605506769,present_value: 7181.005605506769,},
        PresentValuePeriod {period: 1,rate: 1.034,future_value: 250000.0,period_value: 14606.165401600765,present_value: 7181.005605506769,},
        PresentValuePeriod {period: 2,rate: 1.034,future_value: 250000.0,period_value: 29708.940426855956,present_value: 7181.005605506769,},
        PresentValuePeriod {period: 3,rate: 1.034,future_value: 250000.0,period_value: 60427.984828225,present_value: 7181.005605506769,},
        PresentValuePeriod {period: 4,rate: 1.034,future_value: 250000.0,period_value: 122910.52114060965,present_value: 7181.005605506769,},
        PresentValuePeriod {period: 5,rate: 1.034,future_value: 250000.0,period_value: 250000.0,present_value: 7181.005605506769,},
    ],
}

// another option to have the top struct pretty-printed, but any internals will get flattened
// #flat_dbg!(allow_top_struct)) 
present_value_3 = PresentValue {
    rate: 1.034,
    periods: 5,
    present_value: 7181.005605506769,
    future_value: 250000.0,
    example_inner_struct: ExampleInnerStruct {thing: 13, seq: 33, val: 93.45}
    period_values: [
        PresentValuePeriod {period: 0,rate: 1.034,future_value: 250000.0,period_value: 7181.005605506769,present_value: 7181.005605506769,},
        PresentValuePeriod {period: 1,rate: 1.034,future_value: 250000.0,period_value: 14606.165401600765,present_value: 7181.005605506769,},
        PresentValuePeriod {period: 2,rate: 1.034,future_value: 250000.0,period_value: 29708.940426855956,present_value: 7181.005605506769,},
        PresentValuePeriod {period: 3,rate: 1.034,future_value: 250000.0,period_value: 60427.984828225,present_value: 7181.005605506769,},
        PresentValuePeriod {period: 4,rate: 1.034,future_value: 250000.0,period_value: 122910.52114060965,present_value: 7181.005605506769,},
        PresentValuePeriod {period: 5,rate: 1.034,future_value: 250000.0,period_value: 250000.0,present_value: 7181.005605506769,},
    ],
}