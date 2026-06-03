use std::ffi::CString;

unsafe extern "C" {
    fn InitWindow(width: i32, height: i32, title: *const i8);
    fn WindowShouldClose() -> bool;
    fn BeginDrawing();
    fn EndDrawing();
    fn ClearBackground(color: u32);
    fn DrawText(text: *const i8, x: i32, y: i32, size: i32, color: u32);
    fn CloseWindow();
}

fn main() {
    println!("Hello, World");
    // let title = CString::new("Rust + raylib").unwrap();
    //
    // unsafe {
    //     InitWindow(800, 450, title.as_ptr());
    //
    //     while !WindowShouldClose() {
    //         BeginDrawing();
    //         ClearBackground(0xFFFFFFFF);
    //         DrawText(
    //             CString::new("Hello from Rust").unwrap().as_ptr(),
    //             200,
    //             200,
    //             20,
    //             0x000000FF,
    //         );
    //         EndDrawing();
    //     }
    //
    //     CloseWindow();
    // }
}
