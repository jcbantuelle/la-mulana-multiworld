use log::debug;
use slint::Weak;

slint::include_modules!();

pub struct ErrorMessageWindow {
    error_message: ErrorMessage
}

impl ErrorMessageWindow {
    pub fn new() -> ErrorMessageWindow {
        ErrorMessageWindow {
            error_message: ErrorMessage::new().inspect_err(|e| { debug!("ErrorMessageWindow failed to initialize: {:?}", e); }).unwrap()
        }
    }

    pub async fn configure(&self, error_message: String) {
        self.error_message.set_error_message(error_message.into());
        self.configure_close_window().await;
    }

    pub fn open(&self) {
        let _ = self.error_message.run().inspect_err(|e| { debug!("ErrorMessage window failed to run: {:?}", e); });
    }

    pub fn weak(&self) -> Weak<ErrorMessage> {
        self.error_message.as_weak()
    }

    pub fn handle(&self) -> ErrorMessage {
        self.weak().clone().unwrap()
    }

    async fn configure_close_window(&self) {
        let error_message = self.handle();

        self.error_message.on_close_window(move || {
            let _ = error_message.hide();
        })
    }
}
