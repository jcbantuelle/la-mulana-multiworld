use dll_syringe::{process::OwnedProcess, Syringe};
use log::debug;
use slint::Weak;
use std::process;
use std::sync::{Arc, Mutex};

use crate::ap_data::APData;
use crate::consts::LAMULANA_EXECUTABLE_NAME;
use crate::windows::seed_selector::SeedSelectorWindow;

slint::include_modules!();

pub struct LauncherWindow {
    ap_data_handle: Arc<Mutex<Option<APData>>>,
    launcher: Launcher
}

impl LauncherWindow {
    pub fn new(ap_data_handle: Arc<Mutex<Option<APData>>>) -> LauncherWindow {
        LauncherWindow {
            ap_data_handle,
            launcher: Launcher::new().inspect_err(|e| { debug!("LauncherWindow failed to initialize: {:?}", e); }).unwrap()
        }
    }

    pub async fn configure(&self, seed_selector_window: &SeedSelectorWindow) {
        let launcher = self.handle();
        let ap_data = self.ap_data();

        LauncherWindow::update_ap_data(&ap_data, &launcher);
        self.configure_select_seed(seed_selector_window).await;
        self.configure_close_window().await;
        self.configure_launch_game().await;
        self.configure_restore_files(seed_selector_window).await;
    }

    pub fn open(&self) {
        let _ = self.launcher.run().inspect_err(|e| { debug!("Launcher window failed to run: {:?}", e); });
    }

    pub fn weak(&self) -> Weak<Launcher> {
        self.launcher.as_weak()
    }

    pub fn handle(&self) -> Launcher {
        self.weak().clone().unwrap()
    }

    pub fn update_ap_data(ap_data: &APData, launcher: &Launcher) {
        launcher.set_seed_selected(ap_data.seed_selected());
        launcher.set_current_seed(ap_data.seed_name().into());
    }

    pub fn ap_data(&self) -> APData {
        let ap_data = match self.ap_data_handle.lock() {
            Ok(ap_data_lock) => {
                match ap_data_lock.clone() {
                    Some(ap_data) => {
                        Ok(ap_data)
                    },
                    None => {
                        Err("AP_DATA doesn't exist when retrieving from Launcher")
                    }
                }
            },
            Err(_) => {
                Err("Failed to acquire AP_DATA lock when retriving from Launcher")
            }
        };
        ap_data.inspect_err(|e| { debug!("{:?}", e); }).unwrap()
    }

    async fn configure_select_seed(&self, seed_selector_window: &SeedSelectorWindow) {
        let launcher = self.handle();
        let seed_selector = seed_selector_window.handle();

        self.launcher.on_select_seed(move || {
            let _ = seed_selector.show();
            let _ = launcher.hide();
        });
    }

    async fn configure_close_window(&self) {
        let launcher = self.handle();

        self.launcher.on_close_window(move || {
            let _ = launcher.hide();
        })
    }

    async fn configure_launch_game(&self) {
        self.launcher.on_launch_game(move || {
            let _ = slint::spawn_local(async move {
                let _ = tokio::spawn(async move { LauncherWindow::launch_game().await }).await.unwrap();
            });
        });
    }

    async fn configure_restore_files(&self, seed_selector_window: &SeedSelectorWindow) {
        let launcher = self.handle();
        let seed_selector = seed_selector_window.handle();
        let ap_data_handle = Arc::clone(&self.ap_data_handle);

        self.launcher.on_restore(move || {
            match ap_data_handle.lock() {
                Ok(mut ap_data_lock) => {
                    match ap_data_lock.as_mut() {
                        Some(ap_data) => {
                            if ap_data.restore_original_files().is_err() {
                                LauncherWindow::launcher_error(&launcher, "Failed to restore original files");
                            } else {
                                LauncherWindow::launcher_error(&launcher, "");
                                LauncherWindow::update_ap_data(ap_data, &launcher);
                                SeedSelectorWindow::update_ap_data(ap_data, &seed_selector);
                            }
                        },
                        None => {
                            LauncherWindow::launcher_error(&launcher, "AP_DATA doesn't exist during original file restore");
                        }
                    }
                },
                Err(_) => {
                    LauncherWindow::launcher_error(&launcher, "Failed To acquire AP_DATA lock during original file restore");
                }
            }
        });
    }

    fn launcher_error(launcher: &Launcher, error_message: &str) {
        launcher.set_error_message(error_message.to_string().into());
    }

    async fn launch_game() {
        match process::Command::new(LAMULANA_EXECUTABLE_NAME).spawn() {
            Ok(mut p) => {
                let dll = "LaMulanaMW.dll";

                let process_id = p.id();
                let target_process = OwnedProcess::from_pid(process_id).inspect_err(|e| {
                    debug!("Failed to retrieve LaMulanaWin process from launched pid: {:?}", e);
                }).unwrap();
                let syringe = Syringe::for_process(target_process);

                match syringe.inject(dll) {
                    Ok(_) => {
                        p.wait().inspect_err(|e| {
                            debug!("Launcher failed while waiting on LaMulanaWin process to exit: {:?}", e);
                        }).unwrap();
                    },
                    Err(e) => debug!("Failed to inject DLL: {}", e)
                }
            },
            Err(e) => {
                debug!("Could not launch LaMulanaWin: {:?}", e)
            }
        }
    }
}
