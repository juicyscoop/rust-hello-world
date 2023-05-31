use tide::prelude::*;
use tide::Request;
use smol::io;

#[derive(Debug, Deserialize)]
struct Params {
    msg: String,
}

// async fn greet(req: Request<()>) -> tide::Result {
//     let Params { msg } = req.query()?;
//     Ok(msg.into())
// }

#[smol_potat::main]
async fn main() -> io::Result<()> {
    let mut app = tide::new();
    app.at("/").get(|req: Request<()>| async move {
        let Params { msg } = req.query()?;
        Ok(msg)
    });
    // app.at("/greet").get(greet);
    app.listen("127.0.0.1:8081").await?;
    Ok(())
}
