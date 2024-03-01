const MPH_TO_MPS: f32 = 0.44704;
const KMPH_TO_MPS: f32 = 0.27778;
const KNOT_TO_MPS: f32 = 0.51444;

enum Velocity {
    MilesPerHours(f32),
    KiloMetersPerHours(f32),
    Knot(f32),
    MeterPerSeconds(f32),
}

fn convert_to_meter_per_seconds(vel: Velocity) -> f32 {
    return match vel {
        Velocity::MilesPerHours(v) => v * MPH_TO_MPS,
        Velocity::KiloMetersPerHours(v) => v * KMPH_TO_MPS,
        Velocity::Knot(v) => v * KNOT_TO_MPS,
        Velocity::MeterPerSeconds(v) => v,
    };
}

fn main() {}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn ex2_can_convert() {
        assert_abs_diff_eq!(
            convert_to_meter_per_seconds(Velocity::MilesPerHours(10.0)),
            4.4704,
            epsilon = 0.01
        );
        assert_abs_diff_eq!(
            convert_to_meter_per_seconds(Velocity::KiloMetersPerHours(60.0)),
            16.66668,
            epsilon = 0.01
        );
        assert_abs_diff_eq!(
            convert_to_meter_per_seconds(Velocity::Knot(5.0)),
            2.5722222,
            epsilon = 0.01
        );
        assert_abs_diff_eq!(
            convert_to_meter_per_seconds(Velocity::MeterPerSeconds(1.5)),
            1.5,
            epsilon = 0.01
        );
    }
}
