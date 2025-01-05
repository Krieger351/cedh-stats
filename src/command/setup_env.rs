use crate::command::Executor;
use dialoguer::FuzzySelect;
use store::Store;
use types::commander::Commander;

pub struct SetupEnv(Option<Commander>);

impl SetupEnv {
    pub fn new(commander: Option<Commander>) -> Self {
        Self(commander)
    }
}


impl Executor for SetupEnv {
    async fn exec(self, store: &Store<'_>) -> anyhow::Result<()> {
        let commander = if self.0.is_some() { self.0.clone().unwrap() } else {
            let commanders = store.fetch_top_commanders().await?.into_iter().collect::<Vec<Commander>>();

            let selection = FuzzySelect::new()
                .with_prompt("Which commander would you like to configure?")
                .items(&commanders)
                .interact()?;

            commanders[selection].clone()
        };

        println!("You chose: {commander}", );
        tokio::fs::write(".env", format!("COMMANDER=\"{commander}\"")).await?;


        Ok(())
    }
}
