use chimera::control::score::ControlKey;
use chimera::phonetics::ipa::tokenize_ipa;
use chimera::phonetics::planner::{PhraseInput, PlanOptions, plan_phrases};

#[test]
fn phrase_boundaries_include_an_inhale_when_the_pause_allows_it() {
    let plan = plan_phrases(
        &[
            PhraseInput {
                segments: tokenize_ipa("a"),
                pause: Some(0.3),
                ..Default::default()
            },
            PhraseInput {
                segments: tokenize_ipa("a"),
                ..Default::default()
            },
        ],
        &PlanOptions::default(),
    );
    let inhale = plan
        .gestures
        .iter()
        .find(|g| g.label.as_ref().is_some_and(|l| l.as_str() == "inhale"))
        .expect("inhalation gesture");
    assert!(inhale.targets.get(ControlKey::BREATH).unwrap() < 0.0);
    assert!(inhale.t0 >= plan.segments[0].t1);
    assert!(inhale.t1 < plan.segments[1].t0);
}

#[test]
fn short_pauses_do_not_force_an_inhale() {
    let plan = plan_phrases(
        &[
            PhraseInput {
                segments: tokenize_ipa("a"),
                pause: Some(0.1),
                ..Default::default()
            },
            PhraseInput {
                segments: tokenize_ipa("a"),
                ..Default::default()
            },
        ],
        &PlanOptions::default(),
    );
    assert!(
        !plan
            .gestures
            .iter()
            .any(|g| g.label.as_ref().is_some_and(|l| l.as_str() == "inhale"))
    );
}
