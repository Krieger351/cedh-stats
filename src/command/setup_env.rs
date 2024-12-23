use crate::command::Executor;
use crate::store::Store;
use crate::types::commander::Commander;
use dialoguer::FuzzySelect;

pub struct SetupEnv();


impl Executor for SetupEnv {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        let commanders = store.fetch_top_commanders().await?.into_iter().collect::<Vec<Commander>>();

        let selection = FuzzySelect::new()
            .with_prompt("Which commander would you like to configure?")
            .items(&commanders)
            .interact()?;

        println!("You chose: {}", commanders[selection]);

        tokio::fs::write(".env", format!("COMMANDER=\"{}\"", commanders[selection])).await?;


        Ok(())
    }
}
