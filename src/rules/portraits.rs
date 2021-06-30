async fn execute<S: Store>(s: S, deps: Deps) -> () {
    let data = s.register(deps).await;

    let fname = data["fname"];
    let x = data["x"];
    let y = data["y"];

    let file_handle = s.register(fname).await;
}
