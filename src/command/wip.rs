use crate::command::Executor;
use crate::store::Store;

pub struct Wip();


impl Executor for Wip {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        println!("{}", store.commander());
        // let commanders = store.top_commanders().await?.into_iter().collect::<Vec<Commander>>();
        //
        // let selection = FuzzySelect::new()
        //     .with_prompt("What do you choose?")
        //     .items(&commanders)
        //     .interact()
        //     .unwrap();
        //
        // println!("You chose: {}", commanders[selection]);
        Ok(())
    }
}
