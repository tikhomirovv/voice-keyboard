use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, VIRTUAL_KEY,
};

pub fn paste_text(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut inputs: Vec<INPUT> = Vec::new();

    for c in text.encode_utf16() {
        // Событие нажатия клавиши
        let input_down = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),        // Виртуальный код клавиши (0 для Unicode)
                    wScan: c,                   // Скан-код (Unicode символ)
                    dwFlags: KEYEVENTF_UNICODE, // Флаг для Unicode ввода
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        inputs.push(input_down);

        // Событие отпускания клавиши
        let input_up = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: c,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        inputs.push(input_up);
    }

    // Отправляем события ввода
    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32); // Передаем срез и размер структуры
    }

    Ok(())
}
