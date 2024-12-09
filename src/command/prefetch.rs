use crate::command::Executor;
use crate::store::Store;

pub struct Prefetch {}

impl Executor for Prefetch {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        let _ = store.fetch_top_commanders().await?;
        let _ = store.fetch_commander_data().await?;
        // let id_win_rate = store.full_deck_id_win_rate_map().await?;
        //
        // let pb = ProgressBar::new(id_win_rate.len() as u64);
        // pb.set_style(ProgressStyle::with_template("{msg} ({pos}/{len}) [{bar:.cyan/blue}] ({elapsed_precise}/{eta_precise})")
        //     .unwrap()
        //     .progress_chars("#>-"));
        // pb.enable_steady_tick(Duration::new(1, 0));
        //
        // let pb_spawn = pb.clone();
        //
        // let progress_bar_updater = tokio::spawn(async move {
        //     let mut interval = time::interval(Duration::from_millis(500));
        //     while !pb_spawn.is_finished() {
        //         pb_spawn.tick();
        //         interval.tick().await;
        //     }
        // });
        //
        // for id in id_win_rate.keys() {
        //     pb.inc(1);
        //     pb.set_message(format!("Reading: {id}"));
        //     match store.deck_list(id).await {
        //         Ok(_) => {
        // println!("\tSuccess");
        // }
        // Err(_) => {
        // println!("\tFailure");
        // }
        // }
        // }
        // pb.finish_with_message("Finished");
        //
        // progress_bar_updater.await?;
        Ok(())
    }
}
