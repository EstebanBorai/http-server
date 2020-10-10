use crate::handler::file_explorer::html::style::STYLE;
use crate::file_explorer::{FileExplorer, Entry};
use typed_html::dom::DOMTree;
use typed_html::{html, text};
use typed_html::types::Metadata;
use std::fs::ReadDir;
use clap::crate_version;

pub const FOLDER_ICON: &str = r##"<svg height='20px' width='30px'  fill="#437CB0" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" x="0px" y="0px"><g data-name="22"><path d="M21,7H12.72L12,4.68A1,1,0,0,0,11,4H3A1,1,0,0,0,2,5V19a1,1,0,0,0,1,1H21a1,1,0,0,0,1-1V8A1,1,0,0,0,21,7Z"></path></g></svg>"##;
pub const FILE_ICON: &str = r##"<svg height='20px' width='30px'  fill="#437CB0" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.2" baseProfile="tiny" x="0px" y="0px" viewBox="0 0 80 80" xml:space="preserve"><polygon points="65,7.5 65,25 82.5,25"></polygon><polygon points="17.5,7.5 17.5,92.5 82.5,92.5 82.5,30 60,30 60,7.5"></polygon></svg>"##;

pub fn make_document(
    dirname: &str,
    root_dir: &str,
    fexplorer: &FileExplorer,
    entries: ReadDir,
) -> String {
    let version = crate_version!();
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>"HTTP Server | File Explorer"</title>
                <meta name=Metadata::Viewport content="width=device-width, initial-scale=1.0"/>
                <style>{ text!("{}", STYLE) }</style>
            </head>
            <body>
                <header id="current-directory">
                    <div id="container">
                        <article id="dirname">
                            <h2>{ text!("{}", dirname) }</h2>
                            <span>
                                <code class="code">{ text!("{}", root_dir) }</code>
                            </span>
                        </article>
                        <ul id="toolbox"></ul>
                    </div>
                </header>
                <main>
                    <table id="file-table">
                        <thead>
                            <tr>
                                <th id="icon-th"></th>
                                <th>{ text!("{}", "Name") }</th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                entries.into_iter().map(|entry| {
                                    let entry = &Entry::from(entry.unwrap());
                                    let icon = if entry.is_file {
                                        FILE_ICON
                                    } else {
                                        FOLDER_ICON
                                    };
                                
                                    let full_path = entry.path.to_str().unwrap();
                                    let link_text = if let Some(last_slash_index) = full_path.rfind('/') {
                                        &full_path[last_slash_index + 1..]
                                    } else {
                                        full_path.clone()
                                    };

                                    html!(
                                        <tr>
                                            <td><i class="file-icon"></i></td>
                                            <td>
                                                <a href={fexplorer.to_relative_path(full_path).unwrap()}>
                                                    { text!("{}", link_text) }
                                                </a>
                                            </td>
                                        </tr>
                                    )
                                })
                            }
                        </tbody>
                    </table>
                </main>
                <footer id="fs-footer">
                    <code class="code">
                        { text!("{} | {} | ", "http-server", version) }
                        <a href="https://github.com/EstebanBorai/http-server" target="_blank">{ text!("{}", "GitHub") }</a>
                    </code>
                </footer>
            </body>
        </html>
    );

    doc.to_string()
}
