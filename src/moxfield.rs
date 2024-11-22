pub struct Moxfield {}

use crate::data_types::card::Card;
use crate::data_types::deck_id::DeckId;
use crate::data_types::deck_list::DeckList;
use anyhow::Result;
use headless_chrome::protocol::cdp::Runtime::{PropertyPreview, RemoteObject};
use headless_chrome::{Browser, LaunchOptions};

impl Card {
    pub fn from_property_preview(property_preview: &PropertyPreview) -> Option<Self> {
        let str = property_preview.value.clone().unwrap().to_string();
        if str.len() > 0 {
            Some(Card::from_string(str))
        } else {
            None
        }
    }
}

impl DeckList {
    pub fn from_remote_object(remote_object: RemoteObject) -> Option<Self> {
        match remote_object.preview {
            Some(preview) =>
                Some(preview.properties.iter().filter_map(Card::from_property_preview).collect::<DeckList>()),
            _ => None
        }
    }
}

impl Moxfield {
    fn build_url(id: &DeckId) -> String {
        format!("https://www.moxfield.com/decks/{}", id)
    }

    pub async fn get_list(id: &DeckId) -> Result<Option<DeckList>> {
        let browser = Browser::new(LaunchOptions {
            headless: true,
            args: vec!["--disable-permissions".as_ref()],
            ..Default::default()
        })?;
        let tab = browser.new_tab()?;
        tab.navigate_to(&Moxfield::build_url(id))
            .expect("failed to navigate");

        tab.wait_for_xpath("//*[@id=\"subheader-more\"]")?.click()?;
        tab.press_key("Tab")?;
        tab.press_key("Enter")?;
        let result = tab.wait_for_element("body > div.modal.zoom.show.d-block > div > div > div.modal-body > div > textarea:nth-child(4)")?;

        let result = result.call_js_fn(
            "function() {return this.textContent.replaceAll(/\\d /g, '').split('\\n')}",
            vec![],
            false)?;


        Ok(DeckList::from_remote_object(result))
    }
}
