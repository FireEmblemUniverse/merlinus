use genawaiter::{
    sync::{Co, Gen},
    GeneratorState,
};

use super::{rule::RuleDB, store::Store};

enum Request {
    NewDeps(Vec<String>),
}

pub struct Dep {
    callback: Co<Request>,
}

struct State {
    rules: RuleDB,
    store: Store,
}

impl State {
    pub fn build_target(&mut self, t: Target) {
        /*
        let r = self.rules.lookup(t);

        let rule_progress = Gen::new(async move |callback| r.run(Dep { callback }));

        let output = loop {
            let needed = match rule_progress.resume_with(vec![]) {
                GeneratorState::Complete(result) => break result,
                GeneratorState::Yielded(needed) => needed,
            };
        };
        */
    }
}
