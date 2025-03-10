use crate::CACHE_DIR;
use crate::core::adb;
use crate::core::helpers::button_primary;
use crate::core::theme::Theme;
use crate::core::uad_lists::LIST_FNAME;
use crate::core::utils::{NAME, last_modified_date, open_url};
use crate::gui::{UpdateState, style, widgets::text};
use iced::widget::{Space, column, container, row};
use iced::{Alignment, Element, Length, Renderer};
use std::path::PathBuf;

#[cfg(feature = "self-update")]
use crate::core::update::SelfUpdateStatus;

#[derive(Default, Debug, Clone)]
pub struct About {}

#[derive(Debug, Clone)]
pub enum Message {
    UrlPressed(PathBuf),
    UpdateUadLists,
    DoSelfUpdate,
}

impl About {
    pub fn update(&mut self, msg: Message) {
        if let Message::UrlPressed(url) = msg {
            open_url(url);
        }
        // other events are handled by UadGui update()
    }
    pub fn view(&self, update_state: &UpdateState) -> Element<Message, Theme, Renderer> {
        let about_text = text(format!(
            "Universal Android Debloater Next Generation ({NAME}) is a free and open-source community project \naiming at simplifying the removal of pre-installed apps on any Android device."
        ));

        let descr_container = container(about_text)
            .width(Length::Fill)
            .padding(25)
            .style(style::Container::Frame);

        let date = last_modified_date(CACHE_DIR.join(LIST_FNAME));
        let uad_list_text =
            text(format!("{NAME} package list: v{}", date.format("%Y%m%d"))).width(250);
        let last_update_text = text(update_state.uad_list.to_string());
        let uad_lists_btn = button_primary("Update").on_press(Message::UpdateUadLists);

        #[cfg(feature = "self-update")]
        let self_update_btn = button_primary("Update").on_press(Message::DoSelfUpdate);

        #[cfg(feature = "self-update")]
        let uad_version_text =
            text(format!("{NAME} version: v{}", env!("CARGO_PKG_VERSION"))).width(250);

        #[cfg(feature = "self-update")]
        #[rustfmt::skip]
        let self_update_text = update_state.self_update.latest_release.as_ref().map_or_else(||
            if update_state.self_update.status == SelfUpdateStatus::Done {
                "(No update available)".to_string()
            } else {
                update_state.self_update.status.to_string()
            }, |r| if update_state.self_update.status == SelfUpdateStatus::Updating {
                update_state.self_update.status.to_string()
            } else {
                format!("({} available)", r.tag_name)
            });

        #[cfg(feature = "self-update")]
        let last_self_update_text = text(self_update_text).style(style::Text::Default);

        #[cfg(feature = "self-update")]
        let self_update_row = row![uad_version_text, self_update_btn, last_self_update_text,]
            .align_items(Alignment::Center)
            .spacing(10)
            .width(550);

        let uad_list_row = row![uad_list_text, uad_lists_btn, last_update_text,]
            .align_items(Alignment::Center)
            .spacing(10)
            .width(550);

        /*
        There's no need to fetch this info every time the view is updated,
        we could cache it in a `static` `LazyLock`.

        But what if the system updates ADB while the app is running?
        the numbers will be out of sync!

        However, the server will still be the "old" version
        until the next start
        */
        let adb_version_text = text(
            adb::ACommand::new()
                .version()
                .map_err(|e| error!("{e}"))
                .ok()
                // 1st line is the relevant one.
                // 2nd could be useful, too
                .unwrap_or_default()[0]
                // there must be some way to avoid this...
                .clone(),
        )
        .width(250);
        let adb_version_row = row![adb_version_text]
            .align_items(Alignment::Center)
            .width(550);

        let update_column = if cfg!(feature = "self-update") {
            column![uad_list_row, self_update_row, adb_version_row]
        } else {
            column![uad_list_row, adb_version_row]
        }
        .align_items(Alignment::Center)
        .spacing(10);

        let update_container = container(update_column)
            .width(Length::Fill)
            .center_x()
            .padding(10)
            .style(style::Container::Frame);

        let website_btn =
            button_primary("GitHub page").on_press(Message::UrlPressed(PathBuf::from(
                "https://github.com/Universal-Debloater-Alliance/universal-android-debloater",
            )));

        let issue_btn = button_primary("Have an issue?")
            .on_press(Message::UrlPressed(PathBuf::from(
            "https://github.com/Universal-Debloater-Alliance/universal-android-debloater/issues",
        )));

        let log_btn = button_primary("Locate the logfiles")
            .on_press(Message::UrlPressed(CACHE_DIR.to_path_buf()));

        let wiki_btn = button_primary("Wiki").on_press(Message::UrlPressed(PathBuf::from(
            "https://github.com/Universal-Debloater-Alliance/universal-android-debloater/wiki",
        )));

        let row = row![website_btn, wiki_btn, issue_btn, log_btn,].spacing(20);

        let content = column![
            Space::new(Length::Fill, Length::Shrink),
            descr_container,
            update_container,
            row,
        ]
        .width(Length::Fill)
        .spacing(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}
