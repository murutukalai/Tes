To add a theme toggle in the header using `egui`, you can follow these steps:  

### Steps:
1. **Store the theme state**: Use `bool` or an enum (`Light` / `Dark`).
2. **Toggle the theme**: Add a button in the header to switch themes.
3. **Apply the theme**: Modify `Visuals` in `egui::Context`.

### Example:
```rust
use eframe::egui;

struct MyApp {
    dark_mode: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply the theme
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Theme:");
                if ui.button(if self.dark_mode { "🌙" } else { "☀️" }).clicked() {
                    self.dark_mode = !self.dark_mode;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Theme Toggle Example",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
```

### Explanation:
- `self.dark_mode`: Stores the current theme state.
- `ctx.set_visuals()`: Applies dark/light mode.
- A button (`🌙` / `☀️`) toggles between themes.

Let me know if you need modifications!
