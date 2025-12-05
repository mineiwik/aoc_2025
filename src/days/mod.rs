use paste::paste;

/// Macro to simplify adding new days
///
/// # Arguments
///
/// * `$( $day_num:ident ),*` - The list of days in (1..=25)
///
#[macro_export]
macro_rules! init_solver {
    (
        $( $day_num:expr ),*$(,)?
    ) => {
        paste! {
            $(mod [<d $day_num>];)*
        }

        pub fn get_solver(day: u8) -> Option<Box<dyn $crate::utils::DaySolver>> {

            paste! {
                match day {
                    $($day_num => Some(Box::new([<d $day_num>]::[<Day $day_num>])), )*
                    _ => None,
                }
            }
        }
    };
}

init_solver! {
    1,
    2,
    3,
    4,
    5,
}
