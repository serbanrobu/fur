use color_eyre::Result;
use fur::{Context, Env, Expr};
use rustyline::{config::Configurer, DefaultEditor, EditMode};

#[tokio::main]
async fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    rl.set_edit_mode(EditMode::Vi);

    loop {
        let line = rl.readline(">> ")?;

        match line.trim() {
            ":q" | ":quit" => break,
            line => {
                let result = handle(line).await;

                match result {
                    Ok(_) => {}
                    Err(e) => eprintln!("{e}"),
                }
            }
        }
    }

    Ok(())
}

async fn handle(line: &str) -> Result<()> {
    let e: Expr = line.parse()?;
    let t = e.infer(&Context::new())?;

    println!("{} : {}", e.eval(&Env::new()).quote(), t.quote());

    Ok(())
}
