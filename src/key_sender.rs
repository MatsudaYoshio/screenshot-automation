use crate::config::ArrowKey;
use crate::error::KeySendError;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, SendInput,
    VIRTUAL_KEY,
};

pub struct KeySender;

impl KeySender {
    pub fn send_arrow_key(key: ArrowKey) -> Result<(), KeySendError> {
        let vk_code = VIRTUAL_KEY(key.to_vk_code());

        // Create key down event
        let input_down = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk_code,
                    wScan: 0,
                    dwFlags: KEYEVENTF_EXTENDEDKEY,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        // Create key up event
        let input_up = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk_code,
                    wScan: 0,
                    dwFlags: KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        // Send key down event
        unsafe {
            let result = SendInput(&[input_down], std::mem::size_of::<INPUT>() as i32);
            if result == 0 {
                return Err(KeySendError::SendInputFailed);
            }
        }

        // Send key up event
        unsafe {
            let result = SendInput(&[input_up], std::mem::size_of::<INPUT>() as i32);
            if result == 0 {
                return Err(KeySendError::SendInputFailed);
            }
        }

        Ok(())
    }
}
