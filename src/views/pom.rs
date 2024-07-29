use loco_rs::prelude::*;

use crate::models::_entities::poms;

/// Render a list view of poms.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<poms::Model>) -> Result<Response> {
    format::render().view(v, "pom/list.html", serde_json::json!({"items": items}))
}

/// Render a single pom view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &poms::Model) -> Result<Response> {
    format::render().view(v, "pom/show.html", serde_json::json!({"item": item}))
}

/// Render a pom create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "pom/create.html", serde_json::json!({}))
}

/// Render a pom edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &poms::Model) -> Result<Response> {
    format::render().view(v, "pom/edit.html", serde_json::json!({"item": item}))
}
