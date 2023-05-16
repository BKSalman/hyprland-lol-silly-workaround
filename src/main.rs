use hyprland::{
    data::{Client, CursorPosition},
    dispatch::{Dispatch, DispatchType},
    shared::{HyprData, HyprDataActiveOptional},
};

fn main() {
    loop {
        if let Ok(Some(active_client)) = Client::get_active() {
            if active_client.class.contains("league of legends.exe") {
                let max_x = 1920 + 1920;
                let cursor_pos = CursorPosition::get().unwrap();
                if cursor_pos.x < 1920 {
                    Dispatch::call(DispatchType::MoveCursor(1920, cursor_pos.y)).unwrap();
                    println!("moving to x:{} y:{}", 1920, cursor_pos.y);
                } else if cursor_pos.x > max_x {
                    Dispatch::call(DispatchType::MoveCursor(max_x, cursor_pos.y)).unwrap();
                    println!("moving to x:{} y:{}", max_x, cursor_pos.y);
                }
            }
        }
    }
}
