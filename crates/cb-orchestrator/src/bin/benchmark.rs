use cb_orchestrator::Orchestrator;
use cb_types::{AddEventRequest, CreateSessionRequest, Role};
use std::time::Instant;
fn main() {
    let mut samples = Vec::new();
    for i in 0..25 {
        let dir = std::env::temp_dir().join(format!("cb-bench-{}-{i}", std::process::id()));
        let start = Instant::now();
        let mut app = Orchestrator::new(&dir).expect("app");
        let s = app
            .create(CreateSessionRequest {
                name: "benchmark".into(),
            })
            .expect("create");
        app.add_event(
            &s.id,
            AddEventRequest {
                role: Role::User,
                source: "operator".into(),
                content: "Approve staging rehearsal".into(),
                authenticated: true,
            },
        )
        .expect("event");
        let out = app.compact(&s.id).expect("compact");
        assert!(!out.analogues.is_empty());
        samples.push(start.elapsed().as_micros());
    }
    samples.sort();
    println!("{{\"fixture\":\"25 deterministic full compositions\",\"median_us\":{},\"p95_us\":{},\"iterations\":25}}",samples[12],samples[23]);
}
