pub const NUMBER_LED_UINT8: u8 = 12;
pub const NUMBER_LED_USIZE: usize = NUMBER_LED_UINT8 as usize;
pub const LOWEST_VALUE: u16 = 50;
pub const HIGHEST_VALUE: u16 = 4000;
const INTERVAL: u16 = (HIGHEST_VALUE - LOWEST_VALUE) / NUMBER_LED_UINT8 as u16;

pub fn min_value(table: &mut [u8; 12]) {
    for i in table {
        *i = 0;
    }
}

pub fn value_by_range(table: &mut [u8; 12], value: u16) {
    min_value(table);

    let range = value - LOWEST_VALUE;

    for i in 0..(NUMBER_LED_UINT8 - 1) {
        if range < ((i + 1) as u16 * INTERVAL) {
            if i == 0 {
                table[i as usize] = 50;
                table[(i + 1) as usize] = 1;
            } else {
                table[(i - 1) as usize] = 1;
                table[(i) as usize] = 50;
                table[(i + 1) as usize] = 1;
            }
            return;
        }
    }

    table[(NUMBER_LED_UINT8 - 2) as usize] = 1;
    table[(NUMBER_LED_UINT8 - 1) as usize] = 50;
}

pub fn max_value(table: &mut [u8; 12]) {
    for i in table {
        *i = 1;
    }
}
