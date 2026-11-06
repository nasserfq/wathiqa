//! Help ▸ Connect to AI apps («ربط مع الذكاء الاصطناعي»): connects وثيقة's MCP server
//! (`wathiqa-cli mcp`, shipped beside the app) to the AI apps on this computer with one button
//! each, by writing their own settings (the `ai-connect` crate). Other MCP apps get the settings
//! to paste.
//!
//! The window speaks Arabic in the Arabic interface and English otherwise, like the About
//! window's credits. The web build has no MCP server and no settings files, so it says that
//! instead.

use crate::PdfCraftApp;
use crate::theme::{self, Tokens};
use crate::widgets;

/// Arabic in the Arabic interface, English otherwise.
fn l(ar: &'static str, en: &'static str) -> &'static str {
    if crate::i18n::current().code() == crate::i18n::MAIN_LANGUAGE { ar } else { en }
}

/// The Help menu's label for `help.connect_ai` in the current language.
pub(crate) fn menu_label() -> &'static str {
    l("ربط مع الذكاء الاصطناعي…", "Connect to AI apps…")
}

#[derive(Default)]
pub(crate) struct Runtime {
    #[cfg(not(target_arch = "wasm32"))]
    rows: Vec<(ai_connect::Client, ai_connect::State)>,
    /// The last action's result: text and whether it went wrong.
    message: Option<(String, bool)>,
}

/// The MCP server beside the running app.
#[cfg(not(target_arch = "wasm32"))]
fn server() -> Option<ai_connect::Server> {
    let exe = std::env::current_exe().ok()?;
    let command = ai_connect::find_server(&exe, &["wathiqa-cli", "pdfcraft-cli"])?;
    Some(ai_connect::Server { key: "wathiqa".into(), command, args: vec!["mcp".into()] })
}

#[cfg(not(target_arch = "wasm32"))]
fn refresh(rt: &mut Runtime) {
    rt.rows.clear();
    if let (Some(dirs), Some(server)) = (ai_connect::Dirs::system(), server()) {
        rt.rows = ai_connect::Client::ALL.into_iter().map(|c| (c, ai_connect::state(c, &dirs, &server))).collect();
    }
}

/// Open the window (Help ▸ Connect to AI apps, `help.connect_ai`).
pub(crate) fn open(app: &mut PdfCraftApp) {
    app.connect_ai.message = None;
    #[cfg(not(target_arch = "wasm32"))]
    refresh(&mut app.connect_ai);
    app.dialog = Some(crate::Dialog::ConnectAi);
}

#[cfg(not(target_arch = "wasm32"))]
fn state_label(s: &ai_connect::State) -> &'static str {
    use ai_connect::State;
    match s {
        State::NotInstalled => l("غير مثبت", "Not installed"),
        State::NotConnected => l("غير مربوط", "Not connected"),
        State::Connected => l("مربوط ✓", "Connected ✓"),
        State::Outdated => l("يحتاج تحديثًا", "Needs updating"),
        State::Unreadable(_) => l("تعذّرت قراءة إعداداته", "Its settings can't be read"),
    }
}

/// The dialog's content, with its own Close button: returns whether it should close.
pub(crate) fn body(ui: &mut egui::Ui, app: &mut PdfCraftApp) -> bool {
    ui.label(egui::RichText::new(l("ربط مع الذكاء الاصطناعي", "Connect to AI apps")).font(theme::semibold(18.0)));
    ui.add_space(8.0);
    content(ui, app);
    ui.add_space(16.0);
    let mut close = false;
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            close = widgets::pill_button(ui, l("إغلاق", "Close"), true).clicked();
        });
    });
    close
}

#[cfg(target_arch = "wasm32")]
fn content(ui: &mut egui::Ui, _app: &mut PdfCraftApp) {
    ui.label(l(
        "الربط مع تطبيقات الذكاء الاصطناعي متاح في تطبيق وثيقة على الجهاز، لا في المتصفح.",
        "Connecting AI apps is available in the desktop app, not in the web browser.",
    ));
}

