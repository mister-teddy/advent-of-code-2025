use arboard::Clipboard;

pub fn get_input() -> String {
    let warning =
        "📋 Clipboard contains no input. Please copy the test input to your system clipboard";
    let mut clipboard = Clipboard::new().unwrap();
    let content = clipboard.get_text().expect(warning);
    if content.len() == 0 {
        println!("{}", warning);
    } else {
        println!("{}", content);
    }
    content
}
