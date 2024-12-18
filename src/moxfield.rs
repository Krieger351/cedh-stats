use anyhow::Result;

use crate::types::card::Card;
use crate::types::card_list::CardList;
use crate::types::deck_id::DeckId;
use headless_chrome::protocol::cdp::Runtime::RemoteObject;
use headless_chrome::{Browser, LaunchOptions};

pub struct Moxfield {}

trait IntoCardList {
    fn into_card_list(self) -> Option<CardList>;
}
impl IntoCardList for RemoteObject {
    fn into_card_list(self) -> Option<CardList> {
        match self.preview {
            Some(preview) =>
                Some(preview.properties.iter().filter_map(Card::from_property_preview).collect::<CardList>()),
            _ => None
        }
    }
}
impl Moxfield {
    pub async fn get_list(id: &DeckId) -> Result<Option<CardList>> {
        let browser = Browser::new(LaunchOptions {
            headless: true,
            args: vec!["--disable-permissions".as_ref()],
            ..Default::default()
        })?;
        let tab = browser.new_tab()?;
        tab.navigate_to(&id.as_moxfield_url())
            .expect("failed to navigate");

        tab.wait_for_xpath("//*[@id=\"subheader-more\"]")?.click()?;
        tab.press_key("Tab")?;
        tab.press_key("Enter")?;
        let result = tab.wait_for_element("body > div.modal.zoom.show.d-block > div > div > div.modal-body > div > textarea:nth-child(4)")?;

        let result = result.call_js_fn(
            "function() {return this.textContent.replaceAll(/\\d /g, '').split('\\n')}",
            vec![],
            false)?;


        Ok(result.into_card_list())
    }
}
