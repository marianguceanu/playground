use std::ffi::CString;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

unsafe extern "C" {
    fn InitWindow(width: i32, height: i32, title: *const i8);
    fn WindowShouldClose() -> bool;
    fn BeginDrawing();
    fn EndDrawing();
    fn ClearBackground(color: u32);
    fn DrawText(text: *const i8, x: i32, y: i32, size: i32, color: Color);
    fn CloseWindow();
}

fn main() {

    let title = CString::new("Rust + raylib").unwrap();
    let text = CString::new("Hello, world! From Rust this time").unwrap();
    const BLACK:Color = Color { r: 0, g: 0, b: 0, a: 255 };


    unsafe {
        InitWindow(800, 450, title.as_ptr());

        while !WindowShouldClose() {
            BeginDrawing();
                ClearBackground(0xFFFFFF);
                DrawText(
                    text.as_ptr(), 
                    190, 
                    200, 
                    20,    
                    BLACK,
                );
            EndDrawing();
        }

        CloseWindow();
    }
}
