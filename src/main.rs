use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use image::io::Reader as ImageReader;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{stdout, Stdout};
use color_thief::{Color as ThiefColor, ColorFormat};

struct App {
    image_path: String,
    colors: Vec<(u8, u8, u8)>,
}

impl App {
    fn new() -> Self {
        Self {
            image_path: String::new(),
            colors: Vec::new(),
        }
    }

    fn to_pastel(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
        // Converter para tons pastéis aumentando o valor e reduzindo a saturação
        let r = ((r as f32 * 0.7) + 255.0 * 0.3) as u8;
        let g = ((g as f32 * 0.7) + 255.0 * 0.3) as u8;
        let b = ((b as f32 * 0.7) + 255.0 * 0.3) as u8;
        (r, g, b)
    }

    fn get_color_variations(r: u8, g: u8, b: u8) -> Vec<(u8, u8, u8)> {
        let mut variations = Vec::new();
        
        // Cor original
        variations.push((r, g, b));
        
        // Versão mais clara
        let lighter = (
            (r as f32 * 1.2).min(255.0) as u8,
            (g as f32 * 1.2).min(255.0) as u8,
            (b as f32 * 1.2).min(255.0) as u8,
        );
        variations.push(lighter);
        
        // Versão mais escura
        let darker = (
            (r as f32 * 0.8) as u8,
            (g as f32 * 0.8) as u8,
            (b as f32 * 0.8) as u8,
        );
        variations.push(darker);
        
        variations
    }

    fn extract_colors(&mut self) -> Result<()> {
        let img = ImageReader::open(&self.image_path)?.decode()?;
        let colors = color_thief::get_palette(&img.to_rgb8(), ColorFormat::Rgb, 10, 5)?;
        
        // Pega a cor principal (primeira cor)
        let main_color = colors[0];
        let main_variations = Self::get_color_variations(main_color.r, main_color.g, main_color.b);
        
        // Pega duas cores de destaque
        let accent_colors = colors[1..3]
            .iter()
            .map(|c| (c.r, c.g, c.b))
            .collect::<Vec<_>>();
        
        // Combina as variações da cor principal com as cores de destaque
        let mut all_colors = main_variations;
        all_colors.extend(accent_colors);
        
        // Converte todas as cores para tons pastéis
        self.colors = all_colors
            .iter()
            .map(|&(r, g, b)| Self::to_pastel(r, g, b))
            .collect();
        
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut app = App::new();
    let mut terminal = setup_terminal()?;

    'main: loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(f.size());

            let input = Paragraph::new(app.image_path.as_str())
                .style(Style::default())
                .block(Block::default().borders(Borders::ALL).title("Caminho da Imagem"));
            f.render_widget(input, chunks[0]);

            if !app.colors.is_empty() {
                let color_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(100 / 5); 5])
                    .split(chunks[1]);

                for (i, &(r, g, b)) in app.colors.iter().enumerate() {
                    let color_block = Block::default().style(Style::default().bg(Color::Rgb(r, g, b)));
                    f.render_widget(color_block, color_chunks[i]);
                }
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(c) => app.image_path.push(c),
                    KeyCode::Backspace => {
                        app.image_path.pop();
                    }
                    KeyCode::Enter => {
                        if let Err(e) = app.extract_colors() {
                            eprintln!("Erro ao extrair cores: {}", e);
                        }
                    }
                    KeyCode::Esc => break 'main,
                    _ => {}
                }
            }
        }
    }

    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
} 