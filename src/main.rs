enum Measure {
    Milimeter,
    Santimeter,
    Meter,
    Celsius,
    Farenheit,
}

fn conversion(inputType: Measure, value: f64, outputType: Measure) -> Option<f64> {
    match inputType {
        Measure::Milimeter => {
            match outputType {
                Measure::Milimeter => Some(value),
                Measure::Santimeter => Some(0.1 * value),
                Measure::Meter => Some(0.001 * value),
                _ => None
            }
        },
        Measure::Santimeter => {
            match outputType {
                Measure::Milimeter => Some(10. * value),
                Measure::Santimeter => Some(value),
                Measure::Meter => Some(0.01 * value),
                _ => None
            }
        },
        Measure::Meter => {
            match outputType {
                Measure::Milimeter => Some(1000. * value),
                Measure::Santimeter => Some(100. * value),
                Measure::Meter => Some(value),
                _ => None
            }
        },
        Measure::Celsius => {
            match outputType {
                Measure::Celsius => Some(value),
                Measure::Farenheit => {Some(1.8 * value + 32.)}
                _ => None
            }
        },
        Measure::Farenheit => {
            match outputType {
                Measure::Celsius => { Some((value - 32.) / 1.8) },
                Measure::Farenheit => Some(value),
                _ => None
            }
        },
    }
}

fn main() {
    let output = conversion(Measure::Celsius, 32.,  Measure::Farenheit);
    match output {
        Some(value) => println!("{:?}", value),
        None => println!("No known conversion for this types")
    }
}

