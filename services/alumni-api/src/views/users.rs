use maud::{Markup, html};

use super::layout::page;
use crate::models::{CreateUser, User};

pub struct Draft<'a> {
    pub input: &'a CreateUser,
    pub error: &'a str,
}

fn user_fields(name: &str, email: &str) -> Markup {
    html! {
        label { "Name" input name="name" required value=(name); }
        label { "Email" input name="email" type="email" required value=(email); }
    }
}

fn error_message(draft: Option<&Draft>) -> Markup {
    html! {
        @if let Some(draft) = draft {
            p.error role="alert" { (draft.error) }
        }
    }
}

fn delete_form(user: &User) -> Markup {
    html! {
        form.inline method="post" action={ "/users/" (user.id) "/delete" } {
            button.danger type="submit" { "Delete" }
        }
    }
}

pub fn index(users: &[User], draft: Option<&Draft>) -> Markup {
    let (name, email) = draft.map_or(("", ""), |d| {
        (d.input.name.as_str(), d.input.email.as_str())
    });

    page(
        "Users",
        &html! {
            @if users.is_empty() {
                p { "No users yet." }
            } @else {
                table {
                    thead { tr { th { "Id" } th { "Name" } th { "Email" } th { "Actions" } } }
                    tbody {
                        @for user in users {
                            tr {
                                td { (user.id) }
                                td { (user.name) }
                                td { (user.email) }
                                td {
                                    a href={ "/users/" (user.id) } { "Show" }
                                    a href={ "/users/" (user.id) "/edit" } { "Edit" }
                                    (delete_form(user))
                                }
                            }
                        }
                    }
                }
            }
            h2 { "Add user" }
            (error_message(draft))
            form method="post" action="/users" {
                (user_fields(name, email))
                button type="submit" { "Create user" }
            }
        },
    )
}

pub fn show(user: &User) -> Markup {
    page(
        "User",
        &html! {
            dl {
                dt { "Id" } dd { (user.id) }
                dt { "Name" } dd { (user.name) }
                dt { "Email" } dd { (user.email) }
            }
            p {
                a href={ "/users/" (user.id) "/edit" } { "Edit" }
                a href="/users" { "Back to users" }
                (delete_form(user))
            }
        },
    )
}

pub fn edit(user: &User, draft: Option<&Draft>) -> Markup {
    let (name, email) = draft.map_or((user.name.as_str(), user.email.as_str()), |d| {
        (d.input.name.as_str(), d.input.email.as_str())
    });

    page(
        "Edit user",
        &html! {
            (error_message(draft))
            form method="post" action={ "/users/" (user.id) } {
                (user_fields(name, email))
                button type="submit" { "Save" }
            }
            p { a href={ "/users/" (user.id) } { "Cancel" } }
        },
    )
}

pub fn not_found() -> Markup {
    page(
        "User not found",
        &html! { p { a href="/users" { "Back to users" } } },
    )
}
