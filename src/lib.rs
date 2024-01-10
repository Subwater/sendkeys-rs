/// Send press and release keystroke for the given `VIRTUAL_KEY` provided
///
#[macro_export]
macro_rules! send_keys {
    ($key:ident) => {{
        use windows::Win32::UI::Input::KeyboardAndMouse::{
            SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
        };
        let mut input_list: Vec<INPUT> = Vec::new();
        input_list.extend_from_slice(&[
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: $key,
                        ..Default::default()
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: $key,
                        dwFlags: KEYEVENTF_KEYUP,
                        ..Default::default()
                    },
                },
            },
        ]);

        unsafe { SendInput(input_list.as_slice(), std::mem::size_of::<INPUT>() as _) > 0 }
    }}; // ($($key:ident),*) => {{
        //     todo!("Implement multiple keystrokes");
        // }};
        // ($(($modifier:ident,$key:ident)),*) => {{
        //     todo!("Implement keystrokes with modifier");
        // }};
        // ($(($modifier:ident,$key:ident)),*) => {{
        //     todo!("Implement keystrokes with multiple modifiers");
        // }};
}

#[cfg(test)]
mod tests {
    use windows::Win32::UI::Input::KeyboardAndMouse::{VK_VOLUME_DOWN, VK_VOLUME_UP};

    #[test]
    fn volume_up() {
        let result = send_keys!(VK_VOLUME_UP);
        assert_eq!(result, true);
    }

    #[test]
    fn volume_down() {
        let result = send_keys!(VK_VOLUME_DOWN);
        assert_eq!(result, true);
    }
}
