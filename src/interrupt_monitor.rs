use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_ESCAPE, VK_SPACE};

pub struct InterruptMonitor;

impl InterruptMonitor {
    /// スペースキーまたはESCキーが押されているかチェック
    ///
    /// いずれかのキーが押されている場合にtrueを返す
    pub fn check_interrupt() -> bool {
        unsafe {
            // GetAsyncKeyStateは最上位ビットが1の場合、キーが押されている
            let space_pressed = GetAsyncKeyState(VK_SPACE.0 as i32) < 0;
            let esc_pressed = GetAsyncKeyState(VK_ESCAPE.0 as i32) < 0;

            space_pressed || esc_pressed
        }
    }
}
