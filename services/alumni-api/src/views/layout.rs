use maud::{DOCTYPE, Markup, html};

const STYLE: &str = "\
body { margin: 0; background: #d9ed92; color: #184e77; font-family: system-ui, sans-serif; }\
main { max-width: 56rem; margin: 0 auto; padding: 1rem; }\
table { width: 100%; border-collapse: collapse; }\
th, td { padding: .5rem; text-align: left; border-bottom: 1px solid #76c893; }\
a { color: #1e6091; margin-right: .75rem; }\
dt { font-weight: 600; }\
dd { margin: 0 0 .75rem; }\
form { display: grid; gap: .75rem; max-width: 24rem; }\
form.inline { display: inline; }\
label { display: grid; gap: .25rem; }\
input { padding: .5rem; border: 1px solid #76c893; border-radius: .375rem; }\
button { padding: .5rem 1rem; border: 0; border-radius: .375rem; background: #34a0a4; color: #fff; cursor: pointer; }\
button.danger { background: #b91c1c; }\
.error { color: #b91c1c; }";

pub fn page(title: &str, body: &Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                style { (maud::PreEscaped(STYLE)) }
            }
            body {
                main {
                    h1 { (title) }
                    (body)
                }
            }
        }
    }
}
