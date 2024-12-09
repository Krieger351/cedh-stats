use crate::types::card::Card;
use crate::types::card_list::CardList;
use crate::types::deck_id::DeckId;
use headless_chrome::protocol::cdp::Runtime::{PropertyPreview, RemoteObject};
use headless_chrome::{Browser, LaunchOptions, Tab};
use std::sync::Arc;

impl Card {
    pub fn from_property_preview(property_preview: &PropertyPreview) -> Option<Self> {
        let str = property_preview.value.clone().unwrap().to_string();
        if str.len() > 0 {
            Some(str.parse().unwrap())
        } else {
            None
        }
    }
}


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

pub struct Moxfield {}

impl Moxfield {
    fn open_tab(id: &DeckId) -> anyhow::Result<Arc<Tab>> {
        let browser = Browser::new(LaunchOptions {
            headless: true,
            args: vec!["--disable-permissions".as_ref()],
            ..Default::default()
        })?;
        let tab = browser.new_tab()?;
        tab.navigate_to(&id.as_moxfield_url())
            .expect("failed to navigate");
        Ok(tab)
    }

    fn get_card_list(tab: Arc<Tab>) -> anyhow::Result<Option<CardList>> {
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

    pub async fn get_list(id: &DeckId) -> anyhow::Result<Option<CardList>> {
        let tab = Self::open_tab(id)?;

        let list = Self::get_card_list(tab);

        list
    }
}