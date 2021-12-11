use crossterm::style::Color;
use inquire::{Select, Text};
use mem::{search::SearchQueryKind, MemApp, memo::MemoSearchQuery, print_colored, println_colored};
fn main() {
    let mut app = MemApp::init("data.txt");

    let mut show_preview = true;

    loop {
        //println!("{}", command_descriptions);
        if show_preview {
            app.preview_list();
        }
        show_preview = true;
        let ans = Text::new(">").prompt();
        //let ans = Select::new(">", ll.clone()).prompt();

        match ans.unwrap().as_str() {
            "x" | "exit" => break,
            "l" | "list" => {
                app.list();
                show_preview = false;
            }
            "c" | "create" | "new" => {
                app.create_new_memo_interactive();
            }
            "e" | "edit" => {
                let id = Text::new("id: ").prompt_skippable();
                if let Ok(opt) = id {
                    let id: u16 = opt.unwrap().parse().unwrap_or(app.last_id());
                    app.edit_interactive(id)
                }
            }
            "s" | "save" => {
                app.save().unwrap();
            }
            "clear" => {
                app.clear().unwrap();
            }
            query => {



                let qr = app.query(SearchQueryKind::Memo(MemoSearchQuery::Header(query)));
                if !qr.is_empty(){
                    println_colored(&"found in headers:", Color::Blue, Color::Black).unwrap();
                    qr.list_results()
                }
    
                let qr = app.query(SearchQueryKind::Memo(MemoSearchQuery::Content(query)));
                if !qr.is_empty(){
                    println_colored(&"found in contents:", Color::Cyan, Color::Black).unwrap();
                    qr.list_results()
                }
                

                show_preview = false}

        }
    }

    app.save();
}

