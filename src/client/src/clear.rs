use winapi::HANDLE;
use winapi::wincon::CONSOLE_SCREEN_BUFFER_INFO;
use winapi::wincon::COORD;
use winapi::wincon::SMALL_RECT;
use winapi::WORD;
use winapi::DWORD;

static mut CONSOLE_HANDLE: Option<HANDLE> = None;


fn get_output_handle() -> HANDLE {
    unsafe {
        if let Some(handle) = CONSOLE_HANDLE {
            return handle;
        } else {
            let handle = kernel32::GetStdHandle(winapi::STD_OUTPUT_HANDLE);
            CONSOLE_HANDLE = Some(handle);
            return handle;
        }
    }
}

fn get_buffer_info() -> winapi::CONSOLE_SCREEN_BUFFER_INFO {
    let handle = get_output_handle();
    if handle == winapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }
    let mut buffer = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: 0 as WORD,
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: COORD { X: 0, Y: 0 },
    };
    unsafe {
        kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer);
    }
    buffer
}

pub fn clear() {
    let handle = get_output_handle();
    if handle == winapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }

    let screen_buffer = get_buffer_info();
    let console_size: DWORD = screen_buffer.dwSize.X as u32 * screen_buffer.dwSize.Y as u32;
    let coord_screen = COORD { X: 0, Y: 0 };

    let mut amount_chart_written: DWORD = 0;
    unsafe {
        kernel32::FillConsoleOutputCharacterW(
            handle,
            32 as winapi::WCHAR,
            console_size,
            coord_screen,
            &mut amount_chart_written,
        );
    }
    set_cursor_possition(0, 0);
}

fn set_cursor_possition(y: i16, x: i16) {
    let handle = get_output_handle();
    if handle == winapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }
    unsafe {
        kernel32::SetConsoleCursorPosition(handle, COORD { X: x, Y: y });
    }
}
