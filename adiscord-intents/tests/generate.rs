use adiscord_intents::{generate_intent_number, Intent};

#[test]
fn one_intent() {
    assert_eq!(
        generate_intent_number(vec![Intent::AutoModerationExecution]),
        2097152
    );
}

#[test]
fn two_intent() {
    assert_eq!(
        generate_intent_number(vec![Intent::Guilds, Intent::AutoModerationExecution]),
        2097153
    );
}
