use mirajazz::{error::MirajazzError, types::DeviceInput};

use crate::mappings::Kind;

pub fn process_input(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    log::info!("Processing raw input: {}, {}", input, state);

    // We use a fixed maximum size (18) for the shared processor.
    // Device-specific mapping happens in the event loop.
    read_button_press(input, state)
}

fn read_button_states(states: &[u8]) -> Vec<bool> {
    let mut bools = vec![];

    for i in 0..18 {
        bools.push(states[i + 1] != 0);
    }

    bools
}

/// Converts opendeck key index to device key index
pub fn opendeck_to_device(kind: &Kind, key: u8) -> u8 {
    if key < kind.key_count() as u8 {
        match kind {
            Kind::AKP815 => [12, 9, 6, 3, 0, 13, 10, 7, 4, 1, 14, 11, 8, 5, 2][key as usize],
            _ => [12, 9, 6, 3, 0, 15, 13, 10, 7, 4, 1, 16, 14, 11, 8, 5, 2, 17][key as usize],
        }
    } else {
        key
    }
}

/// Converts device key index to opendeck key index
pub fn device_to_opendeck(kind: &Kind, key: usize) -> usize {
    if key < kind.key_count() {
        match kind {
            Kind::AKP815 => [4, 9, 14, 3, 8, 13, 2, 7, 12, 1, 6, 11, 0, 5, 10][key],
            _ => [4, 10, 16, 3, 9, 15, 2, 8, 14, 1, 7, 13, 0, 6, 12, 5, 11, 17][key],
        }
    } else {
        key
    }
}

fn read_button_press(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    let mut button_states = vec![0x01];
    button_states.extend(vec![0u8; 18 + 1]);

    if input == 0 {
        return Ok(DeviceInput::ButtonStateChange(read_button_states(
            &button_states,
        )));
    }

    // Input is 1-based from the device
    let input_idx = input as usize;

    if input_idx < button_states.len() {
        button_states[input_idx] = state;
    }

    Ok(DeviceInput::ButtonStateChange(read_button_states(
        &button_states,
    )))
}
