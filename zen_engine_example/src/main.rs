use serde_json::json;
use zen_engine::DecisionEngine;
use zen_engine::model::DecisionContent;

#[tokio::main]
async fn main() {
    evaluate().await;
}

async fn evaluate() {
    let decision_content: DecisionContent = serde_json::from_str(include_str!("DrinkRule.json")).unwrap();
    
    let engine = DecisionEngine::default();
    let decision = engine.create_decision(decision_content.into());
  
    let result = decision.evaluate(&json!(
        {
            "person": {
                "age": 21
            }
        })).await;

    println!("Rule Result = {:?}", result);

}
