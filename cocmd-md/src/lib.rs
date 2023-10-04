use termimad::MadSkin;

pub fn print_md(markdown: &String) {
    // print with termimad to stdout
    let mut skin = MadSkin::default();
    skin.print_text(markdown);
}
