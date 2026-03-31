use ratatui::widgets::ListState;

pub struct Email {
    pub subject: String,
    pub sender: String,
    pub date: String,
    pub body: String,
    pub read: bool,
}

pub struct App {
    pub emails: Vec<Email>,
    pub list_state: ListState,
    pub quit: bool,
}

impl App {
    pub fn new(emails: Vec<Email>) -> Self {
        let mut app = Self {
            emails,
            list_state: ListState::default(),
            quit: false,
        };
        // Only select if not empty
        if !app.emails.is_empty() {
            app.list_state.select(Some(0));
            app.emails[0].read = true;
        }
        app
    }

    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.emails.len().saturating_sub(1) {
                    self.emails.len().saturating_sub(1)
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        // Mark as read when selected
        if let Some(email) = self.emails.get_mut(i) {
            email.read = true;
        }
    }

    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        // Mark as read when selected
        if let Some(email) = self.emails.get_mut(i) {
            email.read = true;
        }
    }
}
