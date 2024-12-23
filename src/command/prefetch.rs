use crate::command::Executor;
use crate::store::Store;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tokio::time;

pub struct Prefetch {}

impl Executor for Prefetch {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        let pb = ProgressBar::new_spinner();
        pb.set_style(ProgressStyle::with_template("{spinner:.blue} {msg} ({elapsed_precise})")?);
        pb.enable_steady_tick(Duration::new(0, 500));
        pb.set_message(format!("Prefetching data for {}", store.commander()));

        let pb_spawn = pb.clone();
        let progress_bar_updater = tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(250));
            while !pb_spawn.is_finished() {
                pb_spawn.tick();
                interval.tick().await;
            }
        });

        let _ = store.fetch_top_commanders().await?;
        let _ = store.fetch_all_decks().await?;

        pb.finish_with_message("Finished");
        progress_bar_updater.await?;
        Ok(())
    }
}
