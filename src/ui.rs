use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}
