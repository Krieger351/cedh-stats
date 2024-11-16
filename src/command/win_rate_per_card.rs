use crate::store::{Store, TopDeckMethod};

pub async fn exec(store: &Store) {
    let top_lists = store.top_decks(Some(TopDeckMethod::Quartile));
    let card_list_id_map = store.card_list_id_map();

    let top_lists = top_lists.await.unwrap();
    let card_list_id_map = card_list_id_map.await.unwrap();

    for (card, list) in card_list_id_map.iter() {
        let included_win_rate = list.iter().filter_map(|x| top_lists.get(x)).collect::<Vec<&f32>>();
        let len = included_win_rate.len();
        if len > 5 {
            println!("{}: {}", card, included_win_rate.into_iter().sum::<f32>() / len as f32);
        }
    }
}