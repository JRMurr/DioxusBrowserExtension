use browser_apis::tab::TabId;
use miette::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StreamTab {
    pub id: TabId,
    pub channel: String,
}

/// Get all tabs with twitch streams open
pub async fn get_twitch_tabs() -> Result<Vec<StreamTab>> {
    let tabs: Vec<StreamTab> = browser_apis::tab::query(
        serde_json::json!({ "url": ["*://www.twitch.tv/*", "*://player.twitch.tv/*"] }),
    )
    .await?
    .iter()
    .filter_map(|tab| tab.url.as_ref().and_then(|url| {
        url.path_segments().and_then(|mut paths| 
            // get the first element and make sure the rest of the iterator is empty
            match (paths.next(), paths.next()) {
                (Some(channel), None) => Some((channel, tab.id.expect("tab id should exist"))),
                _ => None,
            })
    })).map(|(channel, id)| {
                StreamTab {
                    channel: channel.to_owned(), id
                }
            }).collect();

    Ok(tabs)
}
