use std::env;
use std::fs;
use std::io::{self, Result as IoResult};
use std::path::{Path, PathBuf};

use crossterm::{
    event::{self, Event, KeyCode, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
    backend::CrosstermBackend,
};

#[derive(Clone, Debug)]
struct TreeNode {
    path: PathBuf,
    is_dir: bool,
    depth: usize,
}

struct App {
    tree_nodes: Vec<TreeNode>,
    selected_index: usize,
    show_hidden: bool,
    file_contents: Option<String>,
    scroll_offset: u16, // For vertical scrolling of file contents
    show_third_panel: bool,
    search_input: String,
}

impl App {
    fn new() -> Self {
        let start_path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self {
            tree_nodes: Self::build_tree(&start_path, 0, false),
            selected_index: 0,
            show_hidden: false,
            file_contents: None,
            scroll_offset: 0,
            show_third_panel: false,
            search_input: String::new(),
        }
    }

    fn build_tree(path: &Path, depth: usize, show_hidden: bool) -> Vec<TreeNode> {
        let mut nodes = Vec::new();

        // Read directory entries
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(_) => return nodes,
        };

        // Convert and filter entries
        let mut sorted_entries: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                if !show_hidden {
                    !entry.file_name()
                        .to_str()
                        .map(|name| name.starts_with('.'))
                        .unwrap_or(false)
                } else {
                    true
                }
            })
            .collect();

        sorted_entries.sort_by_key(|a| a.file_name());

        // Convert to tree nodes
        for entry in sorted_entries {
            let entry_path = entry.path();
            let is_dir = entry_path.is_dir();
            
            nodes.push(TreeNode {
                path: entry_path.clone(),
                is_dir,
                depth,
            });

            // Recursively add subdirectories
            if is_dir {
                nodes.extend(Self::build_tree(&entry_path, depth + 1, show_hidden));
            }
        }

        nodes
    }

    fn read_file_contents(&mut self) {
        if let Some(node) = self.tree_nodes.get(self.selected_index) {
            if !node.is_dir {
                match fs::read_to_string(&node.path) {
                    Ok(contents) => self.file_contents = Some(contents),
                    Err(_) => self.file_contents = Some("Unable to read file contents".to_string()),
                }
            } else {
                self.file_contents = None;
            }
        }
    }

    fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        let start_path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        self.tree_nodes = Self::build_tree(&start_path, 0, self.show_hidden);
        self.selected_index = 0;
    }
}

fn main() -> io::Result<()> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App initialization
    let mut app = App::new();

    // Main loop
    loop {
        // Main event loop for handling UI updates
        terminal.draw(|f| ui(f, &mut app))?;
    
        // Handling user input
        if let Event::Key(key) = event::read()? {
            if app.show_third_panel {
                // Handle input for search field in third panel
                match key.code {
                    KeyCode::Char(c) => {
                        app.search_input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.search_input.pop();
                    }
                    KeyCode::Enter => {
                        // Trigger search
                    }
                    KeyCode::Esc => {
                        app.show_third_panel = false; // Close the third panel
                    }
                    _ => {}
                }
            } else {
                // Handle input for tree navigation when third panel is not active
                match key.code {
                    KeyCode::Char('q') => break, // This break exits the main loop
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.selected_index < app.tree_nodes.len() - 1 {
                            app.selected_index += 1;
                            app.read_file_contents();
                            app.scroll_offset = 0;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.selected_index > 0 {
                            app.selected_index -= 1;
                            app.read_file_contents();
                            app.scroll_offset = 0;
                        }
                    }
                    KeyCode::Char('h') => {
                        app.toggle_hidden();
                    }
                    KeyCode::Enter => {
                        app.show_third_panel = true; // Show third panel for search
                    }
                    _ => {}
                }
            }
        }
    }
    

    // Restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    // Create layout with two or three columns based on show_third_panel
    let main_layout = if app.show_third_panel {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33), // Tree view
                Constraint::Percentage(33), // File contents
                Constraint::Percentage(34), // Third panel
            ])
            .split(f.size())
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40), // Tree view
                Constraint::Percentage(60), // File contents
            ])
            .split(f.size())
    };

    // Render tree view (no changes)
    let tree_items: Vec<ListItem> = app.tree_nodes
        .iter()
        .enumerate()
        .map(|(index, node)| {
            let indent = " ".repeat(node.depth * 2);
            let content = format!("{}{}", indent, node.path.file_name().unwrap_or_default().to_string_lossy());

            let style = if index == app.selected_index {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let tree_block = Block::default()
        .title(" Directory Tree ")
        .borders(Borders::ALL);

    let tree_list = List::new(tree_items)
        .block(tree_block);

    f.render_widget(tree_list, main_layout[0]);

    // Render file contents or search field
    let contents_block = Block::default()
        .title(" File Contents ")
        .borders(Borders::ALL);

    if app.show_third_panel {
        // Render search input
        let search_paragraph = Paragraph::new(app.search_input.clone())
            .block(Block::default().title(" Search ").borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(search_paragraph, main_layout[1]);
    } else {
        // Render file contents as before
        let contents = if let Some(contents) = &app.file_contents {
            let lines: Vec<_> = contents
                .lines()
                .skip(app.scroll_offset as usize)
                .take(f.size().height as usize)
                .map(String::from)
                .collect();
            let display_contents = lines.join("\n");
            Paragraph::new(display_contents)
                .block(contents_block)
        } else {
            Paragraph::new("Select a file to view contents")
                .block(contents_block)
        };

        f.render_widget(contents, main_layout[1]);
    }

    // Render third panel if active
    if app.show_third_panel {
        let third_block = Block::default()
            .title(" Third Panel ")
            .borders(Borders::ALL);
        let third_content = Paragraph::new("This is the third panel (currently empty)")
            .block(third_block);
        f.render_widget(third_content, main_layout[2]);
    }
}
