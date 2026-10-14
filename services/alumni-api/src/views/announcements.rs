use maud::{Markup, html};

use super::layout::page;
use crate::models::{Announcement, CreateAnnouncement};

pub struct Draft<'a> {
    pub input: &'a CreateAnnouncement,
    pub error: &'a str,
}

fn announcement_fields(title: &str, body: &str, author_id: u64) -> Markup {
    html! {
        label { "Title" input name="title" required value=(title); }
        label { "Body" textarea name="body" required rows="4" { (body) } }
        label { "Author id" input name="author_id" type="number" min="1" required value=(author_id); }
    }
}

fn error_message(draft: Option<&Draft>) -> Markup {
    html! {
        @if let Some(draft) = draft {
            p.error role="alert" { (draft.error) }
        }
    }
}

fn delete_form(announcement: &Announcement) -> Markup {
    html! {
        form.inline method="post" action={ "/announcements/" (announcement.id) "/delete" } {
            button.danger type="submit" { "Delete" }
        }
    }
}

pub fn index(announcements: &[Announcement], draft: Option<&Draft>) -> Markup {
    let (title, body, author_id) = draft.map_or(("", "", 0), |d| {
        (
            d.input.title.as_str(),
            d.input.body.as_str(),
            d.input.author_id,
        )
    });

    page(
        "Announcements",
        &html! {
            @if announcements.is_empty() {
                p { "No announcements yet." }
            } @else {
                table {
                    thead { tr { th { "Id" } th { "Title" } th { "Author id" } th { "Actions" } } }
                    tbody {
                        @for announcement in announcements {
                            tr {
                                td { (announcement.id) }
                                td { (announcement.title) }
                                td { (announcement.author_id) }
                                td {
                                    a href={ "/announcements/" (announcement.id) } { "Show" }
                                    a href={ "/announcements/" (announcement.id) "/edit" } { "Edit" }
                                    (delete_form(announcement))
                                }
                            }
                        }
                    }
                }
            }
            h2 { "Add announcement" }
            (error_message(draft))
            form method="post" action="/announcements" {
                (announcement_fields(title, body, author_id))
                button type="submit" { "Create announcement" }
            }
        },
    )
}

pub fn show(announcement: &Announcement) -> Markup {
    page(
        "Announcement",
        &html! {
            dl {
                dt { "Id" } dd { (announcement.id) }
                dt { "Title" } dd { (announcement.title) }
                dt { "Body" } dd { (announcement.body) }
                dt { "Author id" } dd { (announcement.author_id) }
            }
            p {
                a href={ "/announcements/" (announcement.id) "/edit" } { "Edit" }
                a href="/announcements" { "Back to announcements" }
                (delete_form(announcement))
            }
        },
    )
}

pub fn edit(announcement: &Announcement, draft: Option<&Draft>) -> Markup {
    let (title, body, author_id) = draft.map_or(
        (
            announcement.title.as_str(),
            announcement.body.as_str(),
            announcement.author_id,
        ),
        |d| {
            (
                d.input.title.as_str(),
                d.input.body.as_str(),
                d.input.author_id,
            )
        },
    );

    page(
        "Edit announcement",
        &html! {
            (error_message(draft))
            form method="post" action={ "/announcements/" (announcement.id) } {
                (announcement_fields(title, body, author_id))
                button type="submit" { "Save" }
            }
            p { a href={ "/announcements/" (announcement.id) } { "Cancel" } }
        },
    )
}

pub fn not_found() -> Markup {
    page(
        "Announcement not found",
        &html! { p { a href="/announcements" { "Back to announcements" } } },
    )
}