#[cfg(not(target_arch = "wasm32"))]
fn content(ui: &mut egui::Ui, app: &mut PdfCraftApp) {
    use ai_connect::State;
    let t = Tokens::get(ui.ctx());
    let error = ui.visuals().error_fg_color;
    ui.label(l(
        "اربط وثيقة بتطبيق ذكاء اصطناعي، فيقرأ ملفات PDF ويعدّلها ويوقّعها ويحفظها حين تطلب منه ذلك بكلامك. اضغط «ربط» بجانب التطبيق ثم أعد تشغيله.",
        "Connect Wathiqa to an AI app so it can read, edit, sign and save your PDFs when you ask it to. Press Connect next to the app, then restart it.",
    ));
    ui.add_space(6.0);
    let Some(server) = server() else {
        ui.colored_label(
            error,
            l(
                "لم يُعثر على خادم MCP ‏(wathiqa-cli) بجانب وثيقة. نزّل وثيقة من جديد.",
                "The MCP server (wathiqa-cli) isn't next to Wathiqa. Download Wathiqa again.",
            ),
        );
        return;
    };
    if ai_connect::temporary_location(&server.command) {
        ui.colored_label(
            ui.visuals().warn_fg_color,
            l(
                "انقل وثيقة إلى مجلد التطبيقات (Applications) وافتحه من هناك أولًا، وإلا توقّف الربط بعد إغلاقه.",
                "Move Wathiqa to the Applications folder and open it from there first, or the connection stops working once it closes.",
            ),
        );
        ui.add_space(4.0);
    }
    let Some(dirs) = ai_connect::Dirs::system() else { return };
    let mut action: Option<(ai_connect::Client, bool)> = None;
    let installed: Vec<_> = app.connect_ai.rows.iter().filter(|(_, s)| *s != State::NotInstalled).cloned().collect();
    if installed.is_empty() {
        ui.label(l(
            "لم نجد على جهازك تطبيق ذكاء اصطناعي نعرفه. ثبّت أحدها، مثل Claude، ثم افتح هذه النافذة من جديد.",
            "No AI app we know of is installed here. Install one, such as Claude, then open this window again.",
        ));
    } else {
        egui::Grid::new("connect_ai_grid").num_columns(3).spacing([16.0, 8.0]).show(ui, |ui| {
            for (client, state) in &installed {
                ui.label(egui::RichText::new(client.name()).strong());
                let label = ui.label(state_label(state));
                if let State::Unreadable(why) = state {
                    label.on_hover_text(why);
                }
                let button = match state {
                    State::NotConnected => Some((l("ربط", "Connect"), false)),
                    State::Outdated => Some((l("تحديث", "Update"), false)),
                    State::Connected => Some((l("إلغاء الربط", "Disconnect"), true)),
                    State::NotInstalled | State::Unreadable(_) => None,
                };
                match button {
                    Some((text, disconnect)) => {
                        if ui.button(text).clicked() {
                            action = Some((*client, disconnect));
                        }
                    }
                    None => {
                        ui.label("");
                    }
                }
                ui.end_row();
            }
        });
    }
    let missing: Vec<&str> = app.connect_ai.rows.iter().filter(|(_, s)| *s == State::NotInstalled).map(|(c, _)| c.name()).collect();
    if !missing.is_empty() {
        ui.add_space(4.0);
        let text = format!("{} {}", l("غير مثبتة على جهازك:", "Not installed here:"), missing.join(l("، ", ", ")));
        ui.label(egui::RichText::new(text).small().color(t.text_muted));
    }
    if let Some((client, disconnect)) = action {
        let result = if disconnect {
            ai_connect::disconnect(client, &dirs, &server).map(|()| l("أُلغي الربط.", "Disconnected.").to_string())
        } else {
            ai_connect::connect(client, &dirs, &server).map(|done| match done {
                ai_connect::Done::Saved if client.needs_restart() => {
                    format!("{} {}", l("تم الربط. أعد تشغيل", "Connected. Restart"), client.name())
                }
                ai_connect::Done::Saved => l("تم الربط.", "Connected.").to_string(),
                ai_connect::Done::OpenLink(link) => {
                    app.open_url(&link);
                    format!("{} {}", l("وافق على الإضافة في", "Confirm the install in"), client.name())
                }
            })
        };
        app.connect_ai.message = Some(match result {
            Ok(text) => (text, false),
            Err(e) => {
                log::warn!("connecting {}: {e}", client.name());
                (format!("{} {e}", l("تعذّر الربط:", "Couldn't connect:")), true)
            }
        });
        refresh(&mut app.connect_ai);
    }
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(l(
            "نصيحة: اذكر wathiqa في طلبك ليستخدم التطبيقُ وثيقة لا Acrobat أو Preview، مثل: «استخدم أدوات wathiqa لفتح الملف…».",
            "Tip: say wathiqa in your request so the AI app uses Wathiqa, not Acrobat or Preview, e.g. \"Use the wathiqa tools to open the file…\".",
        ))
        .small(),
    );
    if let Some((text, bad)) = &app.connect_ai.message {
        ui.add_space(6.0);
        if *bad {
            ui.colored_label(error, text);
        } else {
            ui.label(egui::RichText::new(text).strong());
        }
    }
    ui.add_space(8.0);
    ui.separator();
    ui.label(l(
        "تطبيق آخر يدعم MCP؟ انسخ الإعداد والصقه في إعدادات خوادم MCP فيه.",
        "Another MCP app? Copy the settings and paste them into its MCP servers settings.",
    ));
    if ui.button(l("نسخ الإعداد", "Copy settings")).clicked() {
        ui.ctx().copy_text(server.snippet());
        app.connect_ai.message = Some((l("نُسخ الإعداد.", "Settings copied.").to_string(), false));
    }
}
